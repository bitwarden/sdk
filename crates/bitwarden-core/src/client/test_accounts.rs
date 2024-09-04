#![allow(clippy::unwrap_used)]
use std::collections::HashMap;

use bitwarden_crypto::Kdf;

use crate::{
    mobile::crypto::{
        initialize_org_crypto, initialize_user_crypto, InitOrgCryptoRequest, InitUserCryptoMethod,
        InitUserCryptoRequest,
    },
    Client,
};

impl Client {
    pub async fn init_test_account(account: TestAccount) -> Self {
        let client = Client::new(None);

        client.internal.load_flags(HashMap::from([(
            "enableCipherKeyEncryption".to_owned(),
            true,
        )]));

        initialize_user_crypto(&client, account.user).await.unwrap();

        if let Some(org) = account.org {
            initialize_org_crypto(&client, org).await.unwrap();
        }

        client
    }
}

/// Test Account
///
/// Many of the SDK tests are based on encrypted data provided by the other Bitwarden clients. In
/// order to provide a consistent method of retrieving the data we provide a test account with user
/// keys.
///
/// **Disclaimer:** The server typically encrypts and protects certain fields. In order to allow
/// accounts to be used on other servers this protection was explicitly removed from these data
/// dumps.
pub struct TestAccount {
    user: InitUserCryptoRequest,
    org: Option<InitOrgCryptoRequest>,
}

/// ### `test@bitwarden.com`
///
/// - Email: `test@bitwarden.com`
/// - Password: `asdfasdfasdf`
/// - PBKDF2: `600_000` iterations
///
/// ```sql
/// INSERT INTO vault_dev.dbo.[User] (
///   Id, Name, Email, EmailVerified, MasterPassword, MasterPasswordHint, Culture, SecurityStamp,
///   TwoFactorProviders, TwoFactorRecoveryCode, EquivalentDomains,
///   ExcludedGlobalEquivalentDomains, AccountRevisionDate, [Key], PublicKey, PrivateKey, Premium,
///   PremiumExpirationDate, Storage, MaxStorageGb, Gateway, GatewayCustomerId,
///   GatewaySubscriptionId, LicenseKey, CreationDate, RevisionDate, RenewalReminderDate, Kdf,
///   KdfIterations, ReferenceData, ApiKey, ForcePasswordReset, UsesKeyConnector, FailedLoginCount,
///   LastFailedLoginDate, AvatarColor, KdfMemory, KdfParallelism, LastPasswordChangeDate,
///   LastKdfChangeDate, LastKeyRotationDate, LastEmailChangeDate
/// )
/// VALUES
/// (
///   N 'b1fd4bf2-9643-4787-87f3-b0f00189c33b',
///   N 'Test', N 'test@bitwarden.com',
///   0, N 'AQAAAAEAAYagAAAAEJ3ky9F/Zt5sy3/UAHVvBarMR+tBXYOM5IGgXy4/mx82uptgHgItauyCN+UZTvAqiA==',
///   null, N 'en-US', N 'F3KL7SCJKEXO4LJFVLGZITPEHM7SAVSZ',
///   null, null, null, null, N '2024-01-07 23:56:48.2600000',
///   N '2.Q/2PhzcC7GdeiMHhWguYAQ==|GpqzVdr0go0ug5cZh1n+uixeBC3oC90CIe0hd/HWA/pTRDZ8ane4fmsEIcuc8eMKUt55Y2q/fbNzsYu41YTZzzsJUSeqVjT8/iTQtgnNdpo=|dwI+uyvZ1h/iZ03VQ+/wrGEFYVewBUUl/syYgjsNMbE=',
///   N 'MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA0Ww2chogqCpaAR7Uw448am4b7vDFXiM5kXjFlGfXBlrAdAqTTggEvTDlMNYqPlCo+mBM6iFmTTUY9rpZBvFskMnKvsvpJ47/fehAH2o2e3Ulv/5NFevaVCMCmpkBDtbMbO1A4a3btdRtCP8DsKWMefHauEpaoLxNTLWnOIZVfCMjsSgx2EvULHAZPTtbFwm4+UVKniM4ds4jvOsD85h4jn2aLs/jWJXFfxN8iVSqEqpC2TBvsPdyHb49xQoWWfF0Z6BiNqeNGKEU9Uos1pjL+kzhEzzSpH31PZT/ufJ/oo4+93wrUt57hb6f0jxiXhwd5yQ+9F6wVwpbfkq0IwhjOwIDAQAB',
///   N '2.yN7l00BOlUE0Sb0M//Q53w==|EwKG/BduQRQ33Izqc/ogoBROIoI5dmgrxSo82sgzgAMIBt3A2FZ9vPRMY+GWT85JiqytDitGR3TqwnFUBhKUpRRAq4x7rA6A1arHrFp5Tp1p21O3SfjtvB3quiOKbqWk6ZaU1Np9HwqwAecddFcB0YyBEiRX3VwF2pgpAdiPbSMuvo2qIgyob0CUoC/h4Bz1be7Qa7B0Xw9/fMKkB1LpOm925lzqosyMQM62YpMGkjMsbZz0uPopu32fxzDWSPr+kekNNyLt9InGhTpxLmq1go/pXR2uw5dfpXc5yuta7DB0EGBwnQ8Vl5HPdDooqOTD9I1jE0mRyuBpWTTI3FRnu3JUh3rIyGBJhUmHqGZvw2CKdqHCIrQeQkkEYqOeJRJVdBjhv5KGJifqT3BFRwX/YFJIChAQpebNQKXe/0kPivWokHWwXlDB7S7mBZzhaAPidZvnuIhalE2qmTypDwHy22FyqV58T8MGGMchcASDi/QXI6kcdpJzPXSeU9o+NC68QDlOIrMVxKFeE7w7PvVmAaxEo0YwmuAzzKy9QpdlK0aab/xEi8V4iXj4hGepqAvHkXIQd+r3FNeiLfllkb61p6WTjr5urcmDQMR94/wYoilpG5OlybHdbhsYHvIzYoLrC7fzl630gcO6t4nM24vdB6Ymg9BVpEgKRAxSbE62Tqacxqnz9AcmgItb48NiR/He3n3ydGjPYuKk/ihZMgEwAEZvSlNxYONSbYrIGDtOY+8Nbt6KiH3l06wjZW8tcmFeVlWv+tWotnTY9IqlAfvNVTjtsobqtQnvsiDjdEVtNy/s2ci5TH+NdZluca2OVEr91Wayxh70kpM6ib4UGbfdmGgCo74gtKvKSJU0rTHakQ5L9JlaSDD5FamBRyI0qfL43Ad9qOUZ8DaffDCyuaVyuqk7cz9HwmEmvWU3VQ+5t06n/5kRDXttcw8w+3qClEEdGo1KeENcnXCB32dQe3tDTFpuAIMLqwXs6FhpawfZ5kPYvLPczGWaqftIs/RXJ/EltGc0ugw2dmTLpoQhCqrcKEBDoYVk0LDZKsnzitOGdi9mOWse7Se8798ib1UsHFUjGzISEt6upestxOeupSTOh0v4+AjXbDzRUyogHww3V+Bqg71bkcMxtB+WM+pn1XNbVTyl9NR040nhP7KEf6e9ruXAtmrBC2ah5cFEpLIot77VFZ9ilLuitSz+7T8n1yAh1IEG6xxXxninAZIzi2qGbH69O5RSpOJuJTv17zTLJQIIc781JwQ2TTwTGnx5wZLbffhCasowJKd2EVcyMJyhz6ru0PvXWJ4hUdkARJs3Xu8dus9a86N8Xk6aAPzBDqzYb1vyFIfBxP0oO8xFHgd30Cgmz8UrSE3qeWRrF8ftrI6xQnFjHBGWD/JWSvd6YMcQED0aVuQkuNW9ST/DzQThPzRfPUoiL10yAmV7Ytu4fR3x2sF0Yfi87YhHFuCMpV/DsqxmUizyiJuD938eRcH8hzR/VO53Qo3UIsqOLcyXtTv6THjSlTopQ+JOLOnHm1w8dzYbLN44OG44rRsbihMUQp+wUZ6bsI8rrOnm9WErzkbQFbrfAINdoCiNa6cimYIjvvnMTaFWNymqY1vZxGztQiMiHiHYwTfwHTXrb9j0uPM=|09J28iXv9oWzYtzK2LBT6Yht4IT4MijEkk0fwFdrVQ4=',
///   0, null, null, null, null, null, null,
///   null, N '2024-01-07 23:53:38.5900000',
///   N '2024-01-07 23:53:38.5900000',
///   null, 0, 600000, N '{"id":null}', N '7gp59kKHt9kMlks0BuNC4IjNXYkljR',
///   0, 0, 0, null, null, null, null, null,
///   null, null, null
/// );
///
/// INSERT INTO vault_dev.dbo.Organization (
///   Id, Name, BusinessName, BillingEmail, [Plan], PlanType, Seats, MaxCollections, UseGroups,
///   UseDirectory, UseTotp, SelfHost, Storage, MaxStorageGb, Gateway, GatewayCustomerId,
///   GatewaySubscriptionId, Enabled, LicenseKey, ExpirationDate, CreationDate, RevisionDate,
///   BusinessAddress1, BusinessAddress2, BusinessAddress3, BusinessCountry, BusinessTaxNumber,
///   UsersGetPremium, UseEvents, Use2fa, TwoFactorProviders, UseApi, UsePolicies, Identifier,
///   ReferenceData, UseSso, UseResetPassword, PublicKey, PrivateKey, OwnersNotifiedOfAutoscaling,
///   MaxAutoscaleSeats, UseKeyConnector, UseScim, UseCustomPermissions, UseSecretsManager, Status,
///   UsePasswordManager, SmSeats, SmServiceAccounts, MaxAutoscaleSmSeats,
///   MaxAutoscaleSmServiceAccounts, SecretsManagerBeta, LimitCollectionCreationDeletion,
///   AllowAdminAccessToAllCollectionItems, FlexibleCollections
/// )
/// VALUES
/// (
///   N'1bc9ac1e-f5aa-45f2-94bf-b181009709b8', N'Test org', null, N'test@bitwarden.com', N'Free',
///   0, 2, 2, 0, 0, 0, 0, null, null, null, null, null, 1, N'xrpfAYhMphI1ny6ks4aA', null,
///   N'2024-05-31 09:09:54.7466667', N'2024-05-31 09:09:54.7466667', null, null, null, null,
///   null, 0, 0, 0, null, 0, 0, null, N'{"id":null}', 0, 0,
///   N'MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAmIJbGMk6eZqVE7UxhZ46Weu2jKciqOiOkSVYtGvs61rfe9AXxtLaaZEKN4d4DmkZcF6dna2eXNxZmb7U4pwlttye8ksqISe6IUAZQox7auBpjopdCEPhKRg3BD/u8ks9UxSxgWe+fpebjt6gd5hsl1/5HOObn7SeU6EEU04cp3/eH7a4OTdXxB8oN62HGV9kM/ubM1goILgjoSJDbihMK0eb7b8hPHwcA/YOgKKiu/N3FighccdSMD5Pk+HfjacsFNZQa2EsqW09IvvSZ+iL6HQeZ1vwc/6TO1J7EOfJZFQcjoEL9LVI693efYoMZSmrPEWziZ4PvwpOOGo6OObyMQIDAQAB', N'2.6FggyKVyaKQsfohi5yqgbg==|UU2JeafOB41L5UscGmf4kq15JGDf3Bkf67KECiehTODzbWctVLTgyDk0Qco8/6CMN6nZGXjxR2A4r5ExhmwRNsNxd77G+MprkmiJz+7w33ROZ1ouQO5XjD3wbQ3ssqNiTKId6yAUPBvuAZRixVApauTuADc8QWGixqCQcqZzmU7YSBBIPf652/AEYr4Tk64YihoE39pHiK8MRbTLdRt3EF4LSMugPAPM24vCgUv3w1TD3Fj6sDg/6oi3flOV9SJZX4vCiUXbDNEuD/p2aQrEXVbaxweFOHjTe7F4iawjXw3nG3SO8rUBHcxbhDDVx5rjYactbW5QvHWiyla6uLb6o8WHBneg2EjTEwAHOZE/rBjcqmAJb2sVp1E0Kwq8ycGmL69vmqJPC1GqVTohAQvmEkaxIPpfq24Yb9ZPrADA7iEXBKuAQ1FphFUVgJBJGJbd60sOV1Rz1T+gUwS4wCNQ4l3LG1S22+wzUVlEku5DXFnT932tatqTyWEthqPqLCt6dL1+qa94XLpeHagXAx2VGe8n8IlcADtxqS+l8xQ4heT12WO9kC316vqvg1mnsI56faup9hb3eT9ZpKyxSBGYOphlTWfV1Y/v64f5PYvTo4aL0IYHyLY/9Qi72vFmOpPeHBYgD5t3j+H2CsiU1PkYsBggOmD7xW8FDuT6HWVvwhEJqeibVPK0Lhyj6tgvlSIAvFUaSMFPlmwFNmwfj/AHUhr9KuTfsBFTZ10yy9TZVgf+EofwnrxHBaWUgdD40aHoY1VjfG33iEuajb6buxG3pYFyPNhJNzeLZisUKIDRMQpUHrsE22EyrFFran3tZGdtcyIEK4Q1F0ULYzJ6T9iY25/ZgPy3pEAAMZCtqo3s+GjX295fWIHfMcnjMgNUHPjExjWBHa+ggK9iQXkFpBVyYB1ga/+0eiIhiek3PlgtvpDrqF7TsLK+ROiBw2GJ7uaO3EEXOj2GpNBuEJ5CdodhZkwzhwMcSatgDHkUuNVu0iVbF6/MxVdOxWXKO+jCYM6PZk/vAhLYqpPzu2T2Uyz4nkDs2Tiq61ez6FoCrzdHIiyIxVTzUQH8G9FgSmtaZ7GCbqlhnurYgcMciwPzxg0hpAQT+NZw1tVEii9vFSpJJbGJqNhORKfKh/Mu1P/9LOQq7Y0P2FIR3x/eUVEQ7CGv2jVtO5ryGSmKeq/P9Fr54wTPaNiqN2K+leACUznCdUWw8kZo/AsBcrOe4OkRX6k8LC3oeJXy06DEToatxEvPYemUauhxiXRw8nfNMqc4LyJq2bbT0zCgJHoqpozPdNg6AYWcoIobgAGu7ZQGq+oE1MT3GZxotMPe/NUJiAc5YE9Thb5Yf3gyno71pyqPTVl/6IQuh4SUz7rkgwF/aVHEnr4aUYNoc0PEzd2Me0jElsA3GAneq1I/wngutOWgTViTK4Nptr5uIzMVQs9H1rOMJNorP8b02t1NDu010rSsib9GaaJJq4r4iy46laQOxWoU0ex26arYnk+jw4833WSCTVBIprTgizZ+fKjoY0xwXvI2oOvGNEUCtGFvKFORTaQrlaXZIg1toa2BBVNicyONbwnI3KIu3MgGJ2SlCVXJn8oHFppVHFCdwgN1uDzGiKAhjvr0sZTUtXin2f2CszPTbbo=|fUhbVKrr8CSKE7TZJneXpDGraj5YhRrq9ESo206S+BY=',
///   null, null, 0, 0, 0, 0, 1, 1, null, null, null, null, 0, 1, 1, 0
/// );
///
/// INSERT INTO vault_dev.dbo.OrganizationUser (
///   Id, OrganizationId, UserId, Email, [Key], Status, Type, AccessAll, ExternalId, CreationDate,
///   RevisionDate, Permissions, ResetPasswordKey, AccessSecretsManager
/// )
/// VALUES
/// (
///   N'a5943f26-414f-4ecb-ba69-b181009709bc', N'1bc9ac1e-f5aa-45f2-94bf-b181009709b8',
///   N'b1fd4bf2-9643-4787-87f3-b0f00189c33b', null,
///   N'4.rY01mZFXHOsBAg5Fq4gyXuklWfm6mQASm42DJpx05a+e2mmp+P5W6r54WU2hlREX0uoTxyP91bKKwickSPdCQQ58J45LXHdr9t2uzOYyjVzpzebFcdMw1eElR9W2DW8wEk9+mvtWvKwu7yTebzND+46y1nRMoFydi5zPVLSlJEf81qZZ4Uh1UUMLwXz+NRWfixnGXgq2wRq1bH0n3mqDhayiG4LJKgGdDjWXC8W8MMXDYx24SIJrJu9KiNEMprJE+XVF9nQVNijNAjlWBqkDpsfaWTUfeVLRLctfAqW1blsmIv4RQ91PupYJZDNc8nO9ZTF3TEVM+2KHoxzDJrLs2Q==',
///   2, 0, 1, null, N'2024-05-31 09:09:54.7466667', N'2024-05-31 09:09:54.7466667',
///   null, null, 0
/// );
/// ```
pub fn test_bitwarden_com_account() -> TestAccount {
    TestAccount {
        user: InitUserCryptoRequest {
            kdf_params: Kdf::PBKDF2 {
                iterations: 600_000.try_into().unwrap(),
            },
            email: "test@bitwarden.com".to_owned(),
            private_key: "2.yN7l00BOlUE0Sb0M//Q53w==|EwKG/BduQRQ33Izqc/ogoBROIoI5dmgrxSo82sgzgAMIBt3A2FZ9vPRMY+GWT85JiqytDitGR3TqwnFUBhKUpRRAq4x7rA6A1arHrFp5Tp1p21O3SfjtvB3quiOKbqWk6ZaU1Np9HwqwAecddFcB0YyBEiRX3VwF2pgpAdiPbSMuvo2qIgyob0CUoC/h4Bz1be7Qa7B0Xw9/fMKkB1LpOm925lzqosyMQM62YpMGkjMsbZz0uPopu32fxzDWSPr+kekNNyLt9InGhTpxLmq1go/pXR2uw5dfpXc5yuta7DB0EGBwnQ8Vl5HPdDooqOTD9I1jE0mRyuBpWTTI3FRnu3JUh3rIyGBJhUmHqGZvw2CKdqHCIrQeQkkEYqOeJRJVdBjhv5KGJifqT3BFRwX/YFJIChAQpebNQKXe/0kPivWokHWwXlDB7S7mBZzhaAPidZvnuIhalE2qmTypDwHy22FyqV58T8MGGMchcASDi/QXI6kcdpJzPXSeU9o+NC68QDlOIrMVxKFeE7w7PvVmAaxEo0YwmuAzzKy9QpdlK0aab/xEi8V4iXj4hGepqAvHkXIQd+r3FNeiLfllkb61p6WTjr5urcmDQMR94/wYoilpG5OlybHdbhsYHvIzYoLrC7fzl630gcO6t4nM24vdB6Ymg9BVpEgKRAxSbE62Tqacxqnz9AcmgItb48NiR/He3n3ydGjPYuKk/ihZMgEwAEZvSlNxYONSbYrIGDtOY+8Nbt6KiH3l06wjZW8tcmFeVlWv+tWotnTY9IqlAfvNVTjtsobqtQnvsiDjdEVtNy/s2ci5TH+NdZluca2OVEr91Wayxh70kpM6ib4UGbfdmGgCo74gtKvKSJU0rTHakQ5L9JlaSDD5FamBRyI0qfL43Ad9qOUZ8DaffDCyuaVyuqk7cz9HwmEmvWU3VQ+5t06n/5kRDXttcw8w+3qClEEdGo1KeENcnXCB32dQe3tDTFpuAIMLqwXs6FhpawfZ5kPYvLPczGWaqftIs/RXJ/EltGc0ugw2dmTLpoQhCqrcKEBDoYVk0LDZKsnzitOGdi9mOWse7Se8798ib1UsHFUjGzISEt6upestxOeupSTOh0v4+AjXbDzRUyogHww3V+Bqg71bkcMxtB+WM+pn1XNbVTyl9NR040nhP7KEf6e9ruXAtmrBC2ah5cFEpLIot77VFZ9ilLuitSz+7T8n1yAh1IEG6xxXxninAZIzi2qGbH69O5RSpOJuJTv17zTLJQIIc781JwQ2TTwTGnx5wZLbffhCasowJKd2EVcyMJyhz6ru0PvXWJ4hUdkARJs3Xu8dus9a86N8Xk6aAPzBDqzYb1vyFIfBxP0oO8xFHgd30Cgmz8UrSE3qeWRrF8ftrI6xQnFjHBGWD/JWSvd6YMcQED0aVuQkuNW9ST/DzQThPzRfPUoiL10yAmV7Ytu4fR3x2sF0Yfi87YhHFuCMpV/DsqxmUizyiJuD938eRcH8hzR/VO53Qo3UIsqOLcyXtTv6THjSlTopQ+JOLOnHm1w8dzYbLN44OG44rRsbihMUQp+wUZ6bsI8rrOnm9WErzkbQFbrfAINdoCiNa6cimYIjvvnMTaFWNymqY1vZxGztQiMiHiHYwTfwHTXrb9j0uPM=|09J28iXv9oWzYtzK2LBT6Yht4IT4MijEkk0fwFdrVQ4=".to_owned(),

            method: InitUserCryptoMethod::Password {
                password: "asdfasdfasdf".to_owned(),
                user_key: "2.Q/2PhzcC7GdeiMHhWguYAQ==|GpqzVdr0go0ug5cZh1n+uixeBC3oC90CIe0hd/HWA/pTRDZ8ane4fmsEIcuc8eMKUt55Y2q/fbNzsYu41YTZzzsJUSeqVjT8/iTQtgnNdpo=|dwI+uyvZ1h/iZ03VQ+/wrGEFYVewBUUl/syYgjsNMbE=".to_owned(),
            }
        },
        org: Some(InitOrgCryptoRequest {
            organization_keys: HashMap::from([(
                "1bc9ac1e-f5aa-45f2-94bf-b181009709b8".parse().unwrap(),
                "4.rY01mZFXHOsBAg5Fq4gyXuklWfm6mQASm42DJpx05a+e2mmp+P5W6r54WU2hlREX0uoTxyP91bKKwickSPdCQQ58J45LXHdr9t2uzOYyjVzpzebFcdMw1eElR9W2DW8wEk9+mvtWvKwu7yTebzND+46y1nRMoFydi5zPVLSlJEf81qZZ4Uh1UUMLwXz+NRWfixnGXgq2wRq1bH0n3mqDhayiG4LJKgGdDjWXC8W8MMXDYx24SIJrJu9KiNEMprJE+XVF9nQVNijNAjlWBqkDpsfaWTUfeVLRLctfAqW1blsmIv4RQ91PupYJZDNc8nO9ZTF3TEVM+2KHoxzDJrLs2Q==".parse().unwrap()
            )])
        }),
    }
}

/// ### `legacy@bitwarden.com`
///
/// Account which has a user_key of type `AesCbc256_B64` which is deprecated.
///
/// - Email: `legacy@bitwarden.com`
/// - Password: `asdfasdfasdf`
/// - PBKDF2: `600_000` iterations
///
/// ```sql
/// /// INSERT INTO vault_dev.dbo.[User] (
///   Id, Name, Email, EmailVerified, MasterPassword, MasterPasswordHint, Culture, SecurityStamp,
///   TwoFactorProviders, TwoFactorRecoveryCode, EquivalentDomains,
///   ExcludedGlobalEquivalentDomains, AccountRevisionDate, [Key], PublicKey, PrivateKey, Premium,
///   PremiumExpirationDate, Storage, MaxStorageGb, Gateway, GatewayCustomerId,
///   GatewaySubscriptionId, LicenseKey, CreationDate, RevisionDate, RenewalReminderDate, Kdf,
///   KdfIterations, ReferenceData, ApiKey, ForcePasswordReset, UsesKeyConnector, FailedLoginCount,
///   LastFailedLoginDate, AvatarColor, KdfMemory, KdfParallelism, LastPasswordChangeDate,
///   LastKdfChangeDate, LastKeyRotationDate, LastEmailChangeDate
/// )
/// VALUES
/// (
///   N'764335ba-bb3b-4646-9c19-b1a301229eb2', N'Legacy', N'legacy@bitwarden.com', 1,
///   N'AQAAAAIAAYagAAAAEBPKHRNzY+8XTZGODQaa4hQUW/7qsW73v5O8WmqsS+xplghgtX/qXS1JTHT6PNWUFw==',
///   null, N'en-US', N'a1611dff-b383-4d7f-bb37-18c37507c11e', null, null, null, null,
///   N'2024-07-05 13:27:01.4033333',
///   N'0.8UClLa8IPE1iZT7chy5wzQ==|6PVfHnVk5S3XqEtQemnM5yb4JodxmPkkWzmDRdfyHtjORmvxqlLX40tBJZ+CKxQWmS8tpEB5w39rbgHg/gqs0haGdZG4cPbywsgGzxZ7uNI=',
///   N'MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAvbVTK0cY7j/9GD4UTIZVywT3vWNIiy5LuI1l9MIKbWh+PyVUB5ySR++Z1tDwx5R87AtXwjQS09j6a4X+uGHW4lRQR3Dngb2CpMutLqFjKk7NYWSj3MU5bKlvsCf2Z59ECJEFqo3eAsHYyrL3CixRsZg3LkQcYT+VT/qOOcI6UkMX50lpma2Cvdf3IAgPf6Enkg/WJ3d7D6aU5YtexTY03uKVbOb2UoO2ZJ29VyYEVEJ+0S6pqe9laeHz/oJ9SI3sRqdEYRmyDsbUWJHRJr7eAVesmiUBU/Ls7t4k/oXjfiJ7K5wqL5yCQ0HkqK8zQhi9wB4u7kHwEa8sDJE+cFQ5bQIDAQAB',
///   N'2.leBIE5u0aQUeXi++JzAnrA==|P8x+hs00RJx7epw+49qVtBhLJxE/JTL5dEHg6kq5pbZLdUY8ZvWK49v0EqgHbv1r298N9+msoO9hmdSIVIAZyycemYDSoc1rX4S1KpS/ZMA/Vd3VLFb+o13Ts62GFQ5ygHKgQZfzjU6jO5P/B/0igzFoxyJDomhW5NBC1P9+e/5qNRZN8loKvAaWc/7XtpRayPQqWx+AgYc2ntb1GF5hRVrW4M47bG5ZKllbJWtQKg2sXIy2lDBbKLRFWF4RFzNVcXQGMoPdWLY0f3uTwUH01dyGmFFMbOvfBEuYqmZyPdd93ve8zuFOEqkj46Ulpq2CVG8NvZARTwsdKl6XB0wGuHFoTsDJT2SJGl67pBBKsVRGxy059QW+9hAIB+emIV0T/7+0rvdeSXZ4AbG+oXGEXFTkHefwJKfeT0MBTAjYKr7ZRLgqvf7n39+nCEJU4l22kp8FmjcWIU7AgNipdGHC+UT2yfOcYlvgBgWDcMXcbVDMyus9105RgcW6PHozUj7yjbohI/A3XWmAFufP6BSnmEFCKoik78X/ry09xwiH2rN4KVXe/k9LpRNB2QBGIVsfgCrkxjeE8r0nA59Rvwrhny1z5BkvMW/N1KrGuafg/IYgegx72gJNuZPZlFu1Vs7HxySHmzYvm3DPV7bzCaAxxNtvZmQquNIEnsDQfjJO76iL1JCtDqNJVzGLHTMTr7S5hkOcydcH3kfKwZdA1ULVd2qu0SwOUEP/ECjU/cS5INy6WPYzNMAe/g2DISpQjNwBb5K17PIiGOR7/Q/A6E8pVnkHiAXuUFr9aLOYN9BWSu5Z+BPHH65na2FDmssix5WV09I2sUBfvdNCjkrUGdYgo8E+vOTn35x9GJHF45uhmgC1yAn/+/RSpORlrSVJ7NNP11dn3htUpSsIy/b7ituAu8Ry5mhicFU8CXJL4NeMlXThUt8P++wxs4wMkBvJ8J9NJAVKbAOA2o+GOdjbh6Ww3IRegkurWh4oL/dFSx0LpaXJuw6HFT/LzticPlSwHtUP11hZ81seMsXmkSZd8IugRFfwpPl7N6PVRWDOKxLf4gPqcnJ11TvfasXy1uolV2vZCPbrbbVzQMPdVwL/OzwfhqsIgQZI8rsDMK5D2EX8MaT8MDfGcsYcVTL9PmuZYLpOUnnHX0A1opAAa9iPw3d+eWB/GAyLvKPnMTUqVNos8HcCktXckCshihA8QuBJOwg3m0j2LPSZ5Jvf8gbXauBmt9I4IlJq0xfpgquYY1WNnO8IcWE4N9W+ASvOr9gnduA6CkDeAlyMUFmdpkeCjGMcsV741bTCPApSQlL3/TOT1cjK3iejWpz0OaVHXyg02hW2fNkOfYfr81GvnLvlHxIg4Prw89gKuWU+kQk82lFQo6QQpqbCbJC2FleurD8tYoSY0srhuioVInffvTxw2NMF7FQEqUcsK9AMKSEiDqzBi35Um/fiE3JL4XZBFw8Xzl7X3ab5nlg8X+xD5uSZY+oxD3sDVXjLaQ5JUoys+MCm0FkUj85l0zT6rvM4QLhU1RDK1U51T9HJhh8hsFJsqL4abRzwEWG7PSi859zN4UsgyuQfmBJv/n7QAFCbrJhVBlGB1TKLZRzvgmKoxTYTG3cJFkjetLcUTwrwC9naxAQRfF4=|ufHf73IzJ707dx44w4fjkuD7tDa50OwmmkxcypAT9uQ=',
///   0, null, null, null, null, null, null, null, N'2024-07-04 17:38:06.7866667',
///   N'2024-07-05 13:27:01.4033333', null, 0, 600000,
///   N'{"id":null,"initiationPath":"Registration form"}', N'Yi9yvlPzvbOuilU0f0pFYuEuxcDy11',
///   0, 0, 0, null, null, null, null, null, null, null, N'2024-07-05 13:27:01.4033333'
/// );
/// ```
pub fn test_legacy_user_key_account() -> TestAccount {
    TestAccount {
        user: InitUserCryptoRequest {
            kdf_params: Kdf::PBKDF2 {
                iterations: 600_000.try_into().unwrap(),
            },
            email: "legacy@bitwarden.com".to_owned(),
            private_key: "2.leBIE5u0aQUeXi++JzAnrA==|P8x+hs00RJx7epw+49qVtBhLJxE/JTL5dEHg6kq5pbZLdUY8ZvWK49v0EqgHbv1r298N9+msoO9hmdSIVIAZyycemYDSoc1rX4S1KpS/ZMA/Vd3VLFb+o13Ts62GFQ5ygHKgQZfzjU6jO5P/B/0igzFoxyJDomhW5NBC1P9+e/5qNRZN8loKvAaWc/7XtpRayPQqWx+AgYc2ntb1GF5hRVrW4M47bG5ZKllbJWtQKg2sXIy2lDBbKLRFWF4RFzNVcXQGMoPdWLY0f3uTwUH01dyGmFFMbOvfBEuYqmZyPdd93ve8zuFOEqkj46Ulpq2CVG8NvZARTwsdKl6XB0wGuHFoTsDJT2SJGl67pBBKsVRGxy059QW+9hAIB+emIV0T/7+0rvdeSXZ4AbG+oXGEXFTkHefwJKfeT0MBTAjYKr7ZRLgqvf7n39+nCEJU4l22kp8FmjcWIU7AgNipdGHC+UT2yfOcYlvgBgWDcMXcbVDMyus9105RgcW6PHozUj7yjbohI/A3XWmAFufP6BSnmEFCKoik78X/ry09xwiH2rN4KVXe/k9LpRNB2QBGIVsfgCrkxjeE8r0nA59Rvwrhny1z5BkvMW/N1KrGuafg/IYgegx72gJNuZPZlFu1Vs7HxySHmzYvm3DPV7bzCaAxxNtvZmQquNIEnsDQfjJO76iL1JCtDqNJVzGLHTMTr7S5hkOcydcH3kfKwZdA1ULVd2qu0SwOUEP/ECjU/cS5INy6WPYzNMAe/g2DISpQjNwBb5K17PIiGOR7/Q/A6E8pVnkHiAXuUFr9aLOYN9BWSu5Z+BPHH65na2FDmssix5WV09I2sUBfvdNCjkrUGdYgo8E+vOTn35x9GJHF45uhmgC1yAn/+/RSpORlrSVJ7NNP11dn3htUpSsIy/b7ituAu8Ry5mhicFU8CXJL4NeMlXThUt8P++wxs4wMkBvJ8J9NJAVKbAOA2o+GOdjbh6Ww3IRegkurWh4oL/dFSx0LpaXJuw6HFT/LzticPlSwHtUP11hZ81seMsXmkSZd8IugRFfwpPl7N6PVRWDOKxLf4gPqcnJ11TvfasXy1uolV2vZCPbrbbVzQMPdVwL/OzwfhqsIgQZI8rsDMK5D2EX8MaT8MDfGcsYcVTL9PmuZYLpOUnnHX0A1opAAa9iPw3d+eWB/GAyLvKPnMTUqVNos8HcCktXckCshihA8QuBJOwg3m0j2LPSZ5Jvf8gbXauBmt9I4IlJq0xfpgquYY1WNnO8IcWE4N9W+ASvOr9gnduA6CkDeAlyMUFmdpkeCjGMcsV741bTCPApSQlL3/TOT1cjK3iejWpz0OaVHXyg02hW2fNkOfYfr81GvnLvlHxIg4Prw89gKuWU+kQk82lFQo6QQpqbCbJC2FleurD8tYoSY0srhuioVInffvTxw2NMF7FQEqUcsK9AMKSEiDqzBi35Um/fiE3JL4XZBFw8Xzl7X3ab5nlg8X+xD5uSZY+oxD3sDVXjLaQ5JUoys+MCm0FkUj85l0zT6rvM4QLhU1RDK1U51T9HJhh8hsFJsqL4abRzwEWG7PSi859zN4UsgyuQfmBJv/n7QAFCbrJhVBlGB1TKLZRzvgmKoxTYTG3cJFkjetLcUTwrwC9naxAQRfF4=|ufHf73IzJ707dx44w4fjkuD7tDa50OwmmkxcypAT9uQ=".to_owned(),
            method: InitUserCryptoMethod::Password {
                password: "asdfasdfasdf".to_owned(),
                user_key: "0.8UClLa8IPE1iZT7chy5wzQ==|6PVfHnVk5S3XqEtQemnM5yb4JodxmPkkWzmDRdfyHtjORmvxqlLX40tBJZ+CKxQWmS8tpEB5w39rbgHg/gqs0haGdZG4cPbywsgGzxZ7uNI=".to_owned(),
            }
        },
org: None,
    }
}
