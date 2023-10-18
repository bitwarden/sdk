use crate::{
    error::Result,
    tool::generators::password::{
        passphrase, password, PassphraseGeneratorRequest, PasswordGeneratorRequest,
    },
    Client,
};

pub struct ClientGenerator<'a> {
    pub(crate) _client: &'a crate::Client,
}

impl<'a> ClientGenerator<'a> {
    /// Generates a random password.
    /// A passphrase is a combination of random words separated by a character.
    /// An example of passphrase is `correct horse battery staple`.
    ///
    /// By default, the password contains lowercase 16 characters, but the character
    /// sets and password length can be customized using the `input` parameter.
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
    ///         length: Some(20),
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

    pub async fn passphrase(&self, input: PassphraseGeneratorRequest) -> Result<String> {
        passphrase(input)
    }
}

impl<'a> Client {
    pub fn generator(&'a self) -> ClientGenerator<'a> {
        ClientGenerator { _client: self }
    }
}
