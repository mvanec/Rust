-- Add migration script here
-- Create Projects Table
CREATE TABLE IF NOT EXISTS Projects (
  ProjectId       GUID NOT NULL UNIQUE,
  ProjectName     VARCHAR(255) NOT NULL,
  ProjectDate     DATE NOT NULL,
  PayRate         REAL NOT NULL,
  ProjectDuration INTEGER NOT NULL,    -- The duration in milliseconds
  TotalPay        REAL NOT NULL,
  CONSTRAINT pk_Projects PRIMARY KEY(ProjectID)
);
