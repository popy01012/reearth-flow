import {
  Handle,
  HandleProps,
  ReactFlowState,
  getConnectedEdges,
  useNodeId,
  useStore,
} from "@xyflow/react";
import { memo, useMemo } from "react";

const selector = (s: ReactFlowState) => ({
  nodeInternals: s.nodeLookup,
  edges: s.edges,
});

type Props = Omit<HandleProps, "isConnectable"> & {
  className?: string;
  isConnectable?: number;
};

const CustomHandle: React.FC<Props> = ({ className, ...props }) => {
  const { nodeInternals, edges } = useStore(selector);
  const nodeId = useNodeId();

  const isHandleConnectable = useMemo(() => {
    if (nodeId && props.isConnectable) {
      const node = nodeInternals.get(nodeId);
      if (!node) return false;
      const connectedEdges = getConnectedEdges([node], edges);

      return connectedEdges.length < props.isConnectable;
    }
  }, [nodeInternals, edges, nodeId, props.isConnectable]);

  return (
    <Handle
      {...props}
      isConnectable={isHandleConnectable}
      className={`bg-transparent border-none h-full hover:bg-zinc-600/40 ${className}`}
    />
  );
};

export default memo(CustomHandle);
