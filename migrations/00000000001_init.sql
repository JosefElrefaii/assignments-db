CREATE TABLE assignments (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    role_name TEXT NOT NULL,
    description TEXT NOT NULL,
    git_url TEXT NOT NULL
);

CREATE table applications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT UNIQUE NOT NULL,
    assignment_id SERIAL NOT NULL,
    email TEXT NOT NULL,
    git_url TEXT NOT NULL,

    FOREIGN KEY (assignment_id) REFERENCES assignments(id)
);