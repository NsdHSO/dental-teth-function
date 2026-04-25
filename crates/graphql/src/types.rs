use async_graphql::*;
use serde::{Deserialize, Serialize};

/// GraphQL type for FinancialSanp matching Strapi schema
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct FinancialSanp {
    pub document_id: String,
    pub title: String,
    pub sub_title: String,
    pub img: Option<ComponentSharedMedia>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub published_at: Option<String>,
}

/// ComponentSharedMedia type from Strapi
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct ComponentSharedMedia {
    pub id: String,
    pub name: Option<String>,
    pub url: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub formats: Option<String>,
    pub mime: Option<String>,
    pub size: Option<f64>,
}

/// Input type for creating FinancialSanp
#[derive(Debug, InputObject)]
pub struct CreateFinancialSnapInput {
    pub title: String,
    pub sub_title: String,
    pub img_url: Option<String>,
}

/// Input type for updating FinancialSanp
#[derive(Debug, InputObject)]
pub struct UpdateFinancialSnapInput {
    pub title: Option<String>,
    pub sub_title: Option<String>,
    pub img_url: Option<String>,
}
