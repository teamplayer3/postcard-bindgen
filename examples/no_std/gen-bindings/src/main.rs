use postcard_bindgen::{export_bindings, generate_bindings};

fn main() {
    export_bindings(
        std::env::current_dir()
            .unwrap()
            .join("js_export.js")
            .as_path(),
        generate_bindings!(no_std::Protocol, no_std::Packet, no_std::A1Meta),
    )
    .unwrap();
}
