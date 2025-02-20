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

// Array<Nullable<Int4>> has to be modified to Array<Int4> manually
// I couldn't find a better way to do this
diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        favorite_musics -> Array<Int4>,
        favorite_albums -> Array<Int4>,
        favorite_artists -> Array<Int4>,
        playlists_library -> Array<Int4>,
    }
}

diesel::joinable!(musics -> albums (album_id));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    musics,
    users,
);
