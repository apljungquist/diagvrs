use anyhow::bail;
use diagv::Graph;
use leptos::prelude::RwSignal;
use leptos::prelude::*;
use leptos::wasm_bindgen::UnwrapThrowExt;
use leptos::{component, view, IntoView};
use thaw::*;

enum Order {
    Original,
    Alphabetical,
    Degree,
}

fn render(dot: &str, order_algorithm: &str) -> anyhow::Result<String> {
    let order_algorithm = match order_algorithm {
        "ori" => Order::Original,
        "alp" => Order::Alphabetical,
        "deg" => Order::Degree,
        v => unreachable!("{v}"),
    };

    let graph = Graph::parse_dot(dot)?;
    let mut nodes = graph.nodes();
    let tails = graph.tails();

    match order_algorithm {
        Order::Original => {}
        Order::Degree => {
            nodes.sort_by_key(|k| tails[k].len());
        }
        Order::Alphabetical => nodes.sort(),
    }

    graph.ascii_with_order(&nodes)
}
#[component]
pub fn OutputPage(topology: String, on_edit: impl Fn() + Send + Sync + 'static) -> impl IntoView {
    let order_algorithm = RwSignal::new("ori".to_string());
    view! {
        <Flex style="height: 100dvh; gap: 0;" vertical=true>
            <code style="display:flex; height: 100%; white-space: pre; overflow:auto;">
                {move || render(&topology, &order_algorithm.get()).unwrap_throw()}
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
