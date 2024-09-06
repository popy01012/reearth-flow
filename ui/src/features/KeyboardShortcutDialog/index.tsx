import {
  Dialog,
  DialogContent,
  DialogContentSection,
  DialogContentWrapper,
  DialogHeader,
  DialogTitle,
} from "@flow/components";

import { Shortcuts } from "./components";
import useHooks from "./useHooks";

type Props = {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
};

const KeyboardShortcutDialog: React.FC<Props> = ({ isOpen, onOpenChange }) => {
  const { title, generalShortcuts, editorShortcuts, canvasShortcuts } =
    useHooks();

  return (
    <Dialog open={isOpen} onOpenChange={(o) => onOpenChange(o)}>
      <DialogContent size="2xl" position="center">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
        </DialogHeader>
        <DialogContentWrapper className="flex gap-10">
          <DialogContentSection className="flex-1">
            <p className="text-lg">{generalShortcuts.title}</p>
            <Shortcuts shortcuts={generalShortcuts.shortcuts} />
          </DialogContentSection>
          <DialogContentSection className="flex-1">
            <p className="text-lg">{editorShortcuts.title}</p>
            <Shortcuts shortcuts={editorShortcuts.shortcuts} />
          </DialogContentSection>
          <DialogContentSection className="flex-1">
            <p className="text-lg">{canvasShortcuts.title}</p>
            <Shortcuts shortcuts={canvasShortcuts.shortcuts} />
          </DialogContentSection>
        </DialogContentWrapper>
      </DialogContent>
    </Dialog>
  );
};

export default KeyboardShortcutDialog;
