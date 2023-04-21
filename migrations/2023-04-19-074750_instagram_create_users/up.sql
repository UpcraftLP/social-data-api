CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE instagram_users (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    username TEXT NOT NULL UNIQUE,
    follower_count INTEGER NOT NULL DEFAULT 0,
    following_count INTEGER NOT NULL DEFAULT 0,
    posts_count INTEGER NOT NULL DEFAULT 0,
    
    --managed by diesel
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- faster lookup based on username
CREATE UNIQUE INDEX IF NOT EXISTS ig_users_by_username ON instagram_users (username);

-- set updated_at
SELECT diesel_manage_updated_at('instagram_users');
