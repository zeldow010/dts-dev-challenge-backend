-- Add migration script here

-- Create the enumeration type
CREATE TYPE task_status AS ENUM ('draft','todo','completed');

-- -- Create task_statuses Table
-- CREATE TABLE statuses
-- (
--   status_id uuid NOT NULL DEFAULT gen_random_uuid(),
--   status_name task_status NOT NULL,
--   PRIMARY KEY (status_id)
-- );

-- Create tasks Table
CREATE TABLE tasks
(
  task_id     uuid        NOT NULL DEFAULT gen_random_uuid(),
  title       text        NOT NULL,
  description text,
  due         timestamptz NOT NULL,
  task_status task_status NOT NULL,
  PRIMARY KEY (task_id)
);