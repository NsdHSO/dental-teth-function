use crate::mutation::MutationRoot;
use crate::query::QueryRoot;
use crate::strapi_client::StrapiClient;
use async_graphql::{EmptySubscription, Schema};

pub fn build_schema(
    strapi_client: StrapiClient,
) -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(strapi_client)
        .finish()
}
