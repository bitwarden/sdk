use crate::{
    error::Result,
    tool::generators::passphrase::{passphrase, PassphraseGeneratorRequest},
    tool::generators::password::{password, PasswordGeneratorRequest},
    Client,
};

pub struct ClientGenerator<'a> {
    pub(crate) _client: &'a crate::Client,
}

impl<'a> ClientGenerator<'a> {
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
        let options = input.validate_options()?;
        Ok(passphrase(options))
    }
}

impl<'a> Client {
    pub fn generator(&'a self) -> ClientGenerator<'a> {
        ClientGenerator { _client: self }
    }
}
