-- Add migration script here
-- Create TaskTimes Table
CREATE TABLE IF NOT EXISTS TaskTimes (
  TaskTimeId INTEGER NOT NULL UNIQUE,
  TaskId     GUID NOT NULL,
  StartTime  TIMESTAMP NOT NULL,
  EndTime    TIMESTAMP NOT NULL,
  CONSTRAINT pk_TaskTimes  PRIMARY KEY("TaskTimeId" AUTOINCREMENT),
  CONSTRAINT fk_TimesTasks FOREIGN KEY(TaskID)
    REFERENCES ProjectTasks(TaskId)  ON DELETE CASCADE
);
