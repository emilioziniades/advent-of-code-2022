use std::collections::hash_map::RandomState;
use std::fs;
use std::hash::BuildHasher;
use std::hash::Hasher;

const SMALL_DIR_SIZE: u64 = 100_000;
const DISK_SIZE: u64 = 70_000_000;
const FREE_SPACE_REQUIRED: u64 = 30_000_000;

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
    parent_id: Option<Id>,
    name: String,
    children: Vec<FileSystemItem>,
}

impl Directory {
    fn new(name: &str, parent: Option<Id>) -> Self {
        Directory {
            parent_id: parent,
            name: name.to_string(),
            children: Vec::new(),
            id: id(),
        }
    }

    fn add_child(&mut self, child: FileSystemItem) {
        self.children.push(child);
    }

    fn chdir(&mut self, name: &str) -> &mut Self {
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

    fn find<'a>(&'a mut self, id: Id, result: &mut Option<&'a mut Self>) {
        if self.id == id {
            *result = Some(self);
            return;
        }
        for child in &mut self.children {
            if let FileSystemItem::Directory(child) = child {
                child.find(id, result);
            }
        }
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

struct DirectoryData {
    name: String,
    size: u64,
}

fn collect_directory_data(file: &str) -> Vec<DirectoryData> {
    let commands = fs::read_to_string(file).expect("file exists");
    let commands = commands
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        // skip "$ cd /"
        .skip(1);

    let mut root = Directory::new("/", None);
    let mut cwd = &mut root;

    for command in commands {
        match command[..] {
            ["$", "cd", ".."] => {
                let parent_id = cwd.parent_id.unwrap();
                let mut parent = None;
                root.find(parent_id, &mut parent);

                let Some(parent) = parent else {
                    panic!("directory should have parent");
                };

                cwd = parent;
            }
            ["$", "cd", location] => {
                cwd = cwd.chdir(location);
            }
            ["$", "ls"] => (),
            ["dir", dir_name] => {
                let dir = FileSystemItem::Directory(Directory::new(dir_name, Some(cwd.id)));
                cwd.add_child(dir);
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
        .map(|dir| DirectoryData {
            name: dir.name.clone(),
            size: dir.size().into(),
        })
        .collect()
}

pub fn sum_small_dirs(filename: &str) -> u64 {
    let dirs = collect_directory_data(filename);

    dirs.iter()
        .filter_map(|dir| {
            if dir.size < SMALL_DIR_SIZE {
                Some(dir.size)
            } else {
                None
            }
        })
        .sum()
}

pub fn delete_directory(filename: &str) -> u64 {
    let mut dirs = collect_directory_data(filename);
    dirs.sort_by(|dir_a, dir_b| dir_a.size.cmp(&dir_b.size));

    let root_dir = dirs.pop().unwrap();
    assert_eq!(root_dir.name, "/");

    let free_space = DISK_SIZE - root_dir.size;

    dirs.iter()
        .filter_map(|dir| {
            if free_space + dir.size > FREE_SPACE_REQUIRED {
                Some(dir.size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{day07, fetch_input};

    #[test]
    fn sum_small_dirs() {
        fetch_input(7);

        let tests = vec![
            ("example/day07.txt", 95_437),
            ("input/day07.txt", 1_491_614),
        ];

        for test in tests {
            let (file, want) = test;
            let got = day07::sum_small_dirs(file);
            assert_eq!(got, want, "got {}, wanted {}", got, want)
        }
    }

    #[test]
    fn delete_directory() {
        fetch_input(7);

        let tests = vec![
            ("example/day07.txt", 24_933_642),
            ("input/day07.txt", 6_400_111),
        ];

        for test in tests {
            let (file, want) = test;
            let got = day07::delete_directory(file);
            assert_eq!(got, want, "got {}, wanted {}", got, want)
        }
    }
}
