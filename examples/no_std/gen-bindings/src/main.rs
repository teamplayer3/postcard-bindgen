use postcard_bindgen::{generate_bindings, javascript, python, PackageInfo};

fn main() {
    javascript::build_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "js_no_std_bindings".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        javascript::GenerationSettings::enable_all(),
        generate_bindings!(no_std::Protocol, no_std::Packet, no_std::A1Meta),
    )
    .unwrap();

    python::build_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "py_no_std_bindings".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        python::GenerationSettings::enable_all(),
        generate_bindings!(no_std::Protocol, no_std::Packet, no_std::A1Meta),
    )
    .unwrap();
}
