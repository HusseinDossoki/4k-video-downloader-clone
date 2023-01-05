// @generated automatically by Diesel CLI.

table! {
    smartmode (id) {
        id -> Integer,
        enabled -> Bool,
        format -> Text,
        quality -> Text,
        directory -> Text,
    }
}
