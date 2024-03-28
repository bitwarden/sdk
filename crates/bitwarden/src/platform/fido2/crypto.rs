use crate::error::{Error, Result};
use coset::{
    iana::{self, EnumI64},
    CoseKey,
};
use p256::{pkcs8::EncodePrivateKey, SecretKey};
use passkey::types::{ctap2::Ctap2Error, Bytes};

pub fn cose_key_to_pkcs8(cose_key: &CoseKey) -> Result<Bytes> {
    // cose_key.
    let secret_key = private_key_from_cose_key(cose_key).map_err(|error| {
        log::error!("Failed to extract private key from cose_key: {:?}", error);
        Error::Internal("Failed to extract private key from cose_key".into())
    })?;

    let vec = secret_key
        .to_pkcs8_der()
        .map_err(|error| {
            log::error!("Failed to convert P256 private key to PKC8: {:?}", error);
            Error::Internal("Failed to convert P256 private key to PKC8".into())
        })?
        .as_bytes()
        .to_vec();

    Ok(Bytes::from(vec))
}

/// Copied from `passkey-rs`
/// Extract a cryptographic secret key from a [`CoseKey`].
// possible candidate for a `passkey-crypto` crate?
/*
MIT License

Copyright (c) 2022 1Password

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
fn private_key_from_cose_key(key: &CoseKey) -> Result<SecretKey, Ctap2Error> {
    if !matches!(
        key.alg,
        Some(coset::RegisteredLabelWithPrivate::Assigned(
            iana::Algorithm::ES256
        ))
    ) {
        return Err(Ctap2Error::UnsupportedAlgorithm);
    }
    if !matches!(
        key.kty,
        coset::RegisteredLabel::Assigned(iana::KeyType::EC2)
    ) {
        return Err(Ctap2Error::InvalidCredential);
    }

    key.params
        .iter()
        .find_map(|(k, v)| {
            if let coset::Label::Int(i) = k {
                iana::Ec2KeyParameter::from_i64(*i)
                    .filter(|p| p == &iana::Ec2KeyParameter::D)
                    .and_then(|_| v.as_bytes())
                    .and_then(|b| SecretKey::from_slice(b).ok())
            } else {
                None
            }
        })
        .ok_or(Ctap2Error::InvalidCredential)
}
