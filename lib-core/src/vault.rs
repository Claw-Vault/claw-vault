use std::marker::PhantomData;

use base64::Engine;
use openssl::{
    pkey::Private,
    rsa::{Padding, Rsa},
    sha,
};
use xor_cryptor::XORCryptor;

use crate::{AppError, AppResult, ErrType};

pub trait ClawType {}

pub struct IdHashClaw;
pub struct EncryptClaw;
pub struct DecryptClaw;

impl ClawType for IdHashClaw {}
impl ClawType for EncryptClaw {}
impl ClawType for DecryptClaw {}

const RSA_CIPHER_BITS: u32 = 4096;

pub struct Vault<T> {
    base64_engine: base64::engine::GeneralPurpose,
    hash: String,
    key: String,
    encrypted: String,
    e_pem: String,
    _marker: PhantomData<T>,
}

pub struct EData {
    hash: String,
    key: String,
    encrypted: String,
    e_pem: String,
}

impl<T> Vault<T>
where
    T: ClawType,
{
    pub fn new() -> Vault<IdHashClaw> {
        Vault {
            base64_engine: base64::engine::GeneralPurpose::new(
                &base64::alphabet::URL_SAFE,
                base64::engine::general_purpose::NO_PAD,
            ),
            hash: "".into(),
            key: "".into(),
            encrypted: "".into(),
            e_pem: "".into(),
            _marker: PhantomData,
        }
    }

    pub fn new_dec(EData { hash, key, encrypted, e_pem }: EData) -> Vault<DecryptClaw> {
        Vault {
            base64_engine: base64::engine::GeneralPurpose::new(
                &base64::alphabet::URL_SAFE,
                base64::engine::general_purpose::NO_PAD,
            ),
            hash,
            key,
            encrypted,
            e_pem,
            _marker: PhantomData,
        }
    }

    /// Encodes array to base64 [`String`]
    fn encode_string(&self, buf: &[u8]) -> String {
        let mut encoded_string = String::new();
        self.base64_engine.encode_string(buf, &mut encoded_string);
        encoded_string
    }

    /// Decode base64 array to [`Vec<u8>`]
    fn decode_string(&self, buf: &[u8]) -> AppResult<Vec<u8>> {
        let mut decoded = Vec::<u8>::new();
        match self.base64_engine.decode_vec(buf, &mut decoded) {
            Ok(_) => Ok(decoded),
            Err(err) => Err(AppError::err(ErrType::VaultError, err, "Error decoding base64")),
        }
    }

    /// Generates [`sha::sha256`] hash for the data
    fn sha256(&self, data: &[u8]) -> String {
        hex::encode(sha::sha256(data))
    }
}

impl Vault<IdHashClaw> {
    /// Generates [`sha::sha256`] hash for the data
    pub fn generate_hash(self, data: &str) -> Vault<EncryptClaw> {
        let hash = self.sha256(data.as_bytes());

        Vault {
            base64_engine: self.base64_engine,
            hash,
            key: self.key,
            encrypted: self.encrypted,
            e_pem: self.e_pem,
            _marker: PhantomData,
        }
    }
}

impl Vault<EncryptClaw> {
    pub fn encrypt(self, data: String) -> AppResult<EData> {
        // generate RSA key pairs
        let rsa = match Rsa::generate(RSA_CIPHER_BITS) {
            Ok(rsa) => rsa,
            Err(err) => return Err(AppError::err(ErrType::VaultError, err, "Failed to generate RSA key-pair")),
        };

        // encrypt data
        let mut encrypted = vec![0; rsa.size() as usize];
        rsa.private_encrypt(data.as_bytes(), &mut encrypted, Padding::PKCS1)
            .map_err(|err| AppError::err(ErrType::VaultError, err, "Failed to encrypt data"))?;
        let encrypted = self.encode_string(&encrypted);

        // get public pem from RSA
        let pem = rsa
            .public_key_to_pem()
            .map_err(|e| AppError::err(ErrType::VaultError, e, "Failed to get public key pem"))?;

        let key = self.generate_key(&rsa)?;

        // encrypt pem
        let pem = XORCryptor::encrypt_v2(&key, pem)
            .map_err(|e| AppError::err(ErrType::VaultError, e, "Failed to encrypt pem"))?;
        let pem = self.encode_string(&pem);
        let key = self.encode_string(&key);

        Ok(EData { hash: self.hash, key, encrypted, e_pem: pem })
    }

    /// Generates key encrypting pem using [`Rsa`]
    ///
    /// Returns [`Vec<u8>`] as key
    fn generate_key(&self, rsa: &Rsa<Private>) -> AppResult<Vec<u8>> {
        let time = format!("{}", chrono::Utc::now().timestamp_millis());
        let time = self.encode_string(time.as_bytes());

        let mut key = vec![0; rsa.size() as usize];
        rsa.private_encrypt(time.as_bytes(), &mut key, Padding::PKCS1)
            .map_err(|e| AppError::err(ErrType::VaultError, e, "Failed to encrypt generated key"))?;
        Ok(key)
    }
}

impl Vault<DecryptClaw> {
    pub fn decrypt(self) -> AppResult<String> {
        // base64 decode key and pem
        let key = self.decode_string(self.key.as_bytes())?;
        let pem = self.decode_string(self.e_pem.as_bytes())?;

        // decrypt pem
        let pem = XORCryptor::decrypt_v2(&key, pem)
            .map_err(|e| AppError::err(ErrType::VaultError, e, "Failed to decrypt pem"))?;

        // generate RSA from public key
        let rsa = Rsa::public_key_from_pem(&pem)
            .map_err(|e| AppError::err(ErrType::VaultError, e, "Failed to generate RSA from public key"))?;

        // decode encrypted data
        let data = self.decode_string(self.encrypted.as_bytes())?;

        // decrypt data
        let mut decrypted = vec![0; data.len()];
        rsa.public_decrypt(&data, &mut decrypted, Padding::PKCS1)
            .map_err(|e| AppError::err(ErrType::VaultError, e, "Failed to decrypt data"))?;

        let decrypted = String::from_utf8(decrypted)
            .map_err(|e| AppError::err(ErrType::VaultError, e, "Failed to process buffer into valid string"))?;

        // Remove any null chars
        Ok(String::from(decrypted.trim_matches(char::from(0))))
    }
}
