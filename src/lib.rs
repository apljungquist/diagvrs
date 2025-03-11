mod generating;
mod rendering;
pub mod generators {
    pub use crate::generating::*;
}

pub use rendering::formatted;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

type NormNode = usize;
type NormTree = HashMap<NormNode, Vec<NormNode>>;
pub fn nodes(tree: &NormTree) -> HashSet<&NormNode> {
    let mut result = HashSet::from_iter(tree.keys());
    for nodes in tree.values() {
        for node in nodes {
            if !result.contains(node) {
                result.insert(node);
            }
        }
    }
    result
}

pub fn normalize(tree: &mut NormTree) {
    let expected: HashSet<NormNode> = nodes(tree).into_iter().cloned().collect();
    for node in expected {
        tree.entry(node).or_default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! map (
    { $($key:expr => $value:expr),+ $(,)? } => {
        {
            let mut result = HashMap::new();
            $(
                result.insert($key, $value);
            )+
            result
        }
     };
);

    #[test]
    fn nodes_contain_source() {
        let tree = generators::diagv();
        assert!(nodes(&tree).contains(&&1));
    }

    #[test]
    fn nodes_contain_sink() {
        let tree = generators::diagv();
        assert!(nodes(&tree).contains(&&4));
    }

    #[test]
    fn nodes_contains_exact() {
        let tree = generators::diagv();
        let actual_nodes = nodes(&tree);
        let expected_nodes = [0, 1, 2, 3, 4];
        assert_eq!(actual_nodes, expected_nodes.iter().collect());
    }

    #[test]
    fn normalized_maps_from_every_node() {
        let mut actual = generators::diagv();
        let expected = map! {
            0 => vec![2],
            1 => vec![2, 3],
            2 => vec![4],
            3 => vec![4],
            4 => vec![]
        };
        assert_ne!(actual, expected);
        normalize(&mut actual);
        assert_eq!(actual, expected);
    }
}
