use lib_core::{
    vault::{EData, Vault},
    AppError, AppResult, ErrType,
};

use crate::dto::vault::{
    req::{DecryptRequest, EncryptRequest},
    res::{DecryptResponse, EncryptResponse},
};

use super::Service;

impl Service {
    pub async fn encrypt_data(&self, dto: EncryptRequest) -> AppResult<EncryptResponse> {
        let validity = dto.validity;

        let EData { hash, key, encrypted, e_pem } =
            Vault::new().generate_hash(dto.data).encrypt()?;

        let claw = self.ds.save_claw(encrypted, e_pem, hash, validity).await?;

        Ok(EncryptResponse { id: claw.id.key().to_string(), key, valid_for: validity.to_string() })
    }

    pub async fn decrypt_data(&self, dto: DecryptRequest) -> AppResult<DecryptResponse> {
        let claw =
            self.ds.get_claw(&dto.id).await?.ok_or_else(|| {
                AppError::new(ErrType::NotFound, "Claw not found for requested ID")
            })?;

        let vault = Vault::new_dec(EData {
            hash: claw.sha256,
            key: dto.key,
            encrypted: claw.data,
            e_pem: claw.pem,
        });

        let data = vault.decrypt().and_then(|v| v.validate_and_get())?;

        Ok(DecryptResponse { data })
    }

    pub async fn has_claw(&self, id: String) -> AppResult<()> {
        let claw = self.ds.get_claw(&id).await?;
        claw.map(|_| ())
            .ok_or_else(|| AppError::new(ErrType::NotFound, "Requested claw doesn't exists"))
    }
}
