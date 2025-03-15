use lib_core::{
    AppError, AppResult, ErrType,
    enums::ValidDuration,
    vault::{EData, Vault},
};
use validator::ValidateLength;

use crate::claw_proto::{DecryptRequest, DecryptResponse, EncryptRequest, EncryptResponse};

use super::Service;

impl Service {
    pub async fn encrypt_data(&self, dto: EncryptRequest) -> AppResult<EncryptResponse> {
        if !dto.data.validate_length(Some(1), None, None) {
            return Err(AppError::new(ErrType::ValidationErr, "Data is empty"));
        }
        let validity = ValidDuration::from(dto.validity).ok_or_else(|| {
            AppError::new(ErrType::ValidationErr, "Invalid validity - must be 60, 900 or 1800")
        })?;

        let EData { hash, key, encrypted, e_pem } =
            Vault::cipher().generate_hash(dto.data).encrypt()?;

        let claw = self.ds.save_claw(encrypted, e_pem, hash, validity).await?;

        Ok(EncryptResponse { id: claw.id, key, valid_for: validity.to_string() })
    }

    pub async fn decrypt_data(&self, dto: DecryptRequest) -> AppResult<DecryptResponse> {
        if !dto.id.validate_length(None, None, Some(20)) {
            return Err(AppError::new(ErrType::ValidationErr, "Invalid ID length"));
        }
        if !dto.key.validate_length(Some(32), Some(128), None) {
            return Err(AppError::new(ErrType::ValidationErr, "Invalid key length"));
        }

        let claw =
            self.ds.get_claw(&dto.id).await?.ok_or_else(|| {
                AppError::new(ErrType::NotFound, "Claw not found for requested ID")
            })?;

        let vault = Vault::decipher(EData {
            hash: claw.sha256,
            key: dto.key,
            encrypted: claw.data,
            e_pem: claw.pem,
        });

        let data = vault.decrypt().and_then(|v| v.validate_and_get())?;

        self.ds.delete_claw(&claw.id).await?;

        Ok(DecryptResponse { data })
    }

    pub async fn has_claw(&self, id: String) -> AppResult<()> {
        let claw = self.ds.get_claw(&id).await?;
        claw.map(|_| ())
            .ok_or_else(|| AppError::new(ErrType::NotFound, "Requested claw doesn't exists"))
    }
}
