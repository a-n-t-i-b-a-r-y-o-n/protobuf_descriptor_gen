use anyhow::{anyhow, Result};
use prost_reflect::prost_types::{MethodDescriptorProto, ServiceDescriptorProto};
use regex::Regex;

/// Regex to capture a service
const REGEX_SERVICE: &str = r"service (?P<service_name>.+?) \{(?P<service_content>(\n|.)+?)\}";

/// Regex to capture an RPC
const REGEX_RPC: &str = r"rpc (?P<rpc_name>.+?) \((?P<request_stream>stream)?\s?(?P<request>.+?)\) returns \((?P<response_stream>stream)?\s?(?P<response>.+?)\)";


fn parse_methods(service_content: &str, prefix: &str) -> Result<Vec<MethodDescriptorProto>> {
    let regex_rpc = Regex::new(REGEX_RPC)?;

    let mut methods = vec!();

    for rpc in regex_rpc.captures_iter(service_content) {

        let method_name = match rpc.name("rpc_name") {
            Some(m) => m.as_str(),
            None => return Err(anyhow!("Failed to parse method name")),
        };

        let client_streaming = Some(rpc.name("request_stream").is_some());

        let input_type = rpc.name("request").map(|m| format!("{}.{}", prefix, m.as_str()));

        let server_streaming = Some(rpc.name("response_stream").is_some());

        let output_type = rpc.name("response").map(|m| format!("{}.{}", prefix, m.as_str()));

        let method_descriptor = MethodDescriptorProto {
            name: Some(String::from(method_name)),
            input_type,
            output_type,
            options: None,
            client_streaming,
            server_streaming,
        };

        methods.push(method_descriptor);
    }

    Ok(methods)
}


pub(crate) fn parse_service(input_content: &str, package: &str) -> Result<ServiceDescriptorProto> {
    let regex_service = Regex::new(REGEX_SERVICE)?;

    let service = match regex_service.captures(input_content) {
        Some(captures) => captures,
        None => return Err(anyhow!("Failed to parse protobuf service")),
    };

    let service_name = match service.name("service_name") {
        Some(m) => m.as_str(),
        None => return Err(anyhow!("Failed to extract protobuf service name")),
    };

    let service_content = match service.name("service_content") {
        Some(m) => m.as_str(),
        None => return Err(anyhow!("Failed to extract protobuf service content")),
    };

    let methods = parse_methods(service_content, &format!(".{}", package))?;

    let service_descriptor = ServiceDescriptorProto {
        name: Some(String::from(service_name)),
        method: methods,
        options: None,
    };

    Ok(service_descriptor)
}