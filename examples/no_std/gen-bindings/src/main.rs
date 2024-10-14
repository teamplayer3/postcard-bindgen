use postcard_bindgen::{
    generate_bindings,
    javascript::{build_package, GenerationSettings},
    PackageInfo,
};

fn main() {
    build_package(
        std::env::current_dir().unwrap().as_path(),
        PackageInfo {
            name: "no_std_bindings".into(),
            version: "0.1.0".try_into().unwrap(),
        },
        GenerationSettings::enable_all(),
        generate_bindings!(no_std::Protocol, no_std::Packet, no_std::A1Meta),
    )
    .unwrap();
}
