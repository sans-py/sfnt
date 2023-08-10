use super::sfnt::SFNTReader;
pub struct TTFont {
    reader: SFNTReader,
}

impl TTFont {
    pub fn new(input_path: &str) -> std::io::Result<Self> {
        let mut file = std::fs::File::open(input_path)?;
        let reader = SFNTReader::new(file)?;
        Ok(Self { reader })
    }

    pub fn save_xml(self, output_path: &str) -> std::io::Result<()> {
        println!("TTFont.save_xml(&self, output_path: &str) not yet implemented");
        Ok(())
    }

    pub fn close(&self) {
        println!("TTFont closed")
    }
}
