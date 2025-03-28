use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

pub struct Graph<T> {
    nodes: Vec<T>,
    heads: Vec<Vec<usize>>,
}

impl<T> Graph<T>
where
    T: Debug + Eq + Hash,
{
    pub(crate) fn from_normalized(
        nodes: HashMap<T, usize>,
        mut heads: HashMap<usize, HashSet<usize>>,
    ) -> Self {
        Self {
            heads: (0..nodes.len())
                .map(|i| {
                    heads
                        .remove(&i)
                        .unwrap_or_default()
                        .into_iter()
                        .sorted()
                        .collect()
                })
                .collect(),
            nodes: nodes
                .into_iter()
                .sorted_by_key(|(_, i)| *i)
                .map(|(n, _)| n)
                .collect(),
        }
    }

    pub(crate) fn from_heads(heads: Vec<(T, Vec<T>)>) -> Self {
        let mut nodes = HashMap::new();
        let mut normalize = |node: T| {
            if nodes.contains_key(&node) {
                nodes[&node]
            } else {
                let i = nodes.len();
                nodes.insert(node, i);
                i
            }
        };

        let mut deferred: HashMap<usize, Vec<T>> = HashMap::new();
        for (t, hs) in heads {
            let t = normalize(t);
            deferred.entry(t).or_default().extend(hs);
        }

        let mut heads: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (t, hs) in deferred.into_iter().sorted_by_key(|(t, _)| *t) {
            for h in hs {
                let h = normalize(h);
                heads.entry(t).or_default().insert(h);
            }
        }

        Self::from_normalized(nodes, heads)
    }
    pub(crate) fn from_edges(edges: Vec<(T, T)>) -> Self {
        let mut nodes = HashMap::new();
        let mut normalize = |node: T| {
            if nodes.contains_key(&node) {
                nodes[&node]
            } else {
                let i = nodes.len();
                nodes.insert(node, i);
                i
            }
        };

        let mut deferred: HashMap<usize, Vec<T>> = HashMap::new();
        for (t, h) in edges {
            let t = normalize(t);
            deferred.entry(t).or_default().push(h);
        }

        let mut heads: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (t, hs) in deferred.into_iter().sorted_by_key(|(t, _)| *t) {
            for h in hs {
                let h = normalize(h);
                heads.entry(t).or_default().insert(h);
            }
        }

        Self::from_normalized(nodes, heads)
    }
}

impl<T> Graph<T>
where
    T: Eq + Hash,
{
    pub fn heads(&self) -> HashMap<&T, HashSet<&T>> {
        let mut heads = HashMap::new();
        for (t, hs) in self.heads.iter().enumerate() {
            let t = &self.nodes[t];
            for h in hs.iter() {
                let h = &self.nodes[*h];
                heads.entry(t).or_insert_with(HashSet::new).insert(h);
            }
        }
        for n in self.nodes.iter() {
            heads.entry(n).or_default();
        }
        heads
    }

    pub fn nodes(&self) -> Vec<&T> {
        self.nodes.iter().collect()
    }

    pub fn tails(&self) -> HashMap<&T, HashSet<&T>> {
        let mut tails = HashMap::new();
        for (t, hs) in self.heads.iter().enumerate() {
            let t = &self.nodes[t];
            for h in hs.iter() {
                let h = &self.nodes[*h];
                tails.entry(h).or_insert_with(HashSet::new).insert(t);
            }
        }
        for n in self.nodes.iter() {
            tails.entry(n).or_default();
        }
        tails
    }
}

#[cfg(test)]
mod tests {
    use crate::generators;
    use std::collections::HashSet;

    #[test]
    fn nodes_contain_source() {
        let tree = generators::diagv();
        assert!(tree.nodes.contains(&'d'));
    }

    #[test]
    fn nodes_contain_sink() {
        let tree = generators::diagv();
        assert!(tree.nodes.contains(&'v'));
    }

    #[test]
    fn nodes_contains_exact() {
        let tree = generators::diagv();
        let actual_nodes: HashSet<_> = tree.nodes.into_iter().collect();
        let expected_nodes: HashSet<_> = "diagv".chars().collect();
        assert_eq!(actual_nodes, expected_nodes);
    }

    #[test]
    fn heads_maps_from_every_node() {
        let graph = generators::diagv();
        assert_eq!(5, graph.heads().into_keys().collect::<HashSet<_>>().len());
    }

    #[test]
    fn tails_maps_from_every_node() {
        let graph = generators::diagv();
        assert_eq!(5, graph.tails().into_keys().collect::<HashSet<_>>().len());
    }
}
