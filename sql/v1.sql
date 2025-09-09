-- Needed extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE EXTENSION IF NOT EXISTS citext;

-- Types
CREATE TYPE user_status AS ENUM ('active', 'blocked', 'pending_verification');

-- Principal users table
CREATE TABLE users (
  id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email             CITEXT NOT NULL UNIQUE,
  username          CITEXT NOT NULL UNIQUE,
  password_hash     TEXT,
  is_email_verified BOOLEAN NOT NULL DEFAULT FALSE,
  status            user_status NOT NULL DEFAULT 'pending_verification',
  -- metadata
  created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
  deleted_at        TIMESTAMPTZ,
  last_login_at     TIMESTAMPTZ
);

-- Updated_at trigger
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN NEW.updated_at = now(); RETURN NEW; END;
$$;

CREATE TRIGGER trg_users_updated_at
BEFORE UPDATE ON users
FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- Table for email verification tokens
CREATE TABLE email_verifications (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id) ON DELETE CASCADE,
    token_hash bytea NOT NULL UNIQUE,
    expires_at  TIMESTAMPTZ NOT NULL,
    used_at     TIMESTAMPTZ,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Unique user index
CREATE UNIQUE INDEX idx_email_verifications_user_id ON email_verifications(user_id);
-- Searches by expiration and cleanup
CREATE INDEX ON email_verifications (expires_at);
-- Speed up "pending" queries
CREATE INDEX email_verifications_pending_idx
    ON email_verifications (user_id)
    WHERE used_at IS NULL;

-- Stored procedure for upsert in email_verifications
CREATE OR REPLACE PROCEDURE add_email_verification(
    p_user_id UUID,
    p_token_hash BYTEA,
    p_expires_at TIMESTAMPTZ
)
AS $$
BEGIN
INSERT INTO email_verifications (user_id, token_hash, expires_at, used_at, created_at)
VALUES (p_user_id, p_token_hash, p_expires_at, NULL, now())
    ON CONFLICT (user_id)
    DO UPDATE SET
    token_hash = EXCLUDED.token_hash,
               expires_at = EXCLUDED.expires_at,
               used_at = NULL,
               created_at = now();
END;
$$;

-- Stored procedure to verify email
CREATE OR REPLACE PROCEDURE verify_email(
    p_token_hash BYTEA
)
LANGUAGE plpgsql
AS $$
DECLARE
    v_user_id UUID;
    v_verification_record RECORD;
BEGIN
    -- Find the valid verification record
    SELECT user_id, expires_at, used_at
    INTO v_verification_record
    FROM email_verifications
    WHERE token_hash = p_token_hash;

    -- Verify that the token exists
    IF NOT FOUND THEN
        RAISE EXCEPTION 'Invalid verification token';
    END IF;

    -- Verify that the token has not expired
    IF v_verification_record.expires_at < now() THEN
        RAISE EXCEPTION 'Verification token expired';
    END IF;

    -- Verify that the token has not been used
    IF v_verification_record.used_at IS NOT NULL THEN
        RAISE EXCEPTION 'Verification token already used';
    END IF;

    -- Mark the token as used
    UPDATE email_verifications
    SET used_at = now()
    WHERE token_hash = p_token_hash;

    -- Update the user as verified and change status to active
    UPDATE users
    SET
        is_email_verified = TRUE,
        status = 'active'
    WHERE id = v_verification_record.user_id;

    -- Commit changes
    COMMIT;
END;
$$;
