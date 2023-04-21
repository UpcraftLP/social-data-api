use async_graphql::*;
use uuid::Uuid;

pub struct Query;
pub struct Mutation;

pub type ApiSchema = Schema<Query, Mutation, EmptySubscription>;

#[Object]
impl Query {
    async fn instagram_users(&self) -> Result<Vec<String>> {
        Ok(vec!["Hello".to_string()])
    }
}

#[Object]
impl Mutation {
    async fn add_instagram_user(&self, username: String) -> Result<Uuid> {
        println!("Adding user: {}", username);
        Ok(Uuid::new_v4())
    }
}

pub fn build_schema() -> ApiSchema {
    async_graphql::Schema::build(Query, Mutation, async_graphql::EmptySubscription).finish()
}
