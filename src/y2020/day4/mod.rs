use regex::Regex;
use std::collections::HashMap;

pub fn read_input() -> Vec<Passport> {
    let input = include_str!("input.txt");
    input
        .split("\n\n")
        .map(String::from)
        .map(Passport::new)
        .collect()
}

#[derive(Debug, Clone)]
pub struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    pub fn new(input: String) -> Passport {
        let re = Regex::new(r"(.+):(.+)").unwrap();
        Passport {
            fields: input
                .split_whitespace()
                .filter_map(|field| re.captures(field))
                .map(|caps| (caps[1].to_string(), caps[2].to_string()))
                .collect(),
        }
    }

    pub fn is_valid(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|key| self.fields.get(&key.to_string()).is_some())
    }

    pub fn is_valid_extended(&self) -> bool {
        self.valid_birth()
            && self.valid_issue()
            && self.valid_expiration()
            && self.valid_height()
            && self.valid_hair()
            && self.valid_ecl()
            && self.valid_id()
    }

    fn valid_birth(&self) -> bool {
        let re = Regex::new(r"^\d{4}$").unwrap();
        let year = self
            .fields
            .get("byr")
            .filter(|s| re.is_match(s))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        (1920..=2002).contains(&year)
    }
    fn valid_issue(&self) -> bool {
        let re = Regex::new(r"\d{4}").unwrap();
        let year = self
            .fields
            .get("iyr")
            .filter(|s| re.is_match(s))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        (2010..=2020).contains(&year)
    }
    fn valid_expiration(&self) -> bool {
        let re = Regex::new(r"\d{4}").unwrap();
        let year = self
            .fields
            .get("eyr")
            .filter(|s| re.is_match(s))
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        (2020..=2030).contains(&year)
    }
    fn valid_height(&self) -> bool {
        let re = Regex::new(r"(\d+)(in|cm)").unwrap();
        let (height, metric) = self
            .fields
            .get("hgt")
            .and_then(|s| re.captures(s))
            .map(|cap| (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()))
            .map(|cap| (cap.0.parse::<u32>().unwrap(), cap.1))
            .unwrap_or((0, "none"));
        match metric {
            "cm" => (150..=193).contains(&height),
            "in" => (59..=76).contains(&height),
            _ => false,
        }
    }

    fn valid_hair(&self) -> bool {
        let re = Regex::new(r"^\#[0-9a-f]{6}$").unwrap();
        re.is_match(self.fields.get("hcl").unwrap_or(&"".to_string()))
    }

    fn valid_ecl(&self) -> bool {
        let re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
        re.is_match(self.fields.get("ecl").unwrap_or(&"".to_string()))
    }

    fn valid_id(&self) -> bool {
        let re = Regex::new(r"^\d{9}$").unwrap();
        re.is_match(self.fields.get("pid").unwrap_or(&"".to_string()))
    }
}
