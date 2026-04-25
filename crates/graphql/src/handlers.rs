use crate::{GraphQLSchema, StrapiClient};
use actix_web::{web, HttpResponse, Result};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use serde_json::json;

/// GraphQL query handler
pub async fn graphql_handler(
    schema: web::Data<GraphQLSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// Direct Strapi proxy handler - forwards any GraphQL request directly to Strapi
pub async fn strapi_proxy_handler(
    strapi_client: web::Data<StrapiClient>,
    body: web::Json<serde_json::Value>,
) -> Result<HttpResponse> {
    let query = body
        .get("query")
        .and_then(|q| q.as_str())
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Missing query field"))?;

    let variables = body.get("variables").cloned();

    match strapi_client.execute_query(query, variables).await {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "errors": [{
                "message": format!("Strapi query failed: {}", e)
            }]
        }))),
    }
}

/// GraphQL playground handler (for development)
pub async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>GraphQL Playground</title>
                <link rel="stylesheet" href="https://unpkg.com/graphql-playground-react/build/static/css/index.css" />
                <link rel="shortcut icon" href="https://unpkg.com/graphql-playground-react/build/favicon.png" />
                <script src="https://unpkg.com/graphql-playground-react/build/static/js/middleware.js"></script>
            </head>
            <body>
                <div id="root"></div>
                <script>
                    window.addEventListener('load', function (event) {
                        GraphQLPlayground.init(document.getElementById('root'), {
                            endpoint: '/graphql',
                            settings: {
                                'request.credentials': 'include',
                            },
                            tabs: [
                                {
                                    endpoint: '/graphql',
                                    name: 'Local GraphQL',
                                    query: '# Use strapi() query to proxy to Strapi\nquery {\n  strapi(query: "{ financialSanps { title subTitle } }")\n}'
                                },
                                {
                                    endpoint: '/strapi-proxy',
                                    name: 'Direct Strapi Proxy',
                                    query: '# This goes directly to Strapi\nquery {\n  financialSanps {\n    title\n    subTitle\n  }\n}'
                                }
                            ]
                        })
                    })
                </script>
            </body>
            </html>
            "#,
        ))
}
