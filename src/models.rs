use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub text: String,
}

#[derive(Insertable)]
#[diesel(table_name = recipes)]
pub struct NewRecipe<'a> {
    pub name: &'a String,
    pub description: &'a String,
    pub text: &'a String,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
}
