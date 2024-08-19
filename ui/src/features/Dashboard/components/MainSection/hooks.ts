import { useNavigate } from "@tanstack/react-router";
import { useEffect, useMemo, useRef, useState } from "react";

import { useProject } from "@flow/lib/gql";
import { useCurrentProject, useDialogType } from "@flow/stores";
import { Project, Workspace } from "@flow/types";

export default ({ workspace }: { workspace: Workspace }) => {
  const ref = useRef<HTMLDivElement>(null);

  const [currentProject, setCurrentProject] = useCurrentProject();

  const navigate = useNavigate({ from: "/workspace/$workspaceId" });
  const { useGetWorkspaceProjectsInfinite, deleteProject, updateProject } =
    useProject();
  const [, setDialogType] = useDialogType();
  const { pages, hasNextPage, isFetching, fetchNextPage } =
    useGetWorkspaceProjectsInfinite(workspace.id);

  const [showError, setShowError] = useState(false);
  const [buttonDisabled, setButtonDisabled] = useState(false);
  const [projectToBeDeleted, setProjectToBeDeleted] = useState<
    string | undefined
  >(undefined);
  const [editProject, setEditProject] = useState<undefined | Project>(
    undefined
  );

  const handleProjectSelect = (p: Project) => {
    setCurrentProject(p);
    navigate({ to: `/workspace/${workspace.id}/project/${p.id}` });
  };

  const handleDeleteProject = async (id: string) => {
    setProjectToBeDeleted(undefined);
    await deleteProject(id, workspace.id);
  };

  const handleUpdateValue = (key: "name" | "description", value: string) => {
    if (!editProject) return;
    setEditProject({ ...editProject, [key]: value });
  };

  const handleUpdateProject = async () => {
    if (!editProject || !editProject.name) return;
    setShowError(false);
    setButtonDisabled(true);

    const { project } = await updateProject({
      projectId: editProject.id,
      name: editProject.name,
      description: editProject.description,
    });

    if (!project) {
      setShowError(true);
      setButtonDisabled(false);
      return;
    }

    setButtonDisabled(false);
    setShowError(false);
    setEditProject(undefined);
    return;
  };

  const projects: Project[] | undefined = useMemo(
    () =>
      pages?.reduce((projects, page) => {
        if (page?.projects) {
          projects.push(...page.projects);
        }
        return projects;
      }, [] as Project[]),
    [pages]
  );

  // Auto fills the page
  useEffect(() => {
    if (
      ref.current &&
      ref.current?.scrollHeight <= document.documentElement.clientHeight &&
      hasNextPage &&
      !isFetching
    ) {
      fetchNextPage();
    }
  }, [isFetching, hasNextPage, ref, fetchNextPage]);

  // Loads more projects as scroll reaches the bottom
  useEffect(() => {
    const handleScroll = () => {
      if (
        window.innerHeight + document.documentElement.scrollTop + 5 >=
          document.documentElement.scrollHeight &&
        !isFetching &&
        hasNextPage
      ) {
        fetchNextPage();
      }
    };
    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, [isFetching, fetchNextPage, hasNextPage]);

  return {
    projects,
    ref,
    currentProject,
    projectToBeDeleted,
    editProject,
    showError,
    buttonDisabled,
    setDialogType,
    setEditProject,
    setProjectToBeDeleted,
    handleProjectSelect,
    handleDeleteProject,
    handleUpdateValue,
    handleUpdateProject,
  };
};