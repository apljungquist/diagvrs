use diagv::generators;
use leptos::prelude::RwSignal;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use std::collections::HashMap;
use std::fmt::Display;
use thaw::*;

static DIAGV_INPUT: &str = r#"digraph {
  "d" -> "a";
  "i" -> "a";
  "i" -> "g";
  "a" -> "v";
  "g" -> "v";
}
"#;

fn fmt_dot<N: Display>(input: HashMap<N, Vec<N>>) -> String {
    let mut output = String::new();
    output.push_str("digraph {\n");
    for (tail, heads) in input.into_iter() {
        for head in heads.into_iter() {
            output.push_str(&format!("  \"{tail}\" -> \"{head}\";\n"));
        }
    }
    output.push_str("}\n");
    output
}
#[component]
pub fn InputPage(
    topology: RwSignal<String>,
    on_render: impl Fn() + 'static + Sync + Send,
) -> impl IntoView {
    let on_select = move |key: String| {
        let dot = match key.as_str() {
            "diagv" => DIAGV_INPUT.to_string(),
            "Cycle(9)" => fmt_dot(generators::cycle(9).unwrap()),
            "Sonic(3)" => fmt_dot(generators::sonic(3).unwrap()),
            v => unreachable!("{v}"),
        };
        topology.set(dot);
    };
    view! {
        <Flex style="height: 100dvh; gap: 0;" vertical=true>
            <Textarea
                class="fill-vertical"
                placeholder="Paste or write a graph in the dot format"
                rules=vec![TextareaRule::required(true.into())]
                value=topology
            />
            <Flex justify=FlexJustify::End style="padding: 1em">
                <Menu on_select>
                    <MenuTrigger slot>
                        <Button>"Use an example"</Button>
                    </MenuTrigger>
                    <MenuItem value="diagv">"diagv"</MenuItem>
                    <MenuItem value="Cycle(9)">"Cycle(9)"</MenuItem>
                    <MenuItem value="Sonic(3)">"Sonic(3)"</MenuItem>
                </Menu>
                <Button
                    appearance=ButtonAppearance::Primary
                    on_click=move |_| on_render()
                    icon=icondata_mdi::MdiChevronRight
                >
                    Render
                </Button>
            </Flex>
        </Flex>
    }
}
