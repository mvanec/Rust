-- Add migration script here
-- Create Tasks Table
CREATE TABLE IF NOT EXISTS ProjectTasks (
  TaskId       GUID NOT NULL,
  ProjectId    GUID NOT NULL,
  TaskName     VARCHAR(255) NOT NULL,
  TaskDuration INTEGER NOT NULL,    -- The duration in milliseconds
  CONSTRAINT pk_Tasks PRIMARY KEY(TaskID),
  CONSTRAINT fk_TasksProjects FOREIGN KEY(ProjectID)
    REFERENCES Projects(ProjectID) ON DELETE CASCADE
);
