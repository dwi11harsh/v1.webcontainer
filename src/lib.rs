use std::collections::HashMap;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct VirtualFS {
    // track virtual current directory
    current_dir: String,
    files: HashMap<String, Vec<u8>>,
}

#[wasm_bindgen]
impl VirtualFS {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        VirtualFS {
            current_dir: "/".to_string(),
            files: HashMap::new(),
        }
    }

    pub fn get_curr_dir(&self) -> String {
        self.current_dir.to_string()
    }

    pub fn change_dir(&mut self, path: &str) -> Result<(), String> {
        // basic path validation
        if path.starts_with('/') {
            self.current_dir = path.to_string();
        } else {
            // handle relative path
            let mut new_path = self.current_dir.clone();
            if !new_path.ends_with('/') {
                new_path.push('/');
            }

            new_path.push_str(path);
            self.current_dir = new_path;
        }

        Ok(())
    }

    pub fn all_files(&self) -> String {
        format!("Current virtual directory: {}", self.current_dir)
    }
}
