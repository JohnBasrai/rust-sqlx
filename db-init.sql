create type user_role as enum ('admin', 'user');
CREATE TABLE IF NOT EXISTS users (
    id   bigserial PRIMARY KEY,
    name text      NOT NULL UNIQUE,
    role user_role NOT NULL);
