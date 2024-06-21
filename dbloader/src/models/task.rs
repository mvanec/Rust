use uuid::Uuid;
use sqlx::PgPool;
use async_trait::async_trait;

use crate::traits::model_trait::ModelTrait;

#[derive(Debug, sqlx::FromRow)]
pub struct Task {
    pub task_id: Uuid,
    pub project_id: Uuid,
    pub task_name: String,
    pub task_duration: i32,
}

impl Task {
    pub fn new(task_id: Uuid, project_id: Uuid, task_name: String, task_duration: i32) -> Self {
        Self {
            task_id,
            project_id,
            task_name,
            task_duration
        }
    }
}

#[async_trait(?Send)]
impl ModelTrait for Task {
    async fn create(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO ProjectTasks (TaskId, ProjectId, TaskName, TaskDuration)
             VALUES ($1, $2, $3, $4)",
        )
        .bind(&self.task_id)
        .bind(&self.project_id)
        .bind(&self.task_name)
        .bind(&self.task_duration)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM ProjectTasks WHERE TaskId = $1")
            .bind(&self.task_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
