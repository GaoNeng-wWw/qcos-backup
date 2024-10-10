use std::path::Path;

use data_encoding::HEXLOWER;
pub struct FsLayer {}

impl FsLayer {
    pub fn hash(data: Vec<u8>)->String{
        let res= ring::digest::digest(&ring::digest::SHA256, data.as_slice());
        let hexlower = String::from_utf8(HEXLOWER.decode(res.as_ref()).unwrap()).unwrap();
        hexlower
    }
}