// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Import the TokenCredential trait to bring the get_token method into scope
use azure_core::credentials::TokenCredential;
use azure_identity::AzureCliCredential;
use std::error::Error;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Retrieve the Azure Subscription ID from environment variables.
    // The program will panic if this variable is not set.
    let subscription_id =
        std::env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID required");

    // Create credentials using Azure CLI authentication.
    // This requires the user to be logged in via `az login`.
    let credentials = AzureCliCredential::new(None)?;

    // Define the scope for the token request.
    // ".default" scope requests tokens for the permissions consented for the application.
    let scopes = &["https://management.azure.com/.default"];

    // Request an access token using the credentials and defined scope.
    // The `get_token` method is available because we imported the `TokenCredential` trait.
    let res = credentials.get_token(scopes).await?;

    // Print the token response details for debugging purposes.
    // Note: Be careful logging tokens in production environments.
    eprintln!("Azure CLI response == {res:?}");

    // Construct the URL for the Azure Resource Manager (ARM) API
    // to list storage accounts within the specified subscription.
    let url = Url::parse(&format!(
        "https://management.azure.com/subscriptions/{subscription_id}/providers/Microsoft.Storage/storageAccounts?api-version=2019-06-01"
    ))?;

    // Create a new reqwest HTTP client.
    let client = reqwest::Client::new();

    // Send a GET request to the ARM API endpoint.
    // Include the obtained access token in the Authorization header.
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {}", res.token.secret())) // Use the token's secret value
        .send()
        .await? // Send the request and await the response
        .text() // Read the response body as text
        .await?; // Await the text reading operation

    // Print the response body received from the ARM API.
    println!("{resp}");

    // Return Ok to indicate successful execution.
    Ok(())
}
