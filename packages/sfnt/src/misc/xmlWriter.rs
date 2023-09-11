use std::{io::Write, collections::HashMap};

const INDENT: &str = "  ";

pub struct XMLWriter {
    file: std::fs::File,
    indentwhite: String,
    newlinestr: String,
    indentlevel: usize,
    stack: Vec<String>,
}

impl XMLWriter {
    pub fn new(
        file_or_path: &str,
        //indent_white: Option<&str>,
        //idle_func: Option<&str>, //is enable function param?
        //encoding: Option<&str>, only utf-8
        //newline_str: Option<&str>
    ) -> std::io::Result<Self> {
        let mut file: std::fs::File = std::fs::File::create(file_or_path)?;
        let indentwhite = INDENT.to_owned(); //indent_white.unwrap_or(INDENT).to_owned();
        //let binding = String::from("\n");
        let newlinestr = "\n"; //newline_str.unwrap_or(binding.as_str());
        let mut writer = Self {
            file, 
            indentwhite, 
            newlinestr: newlinestr.to_string(), 
            indentlevel: 0,
            stack: Vec::new(),
        };
        writer.write("<?xml version=\"1.0\" encoding=\"UTF-8\"?>", false);
        writer.newline();
        Ok(writer)
    }

    pub fn close(&mut self) {
        self.file.flush();
    }

    pub fn write(&mut self, string: &str, indent: bool) {
        self._writeraw(string, indent);
    }

    pub fn write_cdata(&mut self, string: &str){
        self._writeraw("<![CDATA[", true);
        self._writeraw(string, false);
        self._writeraw("]]>", false);
    }

    fn _writeraw(&mut self, data: &str, indent: bool) {
        if indent {
            for _ in 0..self.indentlevel {
                self.file.write(self.indentwhite.as_bytes());
            }
        }
        self.file.write_all(data.as_bytes());
    }

    pub fn newline(&mut self) {
        self.file.write(self.newlinestr.as_bytes());
    }

    fn indent(&mut self) {
        self.indentlevel = self.indentlevel + 1;
    }

    fn dedent(&mut self) {
        self.indentlevel = self.indentlevel - 1;
    }

    pub fn simpletag(&mut self, name: &str, value: &HashMap<&str, &str>) {
        self._writeraw("<", true);
        self._writeraw(name, false);

        for (key, value) in value.iter() {
            self._writeraw(" ", false);
            self._writeraw(key ,false);
            self._writeraw("=\"",false);
            self._writeraw(value,false);
            self._writeraw("\"",false);
        }
        self._writeraw("/>", false);
    }

    pub fn begintag(&mut self, name: &str, value: &HashMap<&str, &str>) {
        self._writeraw("<", true);
        self._writeraw(name,false);

        for (key, value) in value.iter() {
            self._writeraw(" ",false);
            self._writeraw(key,false);
            self._writeraw("=\"",false);
            self._writeraw(value,false);
            self._writeraw("\"",false);
        }
        self._writeraw(">",false);
        self.stack.push(name.to_string());
        self.indent();
    }

    pub fn endtag(&mut self) {
        self.dedent();
        let name = self.stack.pop().unwrap();
        self._writeraw("</", true);
        self._writeraw(name.as_str(),false);
        self._writeraw(">",false);
    }

    fn escape(&mut self, string: &str) -> String {
        string
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\r", "&#13;")
    }

    pub fn comment(&mut self, data: &str) {
        let data = self.escape(data);
        let lines = data.split("\n");
        self._writeraw("<!-- ", true);
        self.newline();
        data.split("\n").for_each(|line| {
            self._writeraw("     ", true);
            self._writeraw(line, false);
            self.newline();
        });
        self._writeraw(" -->", true);
    }

}
