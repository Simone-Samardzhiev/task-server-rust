CREATE TABLE priorities
(
    priority VARCHAR(100) PRIMARY KEY
);

INSERT INTO priorities(priority)
VALUES ('Low'),
       ('Medium'),
       ('High'),
       ('Vital');

CREATE TABLE tasks
(
    id          UUID PRIMARY KEY,
    name        VARCHAR(100) NOT NULL,
    description TEXT NOT NULL ,
    priority    VARCHAR(100) REFERENCES priorities (priority),
    date        TIMESTAMPTZ               NOT NULL,
    user_id     INT REFERENCES users (id) NOT NULL
);