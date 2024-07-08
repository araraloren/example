// Run with:
// ```bash
// cargo run --bin client --features desktop
// ```

use axum_server::*;

fn main() {
    dioxus::prelude::launch(app)
}
