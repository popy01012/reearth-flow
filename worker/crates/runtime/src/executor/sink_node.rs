use std::{
    borrow::Cow,
    mem::swap,
    sync::Arc,
    time::{Duration, Instant},
    usize,
};

use crossbeam::channel::{Receiver, Sender, TryRecvError};
use petgraph::graph::NodeIndex;

use crate::{
    builder_dag::NodeKind,
    error_manager::ErrorManager,
    errors::ExecutionError,
    event::Event,
    executor_operation::{ExecutorContext, ExecutorOperation, NodeContext},
    node::{NodeHandle, Sink},
};

use super::execution_dag::ExecutionDag;
use super::{name::Name, receiver_loop::ReceiverLoop};

const DEFAULT_FLUSH_INTERVAL: Duration = Duration::from_millis(20);

struct FlushScheduler {
    receiver: Receiver<Duration>,
    sender: Sender<()>,
    next_schedule: Option<Duration>,
    next_schedule_from: Instant,
    loop_interval: Duration,
}

impl FlushScheduler {
    fn run(&mut self) {
        loop {
            // If we have nothing scheduled, block until we get a schedule
            let mut next_schedule = if self.next_schedule.is_none() {
                match self.receiver.recv() {
                    Ok(v) => Some(v),
                    Err(_) => return,
                }
            } else {
                None
            };

            // Keep postponing the schedule while there are messages
            while let Some(sched) = match self.receiver.try_recv() {
                Ok(next) => Some(next),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => return,
            } {
                next_schedule = Some(sched);
            }

            if let Some(next) = next_schedule {
                self.next_schedule = Some(next);
                self.next_schedule_from = Instant::now();
            }

            let Some(schedule) = self.next_schedule else {
                continue;
            };

            let elapsed = self.next_schedule_from.elapsed();
            if elapsed >= schedule {
                let Ok(_) = self.sender.send(()) else {
                    return;
                };
                self.next_schedule = None;
            } else {
                let time_to_next_schedule = schedule - elapsed;
                std::thread::sleep(self.loop_interval.min(time_to_next_schedule));
            }
        }
    }
}

/// A sink in the execution DAG.
#[derive(Debug)]
pub struct SinkNode {
    /// Node handle in description DAG.
    node_handle: NodeHandle,
    /// Input node handles.
    node_handles: Vec<NodeHandle>,
    /// Input data channels.
    receivers: Vec<Receiver<ExecutorOperation>>,
    /// The sink.
    sink: Box<dyn Sink>,
    max_flush_interval: Duration,
    ops_since_flush: u64,
    flush_scheduler_sender: Sender<Duration>,
    should_flush_receiver: Receiver<()>,
    event_sender: tokio::sync::broadcast::Sender<Event>,
    pub error_manager: Arc<ErrorManager>,
}

impl SinkNode {
    pub fn new(ctx: NodeContext, dag: &mut ExecutionDag, node_index: NodeIndex) -> Self {
        let node = dag.node_weight_mut(node_index);
        let Some(kind) = node.kind.take() else {
            panic!("Must pass in a node")
        };
        let node_handle = node.handle.clone();
        let NodeKind::Sink(sink) = kind else {
            panic!("Must pass in a sink node");
        };

        let (node_handles, receivers) = dag.collect_receivers(node_index);

        let max_flush_interval = sink
            .max_batch_duration_ms()
            .map_or(DEFAULT_FLUSH_INTERVAL, Duration::from_millis);
        let (schedule_sender, schedule_receiver) = crossbeam::channel::bounded(10);
        let (should_flush_sender, should_flush_receiver) = crossbeam::channel::bounded(0);
        let mut scheduler = FlushScheduler {
            receiver: schedule_receiver,
            sender: should_flush_sender,
            next_schedule: None,
            next_schedule_from: Instant::now(),
            loop_interval: max_flush_interval / 5,
        };
        sink.initialize(ctx);
        std::thread::spawn(move || scheduler.run());
        Self {
            node_handle,
            node_handles,
            receivers,
            sink,
            flush_scheduler_sender: schedule_sender,
            should_flush_receiver,
            event_sender: dag.event_hub().sender.clone(),
            max_flush_interval,
            ops_since_flush: 0,
            error_manager: dag.error_manager().clone(),
        }
    }

    pub fn handle(&self) -> &NodeHandle {
        &self.node_handle
    }

    fn flush(&mut self) -> Result<(), ExecutionError> {
        self.sink
            .flush_batch()
            .map_err(|e| ExecutionError::CannotReceiveFromChannel(format!("{:?}", e)))?;
        self.ops_since_flush = 0;
        self.flush_scheduler_sender
            .send(self.max_flush_interval)
            .unwrap();
        let _ = self.event_sender.send(Event::SinkFlushed {
            node: self.node_handle.clone(),
        });
        Ok(())
    }
}

impl Name for SinkNode {
    fn name(&self) -> Cow<str> {
        Cow::Owned(self.node_handle.to_string())
    }
}

struct Select<'a> {
    op_receivers: &'a [Receiver<ExecutorOperation>],
    flush_receiver: &'a Receiver<()>,
    inner: crossbeam::channel::Select<'a>,
    flush_idx: usize,
}

enum ReceiverMsg {
    Op(usize, ExecutorOperation),
    Flush,
}

impl<'a> Select<'a> {
    fn new(
        op_receivers: &'a [Receiver<ExecutorOperation>],
        flush_receiver: &'a Receiver<()>,
    ) -> Self {
        let mut inner = crossbeam::channel::Select::new();
        for recv in op_receivers {
            let _ = inner.recv(recv);
        }
        let flush_idx = inner.recv(flush_receiver);
        Self {
            inner,
            flush_idx,
            op_receivers,
            flush_receiver,
        }
    }

    fn remove(&mut self, idx: usize) {
        self.inner.remove(idx);
    }

    fn recv(&mut self) -> Result<ReceiverMsg, ExecutionError> {
        let msg = self.inner.select();
        let index = msg.index();
        let res = if index == self.flush_idx {
            msg.recv(self.flush_receiver).map(|_| ReceiverMsg::Flush)
        } else {
            msg.recv(&self.op_receivers[index])
                .map(|op| ReceiverMsg::Op(index, op))
        };
        res.map_err(|e| ExecutionError::CannotReceiveFromChannel(format!("{:?}", e)))
    }
}

impl ReceiverLoop for SinkNode {
    fn receivers(&mut self) -> Vec<Receiver<ExecutorOperation>> {
        let mut result = vec![];
        swap(&mut self.receivers, &mut result);
        result
    }

    fn receiver_name(&self, index: usize) -> Cow<str> {
        Cow::Owned(self.node_handles[index].to_string())
    }

    fn receiver_loop(mut self) -> Result<(), ExecutionError> {
        // This is just copied from ReceiverLoop
        let receivers = self.receivers();
        let should_flush_receiver = {
            // Take the receiver. This is fine, as long as we exclusively use the
            // returned receiver and not the one in `self`.
            let (_, mut tmp_recv) = crossbeam::channel::bounded(0);
            swap(&mut self.should_flush_receiver, &mut tmp_recv);
            tmp_recv
        };
        let mut is_terminated = vec![false; receivers.len()];
        self.flush_scheduler_sender
            .send(self.max_flush_interval)
            .unwrap();
        let mut sel = Select::new(&receivers, &should_flush_receiver);
        loop {
            let ReceiverMsg::Op(index, op) = sel.recv()? else {
                self.flush()?;
                continue;
            };

            match op {
                ExecutorOperation::Op { ctx } => {
                    self.on_op(ctx)?;
                }
                ExecutorOperation::Terminate { ctx } => {
                    is_terminated[index] = true;
                    sel.remove(index);
                    if is_terminated.iter().all(|value| *value) {
                        self.on_terminate(ctx)?;
                        return Ok(());
                    }
                }
            }
        }
    }

    fn on_op(&mut self, ctx: ExecutorContext) -> Result<(), ExecutionError> {
        self.sink
            .process(ctx)
            .map_err(|e| ExecutionError::CannotReceiveFromChannel(format!("{:?}", e)))
    }

    fn on_terminate(&mut self, ctx: NodeContext) -> Result<(), ExecutionError> {
        self.sink
            .finish(ctx)
            .map_err(|e| ExecutionError::CannotReceiveFromChannel(format!("{:?}", e)))
    }
}
