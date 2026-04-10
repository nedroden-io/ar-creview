use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub azure_api_key: String,
    pub azure_api_endpoint: String,
    pub azure_api_version: String,
    pub azure_api_deployment: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let api_key = option_env!("AR_TRANSLATE_AZURE_API_KEY").unwrap_or("unknown");
        let api_endpoint = option_env!("AR_TRANSLATE_AZURE_API_ENDPOINT").unwrap_or("unknown");
        let api_version = option_env!("AR_TRANSLATE_AZURE_API_VERSION").unwrap_or("unknown");
        let deployment = option_env!("AR_TRANSLATE_AZURE_API_DEPLOYMENT_NAME").unwrap_or("unknown");

        Ok(Self {
            azure_api_key: api_key.to_string(),
            azure_api_endpoint: api_endpoint.to_string(),
            azure_api_version: api_version.to_string(),
            azure_api_deployment: deployment.to_string(),
        })
    }
}
