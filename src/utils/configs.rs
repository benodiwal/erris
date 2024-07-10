use std::path::Path;
use crate::utils::file;

pub const BUDDY_NAME: &str = "buddy-02";
pub const MODEL: &str = "gpt-3.5-turbo-1106";

pub fn instructions() -> String {
    let path = Path::new("instructions").join("prompts");
    file::read(&path)
}
