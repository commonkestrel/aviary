use std::sync::atomic::AtomicBool;

pub struct FileSystem {
    root: Directory,
}

pub struct FsNode {
    open: AtomicBool,
    path: Path,
    inner: Node,
}

pub enum Node {
    SymLink,
    File(File),
    Directory(Directory),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    permissions: Permissions,
    content: Vec<u8>,
}

pub struct Directory {
    contents: Vec<Node>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Permissions {
    read: bool,
    write: bool,
    execute: bool,
}

type Path = Vec<String>;
