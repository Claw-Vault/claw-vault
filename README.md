# [`ClawVault`](https://claw-vault.up.railway.app)

A platform to share sensitive information with desired recipients.

Meet our mascot, Claw ! 

<img src="assets/favicon.png" width="256px" alt="Claw Vault" />

Claw is a crab who loves to keep things safe and secure with it's strong claws.
He is a symbol of commitment to privacy and security.

You can support him and keep this platform steady using this:

<a href="https://www.buymeacoffee.com/shank03" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 42px !important;width: 148px !important;" ></a>

## About
Meet [Claw Vault](https://claw-vault.up.railway.app), a platform to share sensitive information with desired recipients. Claw Vault encrypts your data, providing a unique ID and key for secure transmission. Your recipient can easily access the encrypted content using the ID and key, ensuring confidentiality. Data is automatically deleted upon access or expiration, prioritizing your privacy.

## Cryptographic Libraries

Here are the cryptographic libraries used in the project:
- [`OpenSSL`](https://crates.io/crates/openssl) - For RSA and SHA256
- [`XORCryptor (XRC)`](https://crates.io/crates/xor_cryptor) - For customized strong XOR encryption
- [`Base64`](https://crates.io/crates/base64) - For Base64 encoding and decoding

## Flow

Here is a flow diagram of how the API works.

![API Flow](assets/flow.png)

## Privacy Policy

If you are interested in privacy policy, you can find it [here](https://claw-vault.up.railway.app/privacy).

Rest assured that the data is not accessible to anyone (including me) except the recipient and the sender with the presense of unique ID and key.
