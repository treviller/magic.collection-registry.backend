CREATE TABLE cards (
    id uuid NOT NULL PRIMARY KEY,
    scryfall_id VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    lang VARCHAR(30) NOT NULL,
    released_at DATE NOT NULL,
    set_id uuid NOT NULL,
    CONSTRAINT fk_set
        FOREIGN KEY (set_id)
            REFERENCES sets (id)
)