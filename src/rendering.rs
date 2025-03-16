#![allow(clippy::if_same_then_else)]
use crate::core::Graph;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use anyhow::bail;

type NormNode = usize;
type NormTree = HashMap<NormNode, Vec<NormNode>>;

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

impl<T> Graph<T>
where
    T: Eq + Hash + Display + Debug,
{
    pub fn ascii(&self) -> anyhow::Result<String> {
        self.ascii_with_order(&self.nodes())
    }

    pub fn ascii_with_order(&self, order: &Vec<&T>) -> anyhow::Result<String> {
        let tree = self
            .heads()
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect();
        formatted(&tree, order)
    }
}

fn formatted<T: Eq + Hash + Display + Debug>(tree: &HashMap<T, Vec<T>>, order: &Vec<T>) -> anyhow::Result<String> {
    let n = order.len();

    let mut names: Vec<String> = Vec::with_capacity(n);
    for node in order {
        if tree[node].contains(node) {
            bail!("Omitting self loop")
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
    Ok(result)
}
