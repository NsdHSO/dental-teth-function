mod handlers;
mod mutation;
mod query;
mod schema;
mod strapi_client;
mod types;

pub use handlers::{graphql_handler, graphql_playground, strapi_proxy_handler};
pub use schema::build_schema;
pub use strapi_client::StrapiClient;
pub use types::*;

pub type GraphQLSchema = async_graphql::Schema<
    query::QueryRoot,
    mutation::MutationRoot,
    async_graphql::EmptySubscription,
>;
