use sqlx::PgPool;
use sqlx::Row;
use tokio;
use ctor::ctor;

use projects::models::project::Project;
use projects::models::task::Task;
use projects::traits::model_trait::ModelTrait;

mod utils;
use crate::utils::create_test_pool;

#[ctor]
fn setup() {
    utils::test_setup();
}

// Create a test pool and a task
async fn setup_test_task(pool: &PgPool) -> (Project, Task) {
    // Create a project
    let project = Project::new(
        uuid::Uuid::new_v4(),
        "Test Project".to_string(),
        chrono::NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
        chrono::NaiveDate::from_ymd_opt(2022, 12, 31).unwrap(),
        40.0,
        3600000,
        310.56
    );
    project.create(pool).await.unwrap();

    // Create a task
    let task = Task::new(
        uuid::Uuid::new_v4(),
        project.project_id,
        "Test Task".to_string(),
        36000
    );
    task.create(pool).await.unwrap();

    (project, task)
}

#[tokio::test]
async fn test_task_create() -> Result<(), sqlx::Error> {
    // Create a test pool
    let pool = create_test_pool().await.unwrap();

    // Setup the test task
    let (_, task) = setup_test_task(&pool).await;

    // Retrieve the task from the database
    let retrieved_task = sqlx::query("SELECT * FROM ProjectTasks WHERE TaskId = $1")
        .bind(&task.task_id)
        .fetch_one(&pool)
        .await?;

    // Check that the retrieved task matches the original task
    let task_id: uuid::Uuid = retrieved_task.get("taskid");
    let project_id: uuid::Uuid = retrieved_task.get("projectid");
    let task_name: String = retrieved_task.get("taskname");
    let task_duration: i32 = retrieved_task.get("taskduration");

    assert_eq!(task_id, task.task_id);
    assert_eq!(project_id, task.project_id);
    assert_eq!(task_name, task.task_name);
    assert_eq!(task_duration, task.task_duration);

    Ok(())
}

#[tokio::test]
async fn test_task_delete() -> Result<(), sqlx::Error> {
    // Create a test pool
    let pool = create_test_pool().await.unwrap();

    // Setup the test task
    let (_, task) = setup_test_task(&pool).await;

    // Delete the task from the database
    task.delete(&pool).await?;

    // Check that the task was deleted
    let count = sqlx::query("SELECT COUNT(*) FROM ProjectTasks WHERE TaskId = $1")
        .bind(&task.task_id)
        .fetch_one(&pool)
        .await?;

    let count: i64 = count.get(0);
    assert_eq!(count, 0);

    Ok(())
}
