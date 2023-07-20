use super::{DecryptPerformanceRequest, DecryptPerformanceResponse, decrypt_performance, EncryptPerformanceRequest, encrypt_performance, Pbkdf2PerformanceRequest, pbkdf2_performance
};
use crate::{error::Result, Client};

pub struct ClientPerformance {}

impl ClientPerformance {
    pub fn decrypt(
        &self,
        input: &DecryptPerformanceRequest,
    ) -> Result<DecryptPerformanceResponse> {
        decrypt_performance(&input)
    }

    pub fn encrypt(&self, input: &EncryptPerformanceRequest) -> Result<()> {
        encrypt_performance(&input)
    }

    pub fn pbkdf2(&self, input: &Pbkdf2PerformanceRequest) -> Result<()> {
        pbkdf2_performance(&input)
    }
}

impl Client {
    pub fn performance(&self) -> ClientPerformance {
        ClientPerformance {}
    }
}
