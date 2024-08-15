#![deny(unused_imports)]

/**
 * Struct to store process information of the window
 */
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ProcessInfo {
    pub process_id: u32,
    pub path: String,
    pub name: String,
    pub exec_name: String,
}

impl ProcessInfo {
    pub fn new(process_id: u32, path: String, name: String, exec_name: String) -> Self {
        Self {
            process_id,
            path,
            name,
            exec_name,
        }
    }
    pub fn compare(&self, other: &Self) -> bool {
        return self.process_id == other.process_id
            && self.path == other.path
            && self.name == other.name
            && self.exec_name == other.exec_name
    }
}
