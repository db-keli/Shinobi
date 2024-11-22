CREATE TABLE keys_tokens (
    id SERIAL PRIMARY KEY,
    jti VARCHAR(255) NOT NULL UNIQUE,
    user_id VARCHAR(255) NOT NULL,
    project_id VARCHAR(255),
    issued_at TIMESTAMP NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    revoked BOOLEAN DEFAULT FALSE
);
