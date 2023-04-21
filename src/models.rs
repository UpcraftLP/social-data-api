use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::*;

#[derive(Queryable, Identifiable, Selectable)]
#[diesel(table_name = instagram_users)]
pub struct InstagramUser {
    pub id: Uuid,
    pub username: String,
    pub follower_count: i32,
    pub following_count: i32,
    pub posts_count: i32,

    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Queryable, Identifiable, Selectable)]
#[diesel(table_name = instagram_datapoints)]
pub struct InstagramData {
    pub id: i32,
    pub user_id: Uuid,
    pub recorded_at: chrono::DateTime<Utc>,
    pub follower_count: i32,
    pub following_count: i32,
    pub posts_count: i32,

    pub updated_at: chrono::DateTime<Utc>,
}
