use aes_gcm::{aead::{consts::{B0, B1}, generic_array::GenericArray, Aead}, aes::cipher::typenum::{UInt, UTerm}, AeadCore, Aes256Gcm, Key, KeyInit};
pub struct Crypto {}

impl Crypto {
    pub fn create_key(password: String) -> GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>{
        let mut key:[u8; 32] = [0;32];
        let mut password = password.as_bytes();
        if password.len() > 32 {
            password = &password[0..32];
        }
        for i in 0..password.len(){
            key[i]=password[i];
        }
        let key: &aes_gcm::aead::generic_array::GenericArray<u8, _> = Key::<Aes256Gcm>::from_slice(&key);
        return key.clone();
    }
    pub fn create_nonce() -> GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>{
        let nonce: GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>> = Aes256Gcm::generate_nonce(&mut aes_gcm::aead::OsRng);
        return nonce;
    }
    pub fn encrypt(password: String, content: &[u8], nonce: &GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>)->Result<Vec<u8>, aes_gcm::Error>{
        let key = &Crypto::create_key(password);
        let key = Key::<Aes256Gcm>::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        cipher.encrypt(&nonce, content)
    }
    pub fn decrypt(password: String, cipher_text: &[u8], nonce: &GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>)->Result<Vec<u8>, aes_gcm::Error>{
        let key = Crypto::create_key(password);
        let cipher = Aes256Gcm::new(&key);
        let data = cipher.decrypt(&nonce, cipher_text);
        let _data=data.clone();
        return data;
    }
}

#[cfg(test)]
mod test {
    use super::Crypto;
    #[test]
    pub fn encrypt(){
        let file = std::fs::read("Cargo.toml").unwrap();
        let nonce = Crypto::create_nonce();
        let cipher = Crypto::encrypt("password".to_string(), file.as_slice(), &nonce);
        assert_eq!(cipher.is_err(), false);
    }

    #[test]
    pub fn decrypt(){
        let file = std::fs::read("Cargo.toml").unwrap();
        let nonce = Crypto::create_nonce();
        let cipher = Crypto::encrypt("password".to_string(), file.as_slice(), &nonce);
        
        let binding = cipher.ok().unwrap();
        let bind = binding.as_slice();
        let data = Crypto::decrypt("password".to_string(), bind, &nonce);
        assert_eq!(data.unwrap(), file);
    }
}