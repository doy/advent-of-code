use crate::prelude::*;

#[derive(Debug)]
pub struct Tree<K: std::hash::Hash + std::cmp::Eq, V> {
    data: V,
    children: HashMap<K, Tree<K, V>>,
}

impl<K, V> Tree<K, V>
where
    K: std::hash::Hash + std::cmp::Eq + std::clone::Clone,
{
    pub fn new(data: V) -> Self {
        Self {
            data,
            children: HashMap::new(),
        }
    }

    pub fn data(&self) -> &V {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut V {
        &mut self.data
    }

    pub fn add_child(&mut self, name: K, data: V) {
        self.children.insert(name, Self::new(data));
    }

    pub fn at<'a, I>(&'a self, path: I) -> Option<&'a Self>
    where
        I: IntoIterator<Item = &'a K>,
    {
        let mut ret = self;
        for entry in path {
            let child = ret.children.get(entry)?;
            ret = child;
        }
        Some(ret)
    }

    pub fn at_mut<'a, I>(&'a mut self, path: I) -> Option<&'a mut Self>
    where
        I: IntoIterator<Item = &'a K>,
    {
        let mut ret = self;
        for entry in path {
            let child = ret.children.get_mut(entry)?;
            ret = child;
        }
        Some(ret)
    }

    pub fn bfs(&self) -> BFS<'_, K, V> {
        let mut next = std::collections::VecDeque::new();
        next.push_front((vec![], self));
        BFS { next }
    }

    pub fn dfs(&self) -> DFS<'_, K, V> {
        DFS {
            next: vec![(vec![], self)],
        }
    }
}

pub struct BFS<'a, K, V>
where
    K: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
{
    next: std::collections::VecDeque<(Vec<K>, &'a Tree<K, V>)>,
}

impl<'a, K, V> Iterator for BFS<'a, K, V>
where
    K: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
{
    type Item = (Vec<K>, &'a Tree<K, V>);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.pop_front()?;

        self.next.extend(next.1.children.iter().map(|(k, v)| {
            (
                next.0
                    .clone()
                    .into_iter()
                    .chain(std::iter::once(k.clone()))
                    .collect(),
                v,
            )
        }));
        Some(next)
    }
}

pub struct DFS<'a, K, V>
where
    K: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
{
    next: Vec<(Vec<K>, &'a Tree<K, V>)>,
}

impl<'a, K, V> Iterator for DFS<'a, K, V>
where
    K: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
{
    type Item = (Vec<K>, &'a Tree<K, V>);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.pop()?;

        self.next.extend(next.1.children.iter().map(|(k, v)| {
            (
                next.0
                    .clone()
                    .into_iter()
                    .chain(std::iter::once(k.clone()))
                    .collect(),
                v,
            )
        }));
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        enum Entry {
            Node,
            Leaf(u32),
        }

        impl Entry {
            fn value(&self) -> u32 {
                match self {
                    Self::Node => 0,
                    Self::Leaf(value) => *value,
                }
            }
        }

        let mut tree = Tree::new(Entry::Node);
        tree.add_child("foo", Entry::Node);
        tree.add_child("bar", Entry::Leaf(23));
        tree.at_mut(&["foo"])
            .unwrap()
            .add_child("baz", Entry::Leaf(42));

        let bfs = tree.bfs().map(|(path, _)| path).collect::<Vec<_>>();
        assert_eq!(bfs[0], Vec::<&str>::new());
        assert!(bfs[1] == vec!["foo"] || bfs[1] == vec!["bar"]);
        assert!(bfs[2] == vec!["foo"] || bfs[2] == vec!["bar"]);
        assert!(bfs[1] != bfs[2]);
        assert_eq!(bfs[3], vec!["foo", "baz"]);

        let dfs = tree.dfs().map(|(path, _)| path).collect::<Vec<_>>();
        assert_eq!(dfs[0], Vec::<&str>::new());
        assert!(dfs[1] == vec!["foo"] || dfs[1] == vec!["bar"]);
        if dfs[1] == vec!["foo"] {
            assert_eq!(dfs[2], vec!["foo", "baz"]);
            assert_eq!(dfs[3], vec!["bar"]);
        } else if dfs[1] == vec!["bar"] {
            assert_eq!(dfs[2], vec!["foo"]);
            assert_eq!(dfs[3], vec!["foo", "baz"]);
        } else {
            unreachable!()
        }

        assert_eq!(
            tree.bfs().map(|(_, tree)| tree.data().value()).sum::<u32>(),
            65
        );
        assert_eq!(
            tree.dfs().map(|(_, tree)| tree.data().value()).sum::<u32>(),
            65
        );
    }
}
