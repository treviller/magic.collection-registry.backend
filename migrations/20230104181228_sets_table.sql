CREATE TYPE set_type AS ENUM ('core', 'expansion', 'masters', 'alchemy', 'masterpiece', 'arsenal', 'from_the_vault', 'spellbook',
'premium_deck', 'duel_deck', 'draft_innovation', 'treasure_chest', 'commander', 'planechase', 'archenemy', 'vanguard', 'funny', 'starter',
 'box', 'promo', 'token', 'memorabilia');

CREATE TABLE sets (
    id uuid NOT NULL PRIMARY KEY,
    code VARCHAR(6) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    set_type SET_TYPE NOT NULL,
    released_at DATE NOT NULL,
    block_code VARCHAR(255),
    block VARCHAR(255),
    parent_set_id uuid DEFAULT NULL,
    card_count INT NOT NULL,
    printed_size INT NOT NULL,
    foil_only BOOLEAN NOT NULL,
    non_foil_only BOOLEAN NOT NULL,
    icon_svg_uri TEXT NOT NULL,
    CONSTRAINT fk_parent_set
        FOREIGN KEY (parent_set_id)
            REFERENCES sets (id)
);