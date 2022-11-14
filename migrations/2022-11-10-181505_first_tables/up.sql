CREATE TABLE users(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    username VARCHAR (255) NOT NULL UNIQUE,
    password TEXT NOT NULL
);

CREATE TYPE token_type AS ENUM ('reset_password');

CREATE TABLE tokens(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    token_type TOKEN_TYPE NOT NULL,
    user_id uuid NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
            REFERENCES users (id)
);