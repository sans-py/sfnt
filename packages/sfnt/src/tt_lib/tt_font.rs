pub struct TTFont {}

impl TTFont {
    pub fn new(input_path: &str) -> std::io::Result<Self> {
        Ok(Self {})
    }

    pub fn save_xml(self, output_path: &str) -> std::io::Result<()> {
        println!("TTFont.save_xml(&self, output_path: &str) not yet implemented");
        Ok(())
    }

    pub fn close(&self) {
        println!("TTFont closed")
    }
}
