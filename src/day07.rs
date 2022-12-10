use std::collections::hash_map::RandomState;
use std::fs;
use std::hash::BuildHasher;
use std::hash::Hasher;

#[derive(Debug)]
pub enum FileSystemItem {
    Directory(Directory),
    File(File),
}

#[derive(Debug)]
pub struct File {
    _name: String,
    size: u32,
}

impl File {
    fn new(name: &str, size: u32) -> Self {
        File {
            size,
            _name: name.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Directory {
    id: Id,
    parent: Option<Id>,
    name: String,
    children: Vec<FileSystemItem>,
}

impl Directory {
    fn new(name: &str, parent: Option<Id>) -> Self {
        Directory {
            parent,
            name: name.to_string(),
            children: Vec::new(),
            id: id(),
        }
    }

    fn add_child(&mut self, child: FileSystemItem) {
        self.children.push(child);
    }

    fn chdir(&mut self, name: &str) -> &mut Self {
        // println!("{self:#?}");
        let child_index = self
            .children
            .iter()
            .position(|child| match child {
                FileSystemItem::Directory(child) => child.name == name,
                FileSystemItem::File(_) => false,
            })
            .unwrap();
        if let FileSystemItem::Directory(child_dir) = &mut self.children[child_index] {
            child_dir
        } else {
            panic!("can't chdir into file");
        }
    }

    fn find(&mut self, id: Id) -> Option<&mut Self> {
        println!("searching {}", self.name);
        if self.name == "/" {
            println!("{self:#?}");
        }

        if self.id == id {
            return Some(self);
        }

        for child in &mut self.children {
            if let FileSystemItem::Directory(child) = child {
                return child.find(id);
            }
        }
        None
    }

    fn size(&self) -> u32 {
        self.children
            .iter()
            .map(|child| match child {
                FileSystemItem::Directory(dir) => dir.size(),
                FileSystemItem::File(file) => file.size,
            })
            .sum()
    }

    fn find_dirs<'a>(&'a self, result: &mut Vec<&'a Directory>) {
        result.push(self);
        for child in &self.children {
            match child {
                FileSystemItem::Directory(child_dir) => child_dir.find_dirs(result),
                FileSystemItem::File(_) => (),
            }
        }
    }
}

type Id = u64;

fn id() -> Id {
    RandomState::new().build_hasher().finish()
}

pub fn sum_small_dirs(file: &str) -> u32 {
    let commands = fs::read_to_string(file).expect("file exists");
    let commands = commands
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        // skip "$ cd /"
        .skip(1);

    let mut root = Directory::new("/", None);
    let mut cwd = &mut root;

    for command in commands {
        println!("{command:?}");
        match command[..] {
            ["$", "cd", ".."] => {
                let parent_id = cwd.parent.unwrap();
                let msg = format!("directory ({}) has parent ({})", cwd.id, parent_id);
                let parent = root.find(parent_id).expect(&msg);
                cwd = parent;
            }
            ["$", "cd", location] => {
                cwd = cwd.chdir(location);
            }
            ["$", "ls"] => (),
            ["dir", dir_name] => {
                let dir = FileSystemItem::Directory(Directory::new(dir_name, Some(cwd.id)));
                cwd.add_child(dir)
            }
            [size, file_name] => {
                let size: u32 = size.parse().unwrap();
                let file = FileSystemItem::File(File::new(file_name, size));
                cwd.add_child(file);
            }
            [..] => panic!("unrecognized command"),
        }
    }

    let dirs: &mut Vec<&Directory> = &mut Vec::new();
    root.find_dirs(dirs);
    dirs.iter()
        .map(|dir| dir.size())
        .filter(|size| size < &100_000)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{day07, fetch_input};

    #[test]
    fn sum_small_dirs() {
        fetch_input(7);

        let tests = vec![("example/day07.txt", 95_437)];
        // let tests = vec![("example/day07.txt", 95_437), ("input/day07.txt", 0)];

        for test in tests {
            let (file, want) = test;
            let got = day07::sum_small_dirs(file);
            assert_eq!(got, want, "got {}, wanted {}", got, want)
        }
    }
}
