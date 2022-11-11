CREATE TABLE users(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    username VARCHAR (255) NOT NULL UNIQUE,
    password TEXT NOT NULL
);

CREATE TABLE tokens(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    token_type VARCHAR (255) NOT NULL,
    user_id uuid NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
            REFERENCES users (id)
);