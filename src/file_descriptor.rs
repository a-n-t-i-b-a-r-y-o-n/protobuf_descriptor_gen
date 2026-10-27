use std::path::PathBuf;
use prost_reflect::prost_types::FileDescriptorSet;

/// Generate a FileDescriptorSet from a given .proto file
pub fn generate_file_descriptor_set(proto_file: &PathBuf) -> anyhow::Result<FileDescriptorSet> {

    let file_descriptor = crate::parser::parse_protobuf_file(proto_file)?;

    let file_descriptor_set = FileDescriptorSet {
        file: vec!(file_descriptor),
    };

    Ok(file_descriptor_set)
}