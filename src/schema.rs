// @generated automatically by Diesel CLI.

diesel::table! {
    channels (id) {
        id -> Int4,
        #[max_length = 1024]
        name -> Varchar,
        owner_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    channels_users (channel_id, user_id) {
        channel_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::joinable!(channels -> users (owner_id));
diesel::joinable!(channels_users -> channels (channel_id));
diesel::joinable!(channels_users -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(channels, channels_users, users);
