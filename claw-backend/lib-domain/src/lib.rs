pub mod datastore;
pub mod dto;
pub mod service;

pub mod claw_proto {
    tonic::include_proto!("claw_proto");

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("claw_descriptor");
}
