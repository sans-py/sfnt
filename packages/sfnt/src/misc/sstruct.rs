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
    calcsize(SFNT_DIRECTORY_FORMAT).expect("calcsiz_error")
}
pub fn get_sfnt_directory_entry_size() -> usize {
    calcsize(SFNT_DIRECTORY_ENTRY_FORMAT).expect("calcsiz_error")
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

pub fn calcsize(format: &str) -> Result<usize, String> {
    let format_string = getformat(format)?;

    let format_string: Vec<char> = format_string.chars().collect();

    let endian = format_string.get(0).expect("Format string is too short");

    let format_table = if *endian == '<' {
        &LIL_ENDIAN_TABLE
    } else {
        &BIG_ENDIAN_TABLE
    };
    let mut size = 0usize;

    let mut index = 1;
    let limit = format_string.len();

    while index < limit {
        let mut c = format_string[index];
        let mut num: usize = 1;

        if c.is_ascii_digit() {
            num = c.to_digit(10).unwrap() as usize;
            index += 1;
            while index < limit {
                c = format_string[index];
                if c.is_ascii_digit() {
                    num = num * 10 + c.to_digit(10).unwrap() as usize;
                    index += 1;
                } else {
                    break;
                }
            }
            if index == limit {
                return Err(String::from("repeat count given without format specifier"));
            }
        }

        let mut item_size: usize = 0;
        for format_def in format_table {
            if format_def.format == c {
                item_size = format_def.size;
                break;
            }
        }
        size += num * item_size;
        index += 1;
    }

    Ok(size)
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

// format, size, alignment
struct FormatDef {
    format: char,
    size: usize,
    alignment: usize,
}
#[rustfmt::skip]
const LIL_ENDIAN_TABLE: [FormatDef; 18] = [
    FormatDef { format: 'x', size: 1, alignment: 0 },
    FormatDef { format: 'b', size: 1, alignment: 0 },
    FormatDef { format: 'B', size: 1, alignment: 0 },
    FormatDef { format: 'c', size: 1, alignment: 0 },
    FormatDef { format: 's', size: 1, alignment: 0 },
    FormatDef { format: 'p', size: 1, alignment: 0 },
    FormatDef { format: 'h', size: 2, alignment: 0 },
    FormatDef { format: 'H', size: 2, alignment: 0 },
    FormatDef { format: 'i', size: 4, alignment: 0 },
    FormatDef { format: 'I', size: 4, alignment: 0 },
    FormatDef { format: 'l', size: 4, alignment: 0 },
    FormatDef { format: 'L', size: 4, alignment: 0 },
    FormatDef { format: 'q', size: 8, alignment: 0 },
    FormatDef { format: 'Q', size: 8, alignment: 0 },
    FormatDef { format: '?', size: 1, alignment: 0 },
    FormatDef { format: 'e', size: 2, alignment: 0 },
    FormatDef { format: 'f', size: 4, alignment: 0 },
    FormatDef { format: 'd', size: 8, alignment: 0 },
];

// alignment 가 조금 다른 것 같지만 여기선 안 쓰는 subset
const BIG_ENDIAN_TABLE: [FormatDef; 18] = LIL_ENDIAN_TABLE;

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
