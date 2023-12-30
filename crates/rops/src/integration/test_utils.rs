use crate::*;

pub trait IntegrationTestUtils: Integration {
    fn mock_key_id_str() -> impl AsRef<str>;

    fn mock_private_key_str() -> impl AsRef<str>;

    fn mock_encrypted_data_key_str() -> &'static str;

    fn set_mock_private_key_env_var() {
        std::env::set_var(Self::private_key_env_var_name(), Self::mock_private_key_str().as_ref())
    }

    fn mock_key_id() -> Self::KeyId {
        Self::parse_key_id(Self::mock_key_id_str().as_ref()).unwrap()
    }

    fn mock_private_key() -> Self::PrivateKey {
        Self::parse_private_key(Self::mock_private_key_str()).unwrap()
    }
}

pub use stub_integration::StubIntegration;
mod stub_integration {

    use super::*;

    pub struct StubIntegration;

    impl Integration for StubIntegration {
        const NAME: &'static str = "stub";
        type KeyId = ();
        type PrivateKey = ();
        type Config = StubIntegrationConfig;

        fn parse_key_id(_key_id_str: &str) -> IntegrationResult<Self::KeyId> {
            unimplemented!()
        }

        fn parse_private_key(_private_key_str: impl AsRef<str>) -> IntegrationResult<Self::PrivateKey> {
            unimplemented!()
        }

        fn encrypt_data_key(_key_id: &Self::KeyId, _data_key: &DataKey) -> IntegrationResult<String> {
            unimplemented!()
        }

        fn decrypt_data_key(_key_id: &Self::KeyId, _encrypted_data_key: &str) -> IntegrationResult<Option<DataKey>> {
            unimplemented!()
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct StubIntegrationConfig(String);

    impl IntegrationConfig<StubIntegration> for StubIntegrationConfig {
        const INCLUDE_DATA_KEY_CREATED_AT: bool = false;

        fn key_id(&self) -> &<StubIntegration as Integration>::KeyId {
            &()
        }
    }
}