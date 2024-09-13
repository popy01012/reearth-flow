package gql

import (
	"context"

	"github.com/reearth/reearth-flow/api/internal/adapter/gql/gqlmodel"
)

func (r *Resolver) Job() JobResolver {
	return &jobResolver{r}
}

type jobResolver struct{ *Resolver }

func (r *jobResolver) Deployment(ctx context.Context, obj *gqlmodel.Job) (*gqlmodel.Deployment, error) {
	return dataloaders(ctx).Deployment.Load(obj.DeploymentID)
}

func (r *jobResolver) Workspace(ctx context.Context, obj *gqlmodel.Job) (*gqlmodel.Workspace, error) {
	return dataloaders(ctx).Workspace.Load(obj.WorkspaceID)
}