use std::time::Duration;

use async_std::task::sleep;
use dioxus::prelude::*;
use sysinfo::Disks;

use crate::utils::MemoryUnit;

#[component]
pub fn Disks(cx: Scope) -> Element {
    let disks = use_disks(cx, Duration::from_millis(500));

    render! {
        div {
            class: "disks",
            DisksInner {
                disks: disks.clone()
            }
        }
    }
}

struct DiskData {
    pub name: String,
    pub fs: String,
    pub total: MemoryUnit,
    pub used: MemoryUnit,
    pub free: MemoryUnit,
}

#[component]
pub fn DisksInner(cx: Scope, disks: UseState<Disks>) -> Element {
    let mut disks_vec: Vec<DiskData> = Vec::new();

    for disk in disks.get().list() {
        let (ts, us) = (disk.total_space(), disk.available_space());

        disks_vec.push(DiskData {
            name: match disk.name().to_os_string().into_string() {
                Ok(v) => v,
                Err(v) => format!("{v:?}"),
            },
            fs: match disk.file_system().to_os_string().into_string() {
                Ok(v) => v,
                Err(v) => format!("{v:?}"),
            },
            total: MemoryUnit::new(ts),
            used: MemoryUnit::new(us),
            free: MemoryUnit::new(ts - us),
        });
    }

    render! {
        for disk in disks_vec {
            div {
                class: "block disk",

                div {
                    div {
                        class: "disk-info",
                        h2 {
                            class: "disk-name",
                            "{disk.name}"
                        }

                        div {
                            class: "value disk-fs",
                            "{disk.fs}"
                        }
                    }
                    div {
                        class: "disk-info",
                        p { "free space" }
                        div {
                            class: "value free",
                            "{disk.free}"
                        }
                    }
                }

                div {
                    class: "disk-space",
                    div {
                        class: "items",
                        div {
                            class: "value used",
                            "{disk.used}"
                        }
                        p { "/" }
                        div {
                            class: "value total",
                            "{disk.total}"
                        }
                    }
                }
            }
        }
    }
}

pub fn use_disks(cx: Scope, tick: Duration) -> UseState<Disks> {
    let disks = use_state(cx, Disks::new_with_refreshed_list);

    use_future!(cx, || {
        let disks = disks.clone();
        async move {
            disks.set(Disks::new_with_refreshed_list());
            sleep(tick).await;
        }
    });

    disks.clone()
}
