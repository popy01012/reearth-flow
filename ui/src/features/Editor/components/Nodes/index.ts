import { NodeTypes } from "@xyflow/react";

import BatchNode from "./BatchNode";
import NoteNode from "./NoteNode";
import ReaderNode from "./ReaderNode";
import TransformerNode from "./TransformerNode";
import WriterNode from "./WriterNode";

export const nodeTypes: NodeTypes = {
  writer: WriterNode,
  reader: ReaderNode,
  transformer: TransformerNode,
  batch: BatchNode,
  note: NoteNode,
};
