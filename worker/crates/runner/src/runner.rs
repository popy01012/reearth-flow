use std::sync::Arc;

use reearth_flow_action_log::factory::LoggerFactory;
use reearth_flow_runtime::shutdown;
use reearth_flow_storage::resolve::StorageResolver;
use reearth_flow_types::workflow::Workflow;
use tracing::Level;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

use crate::orchestrator::Orchestrator;

pub struct Runner;

impl Runner {
    pub fn run(
        job_id: String,
        workflow: Workflow,
        logger_factory: Arc<LoggerFactory>,
        storage_resolver: Arc<StorageResolver>,
    ) {
        setup_logging_and_tracing();
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(30)
            .enable_all()
            .build()
            .unwrap();

        let (_shutdown_sender, shutdown_receiver) = shutdown::new(&runtime);
        let runtime = Arc::new(runtime);
        let orchestraotr = Orchestrator::new(runtime.clone());
        runtime.block_on(async move {
            orchestraotr
                .run_all(
                    job_id,
                    workflow,
                    shutdown_receiver,
                    logger_factory,
                    storage_resolver,
                )
                .await
                .unwrap()
        });
    }
}

pub fn setup_logging_and_tracing() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(Level::INFO.into())
        .from_env_lossy()
        .add_directive("opendal=error".parse().unwrap());
    let registry = tracing_subscriber::registry().with(env_filter);
    let event_format = tracing_subscriber::fmt::format()
        .with_target(true)
        .with_timer(UtcTime::new(
            time::format_description::parse(
                "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z",
            )
            .expect("Time format invalid."),
        ));
    let _ = registry
        .with(
            tracing_subscriber::fmt::layer()
                .event_format(event_format)
                .with_ansi(true),
        )
        .try_init();
}