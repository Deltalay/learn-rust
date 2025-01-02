// @generated automatically by Diesel CLI.

diesel::table! {
    url (id) {
        id -> Integer,
        short_url -> Text,
        long_url -> Text,
        created_at -> Nullable<Timestamp>,
        expires_at -> Nullable<Timestamp>,
        access_count -> Nullable<Integer>,
    }
}
