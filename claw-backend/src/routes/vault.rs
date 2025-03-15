use lib_core::RpcResult;
use lib_domain::claw_proto::claw_vault_server::ClawVault;
use lib_domain::claw_proto::{DecryptRequest, DecryptResponse, EncryptRequest, EncryptResponse};

use crate::app::_App;

#[tonic::async_trait]
impl ClawVault for _App {
    async fn encrypt_data(
        &self,
        request: tonic::Request<EncryptRequest>,
    ) -> RpcResult<EncryptResponse> {
        let (metadata, extension, dto) = request.into_parts();

        let data = self.service().encrypt_data(dto).await.map_err(tonic::Status::from)?;
        Ok(tonic::Response::from_parts(metadata, data, extension))
    }

    async fn decrypt_data(
        &self,
        request: tonic::Request<DecryptRequest>,
    ) -> Result<tonic::Response<DecryptResponse>, tonic::Status> {
        let (metadata, extension, dto) = request.into_parts();

        let data = self.service().decrypt_data(dto).await.map_err(tonic::Status::from)?;
        Ok(tonic::Response::from_parts(metadata, data, extension))
    }
}
