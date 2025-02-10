// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Int4,
        title -> Varchar,
        artists_ids -> Array<Nullable<Int4>>,
        description -> Nullable<Text>,
        youtube_id -> Nullable<Text>,
    }
}

diesel::table! {
    artists (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    musics (id) {
        id -> Int4,
        title -> Varchar,
        artists_ids -> Array<Nullable<Int4>>,
        album_id -> Int4,
        youtube_id -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        userdata -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    musics,
    users,
);
