CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  iterations INTEGER NOT NULL,
  salt BYTEA NOT NULL,
  credential BYTEA NOT NULL
);
CREATE INDEX user_usernames_idx ON users (username);

CREATE TABLE user_sessions (
    id UUID PRIMARY KEY,
    user_id SERIAL NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users (id)
);
CREATE INDEX user_sessions_user_ids_idx ON user_sessions (user_id);

CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    owner_id SERIAL NOT NULL,

    FOREIGN KEY (owner_id) REFERENCES users (id)
);
CREATE INDEX project_slugs_idx ON projects (slug);

CREATE TABLE features (
    id SERIAL PRIMARY KEY,
    project_id SERIAL NOT NULL,
    slug TEXT NOT NULL,
    name TEXT NOT NULL,

    UNIQUE (project_id, slug),
    FOREIGN KEY (project_id) REFERENCES projects (id)
);
CREATE INDEX features_idx ON features (project_id, slug);