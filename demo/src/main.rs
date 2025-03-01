use leptos::ev::InputEvent;
use leptos::prelude::*;

const SAMPLE: &str = include_str!("../../deps.dot");
fn render(dot: &str) -> String {
    diagvrs::demo(dot, diagvrs::Order::ReverseAlphabetical)
}

#[component]
fn Input(on_render: impl Fn(&str) + 'static) -> impl IntoView {
    let (read_input, write_input) = signal(SAMPLE.to_string());

    view! {
        <div style="display: flex; flex-direction: column; height: 100%;">
            <textarea
                prop:value=read_input
                on:input:target=move |ev| write_input.set(ev.target().value())
                style="flex: 1; width: 100%; overflow: scroll; font-family: monospace;"
            ></textarea>
            <div style="padding: 0.5em; text-align: center;">
                <button on:click=move |_| on_render(&read_input.get())>Render</button>
            </div>
        </div>
    }
}

#[component]
fn Output(topology: String, on_edit: impl Fn() + 'static) -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; height: 100%;">
            <code style="display:block; flex: 1; white-space: pre; overflow:scroll;">
                {render(&topology)}
            </code>
            <div style="padding: 0.5em; text-align: center;">
                <button on:click=move |_| on_edit()>Edit input</button>
            </div>
        </div>
    }
}

#[derive(Clone, Copy, Debug)]
enum Phase {
    Topology,
    Visualization,
}

#[component]
fn App() -> impl IntoView {
    let (input, set_input) = signal("diagv".to_string());
    let (phase, set_phase) = signal(Phase::Topology);

    view! {
        <style>
            "html, body { height: 100%; margin: 0; padding: 0; }
            body { display: flex; flex-direction: column; }"
        </style>

        // <Input on_render=move |s|set_input.set(s.to_string())/>
        // <Output text=input.get() on_edit=move||set_phase.set(Phase::Topology)/>

        {move || match phase.get() {
            Phase::Topology => {
                view! {
                    <Input on_render=move |s| {
                        set_input.set(s.to_string());
                        set_phase.set(Phase::Visualization);
                    } />
                }
                    .into_any()
            }
            Phase::Visualization => {
                view! {
                    <Output topology=input.get() on_edit=move || set_phase.set(Phase::Topology) />
                }
                    .into_any()
            }
        }}
    }
}

//

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    let (read_output, write_output) = signal(String::new());
    mount_to_body(move || {
        view! { <App /> }
    })
}