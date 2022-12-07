#![allow(dead_code)]
#![allow(unused_variables)]

use crate::prelude::*;

enum Node {
    Dir(HashMap<String, Node>),
    File(i64),
}

impl Node {
    fn new_dir() -> Self {
        Self::Dir(HashMap::new())
    }
    
    fn new_file(size: i64) -> Self {
        Self::File(size)
    }
    
    fn size(&self) -> i64 {
        match self {
            Self::File(size) => *size,
            Self::Dir(children) => children
                .values()
                .map(|child| child.size())
                .sum(),
        }
    }
    
    fn iter(&self) -> NodeIter<'_> {
        let mut next = std::collections::VecDeque::new();
        next.push_back(("", self));
        NodeIter { next }
    }

    fn resolve_path(&mut self, path: &[String]) -> &mut Self {
        let mut pwd = self;
        for name in path {
            let Node::Dir(children) = pwd 
            else { panic!("invalid path") };

            pwd = children.get_mut(name).unwrap();
        }
        pwd
    }
}

struct NodeIter<'a> {
    next: std::collections::VecDeque<(&'a str, &'a Node)>,
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = (&'a str, &'a Node);
    
    fn next(&mut self) -> Option<Self::Item> {
        let Some(next) = self.next.pop_back()
        else { return None };
        if let Node::Dir(children) = next.1 {
            for (name, child) in children.iter() {
                self.next.push_front((name, child));
            }
        }
        Some(next)
    }
}

pub struct DirTree {
    root: Node,
}

pub fn parse(fh: File) -> Result<DirTree> {
    let mut lines = parse::lines(fh).peekable();
    let mut root = Node::new_dir();
    let mut path = vec![];

    while let Some(cmdline) = lines.by_ref().next() {
        let Some(captures) = &regex_captures!(
            r"^\$ (cd|ls)(?: (.*))?$",
            &cmdline
        )
        else { bail!("didn't match cmdline: '{}'", cmdline) };
        let Some(cmd) = captures.get(1)
        else { bail!("no cmd found in cmdline: '{}'", cmdline) };

        let pwd = root.resolve_path(&path);

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
                        let Node::Dir(children) = pwd 
                        else { panic!("invalid path") };

                        if !children.contains_key(dirname) {
                            children.insert(
                                dirname.to_string(),
                                Node::new_dir(),
                            );
                        }
                        path.push(dirname.to_string());
                    }
                }
            }
            "ls" => {
                loop {
                    let Node::Dir(children) = pwd 
                    else { panic!("invalid path") };

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
                    children.insert(name.as_str().to_string(), node);
                    lines.next();
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(DirTree { root })
}

pub fn part1(tree: DirTree) -> Result<i64> {
    let mut total = 0;
    for (name, node) in tree.root.iter() {
        if matches!(node, Node::File(_)) {
            continue;
        }
        let size = node.size();
        if size <= 100_000 {
            total += size;
        }
    }
    Ok(total)
}

pub fn part2(tree: DirTree) -> Result<i64> {
    let total = tree.root.size();
    let free = 70_000_000 - total;
    let needed = 30_000_000 - free;
    let mut possible = vec![];
    for (name, node) in tree.root.iter() {
        if matches!(node, Node::File(_)) {
            continue;
        }
        let size = node.size();
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
