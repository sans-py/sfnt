pub enum FileType {
    OTF,
    TTC,
    TTF,
    WOFF,
    WOFF2,
    OTX,
    TTX,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::OTF => write!(f, "otf"),
            FileType::TTC => write!(f, "ttc"),
            FileType::TTF => write!(f, "ttf"),
            FileType::WOFF => write!(f, "woff"),
            FileType::WOFF2 => write!(f, "woff2"),
            FileType::OTX => write!(f, "otx"),
            FileType::TTX => write!(f, "ttx"),
        }
    }
}
