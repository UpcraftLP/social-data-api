// @generated automatically by Diesel CLI.

diesel::table! {
    instagram_datapoints (id) {
        id -> Int4,
        user_id -> Uuid,
        recorded_at -> Timestamptz,
        follower_count -> Int4,
        following_count -> Int4,
        posts_count -> Int4,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    instagram_users (id) {
        id -> Uuid,
        username -> Text,
        follower_count -> Int4,
        following_count -> Int4,
        posts_count -> Int4,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(instagram_datapoints -> instagram_users (user_id));

diesel::allow_tables_to_appear_in_same_query!(instagram_datapoints, instagram_users,);
