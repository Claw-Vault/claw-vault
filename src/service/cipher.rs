use base64::Engine;
use openssl::rsa::{Padding, Rsa};

enum CipherError {
    KeyPairGenErr,
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
            CipherError::PubKeyErr => "Failed to get public key",
            CipherError::EncryptErr => "Failed to encrypt the data",
            CipherError::EncodeErr => "Failed to encode the data",
            CipherError::DecodeErr => "Failed to decode the data",
            CipherError::DecryptErr => "Failed to decrypt the data",
        }
    }
}

#[derive(Clone)]
pub(crate) struct Cipher {
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
        let data = data.as_bytes();
        let rsa = match Rsa::generate(Cipher::RSA_BITS) {
            Ok(rsa) => rsa,
            Err(_) => return Err(CipherError::KeyPairGenErr),
        };

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
}
