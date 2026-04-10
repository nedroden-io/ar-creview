use crate::app_settings::AppConfig;
use anyhow::{Result, anyhow};

pub struct AzureClient<'a> {
    api_config: &'a AppConfig,

    http_client: reqwest::Client,
}

impl<'a> AzureClient<'a> {
    pub fn new(api_config: &'a AppConfig) -> Self {
        Self {
            api_config,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn send_openai_request<T: serde::de::DeserializeOwned>(
        &self,
        body: &impl serde::Serialize,
    ) -> Result<T> {
        let target_url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            self.api_config.azure_api_endpoint,
            self.api_config.azure_api_deployment,
            self.api_config.azure_api_version
        );

        let response = self
            .http_client
            .post(target_url)
            .bearer_auth(&self.api_config.azure_api_key)
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Azure API request failed with status: {}",
                response.status()
            ));
        }

        Ok(response.json::<T>().await?)
    }
}
