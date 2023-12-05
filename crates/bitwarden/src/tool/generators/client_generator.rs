use crate::{
    error::Result,
    tool::generators::{
        passphrase::{passphrase, PassphraseGeneratorRequest},
        password::{password, PasswordGeneratorRequest},
        username::{username, UsernameGeneratorRequest},
    },
    Client,
};

pub struct ClientGenerator<'a> {
    pub(crate) _client: &'a crate::Client,
}

impl<'a> ClientGenerator<'a> {
    /// Generates a random password.
    ///
    /// The character sets and password length can be customized using the `input` parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitwarden::{Client, tool::PasswordGeneratorRequest, error::Result};
    /// async fn test() -> Result<()> {
    ///     let input = PasswordGeneratorRequest {
    ///         lowercase: true,
    ///         uppercase: true,
    ///         numbers: true,
    ///         length: 20,
    ///         ..Default::default()
    ///     };
    ///     let password = Client::new(None).generator().password(input).await.unwrap();
    ///     println!("{}", password);
    ///     Ok(())
    /// }
    /// ```
    pub async fn password(&self, input: PasswordGeneratorRequest) -> Result<String> {
        password(input)
    }

    /// Generates a random passphrase.
    /// A passphrase is a combination of random words separated by a character.
    /// An example of passphrase is `correct horse battery staple`.
    ///
    /// The number of words and their case, the word separator, and the inclusion of
    /// a number in the passphrase can be customized using the `input` parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitwarden::{Client, tool::PassphraseGeneratorRequest, error::Result};
    /// async fn test() -> Result<()> {
    ///     let input = PassphraseGeneratorRequest {
    ///         num_words: 4,
    ///         ..Default::default()
    ///     };
    ///     let passphrase = Client::new(None).generator().passphrase(input).await.unwrap();
    ///     println!("{}", passphrase);
    ///     Ok(())
    /// }
    /// ```
    pub async fn passphrase(&self, input: PassphraseGeneratorRequest) -> Result<String> {
        passphrase(input)
    }

    pub async fn username(&self, input: UsernameGeneratorRequest) -> Result<String> {
        username(input).await
    }
}

impl<'a> Client {
    pub fn generator(&'a self) -> ClientGenerator<'a> {
        ClientGenerator { _client: self }
    }
}
