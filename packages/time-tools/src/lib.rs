use chrono;

pub trait Timestamp {
    fn as_chrono(&self) -> chrono::DateTime<chrono::Utc>;
    fn as_timestamp(&self) -> String;
    fn parse_timestamp(str: String) -> Result<std::time::SystemTime, ()>;
}

impl Timestamp for std::time::SystemTime {
    fn as_chrono(&self) -> chrono::DateTime<chrono::Utc> {
        let date: chrono::DateTime<chrono::Utc> = (*self).into();
        date
    }

    fn as_timestamp(&self) -> String {
        self.as_chrono().format("%a %b %e %T %Y").to_string()
    }

    fn parse_timestamp(str: String) -> Result<std::time::SystemTime, ()> {
        let Ok(date) = chrono::DateTime::parse_from_str(str.as_str(), "%a %b %e %T %Y") else {
            return Err(())
        };
        let time: std::time::SystemTime = date.into();
        Ok(time)
    }
}
