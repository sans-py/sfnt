use crate::misc::sstruct::get_sfnt_directory_size;
use std::io::{Error, ErrorKind, Read, Seek};
pub struct SFNTReader {}

impl SFNTReader {
    pub fn new(mut file: std::fs::File) -> std::io::Result<Self> {
        file.seek(std::io::SeekFrom::Start(0))?;
        let mut sfnt_version = [0u8; 4];
        file.read_exact(&mut sfnt_version)?;
        // can be b"wOFF", b"ttcf", or b"OTTO"
        file.seek(std::io::SeekFrom::Start(0))?;

        match &sfnt_version {
            b"ttcf" => {
                unimplemented!("SFNT Reader for True Type Collection not implemented")
            }
            b"wOFF" => {
                unimplemented!("SFNT Reader for WOFF Webfont not implemented")
            }
            _ => {
                let sfnt_directory_size = get_sfnt_directory_size();
                let mut buf = vec![0u8; sfnt_directory_size];
                let Ok(_) = file.read_exact(&mut buf) else {
                    return Err(Error::new(ErrorKind::Other, "Not a TrueType or OpenType font (not enough data)"));
                };

                let sfnt_directory_struct = structure!(">4sHHHH");
                let sfnt_directory = sfnt_directory_struct.unpack(&buf)?;

                println!("{:?}", sfnt_directory);
                let (sfnt_version, num_tables, search_range, entry_selector, range_shift) =
                    sfnt_directory;
                // open type or true type
            }
        }

        println!("SFNTReader.new(input_path: &str) not yet implemented");
        Ok(Self {})
    }
}
