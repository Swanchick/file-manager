use rocket::form::FromForm;
use rocket::fs::TempFile;
use std::io;
use std::path::{Path, PathBuf};
use crate::get_cloud_directory;

#[derive(FromForm)]
pub struct UplaodFileForm<'r> {
    uploaded_file: TempFile<'r>
}

impl<'r> UplaodFileForm<'r> {
    fn get_folder(&self, cloud_key: String, file_name: String) -> io::Result<PathBuf> {
        let path = get_cloud_directory(&cloud_key)?;
        let final_path = path.join(Path::new(&file_name));

        Ok(final_path)
    }

    fn get_file_name(&self) -> String {
        let file_name = self.uploaded_file.raw_name().unwrap();
        let content_type = self.uploaded_file.content_type().unwrap();

        let full_name = format!("{}.{}", file_name.as_str().unwrap(), content_type.extension().unwrap());

        full_name
    }
    
    pub async fn save_file(&mut self, cloud_key: String) -> io::Result<()> {
        let file_name = self.get_file_name();
        
        if let Ok(path) = self.get_folder(cloud_key, file_name) {
            if let Err(e) = self.uploaded_file.copy_to(path).await {
                eprintln!("{}", e);
            };
        }

        Ok(())
    }
}
