import { Lightning } from "@phosphor-icons/react";
import { type DragEvent, memo } from "react";
import { createRoot } from "react-dom/client";

import ActionItem from "@flow/components/ActionItem";
import type { Action } from "@flow/types";

type Props = {
  action: Action;
  selected: boolean;
  onSelect: () => void;
  onSingleClick?: (name?: string) => void;
  onDoubleClick?: (name?: string) => void;
};

const ActionComponent: React.FC<Props> = ({
  action,
  selected,
  onSelect,
  onSingleClick,
  onDoubleClick,
}) => {
  const { name, type } = action;

  const onDragStart = (
    event: DragEvent<HTMLDivElement>,
    actionName: string
  ) => {
    event.dataTransfer.setData("application/reactflow", actionName);
    event.dataTransfer.effectAllowed = "move";
    const dragPreviewContainer = document.createElement("div");
    dragPreviewContainer.style.position = "absolute";
    dragPreviewContainer.style.top = "-1000px"; // Move it offscreen to hide it

    const root = createRoot(dragPreviewContainer);
    root.render(
      <div className="flex size-12 rounded bg-secondary">
        <div
          className={`flex w-full justify-center rounded align-middle  ${type === "reader" ? "bg-[#164E63]/60" : type === "writer" ? "bg-[#635116]/60" : "bg-[#631628]/60"}`}
        >
          <Lightning className="self-center" />
        </div>
      </div>
    );

    document.body.appendChild(dragPreviewContainer);
    event.dataTransfer.setDragImage(dragPreviewContainer, 10, 10);

    // Clean up the container after the drag starts
    setTimeout(() => {
      root.unmount();
      document.body.removeChild(dragPreviewContainer);
    }, 0);
  };

  return (
    <ActionItem
      action={action}
      selected={selected}
      onMouseDown={onSelect}
      onSingleClick={onSingleClick}
      onDoubleClick={onDoubleClick}
      onDragStart={(e) => onDragStart(e, name)}
      draggable
    />
  );
};

export default memo(ActionComponent);
