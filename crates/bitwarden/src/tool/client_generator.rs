use bitwarden_generators::{passphrase, password, username};

use crate::{
    error::Result,
    generators::{PassphraseGeneratorRequest, PasswordGeneratorRequest, UsernameGeneratorRequest},
    Client,
};

pub struct ClientGenerator<'a> {
    pub(crate) client: &'a crate::Client,
}

impl<'a> ClientGenerator<'a> {
    /// Generates a random password.
    ///
    /// The character sets and password length can be customized using the `input` parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitwarden::{Client, generators::PasswordGeneratorRequest, error::Result};
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
        Ok(password(input)?)
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
    /// use bitwarden::{Client, generators::PassphraseGeneratorRequest, error::Result};
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
        Ok(passphrase(input)?)
    }

    /// Generates a random username.
    /// There are different username generation strategies, which can be customized using the
    /// `input` parameter.
    ///
    /// Note that most generation strategies will be executed on the client side, but `Forwarded`
    /// will use third-party services, which may require a specific setup or API key.
    ///
    /// ```
    /// use bitwarden::{Client, generators::{UsernameGeneratorRequest}, error::Result};
    /// async fn test() -> Result<()> {
    ///     let input = UsernameGeneratorRequest::Word {
    ///         capitalize: true,
    ///         include_number: true,
    ///     };
    ///     let username = Client::new(None).generator().username(input).await.unwrap();
    ///     println!("{}", username);
    ///     Ok(())
    /// }
    /// ```
    pub async fn username(&self, input: UsernameGeneratorRequest) -> Result<String> {
        Ok(username(input, self.client.get_http_client()).await?)
    }
}

impl<'a> Client {
    pub fn generator(&'a self) -> ClientGenerator<'a> {
        ClientGenerator { client: self }
    }
}
