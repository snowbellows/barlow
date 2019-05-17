table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        created -> Timestamptz,
        category -> Nullable<Varchar>,
        tags -> Array<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
        created -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
