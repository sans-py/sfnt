use std::io::{Read, Seek};
pub struct SFNTReader {}

impl SFNTReader {
    pub fn new(mut file: std::fs::File) -> std::io::Result<Self> {
        file.seek(std::io::SeekFrom::Start(0))?;
        let mut bytes = [0u8; 4];
        file.read_exact(&mut bytes)?;
        // can be b"wOFF", b"ttcf", or b"OTTO"
        println!("is sfnt_version b\"OTTO\": {:?}", &bytes == b"OTTO");
        file.seek(std::io::SeekFrom::Start(0))?;

        println!("SFNTReader.new(input_path: &str) not yet implemented");
        Ok(Self {})
    }
}
