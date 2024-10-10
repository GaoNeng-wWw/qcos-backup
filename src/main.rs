use app::App;
use crypto::Crypto;
use qcos::{acl::AclHeader, client::Client, request::ErrNo};

pub mod config;
pub mod prepare;
pub mod app;
pub mod fs;
pub mod crypto;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    let root = std::env::current_dir()?;
    let config = config::config(&root);
    prepare::run();
    let app = App::new(config.clone(), &root);
    let dirs = app.read_dir().unwrap();
    log::info!("发现{}个文件", dirs.len());
    let client = Client::new(config.access_key_id, config.secret_access_key, config.bucket, config.region);
    let mut acl_header = AclHeader::new();
    acl_header.insert_object_x_cos_acl(qcos::acl::ObjectAcl::PRIVATE);
    for file in dirs {
        let nonce = Crypto::create_nonce();
        let full_path = file.full_path;
        let metadata = std::fs::metadata(&full_path);
        let mime = mime_guess::from_path(&full_path).first();
        let content = Crypto::encrypt(config.password.clone(), std::fs::read(&full_path).unwrap().as_slice(), &nonce);
        if let Ok(metadata) = metadata {
            let part = client.put_object_get_upload_id(
                &full_path.to_str().unwrap(),
                mime.clone(), 
                Some(qcos::objects::StorageClassEnum::DeepArchive),
                 Some(acl_header.clone())
            ).await;
            if part.error_no != ErrNo::SUCCESS {
                log::error!("{}", part.error_message);
                log::error!("{}", String::from_utf8(part.result).unwrap());
                break;
            }
            let res = String::from_utf8(part.result).unwrap();
            let size = metadata.len();
            let put_res = client.clone().put_object_part(
                &full_path.to_str().unwrap(),
                res.as_str(),
                1,
                content.unwrap(),
                size,
                mime.clone(),
                Some(acl_header.clone())
            )
            .await;

            if put_res.error_no == ErrNo::SUCCESS {
                log::info!("{} 上传成功", &full_path.to_str().unwrap());
                continue;
            }
            log::info!("{} 上传失败", &full_path.to_str().unwrap());
            log::info!("因为: {}", put_res.error_message);
        } else {
            log::error!("{:?}",metadata.err());
        }
    }
    Ok(())
}