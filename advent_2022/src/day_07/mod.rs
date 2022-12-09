//! Solutions to 2020 day 7 problems
//! --- Day 7: No Space Left On Device ---
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::read_file;

#[derive(Debug)]
enum Entry {
    /// a file with size
    File(u32),
    /// a directory containing zero or more [Entry]s
    Dir {
        parent: String,
        children: RefCell<Vec<Rc<Entry>>>,
    },
}

impl Entry {
    pub fn new_dir(parent: String) -> Self {
        Self::Dir {
            parent,
            children: Default::default(),
        }
    }
}

fn parse_terminal(input: &str) -> HashMap<String, Rc<Entry>> {
    let mut result = HashMap::new();
    let mut current_dir = "".to_string();
    let mut path = vec![""];

    let root = Rc::new(Entry::new_dir("/".to_string()));
    result.insert(path.join("/"), root);

    for line in input.lines().filter(|line| line != &"$ ls") {
        if let Some((one, two)) = line.split_once(' ') {
            if two.split_once(' ').unwrap_or(("", "")).0 == "cd" {
                let next_dir = two.split_whitespace().last().unwrap();
                match next_dir {
                    ".." => {
                        // up one dir
                        path.pop();

                        if let Entry::Dir {
                            parent,
                            children: _,
                        } = result.get(&path.join("/")).unwrap().as_ref()
                        {
                            current_dir = parent.clone();
                        }
                    }
                    "/" => {
                        path = vec![""];
                        current_dir = "".to_string();
                    }
                    _ => {
                        // set cwd
                        path.push(next_dir);
                        current_dir = path.join("/");
                    }
                }

                continue;
            }

            match one {
                "dir" => {
                    let parent = path.join("/");
                    let dir = Rc::new(Entry::new_dir(parent.clone()));
                    let dir_name = format!("{}/{}", parent, two.trim());

                    result.insert(dir_name, dir.clone());

                    result.entry(current_dir.clone()).and_modify(|entry| {
                        if let Entry::Dir {
                            parent: _,
                            children,
                        } = entry.as_ref()
                        {
                            children.borrow_mut().push(dir);
                        }
                    });
                }
                _ => {
                    result.entry(current_dir.clone()).and_modify(|entry| {
                        if let Entry::Dir {
                            parent: _,
                            children,
                        } = entry.as_ref()
                        {
                            let file_size = one
                                .parse()
                                .unwrap_or_else(|_| panic!("Failed to parse number: {}", one));
                            let file = Rc::new(Entry::File(file_size));
                            children.borrow_mut().push(file);
                        }
                    });
                }
            }
        }
    }

    result
}

fn size(entry: &Entry) -> u32 {
    match entry {
        Entry::File(size) => *size,
        Entry::Dir {
            parent: _,
            children,
        } => children.borrow().iter().map(|entry| size(entry)).sum(),
    }
}

/// returns the sum of all directories that are of size <= 100000.
pub fn one(file_path: &str) -> u32 {
    let input = read_file(file_path);
    parse_terminal(&input)
        .iter()
        .filter_map(|(_name, entry)| {
            let sum = size(entry);

            if sum <= 100000 {
                Some(sum)
            } else {
                None
            }
        })
        .sum()
}

/// returns the size of the smallest directory that will free enough space
pub fn two(file_path: &str) -> u32 {
    const MAX_SPACE: u32 = 70000000;
    const REQUIRED_SPACE: u32 = 30000000;
    let input = read_file(file_path);
    let fs = parse_terminal(&input);

    let root_size = size(fs.get("").unwrap());
    let available_space = MAX_SPACE - root_size;
    let needed_space = REQUIRED_SPACE - available_space;

    fs.iter()
        .filter_map(|(_name, entry)| {
            let sum = size(entry);

            if sum >= needed_space {
                Some(sum)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn size_t() {
        let dir = Rc::new(Entry::Dir {
            parent: "/".to_string(),
            children: RefCell::new(vec![
                Rc::new(Entry::File(42)),
                Rc::new(Entry::Dir {
                    parent: "a".to_string(),
                    children: RefCell::new(vec![Rc::new(Entry::File(42))]),
                }),
            ]),
        });

        let msg = "should return the size of an entry (and all of its children)";
        let expected = 84;
        let actual = size(&dir);
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should return the sum of all directories that are of size <= 100000.";
        let expected = 95437;
        let actual = one("input/07-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return the size of the smallest directory that will free enough space";
        let expected = 24933642;
        let actual = two("input/07-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
