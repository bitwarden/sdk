use std::str::FromStr;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

use crate::error::Result;

/// A Bitwarden secrets manager JWT Token.
///
/// References:
/// - <https://github.com/bitwarden/server/blob/419760623a7af5f5d0cbdfeb2a7aba8c3608d880/src/Identity/IdentityServer/ClientStore.cs#L125-L126>
/// - <https://github.com/bitwarden/server/blob/419760623a7af5f5d0cbdfeb2a7aba8c3608d880/src/Identity/IdentityServer/ClientStore.cs#L133>
///
/// TODO: We need to expand this to support user based JWT tokens.
#[derive(serde::Deserialize)]
pub struct JWTToken {
    pub exp: u64,
    pub sub: String,
    pub email: Option<String>,
    pub organization: Option<String>,
    pub scope: Vec<String>,
}

impl FromStr for JWTToken {
    type Err = crate::error::Error;

    /// Parses a JWT token from a string.
    ///
    /// **Note:** This function does not validate the token signature.
    fn from_str(s: &str) -> Result<Self> {
        let split = s.split('.').collect::<Vec<_>>();
        if split.len() != 3 {
            return Err("JWT token has an invalid number of parts".into());
        }
        let decoded = URL_SAFE_NO_PAD.decode(split[1])?;
        Ok(serde_json::from_slice(&decoded)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::auth::jwt_token::JWTToken;

    #[test]
    fn can_decode_jwt() {
        let jwt = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjMwMURENkE1MEU4NEUxRDA5MUM4MUQzQjAwQkY5MDEwQz\
        g1REJEOUFSUzI1NiIsInR5cCI6ImF0K2p3dCIsIng1dCI6Ik1CM1dwUTZFNGRDUnlCMDdBTC1RRU1oZHZabyJ9.eyJu\
        YmYiOjE2NzUxMDM1NzcsImV4cCI6MTY3NTEwNzE3NywiaXNzIjoiaHR0cDovL2xvY2FsaG9zdCIsImNsaWVudF9pZCI\
        6IndlYiIsInN1YiI6ImUyNWQzN2YzLWI2MDMtNDBkZS04NGJhLWFmOTYwMTJmNWE0MiIsImF1dGhfdGltZSI6MTY3NT\
        EwMzU0OSwiaWRwIjoiYml0d2FyZGVuIiwicHJlbWl1bSI6ZmFsc2UsImVtYWlsIjoidGVzdEBiaXR3YXJkZW4uY29tI\
        iwiZW1haWxfdmVyaWZpZWQiOnRydWUsInNzdGFtcCI6IkUzNElDWVhRUFRDS01EVldBREZDNktHNDJCQldJRDdJIiwi\
        bmFtZSI6IlRlc3QiLCJvcmdvd25lciI6ImY0ZTQ0YTdmLTExOTAtNDMyYS05ZDRhLWFmOTYwMTMxMjdjYiIsImRldml\
        jZSI6Ijg5Mjg5M2FiLWRkNDMtNDUwYS04NGI1LWFhOWM1YjdiYjJkOCIsImp0aSI6IkEzMkVFNjY5NDdEQzlDNUE2MT\
        IwRURBRTIwNzc5OUJFIiwiaWF0IjoxNjc1MTAzNTc3LCJzY29wZSI6WyJhcGkiLCJvZmZsaW5lX2FjY2VzcyJdLCJhb\
        XIiOlsiQXBwbGljYXRpb24iXX0.AyDkKvjmyaSPQViQSa2sGTKIkDGrUAtDmwpE57K4DDWT0QvwDe7FMktmwiF4LH36\
        wx_FnpH21VI1pzwJeTHXtaz3niANJtQZjzGFsNAna_95vrsxZC2YizgGlt6mX4YIGmAw9DiYrmaN0BvQOEm_caV_u6f\
        a30iz9Kvjxf7cpzeZvPEysxGpB3k3TRYTkFUdV43HiXdhXMBhyyOpFU6Fk6yA41y7-8bGYc5mYGknWktmPD9Yx-1xKL\
        ftFja1SnCoLPWvDeK60lqWZQiT4tZHCYJ7m0bBNCccYHc2Kk2Bo5-UoyDxazPwsqMxeNfjlaUuj3o5N_uQ-4n_gVbeA\
        qWV2wrel5UhYjWnczMSLBtt9p0W35kkBPt3ZAnRWMtQMPNH04p-_L6cG-Xu6lDksBTwaavcmtnCKG8V91826EiQ8MrF\
        wGWQRZV6tPKTDAYCgSAZGBY3QDmPGT5BeFcg5Ag_nYYIIifKP-kv10v_N-TOcT3NeGBOUlAZ-9m7iT7Rk3vC--SDZdA\
        U5turoBFiiPL2XXfAjM7P0r7J91gfXc0FaD6I2jDxOmym5h7Yn5phLsbC2NlIXkZp54dKHICenPl4ve6ndDIJacVeS5\
        f3LEddAPV8cAFza4DjA8pZJLFrMyRvMXcL_PjKF8qPVzqVWh03lfJ4clOIxR2gOuWIc902Y5E";

        let token: JWTToken = jwt.parse().unwrap();
        assert_eq!(token.exp, 1675107177);
        assert_eq!(token.sub, "e25d37f3-b603-40de-84ba-af96012f5a42");
        assert_eq!(token.email.as_deref(), Some("test@bitwarden.com"));
        assert_eq!(token.organization.as_deref(), None);
        assert_eq!(token.scope[0], "api");
        assert_eq!(token.scope[1], "offline_access");
    }
}
