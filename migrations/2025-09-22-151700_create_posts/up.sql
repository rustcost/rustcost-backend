-- Your SQL goes here
CREATE TABLE posts
(
    id        UUID PRIMARY KEY,
    title     VARCHAR NOT NULL,
    body      TEXT    NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE nodes (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    cpu_capacity VARCHAR,
    memory_capacity VARCHAR,
    kubelet_version VARCHAR,
    os_image VARCHAR,
    architecture VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);

-- Index on created_at for faster queries by date
CREATE INDEX idx_nodes_created_at ON nodes(created_at);

-- Index on name for faster lookups by node name
CREATE INDEX idx_nodes_name ON nodes(name);