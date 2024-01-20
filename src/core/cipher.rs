use base64::Engine;
use openssl::rsa::{Padding, Rsa};

pub enum CipherError {
    KeyPairGenErr,
    PubKeyPairGenErr,
    PubKeyErr,
    EncryptErr,
    EncodeErr,
    DecodeErr,
    DecryptErr,
}

impl CipherError {
    fn as_str(&self) -> &'static str {
        match self {
            CipherError::KeyPairGenErr => "Failed to generate RSA key pair",
            CipherError::PubKeyPairGenErr => "Failed to generate RSA from public key",
            CipherError::PubKeyErr => "Failed to get public key",
            CipherError::EncryptErr => "Failed to encrypt the data",
            CipherError::EncodeErr => "Failed to encode the data",
            CipherError::DecodeErr => "Failed to decode the data",
            CipherError::DecryptErr => "Failed to decrypt the data",
        }
    }
}

#[derive(Clone)]
pub struct Cipher {
    base64_engine: base64::engine::GeneralPurpose,
}

impl Cipher {
    const RSA_BITS: u32 = 4096;

    pub fn new() -> Self {
        return Cipher {
            base64_engine: base64::engine::GeneralPurpose::new(
                &base64::alphabet::URL_SAFE,
                base64::engine::general_purpose::NO_PAD,
            ),
        };
    }

    fn encode_string(self, buf: &[u8]) -> String {
        let mut encoded_string = String::new();
        self.base64_engine.encode_string(buf, &mut encoded_string);
        return encoded_string;
    }

    fn decode_string(self, buf: &[u8]) -> Result<Vec<u8>, CipherError> {
        let mut decoded = Vec::<u8>::new();
        match self.base64_engine.decode_vec(buf, &mut decoded) {
            Ok(_) => Ok(decoded),
            Err(_) => Err(CipherError::DecodeErr),
        }
    }

    pub fn encrypt(self, data: String) -> Result<(String, String), CipherError> {
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
        match rsa.public_key_to_pem() {
            Ok(pub_key) => Ok((
                encrypted,
                match String::from_utf8(pub_key) {
                    Ok(pem) => pem,
                    Err(_) => return Err(CipherError::PubKeyErr),
                },
            )),
            Err(_) => Err(CipherError::PubKeyErr),
        }
    }

    pub fn decrypt(self, pub_pem: String, data: String) -> Result<String, CipherError> {
        let rsa = match Rsa::public_key_from_pem(pub_pem.as_bytes()) {
            Ok(rsa) => rsa,
            Err(_) => return Err(CipherError::PubKeyPairGenErr),
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
}
