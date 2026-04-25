use crate::strapi_client::StrapiClient;
use async_graphql::*;
use serde_json::Value;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Health check query
    async fn health(&self) -> String {
        "GraphQL server is running!".to_string()
    }

    /// Direct proxy to Strapi - execute any GraphQL query
    ///
    /// Example:
    /// ```graphql
    /// query {
    ///   strapi(query: "{ financialSanps { title subTitle } }")
    /// }
    /// ```
    async fn strapi(
        &self,
        ctx: &Context<'_>,
        query: String,
        #[graphql(default)] variables: Option<Value>,
    ) -> Result<Value> {
        let strapi_client = ctx
            .data::<StrapiClient>()
            .map_err(|_| Error::new("Strapi client not found in context"))?;

        strapi_client
            .execute_query(&query, variables)
            .await
            .map_err(|e| Error::new(format!("Failed to execute query on Strapi: {}", e)))
    }
}
