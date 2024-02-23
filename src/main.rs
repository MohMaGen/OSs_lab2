
mod ram;
mod utils;
mod system;
mod disks;


use dioxus::prelude::*;


fn main() {
    let head_line = r#"<link rel="stylesheet" href="styles/main.css" />"#.to_string();
    let cfg = dioxus_desktop::Config::new().with_custom_head(head_line);

    dioxus_desktop::launch_cfg(app, cfg);
}


fn app(cx: Scope) -> Element {
    render! {
        ram::Ram {}
        disks::Disks {}
    }
}
