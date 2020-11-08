use juniper::FieldResult;

use crate::graphql::schema::UserObject;
use crate::graphql::Context;

#[derive(Debug)]
pub struct Query;

juniper::graphql_object!(Query: Context | &self | {
  field apiVersion() -> &str {
    "1.0"
  }

  field user(&executor, id: String) -> FieldResult<UserObject> {
    executor.context().get_user(&id)
  }

});
