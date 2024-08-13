use bitwarden_core::Client;

use crate::{
    passphrase, password, username, PassphraseError, PassphraseGeneratorRequest, PasswordError,
    PasswordGeneratorRequest, UsernameError, UsernameGeneratorRequest,
};

pub struct ClientGenerator<'a> {
    client: &'a Client,
}

impl<'a> ClientGenerator<'a> {
    fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Generates a random password.
    ///
    /// The character sets and password length can be customized using the `input` parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use bitwarden_core::Client;
    /// use bitwarden_generators::{ClientGeneratorExt, PassphraseError, PasswordGeneratorRequest};
    ///
    /// async fn test() -> Result<(), PassphraseError> {
    ///     let input = PasswordGeneratorRequest {
    ///         lowercase: true,
    ///         uppercase: true,
    ///         numbers: true,
    ///         length: 20,
    ///         ..Default::default()
    ///     };
    ///     let password = Client::new(None).await.generator().password(input).unwrap();
    ///     println!("{}", password);
    ///     Ok(())
    /// }
    /// ```
    pub fn password(&self, input: PasswordGeneratorRequest) -> Result<String, PasswordError> {
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
    /// use bitwarden_core::Client;
    /// use bitwarden_generators::{ClientGeneratorExt, PassphraseError, PassphraseGeneratorRequest};
    ///
    /// async fn test() -> Result<(), PassphraseError> {
    ///     let input = PassphraseGeneratorRequest {
    ///         num_words: 4,
    ///         ..Default::default()
    ///     };
    ///     let passphrase = Client::new(None).await.generator().passphrase(input).unwrap();
    ///     println!("{}", passphrase);
    ///     Ok(())
    /// }
    /// ```
    pub fn passphrase(&self, input: PassphraseGeneratorRequest) -> Result<String, PassphraseError> {
        passphrase(input)
    }

    /// Generates a random username.
    /// There are different username generation strategies, which can be customized using the
    /// `input` parameter.
    ///
    /// Note that most generation strategies will be executed on the client side, but `Forwarded`
    /// will use third-party services, which may require a specific setup or API key.
    ///
    /// ```
    /// use bitwarden_core::Client;
    /// use bitwarden_generators::{ClientGeneratorExt, UsernameError, UsernameGeneratorRequest};
    ///
    /// async fn test() -> Result<(), UsernameError> {
    ///     let input = UsernameGeneratorRequest::Word {
    ///         capitalize: true,
    ///         include_number: true,
    ///     };
    ///     let username = Client::new(None).await.generator().username(input).await.unwrap();
    ///     println!("{}", username);
    ///     Ok(())
    /// }
    /// ```
    pub async fn username(&self, input: UsernameGeneratorRequest) -> Result<String, UsernameError> {
        username(input, self.client.internal.get_http_client()).await
    }
}

pub trait ClientGeneratorExt<'a> {
    fn generator(&'a self) -> ClientGenerator<'a>;
}

impl<'a> ClientGeneratorExt<'a> for Client {
    fn generator(&'a self) -> ClientGenerator<'a> {
        ClientGenerator::new(self)
    }
}
