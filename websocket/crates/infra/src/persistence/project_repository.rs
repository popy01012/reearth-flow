use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;

use crate::persistence::gcs::gcs_client::GcsClient;
use crate::persistence::redis::redis_client::RedisClient;

use crate::persistence::local_storage::LocalClient;
use flow_websocket_domain::project::{Project, ProjectEditingSession};
use flow_websocket_domain::repository::{
    ProjectEditingSessionRepository, ProjectRepository, ProjectSnapshotRepository,
};
use flow_websocket_domain::snapshot::ProjectSnapshot;

pub struct ProjectRedisRepository {
    redis_client: Arc<RedisClient>,
}

impl ProjectRedisRepository {
    pub fn new(redis_client: Arc<RedisClient>) -> Self {
        Self { redis_client }
    }
}

#[async_trait]
impl ProjectRepository for ProjectRedisRepository {
    async fn get_project(&self, project_id: &str) -> Result<Option<Project>, Box<dyn Error>> {
        let key = format!("project:{}", project_id);
        self.redis_client.get(&key).await
    }
}

#[async_trait]
impl ProjectEditingSessionRepository for ProjectRedisRepository {
    async fn create_session(&self, session: ProjectEditingSession) -> Result<(), Box<dyn Error>> {
        let key = format!("session:{}", session.session_id.as_ref().unwrap());
        self.redis_client.set(key, &session).await?;
        Ok(())
    }

    async fn get_active_session(
        &self,
        project_id: &str,
    ) -> Result<Option<ProjectEditingSession>, Box<dyn Error>> {
        let key = format!("project:{}:active_session", project_id);
        self.redis_client.get(&key).await
    }

    async fn update_session(&self, session: ProjectEditingSession) -> Result<(), Box<dyn Error>> {
        let key = format!("session:{}", session.session_id.as_ref().unwrap());
        self.redis_client.set(key, &session).await?;
        Ok(())
    }
}

pub struct ProjectGcsRepository {
    client: GcsClient,
}

impl ProjectGcsRepository {
    fn _new(client: GcsClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ProjectSnapshotRepository for ProjectGcsRepository {
    async fn create_snapshot(&self, snapshot: ProjectSnapshot) -> Result<(), Box<dyn Error>> {
        let path = format!("snapshot/{}", snapshot.metadata.id);
        self.client.upload(path, &snapshot).await?;
        Ok(())
    }

    async fn get_latest_snapshot(
        &self,
        project_id: &str,
    ) -> Result<Option<ProjectSnapshot>, Box<dyn Error>> {
        let path = format!("snapshot/{}:latest_snapshot", project_id);
        self.client.download(path).await
    }

    async fn get_latest_snapshot_state(&self, project_id: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let path = format!("snapshot/{}:latest_snapshot_state", project_id);
        self.client.download(path).await
    }
}

pub struct ProjectLocalRepository {
    client: Arc<LocalClient>,
}

impl ProjectLocalRepository {
    pub async fn new(base_path: PathBuf) -> io::Result<Self> {
        Ok(Self {
            client: Arc::new(LocalClient::new(base_path).await?),
        })
    }
}

#[async_trait]
impl ProjectRepository for ProjectLocalRepository {
    async fn get_project(
        &self,
        project_id: &str,
    ) -> Result<Option<Project>, Box<dyn std::error::Error>> {
        let path = format!("projects/{}", project_id);
        match self.client.download::<Project>(path).await {
            Ok(project) => Ok(Some(project)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }
}

#[async_trait]
impl ProjectSnapshotRepository for ProjectLocalRepository {
    async fn create_snapshot(
        &self,
        snapshot: ProjectSnapshot,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = format!("snapshots/{}", snapshot.metadata.id);
        self.client.upload(path, &snapshot).await?;

        // Update latest snapshot
        let latest_path = format!("latest_snapshots/{}", snapshot.metadata.project_id);
        self.client.upload(latest_path, &snapshot).await?;

        Ok(())
    }

    async fn get_latest_snapshot(
        &self,
        project_id: &str,
    ) -> Result<Option<ProjectSnapshot>, Box<dyn std::error::Error>> {
        let path = format!("latest_snapshots/{}", project_id);
        match self.client.download::<ProjectSnapshot>(path).await {
            Ok(snapshot) => Ok(Some(snapshot)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_latest_snapshot_state(
        &self,
        project_id: &str,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let path = format!("latest_snapshots/{}", project_id);
        match self.client.download::<ProjectSnapshot>(path).await {
            Ok(snapshot) => Ok(serde_json::to_vec(&snapshot)?),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(vec![]),
            Err(e) => Err(Box::new(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flow_websocket_domain::snapshot::{ObjectDelete, ObjectTenant, ProjectMetadata};
    use tempfile::TempDir;
    use tokio::test;

    async fn setup() -> Result<(TempDir, ProjectLocalRepository), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let repo = ProjectLocalRepository::new(temp_dir.path().to_path_buf()).await?;
        Ok((temp_dir, repo))
    }

    #[test]
    async fn test_get_project_non_existent() -> Result<(), Box<dyn std::error::Error>> {
        let (_temp_dir, repo) = setup().await?;
        let project_id = "non_existent_project";
        assert!(repo.get_project(project_id).await?.is_none());
        Ok(())
    }

    #[test]
    async fn test_get_project_existing() -> Result<(), Box<dyn std::error::Error>> {
        let (_temp_dir, repo) = setup().await?;
        let project_id = "test_project";
        let project = Project {
            id: project_id.to_string(),
            workspace_id: "test_workspace".to_string(),
        };

        repo.client
            .upload(format!("projects/{}", project_id), &project)
            .await?;

        let retrieved_project = repo.get_project(project_id).await?.unwrap();
        assert_eq!(retrieved_project.id, project.id);
        assert_eq!(retrieved_project.workspace_id, project.workspace_id);
        Ok(())
    }

    #[test]
    async fn test_create_and_get_snapshot() -> Result<(), Box<dyn std::error::Error>> {
        let (_temp_dir, repo) = setup().await?;
        let project_id = "test_project";
        let snapshot = create_test_snapshot(project_id);

        repo.create_snapshot(snapshot.clone()).await?;

        let retrieved_snapshot = repo.get_latest_snapshot(project_id).await?.unwrap();
        assert_eq!(retrieved_snapshot.metadata.id, snapshot.metadata.id);
        assert_eq!(
            retrieved_snapshot.metadata.project_id,
            snapshot.metadata.project_id
        );
        Ok(())
    }

    #[test]
    async fn test_get_latest_snapshot_state() -> Result<(), Box<dyn std::error::Error>> {
        let (_temp_dir, repo) = setup().await?;
        let project_id = "test_project";
        let snapshot = create_test_snapshot(project_id);

        repo.create_snapshot(snapshot).await?;

        let snapshot_state = repo.get_latest_snapshot_state(project_id).await?;
        assert!(!snapshot_state.is_empty());
        Ok(())
    }

    fn create_test_snapshot(project_id: &str) -> ProjectSnapshot {
        ProjectSnapshot {
            metadata: ProjectMetadata {
                id: "snap_123".to_string(),
                project_id: project_id.to_string(),
                session_id: Some("session_123".to_string()),
                name: "Test Snapshot".to_string(),
                path: "".to_string(),
            },
            created_by: Some("test_user".to_string()),
            changes_by: vec![],
            tenant: ObjectTenant {
                id: "tenant_123".to_string(),
                key: "tenant_key".to_string(),
            },
            delete: ObjectDelete {
                deleted: false,
                delete_after: None,
            },
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        }
    }
}
