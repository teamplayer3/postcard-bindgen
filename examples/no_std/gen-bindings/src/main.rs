use postcard_bindgen::{build_npm_package, generate_bindings, PackageInfo};

fn main() {
    build_npm_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "test".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        generate_bindings!(no_std::Protocol, no_std::Packet, no_std::A1Meta),
    )
    .unwrap();
}
