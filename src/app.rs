use std::path::PathBuf;
use config::Config;
use glob::PatternError;

use crate::config::AppConfig;

pub struct App {
    pub config: AppConfig,
    pub root: PathBuf
}

pub struct FileInfo {
    pub name: String,
    pub full_path: PathBuf
}

impl App{
    pub fn new(config:AppConfig, root: &PathBuf) -> Self{
        Self { 
            config,
            root: root.clone()
        }
    }
    pub fn read_dir(self)->Result<Vec<FileInfo>, PatternError>{
        if self.config.glob.is_none(){
            return Ok(vec![]);
        }
        let pattern = self.config.glob.unwrap();
        let files = glob::glob(&pattern)?;
        let mut file_infos:Vec<FileInfo> = vec![];
        for file in files {
            let file_path = file.unwrap();
            if self.root.join(file_path.clone()).is_dir(){
                continue;
            }
            let file_name = file_path.file_name().clone().unwrap().to_str().unwrap().to_string();
            file_infos.push(FileInfo {
                name: file_name,
                full_path: self.root.join(file_path)
            });
        }
        Ok(file_infos)
    }
}