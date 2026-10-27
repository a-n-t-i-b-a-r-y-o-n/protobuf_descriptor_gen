use std::str::FromStr;
use anyhow::{anyhow, Result};
use convert_case::ccase;
use prost_reflect::prost_types::{DescriptorProto, FieldDescriptorProto, OneofDescriptorProto};
use prost_reflect::prost_types::field_descriptor_proto::{Label, Type};
use regex::Regex;

/// Regex to capture a message
const REGEX_MESSAGE: &str = r"(?m)^message (?P<message_name_nofields>\S+?) \{\s*\}$|^message (?P<message_name_fields>\S+?) \{(?P<message_content>(\n|.)+?)^\}$";

const REGEX_FIELDS: &str = r"(?m)^  (?P<field_type>\S+?) (?P<field_name>\S+?) = (?P<field_number>\d+);";

const REGEX_ONEOF: &str = r"(?m)^  oneof (?P<oneof_name>.+?) \{(?P<oneof_content>(\n|.)+?)\}";

/// Remove nesting for easier recursive regex
fn remove_nesting(input_content: &str, nesting: usize) -> String {
    if nesting > 0 {
        let mut unnested = String::new();
        for _level in 0..nesting {

            for line in input_content.lines() {
                unnested = format!("{}{}\n", unnested, line.strip_prefix("  ").unwrap_or_else(|| line));
            }
        }

        return unnested
    }

    String::from(input_content)
}

fn map_field_type(type_name: &str) -> Type {
    match type_name {
        "bool"      => Type::Bool,
        "bytes"     => Type::Bytes,
        "double"    => Type::Double,
        "enum"      => Type::Enum,
        "float"     => Type::Float,
        "fixed32"   => Type::Fixed32,
        "fixed64"   => Type::Fixed64,
        "group"     => Type::Group,
        "int32"     => Type::Int32,
        "int64"     => Type::Int64,
        "sfixed32"  => Type::Sfixed32,
        "sfixed64"  => Type::Sfixed64,
        "sint32"    => Type::Sint32,
        "sint64"    => Type::Sint64,
        "string"    => Type::String,
        "uint32"    => Type::Uint32,
        "uint64"    => Type::Uint64,
        _           => Type::Message,
    }
}

fn parse_fields(input_content: &str, nesting: usize, prefix: &str) -> Result<Vec<FieldDescriptorProto>> {
    let mut fields = vec!();

    let regex_fields = Regex::new(REGEX_FIELDS)?;

    let input_content = remove_nesting(input_content, nesting);

    for field in regex_fields.captures_iter(&input_content) {

        let field_name = match field.name("field_name") {
            Some(m) => String::from(m.as_str()),
            None => continue,
        };

        let field_number = match field.name("field_number") {
            Some(m) => i32::from_str(m.as_str())?,
            None => continue,
        };

        let field_type = match field.name("field_type") {
            Some(m) => String::from(m.as_str()),
            None => continue,
        };

        let mapped_field_type = map_field_type(&field_type);
        let field_type_name = if mapped_field_type == Type::Message {
            Some(format!("{}.{}", prefix, field_type))
        } else {
            None
        };

        let descriptor = FieldDescriptorProto {
            name: Some(field_name.clone()),
            number: Some(field_number),
            label: Some(Label::Optional.into()),        // TODO: Support labels
            r#type: Some(mapped_field_type.into()),
            type_name: field_type_name,
            extendee: None,
            default_value: None,
            oneof_index: Some(0),                       // TODO: Support multiple oneof declarations
            json_name: Some(ccase!(camel, field_name)),
            options: None,
            proto3_optional: None,
        };

        fields.push(descriptor);
    }

    Ok(fields)
}

fn parse_oneof(input_content: &str, nesting: usize, prefix: &str) -> Result<Vec<(OneofDescriptorProto, Vec<FieldDescriptorProto>)>> {
    let regex_oneof = Regex::new(REGEX_ONEOF)?;

    let mut oneofs = vec!();

    for oneof in regex_oneof.captures_iter(input_content) {
        let oneof_name = match oneof.name("oneof_name") {
            Some(m) => String::from(m.as_str()),
            None => return Err(anyhow!("Failed to extract oneof name")),
        };

        let oneof_content = match oneof.name("oneof_content") {
            Some(m) => String::from(m.as_str()),
            None => return Err(anyhow!("Failed to extract oneof content"))
        };

        let descriptor = OneofDescriptorProto {
            name: Some(oneof_name),
            options: None,
        };

        let options = parse_fields(&oneof_content, nesting + 1, prefix)?;

        oneofs.push((descriptor, options));
    }

    Ok(oneofs)
}

pub(crate) fn parse_messages(input_content: &str, nesting: usize, prefix: &str) -> Result<Vec<DescriptorProto>> {
    let regex_message = Regex::new(REGEX_MESSAGE)?;

    let mut messages = vec!();

    let input_content = remove_nesting(input_content, nesting);

    for message in regex_message.captures_iter(&input_content) {

        if let Some(m) = message.name("message_name_nofields") {
            let descriptor = DescriptorProto {
                name: Some(String::from(m.as_str())),
                field: vec![],
                extension: vec![],
                nested_type: vec![],
                enum_type: vec![],
                extension_range: vec![],
                oneof_decl: vec![],
                options: None,
                reserved_range: vec![],
                reserved_name: vec![],
            };

            messages.push(descriptor);
        }
        else if let Some(m) = message.name("message_name_fields") {

            let name = String::from(m.as_str());
            let content = match message.name("message_content") {
                Some(m) => m.as_str(),
                None => return Err(anyhow!("Failed to extract message content")),
            };

            // Parse fields
            let mut fields = parse_fields(&content, nesting, &format!("{}.{}", prefix, name))?;

            // Parse OneOf
            let oneofs_and_fields = parse_oneof(content, nesting, &format!("{}.{}", prefix, name))?;

            // Extract oneofs
            let oneof_descriptors = oneofs_and_fields.iter()
                .map(|(a, _)| a.clone())
                .collect::<Vec<OneofDescriptorProto>>();

            // Extract fields in oneofs
            let oneof_fields = oneofs_and_fields.iter()
                .map(|(_, b)| b.clone())
                .flatten();

            fields.extend(oneof_fields);

            // Parse nested message types
            let nested_types = parse_messages(&content, nesting + 1, prefix)?;

            let descriptor = DescriptorProto {
                name: Some(name),
                field: fields,
                extension: vec![],
                nested_type: nested_types,
                enum_type: vec![],
                extension_range: vec![],
                oneof_decl: oneof_descriptors,
                options: None,
                reserved_range: vec![],
                reserved_name: vec![],
            };

            messages.push(descriptor);
        }

    }

    Ok(messages)
}
