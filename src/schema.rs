// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Integer,
        title -> Text,
        artists_ids -> Text,
        description -> Nullable<Text>,
        youtube_id -> Nullable<Text>,
    }
}

diesel::table! {
    artists (id) {
        id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        userdata -> Text,
    }
}

diesel::table! {
    musics (id) {
        id -> Integer,
        title -> Text,
        artists_ids -> Text,
        album_id -> Text,
        youtube_id -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        userdata -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    musics,
    users,
);
