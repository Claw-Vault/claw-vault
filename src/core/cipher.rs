use base64::Engine;
use openssl::rsa::{Padding, Rsa};
use uuid::Uuid;
use xor_cryptor::XORCryptor;

/// Enum for handling errors related to [`Cipher`]
pub enum CipherError {
    XrcInitFailed,
    KeyPairGenErr,
    PubKeyPairErr,
    PubKeyErr,
    EncryptErr,
    DecodeErr,
    DecryptErr,
}

impl CipherError {
    pub fn as_str(&self) -> &'static str {
        match self {
            CipherError::XrcInitFailed => "Failed to initialize XORCryptor",
            CipherError::KeyPairGenErr => "Failed to generate RSA key pair",
            CipherError::PubKeyPairErr => "Failed to generate RSA from public key",
            CipherError::PubKeyErr => "Failed to get public key",
            CipherError::EncryptErr => "Failed to encrypt the data",
            CipherError::DecodeErr => "Failed to decode the data",
            CipherError::DecryptErr => "Failed to decrypt the data",
        }
    }
}

/// Struct for handling cipher operations such as
/// - encrypt
/// - decrypt
/// - pem
/// - hash
#[derive(Clone)]
pub struct Cipher {
    base64_engine: base64::engine::GeneralPurpose,
}

impl Cipher {
    const RSA_BITS: u32 = 4096;

    pub fn new() -> Self {
        Cipher {
            base64_engine: base64::engine::GeneralPurpose::new(
                &base64::alphabet::URL_SAFE,
                base64::engine::general_purpose::NO_PAD,
            ),
        }
    }

    /// Encodes array to base64 [`String`]
    fn encode_string(&self, buf: &[u8]) -> String {
        let mut encoded_string = String::new();
        self.base64_engine.encode_string(buf, &mut encoded_string);
        encoded_string
    }

    /// Decode base64 array to [`Vec<u8>`]
    fn decode_string(&self, buf: &[u8]) -> Result<Vec<u8>, CipherError> {
        let mut decoded = Vec::<u8>::new();
        match self.base64_engine.decode_vec(buf, &mut decoded) {
            Ok(_) => Ok(decoded),
            Err(_) => Err(CipherError::DecodeErr),
        }
    }

    /// Encrypts pem using [`XORCryptor`]
    pub fn encrypt_pem(&self, key: &Uuid, pem: String) -> Result<String, CipherError> {
        let xrc = match XORCryptor::new(&key.to_string()) {
            Ok(xrc) => xrc,
            Err(_) => return Err(CipherError::XrcInitFailed),
        };

        let encrypted = xrc.encrypt_vec(pem.as_bytes().to_vec());
        let pem = self.encode_string(&encrypted);
        Ok(pem)
    }

    /// Decrypts pem using [`XORCryptor`]
    pub fn decrypt_pem(&self, key: &Uuid, pem: String) -> Result<Vec<u8>, CipherError> {
        let xrc = match XORCryptor::new(&key.to_string()) {
            Ok(xrc) => xrc,
            Err(_) => return Err(CipherError::XrcInitFailed),
        };

        let pem = match self.decode_string(pem.as_bytes()) {
            Ok(pem) => pem,
            Err(_) => return Err(CipherError::DecodeErr),
        };
        Ok(xrc.decrypt_vec(pem))
    }

    /// Encrypts the data using [`Rsa`]
    pub fn encrypt(&self, data: String) -> Result<(String, String), CipherError> {
        let rsa = match Rsa::generate(Cipher::RSA_BITS) {
            Ok(rsa) => rsa,
            Err(_) => return Err(CipherError::KeyPairGenErr),
        };

        let data = data.as_bytes();
        let mut encrypted = vec![0; rsa.size() as usize];
        match rsa.private_encrypt(data, &mut encrypted, Padding::PKCS1) {
            Ok(_) => (),
            Err(_) => return Err(CipherError::EncryptErr),
        };

        let encrypted = self.encode_string(encrypted.as_slice());
        let pem = match rsa.public_key_to_pem() {
            Ok(pub_key) => match String::from_utf8(pub_key) {
                Ok(pem) => pem,
                Err(_) => return Err(CipherError::PubKeyErr),
            },
            Err(_) => return Err(CipherError::PubKeyErr),
        };
        Ok((encrypted, pem))
    }

    /// Decrypts the data using [`Rsa`] from `pub_pem`
    pub fn decrypt(&self, pub_pem: Vec<u8>, data: String) -> Result<String, CipherError> {
        let rsa = match Rsa::public_key_from_pem(&pub_pem) {
            Ok(rsa) => rsa,
            Err(_) => return Err(CipherError::PubKeyPairErr),
        };

        let data = match self.decode_string(data.as_bytes()) {
            Ok(data) => data,
            Err(_) => return Err(CipherError::DecodeErr),
        };

        let mut decrypted = vec![0u8; data.len()];
        match rsa.public_decrypt(data.as_slice(), &mut decrypted, Padding::PKCS1) {
            Ok(_) => (),
            Err(_) => return Err(CipherError::DecryptErr),
        };

        let decrypted = match String::from_utf8(decrypted) {
            Ok(data) => data,
            Err(_) => return Err(CipherError::DecryptErr),
        };

        Ok(String::from(decrypted.trim_matches(char::from(0))))
    }

    /// Generates `id` and `hash` from data
    ///
    /// `hash` is the [`md5`] hash of the data
    /// `id` is the [`md5`] hash of the data + current time in millis
    pub fn generate_id_hash(&self, data: &String) -> (String, String) {
        let mut ctx = md5::Context::new();
        let time = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
        let time = format!("{}", time);
        ctx.consume(time.as_bytes());
        ctx.consume(data.as_bytes());

        let id = format!("{:x}", ctx.compute());
        let hash = format!("{:x}", md5::compute(data.as_bytes()));
        (id, hash)
    }
}
