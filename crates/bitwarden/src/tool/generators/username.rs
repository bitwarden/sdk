use crate::{error::Result, wordlist::EFF_LONG_WORD_LIST};
use rand::{seq::SliceRandom, Rng, RngCore};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Enum))]
pub enum AddressType {
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
pub enum UsernameGeneratorType {
    /// Generates a single word username
    Word {
        /// When set to true, capitalizes the first letter of the word. Defaults to false
        capitalize: Option<bool>,
        /// When set to true, includes a 4 digit number at the end of the word. Defaults to false
        include_number: Option<bool>,
    },
    /// Generates an email using your provider's subaddressing capabilities.
    /// Note that not all providers support this functionality.
    /// This will generate an address of the format `youremail+generated@domain.tld`
    Subaddress {
        /// The type of subaddress to add to the base email
        r#type: AddressType,
        /// The full email address to use as the base for the subaddress
        email: String,
    },
    Catchall {
        /// The type of username to use with the catchall email domain
        r#type: AddressType,
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

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(feature = "mobile", derive(uniffi::Record))]
pub struct UsernameGeneratorRequest {
    pub r#type: UsernameGeneratorType,
}

pub(super) async fn username(input: UsernameGeneratorRequest) -> Result<String> {
    use rand::thread_rng;
    use UsernameGeneratorType::*;

    match input.r#type {
        Word {
            capitalize,
            include_number,
        } => {
            let capitalize = capitalize.unwrap_or(true);
            let include_number = include_number.unwrap_or(true);
            Ok(username_word(&mut thread_rng(), capitalize, include_number))
        }
        Subaddress { r#type, email } => Ok(username_subaddress(&mut thread_rng(), r#type, email)),
        Catchall { r#type, domain } => Ok(username_catchall(&mut thread_rng(), r#type, domain)),
        Forwarded { service, website } => {
            use crate::tool::generators::username_forwarders::*;
            use ForwarderServiceType::*;
            match service {
                AddyIo {
                    api_token,
                    domain,
                    base_url,
                } => addyio::generate(api_token, domain, base_url, website).await,
                DuckDuckGo { token } => duckduckgo::generate(token).await,
                Firefox { api_token } => firefox::generate(api_token, website).await,
                Fastmail { api_token } => fastmail::generate(api_token, website).await,
                ForwardEmail { api_token, domain } => {
                    forwardemail::generate(api_token, domain, website).await
                }
                SimpleLogin { api_key } => simplelogin::generate(api_key, website).await,
            }
        }
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

fn username_subaddress(mut rng: impl RngCore, r#type: AddressType, email: String) -> String {
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
        AddressType::Random => random_lowercase_string(&mut rng, 8),
        AddressType::WebsiteName { website } => website,
    };

    format!("{}+{}@{}", email_begin, email_middle, email_end)
}

fn username_catchall(mut rng: impl RngCore, r#type: AddressType, domain: String) -> String {
    if domain.is_empty() {
        return domain;
    }

    let email_start = match r#type {
        AddressType::Random => random_lowercase_string(&mut rng, 8),
        AddressType::WebsiteName { website } => website,
    };

    format!("{}@{}", email_start, domain)
}

fn random_number(mut rng: impl RngCore) -> String {
    let num = rng.gen_range(0..=9999);
    format!("{num:0>4}")
}

fn random_lowercase_string(mut rng: impl RngCore, length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz1234567890";
    (0..length)
        .map(|_| (*CHARSET.choose(&mut rng).expect("slice is not empty")) as char)
        .collect()
}

fn capitalize_first_letter(s: &str) -> String {
    // Unicode case conversion can change the length of the string, so we can't capitalize in place.
    // Instead we extract the first character and convert it to uppercase. This returns
    // an iterator which we collect into a string, and then append the rest of the input.
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
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
        let user = username_subaddress(&mut rng, AddressType::Random, "demo@test.com".into());
        assert_eq!(user, "demo+52iteqjo@test.com");

        let user = username_subaddress(
            &mut rng,
            AddressType::WebsiteName {
                website: "bitwarden.com".into(),
            },
            "demo@test.com".into(),
        );
        assert_eq!(user, "demo+bitwarden.com@test.com");
    }

    #[test]
    fn test_username_catchall() {
        let mut rng = rand_chacha::ChaCha8Rng::from_seed([0u8; 32]);
        let user = username_catchall(&mut rng, AddressType::Random, "test.com".into());
        assert_eq!(user, "52iteqjo@test.com");

        let user = username_catchall(
            &mut rng,
            AddressType::WebsiteName {
                website: "bitwarden.com".into(),
            },
            "test.com".into(),
        );
        assert_eq!(user, "bitwarden.com@test.com");
    }
}
