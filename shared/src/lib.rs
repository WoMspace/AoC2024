use std::time::Duration;

pub fn get_input() -> String {
    let path = match std::env::args().nth(1) {
        Some(s) => s,
        None => {
            println!("Missing required argument: <input path>");
            std::process::exit(1)
        }
    };
    let file = match std::fs::read_to_string(path) {
        Ok(f) => f,
        Err(e) => {
            println!("Error reading file: {e}");
            std::process::exit(1)
        }
    };
    file
}

pub trait PrettyPrint {
    fn fmt_pretty(&self) -> String;
}
impl PrettyPrint for Duration {
    fn fmt_pretty(&self) -> String {
        if self.as_secs() > 3600 {
            let secs = self.as_secs() % 60;
            let minutes = (self.as_secs() / 60) % 60;
            let hours = self.as_secs() / 3600;
            format!("{}:{:#02}:{:#02}", hours, minutes, secs)
        } else if self.as_secs() > 60 {
            format!("{}:{:#02}", self.as_secs()/60, self.as_secs() % 60)
        } else if self.as_millis() > 2_000 {
            format!("{:.2}s", self.as_secs_f32())
        } else if self.as_micros() > 99_999 {
            format!("{}ms", self.as_millis())
        } else if self.as_micros() > 2000 {
            format!("{:.2}ms", self.as_micros() as f32/1000.0)
        } else if self.as_nanos() > 99_999 {
            format!("{}µs", self.as_micros())
        } else if self.as_nanos() > 2000 {
            format!("{:.2}µs", self.as_nanos() as f32 / 1000.0)
        } else {
            format!("{}ns", self.as_nanos())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::PrettyPrint;

    #[test]
    fn test_hours() {
        let time = Duration::from_secs(43_384);
        let pretty_time = time.fmt_pretty();
        assert_eq!(pretty_time, "12:03:04".to_string());
    }
    
    #[test]
    fn test_minutes() {
        let time = Duration::from_secs(723);
        let pretty_time = time.fmt_pretty();
        assert_eq!(pretty_time, "12:03".to_string());
    }

    #[test]
    fn test_seconds() {
        let time = Duration::from_millis(12_034);
        let pretty_time = time.fmt_pretty();
        assert_eq!(pretty_time, "12.03s".to_string());
    }

    #[test]
    fn test_millis_int() {
        let time = Duration::from_micros(123_456);
        let pretty_time = time.fmt_pretty();
        assert_eq!(pretty_time, "123ms".to_string());
    }

    #[test]
    fn test_millis_float() {
        let time = Duration::from_micros(12_034);
        let pretty_time = time.fmt_pretty();
        assert_eq!(pretty_time, "12.03ms".to_string());
    }

    #[test]
    fn test_micros_int() {
        let time = Duration::from_nanos(123_456);
        let pretty_time = time.fmt_pretty();
        assert_eq!(pretty_time, "123µs".to_string());
    }

    #[test]
    fn test_micros_float() {
        let time = Duration::from_nanos(12_345);
        let pretty_time = time.fmt_pretty();
        assert_eq!(pretty_time, "12.35µs".to_string());
    }

    #[test]
    fn test_nanos() {
        let time = Duration::from_nanos(1230);
        let pretty_time = time.fmt_pretty();
        assert_eq!(pretty_time, "1230ns".to_string());
    }
}