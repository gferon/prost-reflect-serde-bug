use std::{io::Result, path::PathBuf};

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src/items.proto");
    // let prost_builder =
    //     prost_build::Config::new().type_attribute(".", "#[derive(Serialize, Deserialize)]");

    let file_descriptor_set_path = std::env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join("file_descriptor_set.bin");

    prost_build::Config::new()
        .file_descriptor_set_path(&file_descriptor_set_path)
        .compile_protos(&["src/items.proto"], &["src/"])?;

    let buf = std::fs::read(&file_descriptor_set_path).unwrap();
    let descriptor =
        prost_reflect::DescriptorPool::decode(buf.as_ref()).expect("Invalid file descriptor");

    let mut builder = prost_build::Config::new();
    for message in descriptor.all_messages() {
        let full_name = message.full_name();
        builder
            .type_attribute(full_name, "#[derive(::prost_reflect::ReflectMessage)]")
            .type_attribute(
                full_name,
                format!(
                    r#"#[prost_reflect(descriptor_pool = "get_descriptor_pool()", message_name = "{full_name}")]"#
                ),
            );
    }
    builder.type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]");

    builder.compile_protos(&["src/items.proto"], &["src/"])?;

    Ok(())
}
