import { useToast } from "@flow/features/NotificationSystem/useToast";
import { DEFAULT_PROJECT_NAME } from "@flow/global-constants";
import { useT } from "@flow/lib/i18n";
import type {
  CreateDeployment,
  DeleteDeployment,
  ExecuteDeployment,
  GetDeployments,
  UpdateDeployment,
} from "@flow/types";
import { yamlToFormData } from "@flow/utils/yamlToFormData";

import { ExecuteDeploymentInput } from "../__gen__/graphql";

import { useQueries } from "./useQueries";

export const useDeployment = () => {
  const { toast } = useToast();
  const t = useT();

  const {
    createDeploymentMutation,
    updateDeploymentMutation,
    deleteDeploymentMutation,
    executeDeploymentMutation,
    useGetDeploymentsInfiniteQuery,
  } = useQueries();

  const createDeployment = async (
    workspaceId: string,
    projectId: string,
    workflowId?: string,
    workflow?: string,
    workflowDescription?: string,
  ): Promise<CreateDeployment> => {
    const { mutateAsync, ...rest } = createDeploymentMutation;
    if (!workflowId || !workflow) {
      toast({
        title: t("Empty workflow detected"),
        description: t("You cannot create a deployment without a workflow."),
      });
      return { deployment: undefined, ...rest };
    }

    try {
      const formData = yamlToFormData(workflow, workflowId);

      const deployment = await mutateAsync({
        workspaceId,
        projectId,
        file: formData,
        description: workflowDescription,
      });
      toast({
        title: t("Deployment Created"),
        description: t("Deployment has been successfully created."),
      });
      return { deployment, ...rest };
    } catch (_err) {
      return { deployment: undefined, ...rest };
    }
  };

  const useUpdateDeployment = async (
    deploymentId: string,
    workflowId: string,
    workflowYaml: string,
    description?: string,
  ): Promise<UpdateDeployment> => {
    const { mutateAsync, ...rest } = updateDeploymentMutation;
    try {
      const deployment = await mutateAsync({
        deploymentId,
        workflowId,
        workflowYaml,
        description,
      });
      return deployment
        ? {
            deployment: {
              id: deployment?.id,
              projectId: deployment.projectId,
              projectName: deployment.project?.name ?? t(DEFAULT_PROJECT_NAME),
              workspaceId: deployment.workspaceId,
              workflowUrl: deployment.workflowUrl,
              description: deployment.description ?? undefined,
              version: deployment.version,
              createdAt: deployment.createdAt,
              updatedAt: deployment.updatedAt,
            },
            ...rest,
          }
        : { deployment: undefined, ...rest };
    } catch (_err) {
      return { deployment: undefined, ...rest };
    }
  };

  const useDeleteDeployment = async (
    deploymentId: string,
    workspaceId: string,
  ): Promise<DeleteDeployment> => {
    const { mutateAsync, ...rest } = deleteDeploymentMutation;
    try {
      const data = await mutateAsync({ deploymentId, workspaceId });
      toast({
        title: t("Successful Deletion"),
        description: t(
          "Deployment has been successfully deleted from your workspace.",
        ),
        variant: "destructive",
      });
      return { deploymentId: data.deploymentId, ...rest };
    } catch (_err) {
      return { deploymentId: undefined, ...rest };
    }
  };

  const useGetDeploymentsInfinite = (projectId?: string): GetDeployments => {
    const { data, ...rest } = useGetDeploymentsInfiniteQuery(projectId);
    return {
      pages: data?.pages,
      ...rest,
    };
  };

  const executeDeployment = async (
    input: ExecuteDeploymentInput,
  ): Promise<ExecuteDeployment> => {
    const { mutateAsync, ...rest } = executeDeploymentMutation;
    try {
      console.log("input", input);
      const job = await mutateAsync(input);
      toast({
        title: t("Deployment Executed"),
        description: t("Deployment has been successfully executed."),
      });
      return { job, ...rest };
    } catch (_err) {
      return { job: undefined, ...rest };
    }
  };

  return {
    createDeployment,
    useGetDeploymentsInfinite,
    useUpdateDeployment,
    useDeleteDeployment,
    executeDeployment,
  };
};
