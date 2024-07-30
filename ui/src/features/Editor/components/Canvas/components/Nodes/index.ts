import { NodeTypes } from "@xyflow/react";

import BatchNode from "./BatchNode";
import EntranceNode from "./EntranceNode";
import ExitNode from "./ExitNode";
import NoteNode from "./NoteNode";
import ReaderNode from "./ReaderNode";
import SubworkflowNode from "./SubworkflowNode";
import TransformerNode from "./TransformerNode";
import WriterNode from "./WriterNode";

export const nodeTypes: NodeTypes = {
  writer: WriterNode,
  reader: ReaderNode,
  transformer: TransformerNode,
  batch: BatchNode,
  note: NoteNode,
  subworkflow: SubworkflowNode,
  entrance: EntranceNode,
  exit: ExitNode,
};
