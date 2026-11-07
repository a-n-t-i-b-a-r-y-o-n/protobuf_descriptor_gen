mod parser;
mod file_descriptor;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;
    use std::str::FromStr;
    use bytes::Bytes;
    use prost_reflect::prost::Message;
    use prost_reflect::prost_types::FileDescriptorSet;

    use crate::file_descriptor::generate_file_descriptor_set;

    #[test]
    fn parse_vad_proto() {

        let input_file = PathBuf::from_str("./proto/vad.proto").unwrap();

        let handmade_descriptor_set = generate_file_descriptor_set(&input_file).unwrap();

        let mut example_file = File::open("./proto/vad.descriptor.bin").unwrap();
        let mut bytes = vec!();
        example_file.read_to_end(&mut bytes).unwrap();

        let proper_descriptor_set = FileDescriptorSet::decode(Bytes::from(bytes)).unwrap();

        assert_eq!(handmade_descriptor_set, proper_descriptor_set);
    }

    #[test]
    fn parse_speaker_proto() {

        let input_file = PathBuf::from_str("./proto/speaker.proto").unwrap();

        let handmade_descriptor_set = generate_file_descriptor_set(&input_file).unwrap();

        let mut example_file = File::open("./proto/speaker.descriptor.bin").unwrap();
        let mut bytes = vec!();
        example_file.read_to_end(&mut bytes).unwrap();

        let proper_descriptor_set = FileDescriptorSet::decode(Bytes::from(bytes)).unwrap();

        assert_eq!(handmade_descriptor_set, proper_descriptor_set);
    }

    #[test]
    fn parse_asr_proto() {

        let input_file = PathBuf::from_str("./proto/asr.proto").unwrap();

        let handmade_descriptor_set = generate_file_descriptor_set(&input_file).unwrap();

        let mut example_file = File::open("./proto/asr.descriptor.bin").unwrap();
        let mut bytes = vec!();
        example_file.read_to_end(&mut bytes).unwrap();

        let proper_descriptor_set = FileDescriptorSet::decode(Bytes::from(bytes)).unwrap();

        assert_eq!(handmade_descriptor_set, proper_descriptor_set);
    }

    #[test]
    fn parse_tts_proto() {

        let input_file = PathBuf::from_str("./proto/tts.proto").unwrap();

        let handmade_descriptor_set = generate_file_descriptor_set(&input_file).unwrap();

        let mut example_file = File::open("./proto/tts.descriptor.bin").unwrap();
        let mut bytes = vec!();
        example_file.read_to_end(&mut bytes).unwrap();

        let proper_descriptor_set = FileDescriptorSet::decode(Bytes::from(bytes)).unwrap();

        assert_eq!(handmade_descriptor_set, proper_descriptor_set);
    }

}
