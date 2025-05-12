// @generated automatically by Diesel CLI.

// Array<Nullable<Int4>> has to be modified to Array<Int4> manually
// I couldn't find a better way to do this

diesel::table! {
    albums (id) {
        id -> Int4,
        title -> Varchar,
        artists_ids -> Array<Int4>,
        description -> Nullable<Text>,
        fetcher -> Nullable<Text>,
        origin_user_id -> Int4,
        youtube_id -> Nullable<Text>,
        spotify_id -> Nullable<Text>,
        deezer_id -> Nullable<Text>,
        apple_music_id -> Nullable<Text>,
    }
}

diesel::table! {
    analytics (id) {
        id -> Int4,
        music_id -> Int4,
        album_id -> Int4,
        user_id -> Int4,
        date_played -> Timestamp,
    }
}

diesel::table! {
    artists (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        youtube_id -> Nullable<Text>,
        spotify_id -> Nullable<Text>,
        deezer_id -> Nullable<Text>,
        apple_music_id -> Nullable<Text>,
    }
}

diesel::table! {
    musics (id) {
        id -> Int4,
        title -> Varchar,
        artists_ids -> Array<Int4>,
        album_id -> Int4,
        duration -> Int4,
        fetcher -> Nullable<Text>,
        youtube_id -> Nullable<Text>,
        spotify_id -> Nullable<Text>,
        deezer_id -> Nullable<Text>,
        apple_music_id -> Nullable<Text>,
    }
}

diesel::table! {
    playlists (id) {
        id -> Int4,
        owner_id -> Int4,
        title -> Text,
        description -> Nullable<Text>,
        musics -> Array<Int4>,
        fetcher -> Nullable<Text>,
        fetcher_id -> Nullable<Text>,
        date_created -> Timestamp,
    }
}

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
diesel::joinable!(playlists -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    analytics,
    artists,
    musics,
    playlists,
    users,
);
