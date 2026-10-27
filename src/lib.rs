use std::fs::File;
use std::io::Read;
use anyhow::{anyhow, Result};
use bytes::Bytes;
use prost_reflect::prost::Message;
use prost_reflect::prost_types::{FileDescriptorProto, FileDescriptorSet};
use regex::Regex;
use crate::messageV1::parse_messages;
use crate::serviceV1::parse_service;

#[allow(non_snake_case)]
mod serviceV1;
#[allow(non_snake_case)]
pub mod messageV1;

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

#[test]
fn v1_parse_vad_proto() {

    let mut input_file = File::open("./proto/vad.proto").unwrap();
    let mut input_content = String::new();
    input_file.read_to_string(&mut input_content).unwrap();

    let (syntax, package) = parse_proto_metadata(&input_content).unwrap();
    let service = parse_service(&input_content, &package).unwrap();

    let messages = parse_messages(&input_content, 0, &format!(".{}", package)).unwrap();

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

    let handmade_descriptor_set = FileDescriptorSet {
        file: vec!(file_descriptor),
    };

    let mut example_file = File::open("./proto/vad.descriptor.bin").unwrap();
    let mut bytes = vec!();
    example_file.read_to_end(&mut bytes).unwrap();

    let proper_descriptor_set = FileDescriptorSet::decode(Bytes::from(bytes)).unwrap();

    assert_eq!(handmade_descriptor_set, proper_descriptor_set);
}
