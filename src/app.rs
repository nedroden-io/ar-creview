use crate::{app_settings, azure, cli, git};

pub async fn run() -> anyhow::Result<()> {
    let app_config = app_settings::AppConfig::load()?;
    let run_config = cli::parse_args()?;
    let azure_client = azure::AzureClient::new(&app_config);

    let client = git::GitClient::new(run_config.target.to_str().unwrap());
    client.stage_changes()?;
    let diff = client.get_diff_with_main()?;

    println!("{}", generate_code_review(&azure_client, &diff).await?);

    Ok(())
}

async fn generate_code_review<'a>(
    azure_client: &azure::AzureClient<'a>,
    diff: &str,
) -> anyhow::Result<String> {
    let response = azure_client
        .send_openai_request::<OpenAiResponse>(
            &OpenAiRequest {
                messages: vec![
                    Message {
                        role: String::from("system"),
                        content: String::from("I want you to review the code changes in the following git diff, as if it were a pull request. Do not provide any additional explanations or comments."),
                    },
                    Message {
                        role: String::from("user"),
                        content: diff.to_string()
                    }
                ],
                max_tokens: 4096,
                temperature: 0,
                top_p: 1,
                model: "gpt-4o".to_string(),
            },
        )
        .await?;

    Ok(response.choices.first().unwrap().message.content.clone())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct OpenAiRequest {
    pub messages: Vec<Message>,
    pub max_tokens: i64,
    pub temperature: i64,
    pub top_p: i64,
    pub model: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Message {
    pub role: String,
    pub content: String,
}
#[derive(serde::Serialize, serde::Deserialize)]
struct OpenAiCompletion {
    pub content: String,
    pub role: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Choice {
    pub index: i64,
    pub message: OpenAiCompletion,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct OpenAiResponse {
    pub choices: Vec<Choice>,
}