#[cfg(feature = "internal")]
fn load_json(path: &str) -> serde_json::Value {
    // The current working directory can be different depending on how you run your tests
    // Instead we use the location of the crate's Cargo.toml file as the root
    let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(path);
    serde_json::from_str(&std::fs::read_to_string(d).unwrap()).unwrap()
}

// TODO: Had to copy this from the bitwarden crate because it's not public, find a better place to put it
#[cfg(feature = "internal")]
async fn start_mock(mocks: Vec<wiremock::Mock>) -> (wiremock::MockServer, bitwarden::Client) {
    let server = wiremock::MockServer::start().await;

    for mock in mocks {
        server.register(mock).await;
    }

    let settings: bitwarden::client::client_settings::ClientSettings =
        bitwarden::client::client_settings::ClientSettings {
            identity_url: format!("http://{}/identity", server.address()),
            api_url: format!("http://{}/api", server.address()),
            user_agent: "Bitwarden Rust-SDK [TEST]".into(),
            device_type: bitwarden::client::client_settings::DeviceType::SDK,
        };

    (server, bitwarden::Client::new(Some(settings)))
}
#[cfg(feature = "internal")]
#[tokio::test]
async fn test_access_token_login() {
    use wiremock::{matchers, Mock, ResponseTemplate};

    use bitwarden::auth::login::PasswordLoginRequest;

    // Create the mock server with the necessary routes for this test
    let (_server, mut client) = start_mock(vec![
            Mock::given(matchers::path("/identity/accounts/prelogin"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({
                    "kdf":0,
                    "kdfIterations":100000,
                    "kdfMemory":null,
                    "kdfParallelism":null
                })
            )),
            Mock::given(matchers::path("/identity/connect/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!({
                    "access_token": "eyJhbGciOiJSUzI1NiIsImtpZCI6IkJCNTA4OTVGQkM2MTMxN0NDMzdDRTAxMEJGNjNCN0ExRjNERDczNzhSUzI1NiIsIng1dCI6InUxQ0pYN3hoTVh6RGZPQVF2Mk8zb2ZQZGMzZyIsInR5cCI6ImF0K2p3dCJ9.eyJpc3MiOiJodHRwOi8vbG9jYWxob3N0IiwibmJmIjoxNzAzMDgzMjExLCJpYXQiOjE3MDMwODMyMTEsImV4cCI6MjgwMzA4NjgxMSwic2NvcGUiOlsiYXBpIiwib2ZmbGluZV9hY2Nlc3MiXSwiYW1yIjpbIkFwcGxpY2F0aW9uIl0sImNsaWVudF9pZCI6IndlYiIsInN1YiI6Ijg4NDFiNThkLWM4ZjEtNGUyZC1iZTJiLWIwNTMwMGQyZjNkNSIsImF1dGhfdGltZSI6MTcwMzA4MzIxMSwiaWRwIjoiYml0d2FyZGVuIiwicHJlbWl1bSI6dHJ1ZSwiZW1haWwiOiJ0ZXN0QGJpdHdhcmRlbi5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwic3N0YW1wIjoiZGE1YjZiYTktOTAzMi00OGZkLWI2Y2EtNTdkZmRmMDlkNzZmIiwibmFtZSI6IlRlc3QiLCJkZXZpY2UiOiIwNzQ1ZDQyNi04ZGFiLTQ4NGEtOTgxNi00OTU5NzIxZDc3YzciLCJqdGkiOiI2QkRGNEVBNzlDMUFDRTE2QzJGRDNCOTM1NTcwMEE4MSJ9.C9g-XO_d1PmaKmHNLkdewmC4SMvf7VZAfqjqQvILBskA88595snpeQUh0QVe7pVwT3dDtgvpG-CkWAPMSjnS1yzzgWq0QDwE4BolMHKDE_rM1sLS09sff1gLenFeVnViADSCkOc5zrKs_SPKbgfvM1844pCZuDgeecrfE3Ld0jdmq2fOoWGcLoGzkIVTYqj9yH9b7KGvr5KagRkpg4dt7P27JXCn8T-Q76iYOQADTegpuSPumzSNfaNW1dmIvKKTOAAM0sB3gxczHpycyahBmg0cSJO8YDtQn4nGOIA_9q888CWdwnUyh2ivvpeXizpwZGICvkfDuzuF_bi-wSeBaQ",
                    "expires_in": 3600,
                    "token_type": "Bearer",
                    "refresh_token": "CE9F217EDC2852BE3A8CD034F0457FAB230571909E109D58CC82664D72AEE1E7-1",
                    "scope": "api offline_access",
                    "PrivateKey": "2.kmLY8NJVuiKBFJtNd/ZFpA==|qOodlRXER+9ogCe3yOibRHmUcSNvjSKhdDuztLlucs10jLiNoVVVAc+9KfNErLSpx5wmUF1hBOJM8zwVPjgQTrmnNf/wuDpwiaCxNYb/0v4FygPy7ccAHK94xP1lfqq7U9+tv+/yiZSwgcT+xF0wFpoxQeNdNRFzPTuD9o4134n8bzacD9DV/WjcrXfRjbBCzzuUGj1e78+A7BWN7/5IWLz87KWk8G7O/W4+8PtEzlwkru6Wd1xO19GYU18oArCWCNoegSmcGn7w7NDEXlwD403oY8Oa7ylnbqGE28PVJx+HLPNIdSC6YKXeIOMnVs7Mctd/wXC93zGxAWD6ooTCzHSPVV50zKJmWIG2cVVUS7j35H3rGDtUHLI+ASXMEux9REZB8CdVOZMzp2wYeiOpggebJy6MKOZqPT1R3X0fqF2dHtRFPXrNsVr1Qt6bS9qTyO4ag1/BCvXF3P1uJEsI812BFAne3cYHy5bIOxuozPfipJrTb5WH35bxhElqwT3y/o/6JWOGg3HLDun31YmiZ2HScAsUAcEkA4hhoTNnqy4O2s3yVbCcR7jF7NLsbQc0MDTbnjxTdI4VnqUIn8s2c9hIJy/j80pmO9Bjxp+LQ9a2hUkfHgFhgHxZUVaeGVth8zG2kkgGdrp5VHhxMVFfvB26Ka6q6qE/UcS2lONSv+4T8niVRJz57qwctj8MNOkA3PTEfe/DP/LKMefke31YfT0xogHsLhDkx+mS8FCc01HReTjKLktk/Jh9mXwC5oKwueWWwlxI935ecn+3I2kAuOfMsgPLkoEBlwgiREC1pM7VVX1x8WmzIQVQTHd4iwnX96QewYckGRfNYWz/zwvWnjWlfcg8kRSe+68EHOGeRtC5r27fWLqRc0HNcjwpgHkI/b6czerCe8+07TWql4keJxJxhBYj3iOH7r9ZS8ck51XnOb8tGL1isimAJXodYGzakwktqHAD7MZhS+P02O+6jrg7d+yPC2ZCuS/3TOplYOCHQIhnZtR87PXTUwr83zfOwAwCyv6KP84JUQ45+DItrXLap7nOVZKQ5QxYIlbThAO6eima6Zu5XHfqGPMNWv0bLf5+vAjIa5np5DJrSwz9no/hj6CUh0iyI+SJq4RGI60lKtypMvF6MR3nHLEHOycRUQbZIyTHWl4QQLdHzuwN9lv10ouTEvNr6sFflAX2yb6w3hlCo7oBytH3rJekjb3IIOzBpeTPIejxzVlh0N9OT5MZdh4sNKYHUoWJ8mnfjdM+L4j5Q2Kgk/XiGDgEebkUxiEOQUdVpePF5uSCE+TPav/9FIRGXGiFn6NJMaU7aBsDTFBLloffFLYDpd8/bTwoSvifkj7buwLYM+h/qcnfdy5FWau1cKav+Blq/ZC0qBpo658RTC8ZtseAFDgXoQZuksM10hpP9bzD04Bx30xTGX81QbaSTNwSEEVrOtIhbDrj9OI43KH4O6zLzK+t30QxAv5zjk10RZ4+5SAdYndIlld9Y62opCfPDzRy3ubdve4ZEchpIKWTQvIxq3T5ogOhGaWBVYnkMtM2GVqvWV//46gET5SH/MdcwhACUcZ9kCpMnWH9CyyUwYvTT3UlNyV+DlS27LMPvaw7tx7qa+GfNCoCBd8S4esZpQYK/WReiS8=|pc7qpD42wxyXemdNPuwxbh8iIaryrBPu8f/DGwYdHTw=",
                    "Key": "2.u2HDQ/nH2J7f5tYHctZx6Q==|NnUKODz8TPycWJA5svexe1wJIz2VexvLbZh2RDfhj5VI3wP8ZkR0Vicvdv7oJRyLI1GyaZDBCf9CTBunRTYUk39DbZl42Rb+Xmzds02EQhc=|rwuo5wgqvTJf3rgwOUfabUyzqhguMYb3sGBjOYqjevc=",
                    "MasterPasswordPolicy": null,
                    "ForcePasswordReset": false,
                    "ResetMasterPassword": false,
                    "Kdf": 0,
                    "KdfIterations": 100000,
                    "KdfMemory": null,
                    "KdfParallelism": null,
                    "UserDecryptionOptions": {
                        "HasMasterPassword": true,
                        "Object": "userDecryptionOptions"
                    }
                })
            )),

            Mock::given(matchers::path("/api/sync"))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                load_json("tests/sync_fixture_response.json")
            )),

        ]).await;

    let kdf = client
        .auth()
        .prelogin("test@bitwarden.com".into())
        .await
        .unwrap();

    let res = client
        .auth()
        .login_password(&PasswordLoginRequest {
            email: "test@bitwarden.com".into(),
            password: "asdfasdfasdf".into(),
            two_factor: None,
            kdf,
        })
        .await
        .unwrap();
    assert!(res.authenticated);

    let sync = client
        .sync(&bitwarden::platform::SyncRequest {
            exclude_subdomains: Some(true),
        })
        .await
        .unwrap();

    assert_eq!(sync.profile.organizations.len(), 1);

    let folders = client
        .vault()
        .folders()
        .decrypt_list(sync.folders)
        .await
        .unwrap();
    assert_eq!(folders.len(), 3);
    assert_eq!(folders[0].name, "FolderA");
    assert_eq!(folders[1].name, "FolderB");
    assert_eq!(folders[2].name, "FolderC");

    let collections = client
        .vault()
        .collections()
        .decrypt_list(sync.collections)
        .await
        .unwrap();

    assert_eq!(collections.len(), 2);
    assert_eq!(collections[0].name, "Default collection");
    assert_eq!(collections[1].name, "Default collection/New Collection");

    let mut iter = sync.ciphers.into_iter();
    macro_rules! next_cipher {
        ($client:ident, $iter:ident) => {
            $client
                .vault()
                .ciphers()
                .decrypt($iter.next().unwrap())
                .await
                .unwrap()
        };
    }

    let c1 = next_cipher!(client, iter);
    assert_eq!(c1.name, "Test Default Collection");

    let c2 = next_cipher!(client, iter);
    assert_eq!(c2.name, "Test New Collection");

    let c3 = next_cipher!(client, iter);
    assert_eq!(c3.name, "Google");
    assert_eq!(
        c3.deleted_date,
        Some("2023-12-20T14:46:13.320Z".parse().unwrap())
    );
    let c4 = next_cipher!(client, iter);
    assert_eq!(c4.name, "Test User A");

    let c5 = next_cipher!(client, iter);
    assert_eq!(c5.name, "Test Card B");

    let c6 = next_cipher!(client, iter);
    assert_eq!(c6.name, "Bruce Wayne");

    assert!(iter.next().is_none());
}
