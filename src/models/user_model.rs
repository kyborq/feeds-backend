use crate::schemas::user_schema::user;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel {
    pub id: i32,
    pub name: String,
    pub login: String,
    pub password: String,
}
