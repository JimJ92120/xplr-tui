#[derive(Debug, Clone, PartialEq)]
pub enum DirectoryItemType {
    File,
    Directory,
}

#[derive(Debug, Clone)]
pub struct DirectoryItem {
    pub name: String,
    pub path_name: String,
    pub item_type: DirectoryItemType,
    pub permissions: String,
}

pub type DirectoryContent = Vec<DirectoryItem>;

#[derive(Debug, Clone)]
pub struct Directory {
    pub name: String,
    pub path_name: String,
    pub content: DirectoryContent,
}

pub type DirectoryList = Vec<Directory>;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Copy,
    Move,
    Delete,
}
