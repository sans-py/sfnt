use std::{io::Write, collections::HashMap};

use crate::tt_lib::tt_font::TTFont;



pub struct XMLReader {
    file: std::fs::File,
    ttfont: TTFont,
    progress: Option<String>,
    quiet: Option<String>,
    root : Option<String>,
    stack_size : usize,
    content_only: bool,
    content_stack : Vec<String>,
}  

impl XMLReader {
    pub fn new(
        file_or_path: &str,
        ttfont : TTFont,
        //progress : Option<Progress>, ??
        //quiet : Option<String>,  ??
        //content_only : bool, ??
    ) -> std::io::Result<Self> {
        let mut file: std::fs::File = std::fs::File::open(file_or_path)?;
        
        let mut reader = Self {
            file,
            ttfont,
            progress: None,
            quiet: None,
            root: None, 
            content_stack: Vec::new(),  
            stack_size: 0,   
            content_only: false,
        };
        
        Ok(reader)
    }

    pub fn read(&mut self, rootless: bool) {
        if rootless {
            self.stack_size += 1;
        }

        if self.progress.is_some() {
            // self.file.seek(0, 2)
            // fileSize = self.file.tell()
            // self.progress.set(0, fileSize // 100 or 1)
            // self.file.seek(0)
            // self._parseFile(self.file)}
        }

        if rootless {
            self.stack_size -= 1;
        }
    }

    pub fn close(&mut self) {
        self.file.flush(); 
    }

    fn _parse_file(&mut self, file: std::fs::File) {
        // xml parsing

        // from xml.parsers.expat import ParserCreate
        // parser = ParserCreate()
        // parser.StartElementHandler = self._startElementHandler //function
        // parser.EndElementHandler = self._endElementHandler //function
        // parser.CharacterDataHandler = self._characterDataHandler // function

        // pos = 0
        // while True:
        //     chunk = file.read(BUFSIZE)
        //     if not chunk:
        //         parser.Parse(chunk, 1)
        //         break
        //     pos = pos + len(chunk)
        //     if self.progress:
        //         self.progress.set(pos // 100)
        //     parser.Parse(chunk, 0)
    }

    //구현 필요
    fn _startElementHandler(&mut self, name: &str, attrs: &HashMap<&str, &str>) {
        if self.stack_size == 1 && self.content_only {
            self.content_stack.push("".to_string());    
            self.stack_size = 2
        }

        self.stack_size += 1;   
        let subFile = attrs.get("src");
        if subFile.is_some() {
            //구현 필요
        }

        //구현 필요
    }

    //구현 필요
    fn _endElementHandler(&mut self, name: &str) {
        self.stack_size -= 1;
        //del self.contnet_stack[-1]

        if self.content_only == false {
            if self.stack_size == 1 {
                self.root = None;
            }
            else if self.stack_size == 2 {
                //name, attrs, content = self.root
                //self.currentTable.fromXML(name, attrs, content, self.ttFont)
                //self.root = None
            }
        }
    }   

    fn _characterDataHandler(&mut self, data: &str) {
        //구현 필요
    }
}