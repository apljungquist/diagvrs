use crate::core::Graph;
use anyhow::bail;
use dot_parser::ast::{EdgeStmt, NodeID, Stmt};
use dot_parser::canonical::Edge;
use itertools::Either;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

/// Return nodes in the order that they should intuitively appear.
fn nodes_in_order(
    graph: dot_parser::ast::Graph<(&str, &str)>,
) -> anyhow::Result<HashMap<String, usize>> {
    let mut nodes = HashMap::new();
    let mut add_node = |node: NodeID| {
        if node.port.is_some() {
            bail!("Node has port")
        }
        if !nodes.contains_key(&node.id) {
            nodes.insert(node.id, nodes.len());
        }
        Ok(())
    };

    let mut deferred = Vec::new();
    for stmt in graph.stmts {
        match stmt {
            Stmt::NodeStmt(stmt) => add_node(stmt.node)?,
            Stmt::EdgeStmt(stmt) => {
                for EdgeStmt { from, next, .. } in stmt.flatten() {
                    match from {
                        Either::Left(node) => add_node(node)?,
                        Either::Right(_) => bail!("Unsupported edge from Subgraph"),
                    };

                    // `.flatten()` makes this impossible
                    debug_assert!(next.next.is_none());
                    match next.to {
                        Either::Left(node) => deferred.push(node),
                        Either::Right(_) => bail!("Unsupported edge to Subgraph"),
                    }
                }
            }
            Stmt::AttrStmt(_) => { /* I don't think these should affect the order */ }
            Stmt::IDEq(_, _) => bail!("Unsupported IDEq"),
            Stmt::Subgraph(_) => bail!("Unsupported Subgraph"),
        }
    }

    for node in deferred {
        add_node(node)?;
    }

    Ok(nodes)
}

fn heads(
    nodes: &HashMap<String, usize>,
    graph: dot_parser::canonical::Graph<(&str, &str)>,
) -> HashMap<usize, HashSet<usize>> {
    let mut heads: HashMap<usize, HashSet<usize>> = HashMap::new();
    for Edge { from, to, .. } in graph.edges.set {
        heads.entry(nodes[&from]).or_default().insert(nodes[&to]);
    }
    heads
}
impl Graph<String> {
    pub fn parse_dot(s: &str) -> anyhow::Result<Self> {
        let ast = dot_parser::ast::Graph::try_from(s)?;
        let nodes = nodes_in_order(ast.clone())?;
        let heads = heads(&nodes, dot_parser::canonical::Graph::from(ast));

        if nodes.iter().any(|(n, _)| n.trim_matches('"') == n) {
            bail!("Nodes must have double quoted id");
        }

        Ok(Self::from_normalized(
            nodes
                .into_iter()
                .map(|(node, i)| (node.trim_matches('"').to_string(), i))
                .collect(),
            heads,
        ))
    }
}
