use regex::Regex;

use crate::types::FileType;
use std::fs::File;
use std::io::Read;

pub fn run() -> Result<(), String> {
    let Some(second_arg) = std::env::args().nth(1) else {
		return Err(format!("Usage: {} [string]", file!()))
	};
    let Ok(mut file_handle) = File::open(&second_arg) else {
		return Err(format!("Error opening file: {}", second_arg))
	};

    let Some(file_type) = guess_file_type(&mut file_handle) else {
		return Err(format!("Error guessing file type: {}", second_arg))
	};
    println!("File type: {}", file_type);

    Ok(())
}

fn guess_file_type(file: &mut File) -> Option<FileType> {
    let mut bytes = [0u8; 256];
    let Ok(_) = file.read_exact(&mut bytes) else {
		return None;
	};
    // Windows Bom Bytes 라고 해서, \xef\xbb\xbf 가 붙어 있는 경우가 있음. 대응해야 함.
    let mut start_ind = 0;
    let bom_bytes = b"\xef\xbb\xbf";
    if bytes.starts_with(bom_bytes) {
        start_ind = 3;
    }

    let head = &bytes[start_ind..(start_ind + 4)];
    match head {
        b"OTTO" => Some(FileType::OTF),
        b"ttcf" => Some(FileType::TTC),
        b"true" => Some(FileType::TTF),
        b"\x00\x01\x00\x00" => Some(FileType::TTF),
        b"wOFF" => Some(FileType::WOFF),
        b"wOF2" => Some(FileType::WOFF2),
        b"<?xm" => {
            // TODO: More Elegant Error handling
            let str =
                std::str::from_utf8(&bytes[start_ind..]).expect("Error converting bytes to string");
            let re = Regex::new(r#"sfntVersion=['"]OTTO["']"#).expect("Regex Error");
            if re.is_match(str) {
                Some(FileType::OTX)
            } else {
                Some(FileType::TTX)
            }
        }
        _ => None,
    }
}
