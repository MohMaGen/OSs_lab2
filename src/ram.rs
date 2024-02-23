use std::time::Duration;

use crate::{system::use_system, utils::get_ram_data};
use dioxus::prelude::*;

#[component]
pub fn Ram(
    cx: Scope,
) -> Element {
    let system = use_system(cx, Duration::from_millis(500));


    let (memory_total, memory_used, swap_total, swap_used) = get_ram_data(system.get());


    render! {
        div {
            class: "memory",
            div {
                class: "block ram",
                h2 { "RAM" }
                div {
                    class: "items",
                    div {
                        class: "value",
                        "{memory_used}"
                    }
                    p { "/" }
                    div {
                        class: "value",
                        "{memory_total}"
                    }
                }
            }
            div {
                class: "block swap",
                h2 { "SWAP" }
                div {
                    class: "items",
                    div {
                        class: "value",
                        "{swap_used}"
                    }
                    p { "/" }
                    div {
                        class: "value",
                        "{swap_total}"
                    }
                }
            }
        }
    }
}
