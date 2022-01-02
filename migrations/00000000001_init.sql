CREATE TABLE assignments (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    role_name TEXT NOT NULL,
    description TEXT NOT NULL,
    git_url TEXT NOT NULL
);