#![allow(clippy::if_same_then_else)]

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::FromIterator;

type NormNode = usize;
type NormTree = HashMap<NormNode, Vec<NormNode>>;

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

pub mod generators {
    use crate::NormTree;
    use std::collections::HashMap;

    pub fn diagv() -> NormTree {
        map! {
            0 => vec![2],
            1 => vec![2, 3],
            2 => vec![4],
            3 => vec![4]
        }
    }

    pub fn cyclic() -> NormTree {
        map! {
            0 => vec![1,4],
            1 => vec![],
            2 => vec![6],
            3 => vec![4],
            4 => vec![8],
            5 => vec![4],
            6 => vec![0],
            7 => vec![],
            8 => vec![2,7]
        }
    }

    pub fn cycle(n: usize) -> Result<NormTree, &'static str> {
        if n < 1 {
            return Err("Implemented only for 0<n");
        }
        let mut result = HashMap::new();
        for i in 0..n - 1 {
            result.insert(i, vec![i + 1]);
        }
        result.insert(n - 1, vec![0]);
        Ok(result)
    }

    pub fn sonic(n: usize) -> Result<HashMap<String, Vec<String>>, &'static str> {
        if !(1..=9).contains(&n) {
            return Err("Implemented only for 0<n<10");
        }
        let origin = String::from("x1");
        let mut result = HashMap::new();
        result.insert(origin.clone(), vec![]);
        for i in 2..n + 1 {
            result.get_mut(&origin).unwrap().push(format!("{}2", i));
            for j in 2..i {
                result.insert(format!("{}{}", i, j), vec![format!("{}{}", i, j + 1)]);
            }
            result.insert(format!("{}{}", i, i), vec![origin.clone()]);
        }
        Ok(result)
    }
}

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

pub fn orderings(tree: &NormTree) -> impl Iterator<Item = Vec<&NormNode>> {
    let ns = nodes(tree);
    let k = ns.len();
    ns.into_iter().permutations(k)
}

pub fn normalize(tree: &mut NormTree) {
    let expected: HashSet<NormNode> = nodes(tree).into_iter().cloned().collect();
    for node in expected {
        tree.entry(node).or_default();
    }
}

fn col_is_dsucc_of_row(tree: &NormTree, row: NormNode, col: NormNode) -> bool {
    tree[&row].contains(&col)
}

fn right_is_dsucc_of_row(tree: &NormTree, row: NormNode, col: NormNode) -> bool {
    match tree[&row].iter().max() {
        Some(last_ddesc) => col < *last_ddesc,
        None => false,
    }
}

fn col_is_dsucc_of_above(tree: &NormTree, row: NormNode, col: NormNode) -> bool {
    for row_above in 0..row {
        if tree[&row_above].contains(&col) {
            return true;
        }
    }
    false
}

fn right_is_dsucc_of_above(tree: &NormTree, row: NormNode, col: NormNode) -> bool {
    for row_above in 0..row {
        if right_is_dsucc_of_row(tree, row_above, col) {
            return true;
        }
    }
    false
}

fn row_is_dpred_of_left(tree: &NormTree, row: NormNode, col: NormNode) -> bool {
    for col_left in 0..col {
        if tree[&row].contains(&col_left) {
            return true;
        }
    }
    false
}

fn anywhere_is_dsucc_of_col(tree: &NormTree, _row: NormNode, col: NormNode) -> bool {
    for dsuccs in tree.values() {
        if dsuccs.contains(&col) {
            return true;
        }
    }
    false
}

fn below_is_dsucc_of_col(tree: &NormTree, row: NormNode, col: NormNode) -> bool {
    let n = tree.len();
    for row_below in row..n {
        if tree[&row_below].contains(&col) {
            return true;
        }
    }
    false
}

fn left_is_dsucc_of_row(tree: &NormTree, row: NormNode, col: NormNode) -> bool {
    for left_col in 0..col {
        if tree[&row].contains(&left_col) {
            return true;
        }
    }
    false
}

static MAX_COL_WIDTH: usize = 10;
static PADDINGS: &str = "          ";
static HORIZONTALS: &str = "----------";

static NOTHING: &str = "";
static PADDING: &str = " ";
static JUNCTION: &str = "+";
static VERTICAL: &str = "|";
static HORIZONTAL: &str = "-";

fn normalized<T: Eq + Hash + Debug>(tree: &HashMap<T, Vec<T>>, order: &[T]) -> NormTree {
    let mut lut = HashMap::new();
    for (i, node) in order.iter().enumerate() {
        lut.insert(node, i);
    }
    let mut result = HashMap::new();
    for (k, old) in tree {
        let mut new = Vec::with_capacity(old.len());
        for v in old {
            new.push(lut[v]);
        }
        result.insert(lut[k], new);
    }
    result
}

pub fn formatted<T: Eq + Hash + Display + Debug>(
    tree: &HashMap<T, Vec<T>>,
    order: &Vec<T>,
) -> String {
    let n = order.len();

    let mut names: Vec<String> = Vec::with_capacity(n);
    for node in order {
        if tree[node].contains(node) {
            panic!("not implemented")
        }

        let name = format!("{}", node);
        if name.len() > MAX_COL_WIDTH {
            panic!("not implemented")
        }
        names.push(name);
    }
    let names = names;

    let mut col_widths = Vec::with_capacity(n);
    for name in names.iter() {
        col_widths.push(name.len());
    }
    let col_widths = col_widths;

    let tree = normalized(tree, order);
    let norm = &tree;
    let mut result = String::new();
    for row in 0..n {
        if row != 0 {
            result.push('\n')
        }
        for col in 0..n {
            let col_width = col_widths[col];

            let ll = if col == 0 && !anywhere_is_dsucc_of_col(norm, row, col) {
                NOTHING
            } else if col < row {
                if col_is_dsucc_of_row(norm, row, col) {
                    JUNCTION
                } else if below_is_dsucc_of_col(norm, row, col) {
                    VERTICAL
                } else if row_is_dpred_of_left(norm, row, col) {
                    HORIZONTAL
                } else {
                    PADDING
                }
            } else if col == row {
                if anywhere_is_dsucc_of_col(norm, row, col) {
                    JUNCTION
                } else if left_is_dsucc_of_row(norm, row, col) {
                    HORIZONTAL
                } else if right_is_dsucc_of_above(norm, row, col) {
                    PADDING
                } else if col != 0 {
                    PADDING
                } else {
                    NOTHING
                }
            } else if col_is_dsucc_of_row(norm, row, col) {
                JUNCTION
            } else if col_is_dsucc_of_above(norm, row, col) {
                VERTICAL
            } else if right_is_dsucc_of_row(norm, row, col) {
                HORIZONTAL
            } else if right_is_dsucc_of_above(norm, row, col) {
                PADDING
            } else {
                NOTHING
            };

            let lr = if col == 0 && !anywhere_is_dsucc_of_col(norm, row, col) {
                NOTHING
            } else if col < row {
                if col_is_dsucc_of_row(norm, row, col) {
                    HORIZONTAL
                } else if row_is_dpred_of_left(norm, row, col) {
                    HORIZONTAL
                } else if below_is_dsucc_of_col(norm, row, col) {
                    PADDING
                } else {
                    PADDING
                }
            } else if col == row {
                if anywhere_is_dsucc_of_col(norm, row, col) {
                    HORIZONTAL
                } else if left_is_dsucc_of_row(norm, row, col) {
                    HORIZONTAL
                } else if right_is_dsucc_of_above(norm, row, col) {
                    PADDING
                } else if col != 0 {
                    PADDING
                } else {
                    NOTHING
                }
            } else if right_is_dsucc_of_row(norm, row, col) {
                HORIZONTAL
            } else if right_is_dsucc_of_above(norm, row, col) {
                PADDING
            } else {
                NOTHING
            };

            let cc = if col < row {
                if col_is_dsucc_of_row(norm, row, col) {
                    &HORIZONTALS[..col_width]
                } else if row_is_dpred_of_left(norm, row, col) {
                    &HORIZONTALS[..col_width]
                } else if below_is_dsucc_of_col(norm, row, col) {
                    &PADDINGS[..col_width]
                } else {
                    &PADDINGS[..col_width]
                }
            } else if col == row {
                &names[col]
            } else if right_is_dsucc_of_row(norm, row, col) {
                &HORIZONTALS[..col_width]
            } else if right_is_dsucc_of_above(norm, row, col) {
                &PADDINGS[..col_width]
            } else {
                NOTHING
            };

            let rr = if col < row {
                if col_is_dsucc_of_row(norm, row, col) {
                    HORIZONTAL
                } else if row_is_dpred_of_left(norm, row, col) {
                    HORIZONTAL
                } else if below_is_dsucc_of_col(norm, row, col) {
                    PADDING
                } else {
                    PADDING
                }
            } else if col == row {
                if right_is_dsucc_of_row(norm, row, col) {
                    HORIZONTAL
                } else if right_is_dsucc_of_above(norm, row, col) {
                    PADDING
                } else {
                    NOTHING
                }
            } else if right_is_dsucc_of_row(norm, row, col) {
                HORIZONTAL
            } else if right_is_dsucc_of_above(norm, row, col) {
                PADDING
            } else {
                NOTHING
            };
            result.push_str(ll);
            result.push_str(lr);
            result.push_str(cc);
            result.push_str(rr);
        }
    }
    result
}

#[cfg(test)]
static DIAGV_TEXT: &str = "\
0-----+
    1-+---+
      +-2-|---+
          +-3-+
              +-4\
";

#[cfg(test)]
static CYCLIC_TEXT: &str = "\
+-0-+-----------+
|   +-1         |
|       +-2-----|-------+
|       |     3-+       |
|       |       +-4-----|-------+
|       |       +-----5 |       |
+-------|---------------+-6     |
        |                   +-7 |
        +-------------------+---+-8\
";

#[cfg(test)]
static SONIC_3_TEXT: &str = "\
+-x1-+----+
+----+-22 |
|         +-32-+
+--------------+-33\
";

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::*;

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

    #[test]
    fn test_format_works_for_diagv() {
        let mut graph = generators::diagv();
        normalize(&mut graph);
        let actual = formatted(&graph, &vec![0, 1, 2, 3, 4]);
        let expected = DIAGV_TEXT;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_format_works_for_cyclic() {
        let mut graph = generators::cyclic();
        normalize(&mut graph);
        let actual = formatted(&graph, &vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let expected = CYCLIC_TEXT;
        assert_eq!(actual, expected);
    }
    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_format_not_implemented_for_cycle_1() {
        let graph = generators::cycle(1).unwrap();
        formatted(&graph, &vec![0]);
    }
    #[test]
    fn test_format_works_for_sonic_3() {
        let graph = generators::sonic(3).unwrap();
        let actual = formatted(
            &graph,
            &vec![
                String::from("x1"),
                String::from("22"),
                String::from("32"),
                String::from("33"),
            ],
        );
        let expected = SONIC_3_TEXT;
        assert_eq!(actual, expected);
    }
}
