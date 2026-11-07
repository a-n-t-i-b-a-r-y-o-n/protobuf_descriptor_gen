# Protobuf Descriptor Generator

Generate Protobuf file descriptors in pure Rust - no `protoc` dependency!


## What can I do with this?

With this library, you can use runtime reflection on services, methods, messages, etc. from protobuf files without
requiring `protoc` to be installed on the target.

This allows you to use [prost-reflect](https://github.com/andrewhickman/prost-reflect) without having to generate
protobuf file descriptors beforehand.

✨ **True runtime reflection!** ✨


## Example usage

Since this crate is intended to be used with `prost-reflect`, the main entrypoints are functions for generating
`FileDescriptorSet` and `DescriptorPool` structs:

```rust
// Example demonstrating `FileDescriptorSet` generation
fn file_descriptor_set_example() {
    // Get path to an input proto file
    let input_proto: PathBuf = proto_path.canonicalize()?;

    // Generate a file descriptor set for the proto file at the given path
    let descriptor_set: FileDescriptorSet = generate_file_descriptor_set(&input_proto)?;
}

// Example demonstrating `DescriptorPool` generation
fn descriptor_pool_example() {
    // Get path to an input proto file
    let input_proto: PathBuf = proto_path.canonicalize()?;
    
    // Generate descriptors for the proto file at the given path
    let descriptor_pool: DescriptorPool = generate_descriptor_pool(&input_proto)?;
}
```

Since this crate produces the existing `DescriptorPool` and `FileDescriptorSet` structs from `prost-reflect`, you can use
the generated descriptors exactly as you would normally:
```rust
// Example demonstrating `DescriptorPool` generation and usage
fn full_descriptor_pool_example() {
    // Get path to an input proto file
    let input_proto = proto_path.canonicalize()?;

    // Generate a descriptor pool for the proto file at the given path
    let descriptor_pool = generate_descriptor_pool(&input_proto)?;

    // Iterate the file descriptors in the pool
    for file in descriptor_pool.files() {

        // Iterate the list of services in the file
        for service in file.services() {

            // Print the service name
            println!("Service: {}" service.name());

            // Iterate the methods in the service
            for method in service.methods() {

                // Print some method information
                println!("Method {} accepts {} and returns {}",
                         method.name(),
                         method.input().full_name(),
                         method.output().full_name()
                );
            }
        }
    }
}
```

## Why, though?

There are multiple solutions today that either require `protoc` to be installed, scuttle precompiled/vendored versions
of `protoc`, or shell out to an external `protoc` installation:
- [protobuf-native](https://github.com/MaterializeInc/rust-protobuf-native/tree/master/protobuf-native) - high-level `libprotobuf` API
- [protobuf-sys](https://github.com/MaterializeInc/rust-protobuf-native/tree/master/protobuf-sys) - generated `libprotobuf` bindings
- [protobuf-src](https://github.com/MaterializeInc/rust-protobuf-native/tree/master/protobuf-src) - vendored Protobuf source to build `libprotobuf` and `protoc`
- [protoc-bin-vendored](https://github.com/stepancheg/rust-protoc-bin-vendored) - `protoc` packed in the Rust crate

The `prost` crate itself (which ships [prost_build](https://docs.rs/prost-build/latest/prost_build/)) also
[requires protoc to be installed](https://crates.io/crates/prost#protoc) in order to compile protobuf files.

In order to use runtime reflection, your code is expected to invoke the installed `protoc` binary like so:
```shell
protoc --include_imports \
    --descriptor_set_out="<output descriptor name>.bin" \
    --proto_path=<some directory where the .proto is> \
    <the .proto to use>
```

Personally, I do have `protoc` installed on my build machine, so the `prost_build` scenario works for build-time
generated reflection.

This crate is meant to solve the protobuf generation problem for _runtime_ reflection.


## How did you do make this?

Without AI, actually!

This project contains just enough code to work with the Protobuf files from my gRPC plugin project, Carcinisation.  
They're reasonably complex (nested messages, oneof declarations, streaming) but far from exhaustive.

I wrote the current parser in a sort of "clean room" style by comparing my existing protobuf file inputs to the
generated file descriptor outputs.

Simple as that!


## Sources

Online interactive FileDescriptorSet generator:  
https://protobuf.com/descriptors/  

Details about descriptors and compilation:  
https://buf.build/docs/reference/descriptors/  
https://buf.build/docs/reference/compilation-and-descriptors/#descriptor-production  