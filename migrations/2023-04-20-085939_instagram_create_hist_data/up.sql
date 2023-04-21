CREATE TABLE instagram_datapoints (
    id SERIAL PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES instagram_users(id),

    recorded_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    follower_count INTEGER NOT NULL DEFAULT 0,
    following_count INTEGER NOT NULL DEFAULT 0,
    posts_count INTEGER NOT NULL DEFAULT 0,
    
    --managed by diesel
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- faster lookup for per-user data
CREATE INDEX IF NOT EXISTS ig_data_by_user_id ON instagram_datapoints (user_id);

-- set updated_at
SELECT diesel_manage_updated_at('instagram_datapoints');
