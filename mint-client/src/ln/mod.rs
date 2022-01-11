use crate::api::MintApi;
use minimint::modules::ln;
use minimint_api::db::RawDatabase;
use std::sync::Arc;

pub struct LnClient {
    pub db: Arc<dyn RawDatabase>,
    pub cfg: ln::config::LightningModuleClientConfig,
    pub api: MintApi,
    pub secp: secp256k1_zkp::Secp256k1<secp256k1_zkp::All>,
}
