use async_graphql::*;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Placeholder for future mutations
    async fn placeholder(&self) -> String {
        "Mutations will be implemented based on your requirements".to_string()
    }
}
