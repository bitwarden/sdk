use bitwarden_crypto::EFF_LONG_WORD_LIST;
use rand::{distributions::Distribution, seq::SliceRandom, Rng, RngCore};
use reqwest::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::util::capitalize_first_letter;

#[derive(Debug, Error)]
pub enum UsernameError {
    #[error("Invalid API Key")]
    InvalidApiKey,
    #[error("Unknown error")]
    Unknown,

    #[error("Received error message from server: [{}] {}", .status, .message)]
    ResponseContent { status: StatusCode, message: String },

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum AppendType {
    /// Generates a random string of 8 lowercase characters as part of your username
    Random,
    /// Uses the websitename as part of your username
    WebsiteName { website: String },
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
/// Configures the email forwarding service to use.
/// For instructions on how to configure each service, see the documentation:
/// <https://bitwarden.com/help/generator/#username-types>
pub enum ForwarderServiceType {
    /// Previously known as "AnonAddy"
    AddyIo {
        api_token: String,
        domain: String,
        base_url: String,
    },
    DuckDuckGo {
        token: String,
    },
    Firefox {
        api_token: String,
    },
    Fastmail {
        api_token: String,
    },
    ForwardEmail {
        api_token: String,
        domain: String,
    },
    SimpleLogin {
        api_key: String,
    },
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum UsernameGeneratorRequest {
    /// Generates a single word username
    Word {
        /// Capitalize the first letter of the word
        capitalize: bool,
        /// Include a 4 digit number at the end of the word
        include_number: bool,
    },
    /// Generates an email using your provider's subaddressing capabilities.
    /// Note that not all providers support this functionality.
    /// This will generate an address of the format `youremail+generated@domain.tld`
    Subaddress {
        /// The type of subaddress to add to the base email
        r#type: AppendType,
        /// The full email address to use as the base for the subaddress
        email: String,
    },
    Catchall {
        /// The type of username to use with the catchall email domain
        r#type: AppendType,
        /// The domain to use for the catchall email address
        domain: String,
    },
    Forwarded {
        /// The email forwarding service to use, see [ForwarderServiceType]
        /// for instructions on how to configure each
        service: ForwarderServiceType,
        /// The website for which the email address is being generated
        /// This is not used in all services, and is only used for display purposes
        website: Option<String>,
    },
}

impl ForwarderServiceType {
    // Generate a username using the specified email forwarding service
    // This requires an HTTP client to be passed in, as the service will need to make API calls
    pub async fn generate(
        self,
        http: &reqwest::Client,
        website: Option<String>,
    ) -> Result<String, UsernameError> {
        use ForwarderServiceType::*;

        use crate::username_forwarders::*;

        match self {
            AddyIo {
                api_token,
                domain,
                base_url,
            } => addyio::generate(http, api_token, domain, base_url, website).await,
            DuckDuckGo { token } => duckduckgo::generate(http, token).await,
            Firefox { api_token } => firefox::generate(http, api_token, website).await,
            Fastmail { api_token } => fastmail::generate(http, api_token, website).await,
            ForwardEmail { api_token, domain } => {
                forwardemail::generate(http, api_token, domain, website).await
            }
            SimpleLogin { api_key } => simplelogin::generate(http, api_key, website).await,
        }
    }
}

/// Implementation of the username generator.
///
/// Note: The HTTP client is passed in as a required parameter for convenience,
/// as some username generators require making API calls.
pub async fn username(
    input: UsernameGeneratorRequest,
    http: &reqwest::Client,
) -> Result<String, UsernameError> {
    use rand::thread_rng;
    use UsernameGeneratorRequest::*;
    match input {
        Word {
            capitalize,
            include_number,
        } => Ok(username_word(&mut thread_rng(), capitalize, include_number)),
        Subaddress { r#type, email } => Ok(username_subaddress(&mut thread_rng(), r#type, email)),
        Catchall { r#type, domain } => Ok(username_catchall(&mut thread_rng(), r#type, domain)),
        Forwarded { service, website } => service.generate(http, website).await,
    }
}

fn username_word(mut rng: impl RngCore, capitalize: bool, include_number: bool) -> String {
    let word = EFF_LONG_WORD_LIST
        .choose(&mut rng)
        .expect("slice is not empty");

    let mut word = if capitalize {
        capitalize_first_letter(word)
    } else {
        word.to_string()
    };

    if include_number {
        word.push_str(&random_number(&mut rng));
    }

    word
}

/// Generate a random 4 digit number, including leading zeros
fn random_number(mut rng: impl RngCore) -> String {
    let num = rng.gen_range(0..=9999);
    format!("{num:0>4}")
}

/// Generate a username using a plus addressed email address
/// The format is <username>+<random-or-website>@<domain>
fn username_subaddress(mut rng: impl RngCore, r#type: AppendType, email: String) -> String {
    if email.len() < 3 {
        return email;
    }

    let (email_begin, email_end) = match email.find('@') {
        Some(pos) if pos > 0 && pos < email.len() - 1 => {
            email.split_once('@').expect("The email contains @")
        }
        _ => return email,
    };

    let email_middle = match r#type {
        AppendType::Random => random_lowercase_string(&mut rng, 8),
        AppendType::WebsiteName { website } => website,
    };

    format!("{}+{}@{}", email_begin, email_middle, email_end)
}

/// Generate a username using a catchall email address
/// The format is <random-or-website>@<domain>
fn username_catchall(mut rng: impl RngCore, r#type: AppendType, domain: String) -> String {
    if domain.is_empty() {
        return domain;
    }

    let email_start = match r#type {
        AppendType::Random => random_lowercase_string(&mut rng, 8),
        AppendType::WebsiteName { website } => website,
    };

    format!("{}@{}", email_start, domain)
}

fn random_lowercase_string(mut rng: impl RngCore, length: usize) -> String {
    const LOWERCASE_ALPHANUMERICAL: &[u8] = b"abcdefghijklmnopqrstuvwxyz1234567890";
    let dist = rand::distributions::Slice::new(LOWERCASE_ALPHANUMERICAL).expect("Non-empty slice");

    dist.sample_iter(&mut rng)
        .take(length)
        .map(|&b| b as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    pub use super::*;

    #[test]
    fn test_username_word() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);
        assert_eq!(username_word(&mut rng, true, true), "Subsystem6314");
        assert_eq!(username_word(&mut rng, true, false), "Silenced");
        assert_eq!(username_word(&mut rng, false, true), "dinginess4487");
    }

    #[test]
    fn test_username_subaddress() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);
        let user = username_subaddress(&mut rng, AppendType::Random, "demo@test.com".into());
        assert_eq!(user, "demo+5wiejdaj@test.com");

        let user = username_subaddress(
            &mut rng,
            AppendType::WebsiteName {
                website: "bitwarden.com".into(),
            },
            "demo@test.com".into(),
        );
        assert_eq!(user, "demo+bitwarden.com@test.com");
    }

    #[test]
    fn test_username_catchall() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([1u8; 32]);
        let user = username_catchall(&mut rng, AppendType::Random, "test.com".into());
        assert_eq!(user, "k9y6yw7j@test.com");

        let user = username_catchall(
            &mut rng,
            AppendType::WebsiteName {
                website: "bitwarden.com".into(),
            },
            "test.com".into(),
        );
        assert_eq!(user, "bitwarden.com@test.com");
    }
}
