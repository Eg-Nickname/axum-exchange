-- Add migration script here
CREATE TABLE IF NOT EXISTS user_permissions (
    user_id  BIGINT NOT NULL,
    token    VARCHAR(50) NOT NULL,
    PRIMARY KEY (user_id, token),
    FOREIGN KEY (user_id)
        REFERENCES users (id)
);