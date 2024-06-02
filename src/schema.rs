// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Integer,
        nickname -> Text,
        last_nickname -> Text,
        active -> Bool,
    }
}
