use bitwarden_crypto::{HashPurpose, MasterKey};

use crate::{
    auth::determine_password_hash,
    client::{LoginMethod, UserLoginMethod},
    error::{Error, Result},
    Client,
};

/// Validate if the provided password matches the password hash stored in the client.
pub(crate) fn validate_password(
    client: &Client,
    password: String,
    password_hash: String,
) -> Result<bool> {
    let login_method = client
        .login_method
        .as_ref()
        .ok_or(Error::NotAuthenticated)?;

    if let LoginMethod::User(login_method) = login_method {
        match login_method {
            UserLoginMethod::Username { email, kdf, .. }
            | UserLoginMethod::ApiKey { email, kdf, .. } => {
                let hash = determine_password_hash(
                    email,
                    kdf,
                    &password,
                    HashPurpose::LocalAuthorization,
                )?;

                Ok(hash == password_hash)
            }
        }
    } else {
        Err(Error::NotAuthenticated)
    }
}

#[cfg(feature = "internal")]
pub(crate) fn validate_password_user_key(
    client: &Client,
    password: String,
    encrypted_user_key: String,
) -> Result<String> {
    let login_method = client
        .login_method
        .as_ref()
        .ok_or(Error::NotAuthenticated)?;

    if let LoginMethod::User(login_method) = login_method {
        match login_method {
            UserLoginMethod::Username { email, kdf, .. }
            | UserLoginMethod::ApiKey { email, kdf, .. } => {
                let master_key = MasterKey::derive(password.as_bytes(), email.as_bytes(), kdf)?;
                let user_key = master_key
                    .decrypt_user_key(encrypted_user_key.parse()?)
                    .map_err(|_| "wrong password")?;

                let enc = client
                    .get_encryption_settings()
                    .map_err(|_| Error::VaultLocked)?;

                let existing_key = enc.get_key(&None).ok_or(Error::VaultLocked)?;

                if user_key.to_vec() != existing_key.to_vec() {
                    return Err("wrong user key".into());
                }

                Ok(master_key
                    .derive_master_key_hash(password.as_bytes(), HashPurpose::LocalAuthorization)?)
            }
        }
    } else {
        Err(Error::NotAuthenticated)
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::password::{validate::validate_password_user_key, validate_password};

    #[test]
    fn test_validate_password() {
        use std::num::NonZeroU32;

        use crate::client::{Client, Kdf, LoginMethod, UserLoginMethod};

        let mut client = Client::new(None);
        client.set_login_method(LoginMethod::User(UserLoginMethod::Username {
            email: "test@bitwarden.com".to_string(),
            kdf: Kdf::PBKDF2 {
                iterations: NonZeroU32::new(100_000).unwrap(),
            },
            client_id: "1".to_string(),
        }));

        let password = "password123".to_string();
        let password_hash = "7kTqkF1pY/3JeOu73N9kR99fDDe9O1JOZaVc7KH3lsU=".to_string();

        let result = validate_password(&client, password, password_hash);

        assert!(result.unwrap());
    }

    #[cfg(feature = "internal")]
    #[test]
    fn test_validate_password_user_key() {
        use std::num::NonZeroU32;

        use crate::client::{Client, Kdf, LoginMethod, UserLoginMethod};

        let mut client = Client::new(None);
        client.set_login_method(LoginMethod::User(UserLoginMethod::Username {
            email: "test@bitwarden.com".to_string(),
            kdf: Kdf::PBKDF2 {
                iterations: NonZeroU32::new(600_000).unwrap(),
            },
            client_id: "1".to_string(),
        }));

        let user_key = "2.Q/2PhzcC7GdeiMHhWguYAQ==|GpqzVdr0go0ug5cZh1n+uixeBC3oC90CIe0hd/HWA/pTRDZ8ane4fmsEIcuc8eMKUt55Y2q/fbNzsYu41YTZzzsJUSeqVjT8/iTQtgnNdpo=|dwI+uyvZ1h/iZ03VQ+/wrGEFYVewBUUl/syYgjsNMbE=";
        let private_key = "2.yN7l00BOlUE0Sb0M//Q53w==|EwKG/BduQRQ33Izqc/ogoBROIoI5dmgrxSo82sgzgAMIBt3A2FZ9vPRMY+GWT85JiqytDitGR3TqwnFUBhKUpRRAq4x7rA6A1arHrFp5Tp1p21O3SfjtvB3quiOKbqWk6ZaU1Np9HwqwAecddFcB0YyBEiRX3VwF2pgpAdiPbSMuvo2qIgyob0CUoC/h4Bz1be7Qa7B0Xw9/fMKkB1LpOm925lzqosyMQM62YpMGkjMsbZz0uPopu32fxzDWSPr+kekNNyLt9InGhTpxLmq1go/pXR2uw5dfpXc5yuta7DB0EGBwnQ8Vl5HPdDooqOTD9I1jE0mRyuBpWTTI3FRnu3JUh3rIyGBJhUmHqGZvw2CKdqHCIrQeQkkEYqOeJRJVdBjhv5KGJifqT3BFRwX/YFJIChAQpebNQKXe/0kPivWokHWwXlDB7S7mBZzhaAPidZvnuIhalE2qmTypDwHy22FyqV58T8MGGMchcASDi/QXI6kcdpJzPXSeU9o+NC68QDlOIrMVxKFeE7w7PvVmAaxEo0YwmuAzzKy9QpdlK0aab/xEi8V4iXj4hGepqAvHkXIQd+r3FNeiLfllkb61p6WTjr5urcmDQMR94/wYoilpG5OlybHdbhsYHvIzYoLrC7fzl630gcO6t4nM24vdB6Ymg9BVpEgKRAxSbE62Tqacxqnz9AcmgItb48NiR/He3n3ydGjPYuKk/ihZMgEwAEZvSlNxYONSbYrIGDtOY+8Nbt6KiH3l06wjZW8tcmFeVlWv+tWotnTY9IqlAfvNVTjtsobqtQnvsiDjdEVtNy/s2ci5TH+NdZluca2OVEr91Wayxh70kpM6ib4UGbfdmGgCo74gtKvKSJU0rTHakQ5L9JlaSDD5FamBRyI0qfL43Ad9qOUZ8DaffDCyuaVyuqk7cz9HwmEmvWU3VQ+5t06n/5kRDXttcw8w+3qClEEdGo1KeENcnXCB32dQe3tDTFpuAIMLqwXs6FhpawfZ5kPYvLPczGWaqftIs/RXJ/EltGc0ugw2dmTLpoQhCqrcKEBDoYVk0LDZKsnzitOGdi9mOWse7Se8798ib1UsHFUjGzISEt6upestxOeupSTOh0v4+AjXbDzRUyogHww3V+Bqg71bkcMxtB+WM+pn1XNbVTyl9NR040nhP7KEf6e9ruXAtmrBC2ah5cFEpLIot77VFZ9ilLuitSz+7T8n1yAh1IEG6xxXxninAZIzi2qGbH69O5RSpOJuJTv17zTLJQIIc781JwQ2TTwTGnx5wZLbffhCasowJKd2EVcyMJyhz6ru0PvXWJ4hUdkARJs3Xu8dus9a86N8Xk6aAPzBDqzYb1vyFIfBxP0oO8xFHgd30Cgmz8UrSE3qeWRrF8ftrI6xQnFjHBGWD/JWSvd6YMcQED0aVuQkuNW9ST/DzQThPzRfPUoiL10yAmV7Ytu4fR3x2sF0Yfi87YhHFuCMpV/DsqxmUizyiJuD938eRcH8hzR/VO53Qo3UIsqOLcyXtTv6THjSlTopQ+JOLOnHm1w8dzYbLN44OG44rRsbihMUQp+wUZ6bsI8rrOnm9WErzkbQFbrfAINdoCiNa6cimYIjvvnMTaFWNymqY1vZxGztQiMiHiHYwTfwHTXrb9j0uPM=|09J28iXv9oWzYtzK2LBT6Yht4IT4MijEkk0fwFdrVQ4=".parse().unwrap();

        client
            .initialize_user_crypto("asdfasdfasdf", user_key.parse().unwrap(), private_key)
            .unwrap();

        let result =
            validate_password_user_key(&client, "asdfasdfasdf".to_string(), user_key.to_string())
                .unwrap();

        assert_eq!(result, "aOvkBXFhSdgrBWR3hZCMRoML9+h5yRblU3lFphCdkeA=");
        assert!(validate_password(&client, "asdfasdfasdf".to_string(), result.to_string()).unwrap())
    }

    #[cfg(feature = "internal")]
    #[test]
    fn test_validate_password_user_key_wrong_password() {
        use std::num::NonZeroU32;

        use crate::client::{Client, Kdf, LoginMethod, UserLoginMethod};

        let mut client = Client::new(None);
        client.set_login_method(LoginMethod::User(UserLoginMethod::Username {
            email: "test@bitwarden.com".to_string(),
            kdf: Kdf::PBKDF2 {
                iterations: NonZeroU32::new(600_000).unwrap(),
            },
            client_id: "1".to_string(),
        }));

        let user_key = "2.Q/2PhzcC7GdeiMHhWguYAQ==|GpqzVdr0go0ug5cZh1n+uixeBC3oC90CIe0hd/HWA/pTRDZ8ane4fmsEIcuc8eMKUt55Y2q/fbNzsYu41YTZzzsJUSeqVjT8/iTQtgnNdpo=|dwI+uyvZ1h/iZ03VQ+/wrGEFYVewBUUl/syYgjsNMbE=";
        let private_key = "2.yN7l00BOlUE0Sb0M//Q53w==|EwKG/BduQRQ33Izqc/ogoBROIoI5dmgrxSo82sgzgAMIBt3A2FZ9vPRMY+GWT85JiqytDitGR3TqwnFUBhKUpRRAq4x7rA6A1arHrFp5Tp1p21O3SfjtvB3quiOKbqWk6ZaU1Np9HwqwAecddFcB0YyBEiRX3VwF2pgpAdiPbSMuvo2qIgyob0CUoC/h4Bz1be7Qa7B0Xw9/fMKkB1LpOm925lzqosyMQM62YpMGkjMsbZz0uPopu32fxzDWSPr+kekNNyLt9InGhTpxLmq1go/pXR2uw5dfpXc5yuta7DB0EGBwnQ8Vl5HPdDooqOTD9I1jE0mRyuBpWTTI3FRnu3JUh3rIyGBJhUmHqGZvw2CKdqHCIrQeQkkEYqOeJRJVdBjhv5KGJifqT3BFRwX/YFJIChAQpebNQKXe/0kPivWokHWwXlDB7S7mBZzhaAPidZvnuIhalE2qmTypDwHy22FyqV58T8MGGMchcASDi/QXI6kcdpJzPXSeU9o+NC68QDlOIrMVxKFeE7w7PvVmAaxEo0YwmuAzzKy9QpdlK0aab/xEi8V4iXj4hGepqAvHkXIQd+r3FNeiLfllkb61p6WTjr5urcmDQMR94/wYoilpG5OlybHdbhsYHvIzYoLrC7fzl630gcO6t4nM24vdB6Ymg9BVpEgKRAxSbE62Tqacxqnz9AcmgItb48NiR/He3n3ydGjPYuKk/ihZMgEwAEZvSlNxYONSbYrIGDtOY+8Nbt6KiH3l06wjZW8tcmFeVlWv+tWotnTY9IqlAfvNVTjtsobqtQnvsiDjdEVtNy/s2ci5TH+NdZluca2OVEr91Wayxh70kpM6ib4UGbfdmGgCo74gtKvKSJU0rTHakQ5L9JlaSDD5FamBRyI0qfL43Ad9qOUZ8DaffDCyuaVyuqk7cz9HwmEmvWU3VQ+5t06n/5kRDXttcw8w+3qClEEdGo1KeENcnXCB32dQe3tDTFpuAIMLqwXs6FhpawfZ5kPYvLPczGWaqftIs/RXJ/EltGc0ugw2dmTLpoQhCqrcKEBDoYVk0LDZKsnzitOGdi9mOWse7Se8798ib1UsHFUjGzISEt6upestxOeupSTOh0v4+AjXbDzRUyogHww3V+Bqg71bkcMxtB+WM+pn1XNbVTyl9NR040nhP7KEf6e9ruXAtmrBC2ah5cFEpLIot77VFZ9ilLuitSz+7T8n1yAh1IEG6xxXxninAZIzi2qGbH69O5RSpOJuJTv17zTLJQIIc781JwQ2TTwTGnx5wZLbffhCasowJKd2EVcyMJyhz6ru0PvXWJ4hUdkARJs3Xu8dus9a86N8Xk6aAPzBDqzYb1vyFIfBxP0oO8xFHgd30Cgmz8UrSE3qeWRrF8ftrI6xQnFjHBGWD/JWSvd6YMcQED0aVuQkuNW9ST/DzQThPzRfPUoiL10yAmV7Ytu4fR3x2sF0Yfi87YhHFuCMpV/DsqxmUizyiJuD938eRcH8hzR/VO53Qo3UIsqOLcyXtTv6THjSlTopQ+JOLOnHm1w8dzYbLN44OG44rRsbihMUQp+wUZ6bsI8rrOnm9WErzkbQFbrfAINdoCiNa6cimYIjvvnMTaFWNymqY1vZxGztQiMiHiHYwTfwHTXrb9j0uPM=|09J28iXv9oWzYtzK2LBT6Yht4IT4MijEkk0fwFdrVQ4=".parse().unwrap();

        client
            .initialize_user_crypto("asdfasdfasdf", user_key.parse().unwrap(), private_key)
            .unwrap();

        let result = validate_password_user_key(&client, "abc".to_string(), user_key.to_string())
            .unwrap_err();

        assert_eq!(result.to_string(), "Internal error: wrong password");
    }
}
