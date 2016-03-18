CREATE TABLE sessions (
  id BIGSERIAL PRIMARY KEY,
  account_id BIGSERIAL REFERENCES users (id),
  session_doc JSONB NOT NULL
);
