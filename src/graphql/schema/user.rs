use juniper::{FieldError, FieldResult};

use crate::domain::model::user::{NewUser, User};
use crate::domain::repository::user_repository::UserRepository;
use crate::graphql::Context;

#[derive(GraphQLObject)]
#[graphql(description = "An user of this system")]
pub struct UserObject {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<User> for UserObject {
    fn from(user: User) -> Self {
        UserObject {
            id: user.id.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }
}

#[derive(Debug, GraphQLInputObject)]
#[graphql(description = "An user of this system")]
pub struct NewUserInput {
    pub name: String,
    pub email: String,
}

impl Context {
    pub fn get_user(&self, id: &str) -> FieldResult<UserObject> {
        let id: i32 = id.parse()?;
        let user = UserRepository::get_user(id, &self)?;
        Ok(user.into())
    }

    pub fn add_user(&self, input: NewUserInput) -> FieldResult<UserObject> {
        let new_user = NewUser {
            name: input.name.clone(),
            email: input.email.clone(),
        };

        let user = UserRepository::create(new_user, &self)
            .map_err(|_| FieldError::from("Cannot create a user"))?;
        Ok(user.into())
    }
}
