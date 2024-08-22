use rocket::form::FromForm;
use rocket::fs::TempFile;
use std::{io, fs};
use std::path::{Path, PathBuf};
use crate::{upload_file, DATA_PATH};

#[derive(FromForm)]
pub struct UplaodFileForm<'r> {
    uploaded_file: TempFile<'r>
}

impl<'r> UplaodFileForm<'r> {
    fn get_folder(&self, cloud_key: String) -> io::Result<PathBuf> {
        let path = Path::new(DATA_PATH);
        if !path.exists() {
            fs::create_dir(path)?;
        }
        let new_path = Path::new(cloud_key.as_str());
        
        let full_path = path.join(new_path);
        if !full_path.exists() {
            fs::create_dir(&full_path)?;
        }

        Ok(full_path)
    }

    fn get_file_name(&self) -> String {
        let file_name = self.uploaded_file.raw_name().unwrap();
        let content_type = self.uploaded_file.content_type().unwrap();

        let full_name = format!("{}.{}", file_name.as_str().unwrap(), content_type.extension().unwrap());

        full_name
    }
    
    pub async fn save_file(&mut self, cloud_key: String) -> io::Result<()> {
        match self.get_folder(cloud_key) {
            Ok(path) => {
                let file_name = self.get_file_name();
                let final_path = path.join(Path::new(&file_name));
                
                println!("{:?}", final_path);
                println!("{}", final_path.exists());
                
                if let Err(e) = self.uploaded_file.copy_to(final_path.into_os_string()).await {
                    eprintln!("{}", e);
                };
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }

        Ok(())
    }
}
