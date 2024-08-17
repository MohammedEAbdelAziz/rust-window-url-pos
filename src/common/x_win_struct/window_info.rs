#![deny(unused_imports)]

use super::{process_info::ProcessInfo, usage_info::UsageInfo, window_position::WindowPosition};

/**
 * Struct to store all informations of the window
 */
#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub id: u32,
    pub os: String,
    pub title: String,
    pub position: WindowPosition,
    pub info: ProcessInfo,
    pub usage: UsageInfo,
    pub url: String,
}

impl WindowInfo {
    pub fn new(
        id: u32,
        os: String,
        title: String,
        position: WindowPosition,
        info: ProcessInfo,
        usage: UsageInfo,
        url: String,
    ) -> Self {
        Self {
            id,
            os,
            title,
            position,
            info,
            usage,
            url,
        }
    }

    pub fn get_id(&self) -> u32 {
        return self.id.clone();
    }

    pub fn get_title(&self) -> String {
        return self.title.clone();
    }

    pub fn get_process_id(&self) -> u32 {
        return self.info.get_process_id();
    }

    pub fn get_path(&self) -> String {
        return self.info.get_path();
    }

    pub fn get_name(&self) -> String {
        return self.info.get_name();
    }

    pub fn get_exec_name(&self) -> String {
        return self.info.get_exec_name();
    }

    pub fn get_url(&self) -> String {
        return self.url.clone();
    }
    
    pub fn compare(&self, other: &WindowInfo) -> bool {
        return self.id == other.id
            && self.os == other.os
            && self.title == other.title
            && self.url == other.url
    }
}
