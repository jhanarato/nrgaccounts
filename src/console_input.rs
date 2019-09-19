use chrono::{ NaiveDate };
use std::io;

pub fn ask_for_date() -> Result<NaiveDate, &'static str> {
    
    let mut input = String::new(); 
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.pop(); // Remove trailing newline.
            parse_date(input.as_str())
        },
        Err(e) => panic!("Unexpected error {}", e),
    }
}

/// Given a date as an ISO formatted string return
/// a chrono date object. Assuming input is valid.
fn parse_date(date_str: &str) -> Result<NaiveDate, &'static str> {
    let date = NaiveDate::parse_from_str(date_str, "%F");
    match date {
        Err(_e) => Err("Failed to parse date"),
        Ok(d) => Ok(d), 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_date_input() {
        let actual_result = parse_date("1999-12-31");
        let expected_result = Ok(NaiveDate::from_ymd(1999, 12, 31));
        assert_eq!(actual_result, expected_result); 
    }

    #[test]
    fn bad_date_input() {
        let actual_result = parse_date("1999-12-31*");
        let expected_result = Err("Failed to parse date.");
        assert_eq!(actual_result, expected_result); 
    }
}
