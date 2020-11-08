use crate::domain::schema::users;
use crate::domain::model::user::*;
use crate::Context;

use diesel;
use diesel::prelude::*;

pub struct UserRepository;

impl UserRepository {

    pub fn create(new_user: NewUser, context: &Context) -> QueryResult<User> {
        let connection = &context.connection.get().unwrap();
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(connection)
    }

    pub fn get_user(id: i32, context: &Context) -> QueryResult<User> {
        let connection = &context.connection.get().unwrap();
        users::table.find(id).first::<User>(connection)
    }

}
