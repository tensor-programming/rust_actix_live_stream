table! {
    messages (id) {
        id -> Uuid,
        username -> Varchar,
        body -> Text,
        ts -> Timestamp,
    }
}
