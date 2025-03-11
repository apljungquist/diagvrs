mod input_page;
mod output_page;

use crate::components::input_page::InputPage;
use crate::components::output_page::OutputPage;
use leptos::prelude::RwSignal;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use thaw::*;

#[derive(Clone, Copy, Debug)]
enum Phase {
    Topology,
    Order,
}
#[component]
pub fn App() -> impl IntoView {
    let topology = RwSignal::new(include_str!("../../deps.dot").to_string());
    let (phase, set_phase) = signal(Phase::Order);

    view! {
        <ConfigProvider>
            {move || match phase.get() {
                Phase::Topology => {
                    view! {
                        <InputPage
                            topology=topology
                            on_render=move || {
                                set_phase.set(Phase::Order);
                            }
                        />
                    }
                        .into_any()
                }
                Phase::Order => {
                    view! {
                        <OutputPage
                            topology=topology.get()
                            on_edit=move || set_phase.set(Phase::Topology)
                        />
                    }
                        .into_any()
                }
            }}
        </ConfigProvider>
    }
}
