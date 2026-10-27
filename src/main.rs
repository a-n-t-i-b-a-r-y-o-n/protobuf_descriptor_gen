use std::fs::File;
use std::io::Read;
use anyhow::{anyhow, Result};
use prost_reflect::DescriptorPool;
use prost_reflect::prost_types::{FileDescriptorProto, FileDescriptorSet};
use regex::Regex;
use crate::message::parse_messages;
use crate::service::parse_service;

mod service;
pub mod message;

/// Regex to capture protobuf metadata
const REGEX_PROTOBUF: &str = r#"syntax = "(?P<syntax>.+?)";
package (?P<package>.+?);"#;

fn parse_proto_metadata(input_content: &str) -> Result<(String, String)> {

    let regex_protobuf = Regex::new(REGEX_PROTOBUF)?;

    let proto_metadata = match regex_protobuf.captures(&input_content) {
        Some(captures) => captures,
        None => return Err(anyhow!("Failed to parse protobuf file")),
    };

    let syntax = match proto_metadata.name("syntax") {
        Some(m) => m.as_str(),
        None => return Err(anyhow!("Failed to extract protobuf syntax")),
    };
    let package = match proto_metadata.name("package") {
        Some(m) => m.as_str(),
        None => return Err(anyhow!("Failed to extract protobuf package")),
    };

    Ok((String::from(syntax), String::from(package)))
}

fn main() -> Result<()> {

    let mut input_file = File::open("./proto/vad.proto")?;
    let mut input_content = String::new();
    input_file.read_to_string(&mut input_content)?;

    let (syntax, package) = parse_proto_metadata(&input_content)?;
    let service = parse_service(&input_content, &package)?;

    let messages = parse_messages(&input_content, 0, &format!(".{}", package))?;

    let file_descriptor = FileDescriptorProto {
        name: Some(String::from("vad.proto")),
        package: Some(package),
        dependency: vec![],
        public_dependency: vec![],
        weak_dependency: vec![],
        message_type: messages,
        enum_type: vec![],
        service: vec!(service),
        extension: vec![],
        options: None,
        source_code_info: None,
        syntax: Some(syntax),
    };

    let descriptor_set = FileDescriptorSet {
        file: vec!(file_descriptor),
    };

    // TODO: Use the DescriptorPool
    let _descriptor_pool = DescriptorPool::from_file_descriptor_set(descriptor_set)?;

    Ok(())
}
