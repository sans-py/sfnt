use crate::misc::sstruct::{
    get_sfnt_directory_entry_size, get_sfnt_directory_size, unpack_sfnt_directory_struct,
};
use std::io::{Error, ErrorKind, Read, Seek};

#[derive(Copy, Clone)]
enum DirectoryEntryType {
    Sfnt,
    Woff,
}

#[derive(Debug)]
struct SfntDirectoryEntry {
    tag: [u8; 4],
    check_sum: u32,
    offset: u32,
    length: u32,
}
pub struct SFNTReader {
    directory_entry_type: DirectoryEntryType,
}

impl SFNTReader {
    pub fn new(mut file: std::fs::File) -> std::io::Result<Self> {
        file.seek(std::io::SeekFrom::Start(0))?;
        let mut sfnt_version = [0u8; 4];
        file.read_exact(&mut sfnt_version)?;
        // can be b"wOFF", b"ttcf", or b"OTTO"
        file.seek(std::io::SeekFrom::Start(0))?;

        let mut directory_entry_type = DirectoryEntryType::Sfnt;

        match &sfnt_version {
            b"ttcf" => {
                unimplemented!("SFNT Reader for True Type Collection not implemented")
            }
            b"wOFF" => {
                directory_entry_type = DirectoryEntryType::Woff;
                unimplemented!("SFNT Reader for WOFF Webfont not implemented")
            }
            _ => {
                let sfnt_directory_size = get_sfnt_directory_size();
                let mut buf = vec![0u8; sfnt_directory_size];
                let Ok(_) = file.read_exact(&mut buf) else {
                    return Err(Error::new(ErrorKind::Other, "Not a TrueType or OpenType font (not enough data)"));
                };

                let sfnt_directory = unpack_sfnt_directory_struct(&mut buf)?;

                println!("{:?}", sfnt_directory);
                let (sfnt_version, num_tables, search_range, entry_selector, range_shift) =
                    sfnt_directory;
                // open type or true type
                println!("num_tables: {:?}", num_tables);
                for _ in 0..num_tables {
                    let table = Self::get_sfnt_directory(directory_entry_type, &mut file);
                    println!("{:?}", table);
                }
            }
        }

        println!("SFNTReader.new(input_path: &str) not yet implemented");
        Ok(Self {
            directory_entry_type,
        })
    }

    fn get_sfnt_directory(
        directory_entry_type: DirectoryEntryType,
        file_handle: &mut std::fs::File,
    ) -> std::io::Result<SfntDirectoryEntry> {
        match directory_entry_type {
            DirectoryEntryType::Sfnt => {
                println!("");

                let sfnt_directory_struct = structure!(">4sIII");
                let size = get_sfnt_directory_entry_size();
                let mut buf = vec![0u8; size];

                file_handle.read_exact(&mut buf)?;

                // entry: (Vec<u8>, u32, u32, u32)
                let entry = sfnt_directory_struct.unpack(&buf)?;

                let tag = std::str::from_utf8(&entry.0).unwrap().to_string();

                println!("tag: {}", tag);
                Ok(SfntDirectoryEntry {
                    tag: [entry.0[0], entry.0[1], entry.0[2], entry.0[3]],
                    check_sum: entry.1,
                    offset: entry.2,
                    length: entry.3,
                })
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
