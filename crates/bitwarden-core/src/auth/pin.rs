use bitwarden_crypto::{EncString, PinKey};

use crate::{
    client::{LoginMethod, UserLoginMethod},
    error::{Error, Result},
    key_management::SymmetricKeyRef,
    Client,
};

pub(crate) fn validate_pin(
    client: &Client,
    pin: String,
    pin_protected_user_key: EncString,
) -> Result<bool> {
    let login_method = client
        .internal
        .get_login_method()
        .ok_or(Error::NotAuthenticated)?;

    #[allow(irrefutable_let_patterns)]
    let LoginMethod::User(login_method) = login_method.as_ref() else {
        return Err(Error::NotAuthenticated);
    };

    match login_method {
        UserLoginMethod::Username { email, kdf, .. }
        | UserLoginMethod::ApiKey { email, kdf, .. } => {
            let ctx = client.internal.get_crypto_service().context();
            #[allow(deprecated)]
            let user_key = ctx.dangerous_get_symmetric_key(SymmetricKeyRef::User)?;

            let pin_key = PinKey::derive(pin.as_bytes(), email.as_bytes(), kdf)?;

            let Ok(decrypted_key) = pin_key.decrypt_user_key(pin_protected_user_key) else {
                return Ok(false);
            };

            Ok(user_key.to_vec() == decrypted_key.to_vec())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use bitwarden_crypto::{Kdf, MasterKey};

    use super::*;
    use crate::client::{Client, LoginMethod, UserLoginMethod};

    fn init_client() -> Client {
        let client = Client::new(None);

        let password = "asdfasdfasdf";
        let email = "test@bitwarden.com";
        let kdf = Kdf::PBKDF2 {
            iterations: NonZeroU32::new(600_000).unwrap(),
        };

        client
            .internal
            .set_login_method(LoginMethod::User(UserLoginMethod::Username {
                email: email.to_string(),
                kdf: kdf.clone(),
                client_id: "1".to_string(),
            }));

        let master_key = MasterKey::derive(password, email, &kdf).unwrap();

        let user_key = "2.Q/2PhzcC7GdeiMHhWguYAQ==|GpqzVdr0go0ug5cZh1n+uixeBC3oC90CIe0hd/HWA/pTRDZ8ane4fmsEIcuc8eMKUt55Y2q/fbNzsYu41YTZzzsJUSeqVjT8/iTQtgnNdpo=|dwI+uyvZ1h/iZ03VQ+/wrGEFYVewBUUl/syYgjsNMbE=";
        let private_key = "2.yN7l00BOlUE0Sb0M//Q53w==|EwKG/BduQRQ33Izqc/ogoBROIoI5dmgrxSo82sgzgAMIBt3A2FZ9vPRMY+GWT85JiqytDitGR3TqwnFUBhKUpRRAq4x7rA6A1arHrFp5Tp1p21O3SfjtvB3quiOKbqWk6ZaU1Np9HwqwAecddFcB0YyBEiRX3VwF2pgpAdiPbSMuvo2qIgyob0CUoC/h4Bz1be7Qa7B0Xw9/fMKkB1LpOm925lzqosyMQM62YpMGkjMsbZz0uPopu32fxzDWSPr+kekNNyLt9InGhTpxLmq1go/pXR2uw5dfpXc5yuta7DB0EGBwnQ8Vl5HPdDooqOTD9I1jE0mRyuBpWTTI3FRnu3JUh3rIyGBJhUmHqGZvw2CKdqHCIrQeQkkEYqOeJRJVdBjhv5KGJifqT3BFRwX/YFJIChAQpebNQKXe/0kPivWokHWwXlDB7S7mBZzhaAPidZvnuIhalE2qmTypDwHy22FyqV58T8MGGMchcASDi/QXI6kcdpJzPXSeU9o+NC68QDlOIrMVxKFeE7w7PvVmAaxEo0YwmuAzzKy9QpdlK0aab/xEi8V4iXj4hGepqAvHkXIQd+r3FNeiLfllkb61p6WTjr5urcmDQMR94/wYoilpG5OlybHdbhsYHvIzYoLrC7fzl630gcO6t4nM24vdB6Ymg9BVpEgKRAxSbE62Tqacxqnz9AcmgItb48NiR/He3n3ydGjPYuKk/ihZMgEwAEZvSlNxYONSbYrIGDtOY+8Nbt6KiH3l06wjZW8tcmFeVlWv+tWotnTY9IqlAfvNVTjtsobqtQnvsiDjdEVtNy/s2ci5TH+NdZluca2OVEr91Wayxh70kpM6ib4UGbfdmGgCo74gtKvKSJU0rTHakQ5L9JlaSDD5FamBRyI0qfL43Ad9qOUZ8DaffDCyuaVyuqk7cz9HwmEmvWU3VQ+5t06n/5kRDXttcw8w+3qClEEdGo1KeENcnXCB32dQe3tDTFpuAIMLqwXs6FhpawfZ5kPYvLPczGWaqftIs/RXJ/EltGc0ugw2dmTLpoQhCqrcKEBDoYVk0LDZKsnzitOGdi9mOWse7Se8798ib1UsHFUjGzISEt6upestxOeupSTOh0v4+AjXbDzRUyogHww3V+Bqg71bkcMxtB+WM+pn1XNbVTyl9NR040nhP7KEf6e9ruXAtmrBC2ah5cFEpLIot77VFZ9ilLuitSz+7T8n1yAh1IEG6xxXxninAZIzi2qGbH69O5RSpOJuJTv17zTLJQIIc781JwQ2TTwTGnx5wZLbffhCasowJKd2EVcyMJyhz6ru0PvXWJ4hUdkARJs3Xu8dus9a86N8Xk6aAPzBDqzYb1vyFIfBxP0oO8xFHgd30Cgmz8UrSE3qeWRrF8ftrI6xQnFjHBGWD/JWSvd6YMcQED0aVuQkuNW9ST/DzQThPzRfPUoiL10yAmV7Ytu4fR3x2sF0Yfi87YhHFuCMpV/DsqxmUizyiJuD938eRcH8hzR/VO53Qo3UIsqOLcyXtTv6THjSlTopQ+JOLOnHm1w8dzYbLN44OG44rRsbihMUQp+wUZ6bsI8rrOnm9WErzkbQFbrfAINdoCiNa6cimYIjvvnMTaFWNymqY1vZxGztQiMiHiHYwTfwHTXrb9j0uPM=|09J28iXv9oWzYtzK2LBT6Yht4IT4MijEkk0fwFdrVQ4=".parse().unwrap();

        client
            .internal
            .initialize_user_crypto_master_key(master_key, user_key.parse().unwrap(), private_key)
            .unwrap();

        client
    }

    #[test]
    fn test_validate_valid_pin() {
        let pin = "1234".to_string();
        let pin_protected_user_key = "2.BXgvdBUeEMyvumqAJkAzPA==|JScDPoqOkVdrC1X755Ubt8tS9pC/thvrvNf5CyNcRg8HZtZ466EcRo7aCqwUzLyTVNRkbCYtFYT+09acGGHur8tGuS7Kmg/pYeaUo4K0UKI=|NpIFg5P9z0SN1MffbixD9OQE0l+NiNmnRQJs/kTsyoQ="
        .parse()
        .unwrap();

        let client = init_client();
        assert!(validate_pin(&client, pin.clone(), pin_protected_user_key).unwrap());
    }

    #[test]
    fn test_validate_invalid_pin() {
        let pin = "1234".to_string();
        let pin_protected_user_key = "2.BXgvdBUeEMyvumqAJkAyPA==|JScDPoqOkVdrC1X755Ubt8tS9pC/thvrvNf5CyNcRg8HZtZ466EcRo7aCqwUzLyTVNRkbCYtFYT+09acGGHur8tGuS7Kmg/pYeaUo4K0UKI=|NpIFg5P9z0SN1MffbixD9OQE0l+NiNmnRQJs/kTsyoQ="
        .parse()
        .unwrap();

        let client = init_client();
        assert!(!validate_pin(&client, pin.clone(), pin_protected_user_key).unwrap());
    }
}
