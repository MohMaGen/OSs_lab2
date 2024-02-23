use async_std::task::sleep;
use dioxus::prelude::*;
use std::time::Duration;
use sysinfo::System;




pub fn use_system(cx: Scope, tick: Duration) -> UseState<System> {
    let system = use_state(cx, System::new_all);

    use_future!(cx, || {
        let system = system.clone();
        async move {
            loop {
                system.set(System::new_all());
                sleep(tick).await;
            }
        }
    });

    system.clone()
}


