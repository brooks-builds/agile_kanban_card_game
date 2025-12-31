CREATE TABLE IF NOT EXISTS agile_game (
    game_id  UUID PRIMARY KEY DEFAULT uuidv4(),
    name     VARCHAR(255) NOT NULL,
    code     INT NOT NULL
);
