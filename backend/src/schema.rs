table! {
    guests (email) {
        email -> Varchar,
        name -> Varchar,
        vegetarian -> Varchar,
        kid -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    guests,
);
