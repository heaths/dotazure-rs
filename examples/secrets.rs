// Copyright 2025 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::SecretClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotazure::load()?;

    let endpoint = env::var("AZURE_KEYVAULT_URL")?;
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(&endpoint, credential.clone(), None)?;

    let secret = client
        .get_secret("my-secret", "", None)
        .await?
        .into_body()
        .await?;
    let secret = secret
        .value
        .as_ref()
        .map_or_else(|| "(none)", |v| v.as_str());

    println!("{}", secret);

    Ok(())
}
