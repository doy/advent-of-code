#![allow(dead_code)]
#![allow(unused_variables)]

use advent_of_code::prelude::*;

pub enum Node {
    Dir,
    File(u64),
}

impl Node {
    fn new_dir() -> Self {
        Self::Dir
    }

    fn new_file(size: u64) -> Self {
        Self::File(size)
    }

    fn size(&self) -> u64 {
        match self {
            Self::Dir => 0,
            Self::File(size) => *size,
        }
    }
}

fn tree_size(tree: &Tree<String, Node>) -> u64 {
    tree.bfs().map(|(_, tree)| tree.data().size()).sum()
}

pub fn parse(fh: File) -> Result<Tree<String, Node>> {
    let mut lines = parse::raw_lines(fh).peekable();
    let mut root = Tree::new(Node::Dir);
    let mut path = vec![];

    while let Some(cmdline) = lines.by_ref().next() {
        let Some(captures) = &regex_captures!(
            r"^\$ (cd|ls)(?: (.*))?$",
            &cmdline
        )
        else { bail!("didn't match cmdline: '{}'", cmdline) };
        let Some(cmd) = captures.get(1)
        else { bail!("no cmd found in cmdline: '{}'", cmdline) };

        let pwd = root.at_mut(&path).unwrap();

        match cmd.as_str() {
            "cd" => {
                let Some(arg) = captures.get(2)
                else { bail!("no arg found for cd: '{}'", cmdline) };
                match arg.as_str() {
                    ".." => {
                        path.pop();
                    }
                    "/" => path = vec![],
                    dirname => {
                        if pwd.at(&[dirname.to_string()]).is_none() {
                            pwd.add_child(
                                dirname.to_string(),
                                Node::new_dir(),
                            );
                        }
                        path.push(dirname.to_string());
                    }
                }
            }
            "ls" => loop {
                let Some(lsline) = lines.peek()
                    else { break };
                let Some(captures) = &regex_captures!(
                        r"^(dir|[0-9]+) (.*)$",
                        lsline
                    )
                    else { break };
                let Some(data) = captures.get(1)
                    else { break };
                let Some(name) = captures.get(2)
                    else { break };

                let node = match data.as_str() {
                    "dir" => Node::new_dir(),
                    size => Node::new_file(size.parse().unwrap()),
                };
                pwd.add_child(name.as_str().to_string(), node);
                lines.next();
            },
            _ => unreachable!(),
        }
    }

    Ok(root)
}

pub fn part1(tree: Tree<String, Node>) -> Result<u64> {
    let mut total = 0;
    for (_, tree) in tree.bfs() {
        if matches!(tree.data(), Node::File(_)) {
            continue;
        }
        let size = tree_size(tree);
        if size <= 100_000 {
            total += size;
        }
    }
    Ok(total)
}

pub fn part2(tree: Tree<String, Node>) -> Result<u64> {
    let total = tree_size(&tree);
    let free = 70_000_000 - total;
    let needed = 30_000_000 - free;
    let mut possible = vec![];
    for (_, tree) in tree.bfs() {
        if matches!(tree.data(), Node::File(_)) {
            continue;
        }
        let size = tree_size(tree);
        if size > needed {
            possible.push(size);
        }
    }
    Ok(possible.iter().copied().min().unwrap())
}

#[test]
fn test() {
    assert_eq!(
        part1(parse(parse::data(2022, 7).unwrap()).unwrap()).unwrap(),
        1297683
    );
    assert_eq!(
        part2(parse(parse::data(2022, 7).unwrap()).unwrap()).unwrap(),
        5756764
    );
}
