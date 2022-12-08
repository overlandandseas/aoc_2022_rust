use std::{cell::RefCell, rc::Rc};

struct File {
    size: usize,
}

struct Directory {
    name: String,
    files: Vec<File>,
    dirs: Vec<Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    fn add_dir(&mut self, new_directory: Rc<RefCell<Directory>>) {
        self.dirs.push(new_directory);
    }

    fn add_file(&mut self, size: usize) {
        self.files.push(File { size });
    }

    fn new(name: &str) -> Directory {
        Directory {
            name: name.to_owned(),
            dirs: vec![],
            files: vec![],
            parent: None,
        }
    }

    fn size(&self) -> usize {
        self.files.iter().fold(0, |acc, f| acc + f.size)
            + self.dirs.iter().fold(0, |acc, d| acc + d.borrow().size())
    }

    fn totals(&self, counting: &mut Vec<usize>) -> (usize, Vec<usize>) {
        let total_size = self.files.iter().fold(0, |acc, file| acc + file.size)
            + self
                .dirs
                .iter()
                .fold(0, |acc, dir| acc + dir.borrow().totals(counting).0);

        counting.push(total_size);
        (total_size, counting.to_vec())
    }

    fn totals_vec(&self) -> Vec<usize> {
        let mut counting: Vec<usize> = vec![];
        let (_, totals_vec) = self.totals(&mut counting);

        totals_vec
    }
}

#[derive(PartialEq)]
enum Mode {
    Command,
    Insert,
}

fn init_filesystem(input: &str) -> Rc<RefCell<Directory>> {
    let mut current_mode = Mode::Command;
    let root = Rc::new(RefCell::new(Directory::new("/")));
    let mut current = Rc::clone(&root);

    for line in input.lines() {
        if Mode::Insert == current_mode {
            let (size, name) = line.split_once(' ').unwrap();
            if let Ok(val) = size.parse::<usize>() {
                // Number found, add file to current directory
                current.borrow_mut().add_file(val)
            } else {
                match size {
                    "$" => current_mode = Mode::Command,
                    "dir" => {
                        let child = Rc::new(RefCell::new(Directory::new(name)));
                        current.borrow_mut().add_dir(Rc::clone(&child));
                        let mut mut_child = child.borrow_mut();

                        mut_child.parent = Some(Rc::clone(&current));
                    }
                    _ => panic!("Unknown Command {size}"),
                }
            };
        }

        if Mode::Command == current_mode {
            let args = line.split(' ').collect::<Vec<_>>();
            match args[1] {
                "ls" => current_mode = Mode::Insert,
                "cd" => {
                    if args[2] == ".." {
                        let current_clone = Rc::clone(&current);
                        current = Rc::clone(current_clone.borrow_mut().parent.as_ref().unwrap());
                    } else if args[2] != "/" {
                        let cloned = Rc::clone(&current);
                        let child_dir = cloned.borrow();

                        let thing = child_dir
                            .dirs
                            .iter()
                            .find(|c| c.borrow().name == args[2])
                            .unwrap();

                        current = Rc::clone(thing);
                    }
                }
                _ => panic!("Command {} not found", args[1]),
            }
        }
    }
    root
}

fn part_one(input: &str) -> usize {
    let root = init_filesystem(input);
    let totals_vec = root.borrow().totals_vec();
    totals_vec.iter().filter(|c| *c < &100000).sum()
}

fn part_two(input: &str) -> usize {
    let root = init_filesystem(input);
    let total_size = 70_000_000;
    let free_needed = 30_000_000 - (total_size - root.borrow().size());

    let totals_vec = root.borrow().totals_vec();
    *totals_vec
        .iter()
        .filter(|c| *c >= &free_needed)
        .min()
        .unwrap()
}
fn main() {
    let input = include_str!("../input");
    println!("Part One: {}", part_one(&input));
    println!("Part Two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&TEST_INPUT), 95437);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&TEST_INPUT), 24933642);
    }
}
