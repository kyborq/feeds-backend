diesel::table! {
    user (id) {
        id -> Int4,
        name -> Varchar,
        login -> Varchar,
        password -> Varchar,
    }
}
