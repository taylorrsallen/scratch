//////////////////////////////////=////////////////////////////////=////////////////////////////////
// USE
use crate::*;
pub use serde::*;
pub use std::env::*;
pub use std::fs::*;
pub use std::io::*;

//////////////////////////////////=////////////////////////////////=////////////////////////////////
// FUNCTIONS
pub struct Data;
impl Data {
    pub fn try_read_file_to_bytes(
        path: &str,
    ) -> Option<Vec<u8>> {
        if let Ok(file) = File::open(path) {
            let mut buf_reader = BufReader::new(file);
            let mut bytes: Vec<u8> = vec![];
            
            if let Ok(_) = buf_reader.read_to_end(&mut bytes) {
                if bytes.len() != 0 {
                    return Some(bytes);
                }
            }
        }
    
        None
    }
    
    pub fn try_read_file_to_string(
        path: &str,
    ) -> Option<String> {
        if let Ok(file) = File::open(path) {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            
            if let Ok(_) = buf_reader.read_to_string(&mut contents) {
                if contents.as_str().len() != 0 {
                    return Some(contents);
                }
            }
        }
    
        None
    }
    
    //================================-================================-================================ 
    // Write
    pub fn try_write_file(
        path: &str,
        bytes: &[u8],
    ) -> bool {
        if let Ok(mut file) = File::create(path) {
            if let Ok(_) = file.write_all(bytes) {
                return true;
            }
        }
    
        false
    }
    
    //================================-================================-================================ 
    // Serialize
    pub fn to_ron_string_pretty<T: Serialize>(
        data: &T,
    ) -> ron::Result<String> {
        ron::ser::to_string_pretty(data, ron::ser::PrettyConfig::default())
    }
}