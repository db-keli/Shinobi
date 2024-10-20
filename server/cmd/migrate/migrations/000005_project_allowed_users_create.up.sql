CREATE TABLE project_allowed_users (
    project_id BIGINT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL,
    PRIMARY KEY (project_id, user_id)
);
