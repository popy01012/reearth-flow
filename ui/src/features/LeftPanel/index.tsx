import { TransformIcon } from "@radix-ui/react-icons";
import { Link, useParams } from "@tanstack/react-router";
import { Database, Disc, Zap, AlignCenter, Cog, Search } from "lucide-react";
import { useState } from "react";

import { FlowLogo, Tree, TreeDataItem, IconButton } from "@flow/components";
import { useT } from "@flow/providers";
import { useDialogType } from "@flow/stores";
import { Workflow } from "@flow/types";

// import HomeMenu from "./components/HomeMenu";

type Tab = "navigator" | "assets";

type Props = {
  data?: Workflow;
};

const LeftPanel: React.FC<Props> = ({ data }) => {
  const t = useT();
  const { workspaceId } = useParams({ strict: false });
  const [isPanelOpen, setPanelOpen] = useState(false);
  const [selectedTab, setSelectedTab] = useState<Tab>("navigator");

  const [_content, setContent] = useState("Admin Page");

  const [, setDialogType] = useDialogType();

  const treeContent: TreeDataItem[] = [
    ...(data?.nodes
      ?.filter(n => n.type === "reader")
      .map(n => ({
        id: n.id,
        name: n.data.name ?? "untitled",
        icon: Database,
      })) ?? []),
    ...(data?.nodes
      ?.filter(n => n.type === "writer")
      .map(n => ({
        id: n.id,
        name: n.data.name ?? "untitled",
        icon: Disc,
      })) ?? []),
    {
      id: "transformer",
      name: t("Transformers"),
      icon: Zap,
      children: data?.nodes
        ?.filter(n => n.type === "transformer")
        .map(n => ({
          id: n.id,
          name: n.data.name ?? "untitled",
          // icon: Disc,
        })),
    },
  ];

  const tabContents: { id: Tab; title: string; component: React.ReactNode }[] = [
    {
      id: "navigator",
      title: t("Canvas Navigation"),
      component: data && (
        <Tree
          data={treeContent}
          className="flex-shrink-0 w-full px-1 text-zinc-300 rounded truncate"
          // initialSlelectedItemId="1"
          onSelectChange={item => setContent(item?.name ?? "")}
          // folderIcon={Folder}
          // itemIcon={Database}
        />
      ),
    },
    {
      id: "assets",
      title: t("Transformer list"),
      component: (
        <div className="flex flex-col gap-2 px-1">
          <div className="flex gap-2 items-center">
            <Zap className="w-[15px] h-[15px] stroke-1" />
            <p className="text-sm font-extralight">Transformer</p>
          </div>
          <div className="flex gap-2 items-center">
            <Zap className="w-[15px] h-[15px] stroke-1" />
            <p className="text-sm font-extralight">Transformer</p>
          </div>
          <div className="flex gap-2 items-center">
            <Zap className="w-[15px] h-[15px] stroke-1" />
            <p className="text-sm font-extralight">Transformer</p>
          </div>
          <div className="flex gap-2 items-center">
            <Zap className="w-[15px] h-[15px] stroke-1" />
            <p className="text-sm font-extralight">Transformer</p>
          </div>
        </div>
      ),
    },
  ];

  const handleTabChange = (tab: Tab) => {
    if (tab === selectedTab) {
      setPanelOpen(!isPanelOpen);
    } else {
      setSelectedTab(tab);
      if (!isPanelOpen) {
        setPanelOpen(true);
      }
    }
  };

  return (
    <>
      <div
        className="absolute left-12 z-10 flex flex-1 flex-col gap-3 h-full w-[300px] bg-zinc-900 border-r border-zinc-700 transition-all overflow-auto"
        style={{
          transform: `translateX(${isPanelOpen ? "0" : "-100%"})`,
          transitionDuration: isPanelOpen ? "500ms" : "300ms",
          transitionProperty: "transform",
          transitionTimingFunction: "cubic-bezier(0.4, 0, 0.2, 1)",
        }}>
        <div className="flex flex-col gap-2 px-4 py-2 border-b border-zinc-700/50">
          <p className="text-lg font-extralight">
            {tabContents?.find(tc => tc.id === selectedTab)?.title}
          </p>
        </div>
        <div className="flex flex-col gap-2 overflow-auto">
          {/* {content.title && <p className="text-md">{content.title}</p>} */}
          {tabContents?.find(tc => tc.id === selectedTab)?.component}
        </div>
      </div>
      <aside className="relative hidden h-full w-12 z-10 flex-col border-r border-zinc-700 bg-zinc-800 sm:flex">
        <nav className="flex flex-col items-center gap-4 px-2 sm:py-4">
          <Link
            to={`/workspace/${workspaceId}`}
            className="group flex h-9 w-9 shrink-0 items-center justify-center gap-2 rounded-full border border-red-900 text-lg font-semibold text-primary-foreground md:h-8 md:w-8 md:text-base hover:bg-red-900">
            <FlowLogo className="h-4 w-4 transition-all group-hover:scale-110" />
            <span className="sr-only">{t("Dashboard")}</span>
          </Link>
          <IconButton
            className={`flex h-9 w-9 items-center justify-center rounded-lg text-zinc-500 transition-colors hover:text-zinc-300 md:h-8 md:w-8 ${selectedTab === "navigator" && "bg-zinc-700 text-zinc-300"}`}
            icon={<AlignCenter className="h-5 w-5" />}
            onClick={() => handleTabChange("navigator")}
          />
          <IconButton
            className={`flex h-9 w-9 items-center justify-center rounded-lg text-zinc-500 transition-colors hover:text-zinc-300 md:h-8 md:w-8 ${selectedTab === "assets" && "bg-zinc-700 text-zinc-300"}`}
            icon={<TransformIcon className="h-5 w-5 stroke-1" />}
            onClick={() => handleTabChange("assets")}
          />
        </nav>
        <nav className="mt-auto flex flex-col items-center gap-4 px-2 py-2">
          <Search
            className="stroke-1 text-zinc-400 cursor-pointer hover:text-zinc-300"
            onClick={() => setDialogType("canvas-search")}
          />
          <Link
            className="flex h-9 w-9 items-center justify-center rounded-lg text-zinc-400 transition-colors hover:text-zinc-300 md:h-8 md:w-8"
            to={"/settings"}>
            <Cog className="h-5 w-5" />
            <span className="sr-only">Settings</span>
          </Link>
        </nav>
      </aside>
    </>
  );
};

export default LeftPanel;
