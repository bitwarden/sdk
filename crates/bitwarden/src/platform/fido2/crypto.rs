use coset::{
    iana::{self, Algorithm, EnumI64},
    CoseKey, CoseKeyBuilder,
};
use p256::{ecdsa::SigningKey, pkcs8::EncodePrivateKey, SecretKey};
use passkey::types::ctap2::Ctap2Error;

use crate::error::{Error, Result};

pub fn cose_key_to_pkcs8(cose_key: &CoseKey) -> Result<Vec<u8>> {
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

    Ok(vec)
}

pub fn pkcs8_to_cose_key(secret_key: &[u8]) -> Result<CoseKey> {
    let secret_key = SecretKey::from_slice(secret_key).map_err(|error| {
        log::error!("Failed to extract private key from secret_key: {:?}", error);
        Error::Internal("Failed to extract private key from secret_key".into())
    })?;

    let cose_key_pair = CoseKeyPair::from_secret_key(&secret_key, iana::Algorithm::ES256);
    Ok(cose_key_pair.private)
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

// Allow unused to keep the code the same as passkey-rs
#[allow(unused)]
struct CoseKeyPair {
    public: CoseKey,
    private: CoseKey,
}

// Allow unwraps to keep the code the same as passkey-rs
#[allow(clippy::unwrap_used)]
impl CoseKeyPair {
    fn from_secret_key(private_key: &SecretKey, algorithm: Algorithm) -> Self {
        let public_key = SigningKey::from(private_key)
            .verifying_key()
            .to_encoded_point(false);
        // SAFETY: These unwraps are safe because the public_key above is not compressed (false
        // parameter) therefore x and y are guarateed to contain values.
        let x = public_key.x().unwrap().as_slice().to_vec();
        let y = public_key.y().unwrap().as_slice().to_vec();
        let private = CoseKeyBuilder::new_ec2_priv_key(
            iana::EllipticCurve::P_256,
            x.clone(),
            y.clone(),
            private_key.to_bytes().to_vec(),
        )
        .algorithm(algorithm)
        .build();
        let public = CoseKeyBuilder::new_ec2_pub_key(iana::EllipticCurve::P_256, x, y)
            .algorithm(algorithm)
            .build();

        Self { public, private }
    }
}

#[cfg(test)]
mod tests {
    use coset::iana;
    use p256::{
        ecdsa::{
            signature::{Signer, Verifier},
            SigningKey,
        },
        SecretKey,
    };
    use passkey::types::{ctap2::AuthenticatorData, rand::random_vec};

    use super::{private_key_from_cose_key, CoseKeyPair};

    #[test]
    fn private_key_cose_round_trip_sanity_check() {
        let private_key = {
            let mut rng = rand::thread_rng();
            SecretKey::random(&mut rng)
        };
        let CoseKeyPair {
            private: private_cose,
            ..
        } = CoseKeyPair::from_secret_key(&private_key, iana::Algorithm::ES256);
        let public_signing_key = SigningKey::from(&private_key);
        let public_key = public_signing_key.verifying_key();

        let auth_data = AuthenticatorData::new("future.1password.com", None);
        let mut signature_target = auth_data.to_vec();
        signature_target.extend(random_vec(32));

        let secret_key = private_key_from_cose_key(&private_cose).expect("to get a private key");

        let private_key = SigningKey::from(secret_key);
        let signature: p256::ecdsa::Signature = private_key.sign(&signature_target);

        public_key
            .verify(&signature_target, &signature)
            .expect("failed to verify signature")
    }
}
