export type GeneralKeys =
  | "f" // fullscreen
  | "/" // keyboard shortcuts dialog
  | "r" // reader dialog
  | "t" // transformer dialog
  | "w" // writer dialog
  | "l" // bottom panel logs
  | "p" // bottom panel preview
  | "c"; // left panel canvas navigator

export type EditorKeys =
  | "f" // fullscreen
  | "r" // reader dialog
  | "t" // transformer dialog
  | "w" // writer dialog
  | "l" // bottom panel logs
  | "p" // bottom panel preview
  | "c"; // left panel canvas navigator

export type CanvasKeys =
  | "c" // w CMD = Copy, wout CMD = left panel canvas navigator
  | "v" // paste
  | "z" // w CMD = undo, w CMD + SHIFT = redo
  | "+" // zoom in
  | "=" // zoom in (alternative - depends on keyboard layout)
  | "-" // zoom out
  | "0"; // fit view

export type PossibleKeys = GeneralKeys | EditorKeys | CanvasKeys;

type PossibleActions =
  | "zoomIn"
  | "zoomOut"
  | "fitView"
  | "copy"
  | "paste"
  | "undo"
  | "redo"
  | "fullscreen"
  | "shortcutsDialog"
  | "readerDialog"
  | "transformerDialog"
  | "writerDialog"
  | "bottomPanelLogs"
  | "bottomPanelPreview"
  | "leftPanelCanvasNavigator";

export type KeyBinding<K extends PossibleKeys = PossibleKeys> = {
  key: K;
  commandKey?: boolean;
  shiftKey?: boolean;
  altKey?: boolean;
};

export type Shortcut<K extends PossibleKeys = PossibleKeys> = {
  keyBinding?: KeyBinding<K>;
  description: string;
};

export type Shortcuts<K extends PossibleKeys = PossibleKeys> = {
  title: string;
  shortcuts: Shortcut<K>[];
};

export const GeneralKeyBindings: Partial<
  Record<PossibleActions, KeyBinding<GeneralKeys>>
> = {
  shortcutsDialog: { key: "/", commandKey: true },
};

export const EditorKeyBindings: Partial<
  Record<PossibleActions, KeyBinding<EditorKeys>>
> = {
  fullscreen: { key: "f", commandKey: true },
  readerDialog: { key: "r" },
  transformerDialog: { key: "t" },
  writerDialog: { key: "w" },
  bottomPanelLogs: { key: "l", commandKey: true },
  bottomPanelPreview: { key: "p", commandKey: true },
  leftPanelCanvasNavigator: { key: "c", commandKey: true },
};

export const CanvasKeyBindings: Partial<
  Record<PossibleActions, KeyBinding<CanvasKeys>>
> = {
  copy: { key: "c", commandKey: true },
  paste: { key: "v", commandKey: true },
  undo: { key: "z", commandKey: true },
  redo: { key: "z", commandKey: true, shiftKey: true },
  zoomIn: { key: "+" },
  zoomOut: { key: "-" },
  fitView: { key: "0", commandKey: true },
};
