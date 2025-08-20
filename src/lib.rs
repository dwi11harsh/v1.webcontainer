#![allow(unused)]
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

#[derive(Debug, Clone)]
pub struct Directory {
    // name of this directory (not the full path)
    pub name: String,
    // optional parent directory none for root)
    pub parent: Option<Box<Directory>>,
    // child directories by name
    pub children: HashMap<String, Directory>,
    // files in this directory by name
    pub files: HashMap<String, Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileType {
    Dir,
    File,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeleteOption {
    R,
}

pub struct GrepMatch {
    pub file_path: String,
    pub line_number: usize,
    pub line_content: String,
}

impl Directory {
    // constructor
    pub fn mkdir(name: String, parent: Option<Box<Directory>>) -> Self {
        Directory {
            name,
            parent,
            children: HashMap::new(),
            files: HashMap::new(),
        }
    }

    pub fn ls(&self) -> HashMap<FileType, String> {
        let mut list: HashMap<FileType, String> = HashMap::new();

        // directory names
        for (name, _) in &self.children {
            list.insert(FileType::Dir, name.clone());
        }

        // file names
        for (name, _) in &self.files {
            list.insert(FileType::File, name.clone());
        }
        list
    }

    pub fn rm(&self, name: String, flag: Option<DeleteOption>) -> Result<(), String> {
        match flag {
            Some(flg) => {
                // delete directory
                Ok(())
            }
            None => {
                // delete file
                Ok(())
            }
        }
    }

    pub fn grep(&self, pattern: String) -> Result<Vec<GrepMatch>, String> {
        Ok(Vec::new())
    }

    // this will actually return &String
    pub fn cat(&self) -> String {
        "this will return the content of specified file".to_string()
    }

    // another cat method but this one can take multiple strings (filenames) and then combine them
    pub fn cat_multiple(&self, filenames: Vec<String>) -> Result<String, String> {
        Ok("combined content".to_string())
    }

    pub fn head(&self, file_name: String) -> Result<String, String> {
        Ok("dummy head".to_string())
    }

    pub fn tail(&self, file_name: String) -> Result<String, String> {
        Ok("dummy tail".to_string())
    }
}
