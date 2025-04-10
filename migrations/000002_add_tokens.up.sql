CREATE TABLE IF NOT EXISTS tokens
(
    id      UUID PRIMARY KEY,
    exp     TIMESTAMP                 NOT NULL,
    user_id INT REFERENCES users (id) NOT NULL
);