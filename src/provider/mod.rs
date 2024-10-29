use async_trait::async_trait;
use openai::OpenAIProvider;
use phind::PhindProvider;

use crate::{git_commit::GitCommit, ProviderType};

pub mod openai;
pub mod phind;

#[async_trait]
pub trait AIProvider {
    async fn explain(&self, commit: GitCommit) -> Result<String, Box<dyn std::error::Error>>;
}

pub enum LumenProvider {
    OpenAI(Box<OpenAIProvider>),
    Phind(Box<PhindProvider>),
}

impl LumenProvider {
    pub fn new(
        client: reqwest::Client,
        provider_type: ProviderType,
        api_key: Option<String>,
    ) -> Self {
        match provider_type {
            ProviderType::OpenAI => {
                let api_key = api_key.expect(
                    "api_key will always be Some when provider is OpenAI due to required_if_eq",
                );
                LumenProvider::OpenAI(Box::new(OpenAIProvider::new(client, api_key)))
            }
            ProviderType::Phind => LumenProvider::Phind(Box::new(PhindProvider::new(client, None))),
        }
    }
}

#[async_trait]
impl AIProvider for LumenProvider {
    async fn explain(&self, commit: GitCommit) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            LumenProvider::OpenAI(provider) => provider.explain(commit).await,
            LumenProvider::Phind(provider) => provider.explain(commit).await,
        }
    }
}
