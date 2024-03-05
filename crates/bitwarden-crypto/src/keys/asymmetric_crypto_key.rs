use std::pin::Pin;

use rsa::{pkcs8::DecodePublicKey, RsaPrivateKey, RsaPublicKey};

use super::key_encryptable::CryptoKey;
use crate::error::{CryptoError, Result};

/// Trait to allow both [`AsymmetricCryptoKey`] and [`AsymmetricPublicCryptoKey`] to be used to
/// encrypt [AsymmetricEncString](crate::AsymmetricEncString).
pub trait AsymmetricEncryptable {
    fn to_public_key(&self) -> &RsaPublicKey;
}

/// An asymmetric public encryption key. Can only encrypt
/// [AsymmetricEncString](crate::AsymmetricEncString), usually accompanied by a
/// [AsymmetricCryptoKey]
pub struct AsymmetricPublicCryptoKey {
    pub(crate) key: RsaPublicKey,
}

impl AsymmetricPublicCryptoKey {
    /// Build a public key from the SubjectPublicKeyInfo DER.
    pub fn from_der(der: &[u8]) -> Result<Self> {
        Ok(Self {
            key: rsa::RsaPublicKey::from_public_key_der(der)
                .map_err(|_| CryptoError::InvalidKey)?,
        })
    }
}

impl AsymmetricEncryptable for AsymmetricPublicCryptoKey {
    fn to_public_key(&self) -> &RsaPublicKey {
        &self.key
    }
}

/// An asymmetric encryption key. Contains both the public and private key. Can be used to both
/// encrypt and decrypt [`AsymmetricEncString`](crate::AsymmetricEncString).
pub struct AsymmetricCryptoKey {
    // RsaPrivateKey is not a Copy type so this isn't completely necessary, but
    // to keep the compiler from making stack copies when moving this struct around,
    // we use a Box to keep the values on the heap. We also pin the box to make sure
    // that the contents can't be pulled out of the box and moved
    pub(crate) key: Pin<Box<RsaPrivateKey>>,
}

// Note that RsaPrivateKey already implements ZeroizeOnDrop, so we don't need to do anything
// We add this assertion to make sure that this is still true in the future
const _: () = {
    fn assert_zeroize_on_drop<T: zeroize::ZeroizeOnDrop>() {}
    fn assert_all() {
        assert_zeroize_on_drop::<RsaPrivateKey>();
    }
};

impl zeroize::ZeroizeOnDrop for AsymmetricCryptoKey {}

impl AsymmetricCryptoKey {
    /// Generate a random AsymmetricCryptoKey (RSA-2048).
    pub fn generate<R: rand::CryptoRng + rand::RngCore>(rng: &mut R) -> Self {
        let bits = 2048;

        Self {
            key: Box::pin(RsaPrivateKey::new(rng, bits).expect("failed to generate a key")),
        }
    }

    pub fn from_pem(pem: &str) -> Result<Self> {
        use rsa::pkcs8::DecodePrivateKey;
        Ok(Self {
            key: Box::pin(RsaPrivateKey::from_pkcs8_pem(pem).map_err(|_| CryptoError::InvalidKey)?),
        })
    }

    pub fn from_der(der: &[u8]) -> Result<Self> {
        use rsa::pkcs8::DecodePrivateKey;
        Ok(Self {
            key: Box::pin(RsaPrivateKey::from_pkcs8_der(der).map_err(|_| CryptoError::InvalidKey)?),
        })
    }

    pub fn to_der(&self) -> Result<Vec<u8>> {
        use rsa::pkcs8::EncodePrivateKey;
        Ok(self
            .key
            .to_pkcs8_der()
            .map_err(|_| CryptoError::InvalidKey)?
            .as_bytes()
            .to_owned())
    }

    pub fn to_public_der(&self) -> Result<Vec<u8>> {
        use rsa::pkcs8::EncodePublicKey;
        Ok(self
            .to_public_key()
            .to_public_key_der()
            .map_err(|_| CryptoError::InvalidKey)?
            .as_bytes()
            .to_owned())
    }
}

impl AsymmetricEncryptable for AsymmetricCryptoKey {
    fn to_public_key(&self) -> &RsaPublicKey {
        (*self.key).as_ref()
    }
}

impl CryptoKey for AsymmetricCryptoKey {}

// We manually implement these to make sure we don't print any sensitive data
impl std::fmt::Debug for AsymmetricCryptoKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsymmetricCryptoKey").finish()
    }
}

#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose::STANDARD, Engine};

    use crate::{
        AsymmetricCryptoKey, AsymmetricEncString, AsymmetricPublicCryptoKey, KeyDecryptable,
    };

    #[test]
    fn test_asymmetric_crypto_key() {
        let pem_key_str = "-----BEGIN PRIVATE KEY-----
MIIEwAIBADANBgkqhkiG9w0BAQEFAASCBKowggSmAgEAAoIBAQDiTQVuzhdygFz5
qv14i+XFDGTnDravzUQT1hPKPGUZOUSZ1gwdNgkWqOIaOnR65BHEnL0sp4bnuiYc
afeK2JAW5Sc8Z7IxBNSuAwhQmuKx3RochMIiuCkI2/p+JvUQoJu6FBNm8OoJ4Cwm
qqHGZESMfnpQDCuDrB3JdJEdXhtmnl0C48sGjOk3WaBMcgGqn8LbJDUlyu1zdqyv
b0waJf0iV4PJm2fkUl7+57D/2TkpbCqURVnZK1FFIEg8mr6FzSN1F2pOfktkNYZw
P7MSNR7o81CkRSCMr7EkIVa+MZYMBx106BMK7FXgWB7nbSpsWKxBk7ZDHkID2fam
rEcVtrzDAgMBAAECggEBAKwq9OssGGKgjhvUnyrLJHAZ0dqIMyzk+dotkLjX4gKi
szJmyqiep6N5sStLNbsZMPtoU/RZMCW0VbJgXFhiEp2YkZU/Py5UAoqw++53J+kx
0d/IkPphKbb3xUec0+1mg5O6GljDCQuiZXS1dIa/WfeZcezclW6Dz9WovY6ePjJ+
8vEBR1icbNKzyeINd6MtPtpcgQPHtDwHvhPyUDbKDYGbLvjh9nui8h4+ZUlXKuVR
jB0ChxiKV1xJRjkrEVoulOOicd5r597WfB2ghax3pvRZ4MdXemCXm3gQYqPVKach
vGU+1cPQR/MBJZpxT+EZA97xwtFS3gqwbxJaNFcoE8ECgYEA9OaeYZhQPDo485tI
1u/Z7L/3PNape9hBQIXoW7+MgcQ5NiWqYh8Jnj43EIYa0wM/ECQINr1Za8Q5e6KR
J30FcU+kfyjuQ0jeXdNELGU/fx5XXNg/vV8GevHwxRlwzqZTCg6UExUZzbYEQqd7
l+wPyETGeua5xCEywA1nX/D101kCgYEA7I6aMFjhEjO71RmzNhqjKJt6DOghoOfQ
TjhaaanNEhLYSbenFz1mlb21mW67ulmz162saKdIYLxQNJIP8ZPmxh4ummOJI8w9
ClHfo8WuCI2hCjJ19xbQJocSbTA5aJg6lA1IDVZMDbQwsnAByPRGpaLHBT/Q9Bye
KvCMB+9amXsCgYEAx65yXSkP4sumPBrVHUub6MntERIGRxBgw/drKcPZEMWp0FiN
wEuGUBxyUWrG3F69QK/gcqGZE6F/LSu0JvptQaKqgXQiMYJsrRvhbkFvsHpQyUcZ
UZL1ebFjm5HOxPAgrQaN/bEqxOwwNRjSUWEMzUImg3c06JIZCzbinvudtKECgYEA
kY3JF/iIPI/yglP27lKDlCfeeHSYxI3+oTKRhzSAxx8rUGidenJAXeDGDauR/T7W
pt3pGNfddBBK9Z3uC4Iq3DqUCFE4f/taj7ADAJ1Q0Vh7/28/IJM77ojr8J1cpZwN
Zy2o6PPxhfkagaDjqEeN9Lrs5LD4nEvDkr5CG1vOjmMCgYEAvIBFKRm31NyF8jLi
CVuPwC5PzrW5iThDmsWTaXFpB3esUsbICO2pEz872oeQS+Em4GO5vXUlpbbFPzup
PFhA8iMJ8TAvemhvc7oM0OZqpU6p3K4seHf6BkwLxumoA3vDJfovu9RuXVcJVOnf
DnqOsltgPomWZ7xVfMkm9niL2OA=
-----END PRIVATE KEY-----";

        let der_key_vec = STANDARD.decode("MIIEwAIBADANBgkqhkiG9w0BAQEFAASCBKowggSmAgEAAoIBAQDiTQVuzhdygFz5qv14i+XFDGTnDravzUQT1hPKPGUZOUSZ1gwdNgkWqOIaOnR65BHEnL0sp4bnuiYcafeK2JAW5Sc8Z7IxBNSuAwhQmuKx3RochMIiuCkI2/p+JvUQoJu6FBNm8OoJ4CwmqqHGZESMfnpQDCuDrB3JdJEdXhtmnl0C48sGjOk3WaBMcgGqn8LbJDUlyu1zdqyvb0waJf0iV4PJm2fkUl7+57D/2TkpbCqURVnZK1FFIEg8mr6FzSN1F2pOfktkNYZwP7MSNR7o81CkRSCMr7EkIVa+MZYMBx106BMK7FXgWB7nbSpsWKxBk7ZDHkID2famrEcVtrzDAgMBAAECggEBAKwq9OssGGKgjhvUnyrLJHAZ0dqIMyzk+dotkLjX4gKiszJmyqiep6N5sStLNbsZMPtoU/RZMCW0VbJgXFhiEp2YkZU/Py5UAoqw++53J+kx0d/IkPphKbb3xUec0+1mg5O6GljDCQuiZXS1dIa/WfeZcezclW6Dz9WovY6ePjJ+8vEBR1icbNKzyeINd6MtPtpcgQPHtDwHvhPyUDbKDYGbLvjh9nui8h4+ZUlXKuVRjB0ChxiKV1xJRjkrEVoulOOicd5r597WfB2ghax3pvRZ4MdXemCXm3gQYqPVKachvGU+1cPQR/MBJZpxT+EZA97xwtFS3gqwbxJaNFcoE8ECgYEA9OaeYZhQPDo485tI1u/Z7L/3PNape9hBQIXoW7+MgcQ5NiWqYh8Jnj43EIYa0wM/ECQINr1Za8Q5e6KRJ30FcU+kfyjuQ0jeXdNELGU/fx5XXNg/vV8GevHwxRlwzqZTCg6UExUZzbYEQqd7l+wPyETGeua5xCEywA1nX/D101kCgYEA7I6aMFjhEjO71RmzNhqjKJt6DOghoOfQTjhaaanNEhLYSbenFz1mlb21mW67ulmz162saKdIYLxQNJIP8ZPmxh4ummOJI8w9ClHfo8WuCI2hCjJ19xbQJocSbTA5aJg6lA1IDVZMDbQwsnAByPRGpaLHBT/Q9ByeKvCMB+9amXsCgYEAx65yXSkP4sumPBrVHUub6MntERIGRxBgw/drKcPZEMWp0FiNwEuGUBxyUWrG3F69QK/gcqGZE6F/LSu0JvptQaKqgXQiMYJsrRvhbkFvsHpQyUcZUZL1ebFjm5HOxPAgrQaN/bEqxOwwNRjSUWEMzUImg3c06JIZCzbinvudtKECgYEAkY3JF/iIPI/yglP27lKDlCfeeHSYxI3+oTKRhzSAxx8rUGidenJAXeDGDauR/T7Wpt3pGNfddBBK9Z3uC4Iq3DqUCFE4f/taj7ADAJ1Q0Vh7/28/IJM77ojr8J1cpZwNZy2o6PPxhfkagaDjqEeN9Lrs5LD4nEvDkr5CG1vOjmMCgYEAvIBFKRm31NyF8jLiCVuPwC5PzrW5iThDmsWTaXFpB3esUsbICO2pEz872oeQS+Em4GO5vXUlpbbFPzupPFhA8iMJ8TAvemhvc7oM0OZqpU6p3K4seHf6BkwLxumoA3vDJfovu9RuXVcJVOnfDnqOsltgPomWZ7xVfMkm9niL2OA=").unwrap();

        // Load the two different formats and check they are the same key
        let pem_key = AsymmetricCryptoKey::from_pem(pem_key_str).unwrap();
        let der_key = AsymmetricCryptoKey::from_der(&der_key_vec).unwrap();
        assert_eq!(pem_key.key, der_key.key);

        // Check that the keys can be converted back to DER
        assert_eq!(der_key.to_der().unwrap(), der_key_vec);
        assert_eq!(pem_key.to_der().unwrap(), der_key_vec);
    }

    #[test]
    fn test_encrypt_public_decrypt_private() {
        let private_key = STANDARD
            .decode(concat!(
                "MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCu9xd+vmkIPoqH",
                "NejsFZzkd1xuCn1TqGTT7ANhAEnbI/yaVt3caI30kwUC2WIToFpNgu7Ej0x2TteY",
                "OgrLrdcC4jy1SifmKYv/v3ZZxrd/eqttmH2k588panseRwHK3LVk7xA+URhQ/bjL",
                "gPM59V0uR1l+z1fmooeJPFz5WSXNObc9Jqnh45FND+U/UYHXTLSomTn7jgZFxJBK",
                "veS7q6Lat7wAnYZCF2dnPmhZoJv+SKPltA8HAGsgQGWBF1p5qxV1HrAUk8kBBnG2",
                "paj0w8p5UM6RpDdCuvKH7j1LiuWffn3b9Z4dgzmE7jsMmvzoQtypzIKaSxhqzvFO",
                "od9V8dJdAgMBAAECggEAGGIYjOIB1rOKkDHP4ljXutI0mCRPl3FMDemiBeppoIfZ",
                "G/Q3qpAKmndDt0Quwh/yfcNdvZhf1kwCCTWri/uPz5fSUIyDV3TaTRu0ZWoHaBVj",
                "Hxylg+4HRZUQj+Vi50/PWr/jQmAAVMcrMfcoTl82q2ynmP/R1vM3EsXOCjTliv5B",
                "XlMPRjj/9PDBH0dnnVcAPDOpflzOTL2f4HTFEMlmg9/tZBnd96J/cmfhjAv9XpFL",
                "FBAFZzs5pz0rwCNSR8QZNonnK7pngVUlGDLORK58y84tGmxZhGdne3CtCWey/sJ4",
                "7QF0Pe8YqWBU56926IY6DcSVBuQGZ6vMCNlU7J8D2QKBgQDXyh3t2TicM/n1QBLk",
                "zLoGmVUmxUGziHgl2dnJiGDtyOAU3+yCorPgFaCie29s5qm4b0YEGxUxPIrRrEro",
                "h0FfKn9xmr8CdmTPTcjJW1+M7bxxq7oBoU/QzKXgIHlpeCjjnvPJt0PcNkNTjCXv",
                "shsrINh2rENoe/x79eEfM/N5eQKBgQDPkYSmYyALoNq8zq0A4BdR+F5lb5Fj5jBH",
                "Jk68l6Uti+0hRbJ2d1tQTLkU+eCPQLGBl6fuc1i4K5FV7v14jWtRPdD7wxrkRi3j",
                "ilqQwLBOU6Bj3FK4DvlLF+iYTuBWj2/KcxflXECmsjitKHLK6H7kFEiuJql+NAHU",
                "U9EFXepLBQKBgQDQ+HCnZ1bFHiiP8m7Zl9EGlvK5SwlnPV9s+F1KJ4IGhCNM09UM",
                "ZVfgR9F5yCONyIrPiyK40ylgtwqQJlOcf281I8irUXpsfg7+Gou5Q31y0r9NLUpC",
                "Td8niyePtqMdGjouxD2+OHXFCd+FRxFt4IMi7vnxYr0csAVAXkqWlw7PsQKBgH/G",
                "/PnQm7GM3BrOwAGB8dksJDAddkshMScblezTDYP0V43b8firkTLliCo5iNum357/",
                "VQmdSEhXyag07yR/Kklg3H2fpbZQ3X7tdMMXW3FcWagfwWw9C4oGtdDM/Z1Lv23J",
                "XDR9je8QV4OBGul+Jl8RfYx3kG94ZIfo8Qt0vP5hAoGARjAzdCGYz42NwaUk8n94",
                "W2RuKHtTV9vtjaAbfPFbZoGkT7sXNJVlrA0C+9f+H9rOTM3mX59KrjmLVzde4Vhs",
                "avWMShuK4vpAiDQLU7GyABvi5CR6Ld+AT+LSzxHhVe0ASOQPNCA2SOz3RQvgPi7R",
                "GDgRMUB6cL3IRVzcR0dC6cY=",
            ))
            .unwrap();

        let public_key = STANDARD
            .decode(concat!(
                "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEArvcXfr5pCD6KhzXo7BWc",
                "5Hdcbgp9U6hk0+wDYQBJ2yP8mlbd3GiN9JMFAtliE6BaTYLuxI9Mdk7XmDoKy63X",
                "AuI8tUon5imL/792Wca3f3qrbZh9pOfPKWp7HkcByty1ZO8QPlEYUP24y4DzOfVd",
                "LkdZfs9X5qKHiTxc+VklzTm3PSap4eORTQ/lP1GB10y0qJk5+44GRcSQSr3ku6ui",
                "2re8AJ2GQhdnZz5oWaCb/kij5bQPBwBrIEBlgRdaeasVdR6wFJPJAQZxtqWo9MPK",
                "eVDOkaQ3Qrryh+49S4rln3592/WeHYM5hO47DJr86ELcqcyCmksYas7xTqHfVfHS",
                "XQIDAQAB",
            ))
            .unwrap();

        let private_key = AsymmetricCryptoKey::from_der(&private_key).unwrap();
        let public_key = AsymmetricPublicCryptoKey::from_der(&public_key).unwrap();

        let plaintext = "Hello, world!";
        let encrypted =
            AsymmetricEncString::encrypt_rsa2048_oaep_sha1(plaintext.as_bytes(), &public_key)
                .unwrap();
        let decrypted: String = encrypted.decrypt_with_key(&private_key).unwrap();

        assert_eq!(plaintext, decrypted);
    }
}
