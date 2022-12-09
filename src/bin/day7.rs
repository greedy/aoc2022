use std::collections::HashMap;

use aoc2022::prelude::*;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<7>
}

type InodeNumber = usize;

pub struct Filesystem {
    inodes: Vec<FSNode>,
}

impl Filesystem {
    pub fn new() -> Self {
        let root_directory = DirectoryNode { parent_dir: 0, name: String::new(), contents: HashMap::new() };
        let inodes : Vec<FSNode> = vec![root_directory.into()];
        Self { inodes }
    }

    pub fn root_directory(&self) -> &DirectoryNode {
        let root_node = self.get_node(0).unwrap();
        root_node.try_into().unwrap()
    }

    pub fn root_directory_mut(&mut self) -> &mut DirectoryNode {
        let root_node = self.get_node_mut(0).unwrap();
        root_node.try_into().unwrap()
    }

    pub fn root_inode_number(&self) -> InodeNumber { 0 }

    pub fn add_node<'a, T:Into<FSNode>>(&'a mut self, node: T) -> (InodeNumber, &'a mut T) 
        where
            &'a mut FSNode: TryInto<&'a mut T>,
            <&'a mut FSNode as TryInto<&'a mut T>>::Error: std::fmt::Debug
    {
        let fsnode = node.into();
        let inode_number = self.inodes.len();
        self.inodes.push(fsnode);
        let fsnode = self.inodes.last_mut().unwrap();
        (inode_number, fsnode.try_into().unwrap())
    }

    pub fn mkdir<S:std::string::ToString>(&mut self, parent: InodeNumber, name:S) -> Result<(InodeNumber, &mut DirectoryNode)> {
        let _: &DirectoryNode =
            self.get_node(parent)
            .ok_or_else(|| eyre!("Parent directory does not exist"))
            .and_then(|n| n.try_into().map_err(|_| eyre!("Parent is not a directory")))?;
        let new_dir = DirectoryNode { parent_dir: parent, name: name.to_string(), contents: HashMap::new() };
        let (new_inode, _) = self.add_node(new_dir);
        // already validated above
        let parent_dir : &mut DirectoryNode = self.get_node_mut(parent).unwrap().try_into().unwrap();
        parent_dir.contents.insert(name.to_string(), new_inode);
        // we just added this one
        let new_dir = self.get_node_mut(new_inode).unwrap().try_into().unwrap();
        Ok((new_inode, new_dir))
    }

    pub fn mkfile<S:std::string::ToString>(&mut self, parent: InodeNumber, name:S, size: usize) -> Result<(InodeNumber, &mut FileNode)> {
        let _ : &DirectoryNode =
            self.get_node(parent)
            .ok_or_else(|| eyre!("Parent directory does not exist"))
            .and_then(|n| n.try_into().map_err(|_| eyre!("Parent is not a directory")))?;
        let new_file = FileNode { name: name.to_string(), size };
        let (new_inode, _) = self.add_node(new_file);
        // already validated above
        let parent_dir : &mut DirectoryNode = self.get_node_mut(parent).unwrap().try_into().unwrap();
        parent_dir.contents.insert(name.to_string(), new_inode);
        let new_file = (&mut self.inodes[new_inode]).try_into().unwrap();
        Ok((new_inode, new_file))
    }

    pub fn is_dir(&self, inode: InodeNumber) -> bool {
        self.inodes.get(inode)
            .map(|n| n.is_dir())
            .unwrap_or(false)
    }

    pub fn get_node(&self, inode: InodeNumber) -> Option<&FSNode> {
        self.inodes.get(inode)
    }

    pub fn get_node_mut(&mut self, inode: InodeNumber) -> Option<&mut FSNode> {
        self.inodes.get_mut(inode)
    }

    pub fn dir_size(&self, inode: InodeNumber) -> Result<usize> {
        let this_dir: &DirectoryNode =
            self.get_node(inode)
            .ok_or_else(|| eyre!("Node does not exist"))
            .and_then(|n| n.try_into().map_err(|_| eyre!("Node is not a directory")))?;
        let size = this_dir.contents.values()
            .map(|child_inode| {
                match self.get_node(*child_inode) {
                    None => panic!("Directory hold a non-existant node"),
                    Some(FSNode::File(FileNode {size, ..})) => *size,
                    Some(FSNode::Directory(_)) => self.dir_size(*child_inode).expect("Filesystem has errors")
                }
            })
        .sum();
        Ok(size)
    }

    pub fn directories(&self) -> Directories<'_> {
        Directories(self, 0)
    }
}

pub struct Directories<'a>(&'a Filesystem, usize);

impl<'a> Iterator for Directories<'a> {
    type Item = (InodeNumber, &'a DirectoryNode);

    fn next(&mut self) -> Option<Self::Item> {
        while self.1 < self.0.inodes.len() {
            let this_inode_num = self.1;
            self.1 += 1;
            match self.0.get_node(this_inode_num) {
                None => return None,
                Some(FSNode::File(_)) => (),
                Some(FSNode::Directory(d)) => return Some((this_inode_num, d))
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct FileNode { name: String, size: usize }
#[derive(Debug)]
pub struct DirectoryNode { parent_dir: InodeNumber, name: String, contents: HashMap<String, InodeNumber> }

impl DirectoryNode {
    pub fn new(parent_dir: InodeNumber, name: String) -> Self {
        let mut contents = HashMap::new();
        // contents.insert("..".to_owned(), parent_dir);
        Self { parent_dir, name, contents }
    }
}

impl From<FileNode> for FSNode {
    fn from(node: FileNode) -> Self {
        Self::File(node)
    }
}

impl From<DirectoryNode> for FSNode {
    fn from(node: DirectoryNode) -> Self {
        Self::Directory(node)
    }
}

impl TryFrom<FSNode> for DirectoryNode {
    type Error = FSNode;

    fn try_from(node: FSNode) -> Result<Self, Self::Error> {
        match node {
            FSNode::Directory(dnode) => Ok(dnode),
            _ => Err(node)
        }
    }
}

impl TryFrom<FSNode> for FileNode {
    type Error = FSNode;

    fn try_from(node: FSNode) -> Result<Self, Self::Error> {
        match node {
            FSNode::File(fnode) => Ok(fnode),
            _ => Err(node)
        }
    }
}

impl<'a> TryFrom<&'a FSNode> for &'a DirectoryNode {
    type Error = &'a FSNode;

    fn try_from(node: &'a FSNode) -> Result<Self, Self::Error> {
        match node {
            FSNode::Directory(dnode) => Ok(dnode),
            _ => Err(node)
        }
    }
}

impl<'a> TryFrom<&'a FSNode> for &'a FileNode {
    type Error = &'a FSNode;

    fn try_from(node: &'a FSNode) -> Result<Self, Self::Error> {
        match node {
            FSNode::File(fnode) => Ok(fnode),
            _ => Err(node)
        }
    }
}

impl<'a> TryFrom<&'a mut FSNode> for &'a mut DirectoryNode {
    type Error = &'a mut FSNode;

    fn try_from(node: &'a mut FSNode) -> Result<Self, Self::Error> {
        match node {
            FSNode::Directory(dnode) => Ok(dnode),
            _ => Err(node)
        }
    }
}

impl<'a> TryFrom<&'a mut FSNode> for &'a mut FileNode {
    type Error = &'a mut FSNode;

    fn try_from(node: &'a mut FSNode) -> Result<Self, Self::Error> {
        match node {
            FSNode::File(fnode) => Ok(fnode),
            _ => Err(node)
        }
    }
}

#[derive(Debug)]
pub enum FSNode {
    File(FileNode),
    Directory(DirectoryNode)
}

impl FSNode {
    pub fn is_dir(&self) -> bool {
        match self {
            Self::Directory(_) => true,
            _ => false
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            Self::File(_) => true,
            _ => false
        }
    }
}

struct Shell {
    filesystem: Filesystem,
    working_directory: InodeNumber,
}

impl Shell {
    pub fn new() -> Self {
        let mut filesystem = Filesystem::new();
        let working_directory = filesystem.root_inode_number();
        Self { filesystem, working_directory }
    }

    pub fn working_directory(&self) -> &DirectoryNode {
        let dirnode = &self.filesystem.inodes[self.working_directory];
        dirnode.try_into().unwrap()
    }

    pub fn working_directory_mut(&mut self) -> &mut DirectoryNode {
        let dirnode = &mut self.filesystem.inodes[self.working_directory];
        dirnode.try_into().unwrap()
    }

    pub fn cd(&mut self, name: &str) -> Result<()> {
        let new_cwd_inode = {
            let cwd = self.working_directory();
            match cwd.contents.get(name) {
                Some(inode) => *inode,
                None => {
                    self.filesystem.mkdir(self.working_directory, name)?.0
                }
            }
        };
        self.working_directory = new_cwd_inode;
        Ok(())
    }

    pub fn touch(&mut self, file_name: &str, size: usize) -> Result<()> {
        self.filesystem.mkfile(self.working_directory, file_name, size)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let mut shell = Shell::new();

    for line in cli.input.get_input()?.lines() {
        let line = line?;

        if let Some(cd_dir) = line.strip_prefix("$ cd ") {
            if cd_dir == "/" {
                shell.working_directory = 0;
            } else if cd_dir == ".." {
                shell.working_directory = shell.working_directory().parent_dir;
            } else {
                shell.cd(cd_dir)?;
            }
            continue;
        }

        if line.starts_with("$") {
            continue;
        }

        if line.starts_with("dir") {
            continue;
        }

        if let Some((file_size, file_name)) = line.split_once(' ') {
            let file_size : usize = file_size.parse()?;
            shell.touch(file_name, file_size)?;
        }
    }

    let total : usize = shell.filesystem.directories()
        .filter_map(|(inode, d)| {
            let size = shell.filesystem.dir_size(inode).unwrap();
            dbg!(d, size);
            if size <= 100000 {
                Some(size)
            } else {
                None
            }
        }).sum();

    println!("sum of the total sizes of those directories is {}", total);

    let total_space = 70000000;

    let needed_space = 30000000;

    let used_space = shell.filesystem.dir_size(0).unwrap();

    let free_space = total_space - used_space;

    let need_to_free = needed_space - free_space;

    if let Some((size, name)) = shell.filesystem.directories()
        .filter_map(|(inode, d)| {
            let size = shell.filesystem.dir_size(inode).unwrap();
            if size >= need_to_free {
                Some((size, d.name.clone()))
            } else {
                None
            }
        })
    .min_by_key(|x| x.0)
    {
        println!("you should delete {} to free up {} bytes", name, size);
    }

    Ok(())
}
