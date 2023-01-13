use log::info;
use std::path::PathBuf;

pub fn get_content_type(path: PathBuf) -> (String, String) {
    let mime = mime_guess::from_path(&path);
    match mime.first() {
        None => ("".to_string(), "".to_string()),
        Some(file_type) => {
            info!("file mime {}", file_type);
            let path_name: String = path.into_os_string().into_string().unwrap();
            if path_name.contains(".") {
                let vec: Vec<&str> = path_name.split(".").collect();
                let suffix = vec[vec.len() - 1];
                let suffix = String::from(suffix);
                return (file_type.to_string(), suffix);
            }
            (file_type.to_string(), "".to_string())
        }
    }
}
