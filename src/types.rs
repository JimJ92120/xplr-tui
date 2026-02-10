#[derive(Debug, Clone, PartialEq)]
pub enum DirectoryItemType {
    File,
    Directory,
}

#[derive(Clone)]
pub struct DirectoryItem {
    pub name: String,
    pub path_name: String,
    pub item_type: DirectoryItemType, 
}

pub type DirectoryContent = Vec<DirectoryItem>;

#[derive(Clone)]
pub struct Directory {
    pub path_name: String,
    pub content: DirectoryContent,
}
