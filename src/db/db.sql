create extension "uuid-ossp";

CREATE TABLE users
(
    id              uuid primary key     default uuid_generate_v4(),
    first_name      text        not null,
    last_name       text        not null,
    email           text unique not null,
    hashed_password text        not null,
    created_at      timestamptz not null default now(),
    updated_at      timestamptz not null default now(),
    deleted_at      timestamptz
);

CREATE INDEX users_email ON users (email);

CREATE TABLE access_tokens
(
    id         uuid primary key     default uuid_generate_v4(),
    user_id    uuid        not null references users (id),
    token      text        not null unique,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    expires_at timestamptz not null
);

CREATE TABLE application
(
    id               uuid primary key default uuid_generate_v4(),
    application_name text not null,
    user_id          uuid not null references users (id)
);