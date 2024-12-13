create extension "uuid-ossp";

CREATE TABLE users
(
    id         uuid primary key   default uuid_generate_v4(),
    first_name text      not null,
    last_name  text      not null,
    email      text      not null,
    password   text      not null,
    created_at timestamp not null default now(),
    updated_at timestamp not null default now(),
    deleted_at timestamp
);

CREATE TABLE application
(
    id               uuid primary key default uuid_generate_v4(),
    application_name text not null,
    user_id          uuid not null references users (id)
);