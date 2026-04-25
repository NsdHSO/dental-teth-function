use crate::types::FinancialSanp;
use anyhow::Result;
use serde_json::json;

#[derive(Clone)]
pub struct StrapiClient {
    endpoint: String,
    client: reqwest::Client,
}

impl StrapiClient {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            client: reqwest::Client::new(),
        }
    }

    /// Fetch financial snaps from Strapi GraphQL endpoint
    pub async fn fetch_financial_snaps(&self) -> Result<Vec<FinancialSanp>> {
        let query = r#"
            query {
                financialSanps {
                    title
                    subTitle
                }
            }
        "#;

        let response = self
            .client
            .post(&self.endpoint)
            .json(&json!({ "query": query }))
            .send()
            .await?;

        let body: serde_json::Value = response.json().await?;

        // Debug: print the response to see structure
        println!(
            "Strapi Response: {}",
            serde_json::to_string_pretty(&body).unwrap_or_default()
        );

        // Extract data from response - try simplified structure first
        if let Some(snaps_array) = body
            .get("data")
            .and_then(|d| d.get("financialSanps"))
            .and_then(|fs| fs.as_array())
        {
            // Direct array structure (Apollo might return this)
            let snaps = snaps_array
                .iter()
                .enumerate()
                .filter_map(|(idx, item)| {
                    Some(FinancialSanp {
                        document_id: idx.to_string(),
                        title: item.get("title")?.as_str()?.to_string(),
                        sub_title: item.get("subTitle")?.as_str()?.to_string(),
                        img: None,
                        created_at: None,
                        updated_at: None,
                        published_at: None,
                    })
                })
                .collect();
            Ok(snaps)
        } else if let Some(snaps_data) = body
            .get("data")
            .and_then(|d| d.get("financialSanps"))
            .and_then(|fs| fs.get("data"))
            .and_then(|d| d.as_array())
        {
            // Nested Strapi structure
            let snaps = snaps_data
                .iter()
                .filter_map(|item| {
                    let document_id = item
                        .get("documentId")
                        .or_else(|| item.get("id"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string();

                    let attrs = item.get("attributes")?;

                    Some(FinancialSanp {
                        document_id,
                        title: attrs.get("title")?.as_str()?.to_string(),
                        sub_title: attrs.get("subTitle")?.as_str()?.to_string(),
                        img: None,
                        created_at: None,
                        updated_at: None,
                        published_at: None,
                    })
                })
                .collect();
            Ok(snaps)
        } else {
            Ok(vec![])
        }
    }

    /// Fetch single financial snap by documentId from Strapi
    pub async fn fetch_financial_snap(&self, document_id: &str) -> Result<Option<FinancialSanp>> {
        let query = format!(
            r#"
            query {{
                financialSanp(documentId: "{}") {{
                    title
                    subTitle
                }}
            }}
            "#,
            document_id
        );

        let response = self
            .client
            .post(&self.endpoint)
            .json(&json!({ "query": query }))
            .send()
            .await?;

        let body: serde_json::Value = response.json().await?;

        println!(
            "Strapi Single Response: {}",
            serde_json::to_string_pretty(&body).unwrap_or_default()
        );

        if let Some(snap) = body.get("data").and_then(|d| d.get("financialSanp")) {
            if snap.is_null() {
                return Ok(None);
            }

            Ok(Some(FinancialSanp {
                document_id: document_id.to_string(),
                title: snap
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                sub_title: snap
                    .get("subTitle")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                img: None,
                created_at: None,
                updated_at: None,
                published_at: None,
            }))
        } else {
            Ok(None)
        }
    }

    /// Generic method to execute any GraphQL query against Strapi
    pub async fn execute_query(
        &self,
        query: &str,
        variables: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let mut body = json!({ "query": query });

        if let Some(vars) = variables {
            body["variables"] = vars;
        }

        let response = self.client.post(&self.endpoint).json(&body).send().await?;

        let result: serde_json::Value = response.json().await?;
        Ok(result)
    }
}
