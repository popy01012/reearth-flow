// Code generated by github.com/99designs/gqlgen, DO NOT EDIT.

package gqlmodel

import (
	"fmt"
	"io"
	"strconv"
	"time"

	"github.com/99designs/gqlgen/graphql"
	"github.com/reearth/reearthx/usecasex"
)

type Node interface {
	IsNode()
	GetID() ID
}

type AddMemberToWorkspaceInput struct {
	WorkspaceID ID   `json:"workspaceId"`
	UserID      ID   `json:"userId"`
	Role        Role `json:"role"`
}

type AddMemberToWorkspacePayload struct {
	Workspace *Workspace `json:"workspace"`
}

type Asset struct {
	ID          ID         `json:"id"`
	CreatedAt   time.Time  `json:"createdAt"`
	WorkspaceID ID         `json:"workspaceId"`
	Name        string     `json:"name"`
	Size        int64      `json:"size"`
	URL         string     `json:"url"`
	ContentType string     `json:"contentType"`
	Workspace   *Workspace `json:"Workspace,omitempty"`
}

func (Asset) IsNode()        {}
func (this Asset) GetID() ID { return this.ID }

type AssetConnection struct {
	Edges      []*AssetEdge `json:"edges"`
	Nodes      []*Asset     `json:"nodes"`
	PageInfo   *PageInfo    `json:"pageInfo"`
	TotalCount int          `json:"totalCount"`
}

type AssetEdge struct {
	Cursor usecasex.Cursor `json:"cursor"`
	Node   *Asset          `json:"node,omitempty"`
}

type CreateAssetInput struct {
	WorkspaceID ID             `json:"workspaceId"`
	File        graphql.Upload `json:"file"`
}

type CreateAssetPayload struct {
	Asset *Asset `json:"asset"`
}

type CreateDeploymentInput struct {
	WorkspaceID ID             `json:"workspaceId"`
	ProjectID   ID             `json:"projectId"`
	File        graphql.Upload `json:"file"`
}

type CreateProjectInput struct {
	WorkspaceID ID      `json:"workspaceId"`
	Name        *string `json:"name,omitempty"`
	Description *string `json:"description,omitempty"`
	Archived    *bool   `json:"archived,omitempty"`
}

type CreateWorkspaceInput struct {
	Name string `json:"name"`
}

type CreateWorkspacePayload struct {
	Workspace *Workspace `json:"workspace"`
}

type DeleteDeploymentInput struct {
	DeploymentID ID `json:"deploymentId"`
}

type DeleteDeploymentPayload struct {
	DeploymentID ID `json:"deploymentId"`
}

type DeleteMeInput struct {
	UserID ID `json:"userId"`
}

type DeleteMePayload struct {
	UserID ID `json:"userId"`
}

type DeleteProjectInput struct {
	ProjectID ID `json:"projectId"`
}

type DeleteProjectPayload struct {
	ProjectID ID `json:"projectId"`
}

type DeleteWorkspaceInput struct {
	WorkspaceID ID `json:"workspaceId"`
}

type DeleteWorkspacePayload struct {
	WorkspaceID ID `json:"workspaceId"`
}

type Deployment struct {
	ID          ID         `json:"id"`
	ProjectID   ID         `json:"projectId"`
	WorkspaceID ID         `json:"workspaceId"`
	WorkflowURL string     `json:"workflowUrl"`
	Version     string     `json:"version"`
	CreatedAt   time.Time  `json:"createdAt"`
	UpdatedAt   time.Time  `json:"updatedAt"`
	Project     *Project   `json:"project,omitempty"`
	Workspace   *Workspace `json:"workspace,omitempty"`
}

func (Deployment) IsNode()        {}
func (this Deployment) GetID() ID { return this.ID }

type DeploymentConnection struct {
	Edges      []*DeploymentEdge `json:"edges"`
	Nodes      []*Deployment     `json:"nodes"`
	PageInfo   *PageInfo         `json:"pageInfo"`
	TotalCount int               `json:"totalCount"`
}

type DeploymentEdge struct {
	Cursor usecasex.Cursor `json:"cursor"`
	Node   *Deployment     `json:"node,omitempty"`
}

type DeploymentPayload struct {
	Deployment *Deployment `json:"deployment"`
}

type ExecuteDeploymentInput struct {
	DeploymentID ID `json:"deploymentId"`
}

type InputGraph struct {
	ID    ID                   `json:"id"`
	Name  string               `json:"name"`
	Nodes []*InputWorkflowNode `json:"nodes"`
	Edges []*InputWorkflowEdge `json:"edges"`
}

type InputWorkflow struct {
	ID           ID            `json:"id"`
	Name         string        `json:"name"`
	EntryGraphID ID            `json:"entryGraphId"`
	With         interface{}   `json:"with,omitempty"`
	Graphs       []*InputGraph `json:"graphs"`
}

type InputWorkflowEdge struct {
	ID       ID     `json:"id"`
	To       ID     `json:"to"`
	From     ID     `json:"from"`
	FromPort string `json:"fromPort"`
	ToPort   string `json:"toPort"`
}

type InputWorkflowNode struct {
	ID         ID          `json:"id"`
	Name       string      `json:"name"`
	Type       *string     `json:"type,omitempty"`
	Action     *string     `json:"action,omitempty"`
	SubGraphID *ID         `json:"subGraphId,omitempty"`
	With       interface{} `json:"with,omitempty"`
}

type Job struct {
	ID           ID          `json:"id"`
	DeploymentID ID          `json:"deploymentId"`
	WorkspaceID  ID          `json:"workspaceId"`
	Status       JobStatus   `json:"status"`
	StartedAt    time.Time   `json:"startedAt"`
	CompletedAt  *time.Time  `json:"completedAt,omitempty"`
	Deployment   *Deployment `json:"deployment,omitempty"`
	Workspace    *Workspace  `json:"workspace,omitempty"`
}

func (Job) IsNode()        {}
func (this Job) GetID() ID { return this.ID }

type JobConnection struct {
	Edges      []*JobEdge `json:"edges"`
	Nodes      []*Job     `json:"nodes"`
	PageInfo   *PageInfo  `json:"pageInfo"`
	TotalCount int        `json:"totalCount"`
}

type JobEdge struct {
	Cursor usecasex.Cursor `json:"cursor"`
	Node   *Job            `json:"node,omitempty"`
}

type JobPayload struct {
	Job *Job `json:"job"`
}

type Me struct {
	ID            ID           `json:"id"`
	Name          string       `json:"name"`
	Email         string       `json:"email"`
	MyWorkspaceID ID           `json:"myWorkspaceId"`
	Auths         []string     `json:"auths"`
	Workspaces    []*Workspace `json:"workspaces"`
	MyWorkspace   *Workspace   `json:"myWorkspace,omitempty"`
}

type Mutation struct {
}

type PageInfo struct {
	StartCursor     *usecasex.Cursor `json:"startCursor,omitempty"`
	EndCursor       *usecasex.Cursor `json:"endCursor,omitempty"`
	HasNextPage     bool             `json:"hasNextPage"`
	HasPreviousPage bool             `json:"hasPreviousPage"`
}

type Pagination struct {
	First  *int             `json:"first,omitempty"`
	Last   *int             `json:"last,omitempty"`
	After  *usecasex.Cursor `json:"after,omitempty"`
	Before *usecasex.Cursor `json:"before,omitempty"`
}

type Project struct {
	ID                ID         `json:"id"`
	IsArchived        bool       `json:"isArchived"`
	IsBasicAuthActive bool       `json:"isBasicAuthActive"`
	BasicAuthUsername string     `json:"basicAuthUsername"`
	BasicAuthPassword string     `json:"basicAuthPassword"`
	CreatedAt         time.Time  `json:"createdAt"`
	UpdatedAt         time.Time  `json:"updatedAt"`
	Version           int        `json:"version"`
	Name              string     `json:"name"`
	Description       string     `json:"description"`
	WorkspaceID       ID         `json:"workspaceId"`
	Workspace         *Workspace `json:"workspace,omitempty"`
}

func (Project) IsNode()        {}
func (this Project) GetID() ID { return this.ID }

type ProjectConnection struct {
	Edges      []*ProjectEdge `json:"edges"`
	Nodes      []*Project     `json:"nodes"`
	PageInfo   *PageInfo      `json:"pageInfo"`
	TotalCount int            `json:"totalCount"`
}

type ProjectEdge struct {
	Cursor usecasex.Cursor `json:"cursor"`
	Node   *Project        `json:"node,omitempty"`
}

type ProjectPayload struct {
	Project *Project `json:"project"`
}

type Query struct {
}

type RemoveAssetInput struct {
	AssetID ID `json:"assetId"`
}

type RemoveAssetPayload struct {
	AssetID ID `json:"assetId"`
}

type RemoveMemberFromWorkspaceInput struct {
	WorkspaceID ID `json:"workspaceId"`
	UserID      ID `json:"userId"`
}

type RemoveMemberFromWorkspacePayload struct {
	Workspace *Workspace `json:"workspace"`
}

type RemoveMyAuthInput struct {
	Auth string `json:"auth"`
}

type RunProjectInput struct {
	ProjectID   ID             `json:"projectId"`
	WorkspaceID ID             `json:"workspaceId"`
	File        graphql.Upload `json:"file"`
}

type RunProjectPayload struct {
	ProjectID ID   `json:"projectId"`
	Started   bool `json:"started"`
}

type SignupInput struct {
	UserID      *ID     `json:"userId,omitempty"`
	WorkspaceID *ID     `json:"workspaceId,omitempty"`
	Secret      *string `json:"secret,omitempty"`
}

type SignupPayload struct {
	User      *User      `json:"user"`
	Workspace *Workspace `json:"workspace"`
}

type UpdateDeploymentInput struct {
	DeploymentID ID             `json:"deploymentId"`
	File         graphql.Upload `json:"file"`
}

type UpdateMeInput struct {
	Name                 *string `json:"name,omitempty"`
	Email                *string `json:"email,omitempty"`
	Password             *string `json:"password,omitempty"`
	PasswordConfirmation *string `json:"passwordConfirmation,omitempty"`
}

type UpdateMePayload struct {
	Me *Me `json:"me"`
}

type UpdateMemberOfWorkspaceInput struct {
	WorkspaceID ID   `json:"workspaceId"`
	UserID      ID   `json:"userId"`
	Role        Role `json:"role"`
}

type UpdateMemberOfWorkspacePayload struct {
	Workspace *Workspace `json:"workspace"`
}

type UpdateProjectInput struct {
	ProjectID         ID      `json:"projectId"`
	Name              *string `json:"name,omitempty"`
	Description       *string `json:"description,omitempty"`
	Archived          *bool   `json:"archived,omitempty"`
	IsBasicAuthActive *bool   `json:"isBasicAuthActive,omitempty"`
	BasicAuthUsername *string `json:"basicAuthUsername,omitempty"`
	BasicAuthPassword *string `json:"basicAuthPassword,omitempty"`
}

type UpdateWorkspaceInput struct {
	WorkspaceID ID     `json:"workspaceId"`
	Name        string `json:"name"`
}

type UpdateWorkspacePayload struct {
	Workspace *Workspace `json:"workspace"`
}

type User struct {
	ID    ID      `json:"id"`
	Name  string  `json:"name"`
	Email string  `json:"email"`
	Host  *string `json:"host,omitempty"`
}

func (User) IsNode()        {}
func (this User) GetID() ID { return this.ID }

type Workspace struct {
	ID       ID                 `json:"id"`
	Name     string             `json:"name"`
	Members  []*WorkspaceMember `json:"members"`
	Personal bool               `json:"personal"`
	Assets   *AssetConnection   `json:"assets"`
	Projects *ProjectConnection `json:"projects"`
}

func (Workspace) IsNode()        {}
func (this Workspace) GetID() ID { return this.ID }

type WorkspaceMember struct {
	UserID ID    `json:"userId"`
	Role   Role  `json:"role"`
	User   *User `json:"user,omitempty"`
}

type AssetSortType string

const (
	AssetSortTypeDate AssetSortType = "DATE"
	AssetSortTypeSize AssetSortType = "SIZE"
	AssetSortTypeName AssetSortType = "NAME"
)

var AllAssetSortType = []AssetSortType{
	AssetSortTypeDate,
	AssetSortTypeSize,
	AssetSortTypeName,
}

func (e AssetSortType) IsValid() bool {
	switch e {
	case AssetSortTypeDate, AssetSortTypeSize, AssetSortTypeName:
		return true
	}
	return false
}

func (e AssetSortType) String() string {
	return string(e)
}

func (e *AssetSortType) UnmarshalGQL(v interface{}) error {
	str, ok := v.(string)
	if !ok {
		return fmt.Errorf("enums must be strings")
	}

	*e = AssetSortType(str)
	if !e.IsValid() {
		return fmt.Errorf("%s is not a valid AssetSortType", str)
	}
	return nil
}

func (e AssetSortType) MarshalGQL(w io.Writer) {
	fmt.Fprint(w, strconv.Quote(e.String()))
}

type JobStatus string

const (
	JobStatusPending   JobStatus = "PENDING"
	JobStatusRunning   JobStatus = "RUNNING"
	JobStatusCompleted JobStatus = "COMPLETED"
	JobStatusFailed    JobStatus = "FAILED"
)

var AllJobStatus = []JobStatus{
	JobStatusPending,
	JobStatusRunning,
	JobStatusCompleted,
	JobStatusFailed,
}

func (e JobStatus) IsValid() bool {
	switch e {
	case JobStatusPending, JobStatusRunning, JobStatusCompleted, JobStatusFailed:
		return true
	}
	return false
}

func (e JobStatus) String() string {
	return string(e)
}

func (e *JobStatus) UnmarshalGQL(v interface{}) error {
	str, ok := v.(string)
	if !ok {
		return fmt.Errorf("enums must be strings")
	}

	*e = JobStatus(str)
	if !e.IsValid() {
		return fmt.Errorf("%s is not a valid JobStatus", str)
	}
	return nil
}

func (e JobStatus) MarshalGQL(w io.Writer) {
	fmt.Fprint(w, strconv.Quote(e.String()))
}

type NodeType string

const (
	NodeTypeAsset     NodeType = "ASSET"
	NodeTypeProject   NodeType = "PROJECT"
	NodeTypeUser      NodeType = "USER"
	NodeTypeWorkspace NodeType = "WORKSPACE"
)

var AllNodeType = []NodeType{
	NodeTypeAsset,
	NodeTypeProject,
	NodeTypeUser,
	NodeTypeWorkspace,
}

func (e NodeType) IsValid() bool {
	switch e {
	case NodeTypeAsset, NodeTypeProject, NodeTypeUser, NodeTypeWorkspace:
		return true
	}
	return false
}

func (e NodeType) String() string {
	return string(e)
}

func (e *NodeType) UnmarshalGQL(v interface{}) error {
	str, ok := v.(string)
	if !ok {
		return fmt.Errorf("enums must be strings")
	}

	*e = NodeType(str)
	if !e.IsValid() {
		return fmt.Errorf("%s is not a valid NodeType", str)
	}
	return nil
}

func (e NodeType) MarshalGQL(w io.Writer) {
	fmt.Fprint(w, strconv.Quote(e.String()))
}

type Role string

const (
	RoleReader     Role = "READER"
	RoleWriter     Role = "WRITER"
	RoleMaintainer Role = "MAINTAINER"
	RoleOwner      Role = "OWNER"
)

var AllRole = []Role{
	RoleReader,
	RoleWriter,
	RoleMaintainer,
	RoleOwner,
}

func (e Role) IsValid() bool {
	switch e {
	case RoleReader, RoleWriter, RoleMaintainer, RoleOwner:
		return true
	}
	return false
}

func (e Role) String() string {
	return string(e)
}

func (e *Role) UnmarshalGQL(v interface{}) error {
	str, ok := v.(string)
	if !ok {
		return fmt.Errorf("enums must be strings")
	}

	*e = Role(str)
	if !e.IsValid() {
		return fmt.Errorf("%s is not a valid Role", str)
	}
	return nil
}

func (e Role) MarshalGQL(w io.Writer) {
	fmt.Fprint(w, strconv.Quote(e.String()))
}
