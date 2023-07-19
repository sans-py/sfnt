use regex::Regex;

const fn fixed_point_mappings(point: u8) -> Result<&'static str, ()> {
    match point {
        8 => Ok("b"),
        16 => Ok("h"),
        32 => Ok("l"),
        _ => Err(()),
    }
}
const SFNT_DIRECTORY_FORMAT: &str = "
    > # big endian
    sfnt_version: 4s
    num_tables: H
    search_range: H
    entry_selector: H
    range_shift: H
";

const SFNT_DIRECTORY_ENTRY_FORMAT: &str = "
		> # big endian
		tag:            4s
		checkSum:       L
		offset:         L
		length:         L
";

pub fn get_sfnt_directory_size() -> usize {
    calcsize(SFNT_DIRECTORY_FORMAT)
}
pub fn get_sfnt_directory_entry_size() -> usize {
    calcsize(SFNT_DIRECTORY_ENTRY_FORMAT)
}

fn get_element_regex() -> Regex {
    let words: [&str; 8] = [
        r"\s*",                      /* whitespace */
        r"([A-Za-z_][A-Za-z_0-9]*)", /* name (python identifier) */
        r"\s*:\s*",                  /* whitespace : whitespace */
        r"([xcbB?hHiIlLqQfd]|",      /*  # formatchar... */
        r"[0-9]+[ps]|",              /*  ...formatchar... */
        r"([0-9]+)\.([0-9]+)(F))",   /* ...formatchar */
        r"\s*",                      /* whitespace */
        r"(#.*)?$",                  /* [comment] + end of string */
    ];
    let re: String = words.join("");
    let re = Regex::new(&re).unwrap();
    re
}

fn get_extra_regex() -> Regex {
    Regex::new(r"^\s*([x@=<>!])\s*(#.*)?$").unwrap()
}

fn get_empty_regex() -> Regex {
    Regex::new(r"^\s*(#.*)?$").unwrap()
}

pub fn calcsize(format: &str) -> usize {
    let format_string = getformat(format);
    0
}
fn getformat(fmt: &str) -> Result<String, String> {
    let delimiter = Regex::new("[\n;]").unwrap();
    let lines = delimiter.split(fmt);
    let mut format_string = String::from("");
    let empty_regex = get_empty_regex();
    let extra_regex = get_extra_regex();
    let element_regex = get_element_regex();

    for line in lines {
        if empty_regex.is_match(line) {
            continue;
        }

        match extra_regex.captures(line) {
            None => (),
            Some(caps) => match caps.get(1) {
                Some(format_char) => {
                    if format_string.len() > 0 {
                        if format_char.as_str() == "x" {
                            format_string.push_str("x");
                            continue;
                        } else {
                            return Err(String::from("a special fmt char must be first"));
                        }
                    } else {
                        format_string.push_str(format_char.as_str());
                        continue;
                    }
                }
                None => (),
            },
        }

        let Some(matches) = element_regex.captures(line) else {
            return Err(format!("syntax error in fmt: {}", line));
        };
        // let name = matches.get(0).unwrap().as_str();
        let mut format_char = matches.get(2).unwrap().as_str();

        // TODO: keep_pad_byte
        if let Some(c) = matches.get(3) {
            let before: u8 = c.as_str().parse().unwrap();
            let after: u8 = matches.get(4).unwrap().as_str().parse().unwrap();
            let bits = before + after;
            match fixed_point_mappings(bits) {
                Ok(c) => format_char = c,
                Err(_) => return Err(String::from("fixed point must be 8, 16 or 32 bits long")),
            }
        }
        format_string.push_str(format_char);
    }
    Ok(format_string)
}

#[cfg(test)]
mod sstruct_tests {

    use super::*;

    #[test]
    fn test_get_sfnt_directory_format_string() {
        let format_string = getformat(SFNT_DIRECTORY_FORMAT);
        assert_eq!(format_string, Ok(String::from(">4sHHHH")))
    }
    #[test]
    fn test_get_sfnt_directory_entry_format_string() {
        let format_string = getformat(SFNT_DIRECTORY_ENTRY_FORMAT);
        assert_eq!(format_string, Ok(String::from(">4sLLL")))
    }

    #[test]
    fn test_get_sfnt_directory_size() {
        assert_eq!(get_sfnt_directory_size(), 12)
    }
    #[test]
    fn test_get_sfnt_directory_entry_size() {
        assert_eq!(get_sfnt_directory_entry_size(), 16)
    }
}
