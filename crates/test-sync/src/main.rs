use bitwarden::{
    auth::login::AccessTokenLoginRequest, client::client_settings::ClientSettings,
    secrets_manager::secrets::SecretsSyncRequest, Client,
};
use chrono::{DateTime, FixedOffset, Utc};
use color_eyre::eyre::Result;
use uuid::Uuid;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let identity_url = "http://localhost:33656".to_string();
    let api_url = " http://localhost:4000".to_string();
    let access_token = "access_token_goes_here".to_string();
    let organization_id: Uuid = "organization_id_goes_here"
        .parse()
        .expect("Failed to parse organization ID");

    let settings: ClientSettings = ClientSettings {
        identity_url,
        api_url,
        ..Default::default()
    };

    let mut client = bitwarden::Client::new(Some(settings));

    let asdf = client
        .auth()
        .login_access_token(&AccessTokenLoginRequest {
            access_token,
            state_file: None,
        })
        .await?;
    println!("{:?}", asdf);

    call_with_specific_date(&mut client, organization_id).await?;
    call_with_current_date(&mut client, organization_id).await?;
    call_with_no_date(&mut client, organization_id).await?;

    Ok(())
}

async fn call_with_specific_date(client: &mut Client, organization_id: Uuid) -> Result<()> {
    let last_sync_date =
        DateTime::<FixedOffset>::parse_from_rfc3339("2024-03-25T19:48:06.813330+00:00")
            .unwrap()
            .with_timezone(&Utc);

    call_sync(
        client,
        &SecretsSyncRequest {
            organization_id: organization_id,
            last_synced_date: Some(last_sync_date),
        },
    )
    .await?;

    Ok(())
}

async fn call_with_current_date(client: &mut Client, organization_id: Uuid) -> Result<()> {
    call_sync(
        client,
        &SecretsSyncRequest {
            organization_id: organization_id,
            last_synced_date: Some(Utc::now()),
        },
    )
    .await?;

    Ok(())
}

async fn call_with_no_date(client: &mut Client, organization_id: Uuid) -> Result<()> {
    call_sync(
        client,
        &SecretsSyncRequest {
            organization_id: organization_id,
            last_synced_date: None,
        },
    )
    .await?;

    Ok(())
}

async fn call_sync(client: &mut Client, request: &SecretsSyncRequest) -> Result<()> {
    let sync_response = client.secrets().sync(request).await?;

    if sync_response.has_changes {
        println!("{:?}", sync_response);
    } else {
        println!("{:?}", sync_response.has_changes);
    }

    Ok(())
}
