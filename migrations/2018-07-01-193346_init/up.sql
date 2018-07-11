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
    creator_id SERIAL NOT NULL,
    slug TEXT NOT NULL,
    name TEXT NOT NULL,

    UNIQUE (project_id, slug),
    FOREIGN KEY (creator_id) REFERENCES users (id),
    FOREIGN KEY (project_id) REFERENCES projects (id)
);
CREATE INDEX features_idx ON features (project_id, slug);

CREATE TABLE steps (
    id SERIAL PRIMARY KEY,
    slug TEXT NOT NULL,
    feature_id SERIAL NOT NULL,
    creator_id SERIAL NOT NULL,
    step_type TEXT NOT NULL,
    value TEXT NOT NULL,
    position SERIAL NOT NULL CHECK (position > 0),

    UNIQUE (feature_id, position) DEFERRABLE INITIALLY DEFERRED,
    UNIQUE (feature_id, slug),
    FOREIGN KEY (creator_id) REFERENCES users (id),
    FOREIGN KEY (feature_id) REFERENCES features (id)
);
CREATE INDEX step_feature_idx ON steps (feature_id, slug);
CREATE INDEX step_values_idx ON steps (step_type, value);