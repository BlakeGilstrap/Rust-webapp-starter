table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}
table! {
    article (id) {
        id -> Int4,
        user_id -> Int4,
        category -> Text,
        title -> Text,
        body -> Text,
        created_at -> Timestamp,
    }
}