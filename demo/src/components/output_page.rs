use dot_parser::canonical;
use leptos::prelude::RwSignal;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use std::collections::HashMap;
use thaw::*;
enum Order {
    Original,
    Alphabetical,
    Degree,
}
fn demo(dot: &str, strategy: Order) -> String {
    let ast = dot_parser::ast::Graph::try_from(dot).unwrap();
    let graph = canonical::Graph::from(ast.clone());

    let mut order: Vec<String> = Vec::new();
    for stmt in ast.stmts.into_iter() {
        if let Some(edge) = stmt.get_edge() {
            let tail = edge
                .from
                .expect_left("Oops")
                .id
                .trim_matches('"')
                .to_string();
            if !order.contains(&tail) {
                order.push(tail);
            }
        }
    }
    let mut tree: HashMap<String, Vec<String>> = HashMap::new();
    for edge in graph.edges.set {
        let from = edge.from.trim_matches('"').to_string();
        let to = edge.to.trim_matches('"').to_string();
        if !order.contains(&to) {
            order.push(to.clone());
        }
        if !order.contains(&from) {
            order.push(from.clone());
        }
        tree.entry(from).or_default().push(to);
    }

    for k in &order {
        tree.entry(k.clone()).or_default();
    }

    let mut inv_tree: HashMap<String, Vec<String>> = HashMap::new();
    for (k, vs) in tree.iter() {
        inv_tree.entry(k.clone()).or_default();
        for v in vs {
            let entry = inv_tree.entry(v.clone()).or_default();
            if !entry.contains(k) {
                entry.push(k.clone());
            }
        }
    }

    match strategy {
        Order::Original => {}
        Order::Degree => {
            order.sort_by_key(|k| inv_tree[k].len());
        }
        Order::Alphabetical => order.sort(),
    }

    diagv::formatted(&tree, &order)
}

fn render(dot: &str, order_algorithm: &str) -> String {
    let order_algorithm = match order_algorithm {
        "ori" => Order::Original,
        "alp" => Order::Alphabetical,
        "deg" => Order::Degree,
        v => unreachable!("{v}"),
    };
    demo(dot, order_algorithm)
}
#[component]
pub fn OutputPage(topology: String, on_edit: impl Fn() + Send + Sync + 'static) -> impl IntoView {
    let order_algorithm = RwSignal::new("ori".to_string());
    view! {
        <Flex style="height: 100dvh; gap: 0;" vertical=true>
            <code style="display:flex; height: 100%; white-space: pre; overflow:auto;">
                {move || render(&topology, &order_algorithm.get())}
            </code>
            <Flex style="padding: 1em">
                <Button
                    appearance=ButtonAppearance::Secondary
                    on:click=move |_| on_edit()
                    icon=icondata_mdi::MdiChevronLeft
                >
                    Edit input
                </Button>
                <RadioGroup value=order_algorithm>
                    <Radio value="ori" label="Original" />
                    <Radio value="alp" label="Alphabetical" />
                    <Radio value="deg" label="Degree" />
                </RadioGroup>
            </Flex>
        </Flex>
    }
}
