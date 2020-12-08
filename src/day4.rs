use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;

pub enum Scale {
    Cm,
    In
}

pub struct Height {
    number: i64,
    scale: Option<Scale>
}

impl Height {
    fn from_str(string: &str) -> Self {
        let re: Regex = Regex::new("in|cm").unwrap();
        if re.is_match(string) {
            let scale_pos = string.len() - 2;
            let scale = match &string[scale_pos..] {
                "cm" => Scale::Cm,
                "in" => Scale::In,
                _ => panic!("Unknown scale")
            };
            let number = &string[..scale_pos];
            Self {
                number: number.parse().unwrap(),
                scale: Some(scale)
            }
        } else {
            Self {
                number: string.parse().unwrap(),
                scale: None
            }
        }
    }

    fn is_valid(&self) -> bool {
        match self.scale {
            Some(Scale::Cm) => { self.number >= 150 && self.number <= 193 },
            Some(Scale::In) => { self.number >= 59 && self.number <= 76 },
            None => false
        }
    }
}

#[derive(Default)]
pub struct Passport {
    birth_year: Option<i64>,
    issue_year: Option<i64>,
    expiration_year: Option<i64>,
    height: Option<Height>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>
}

lazy_static! {
    static ref RE_HCL: Regex = Regex
        ::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref RE_ECL: Regex = Regex
        ::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$")
        .unwrap();
    static ref RE_PID: Regex = Regex
        ::new(r"^[0-9]{9}$")
        .unwrap();
}

impl Passport {
    fn from_string(input: &String) -> Self {
        let mut passport = Passport::default();
        input
            .lines()
            .for_each(|line| {
                line
                    .split(" ")
                    .for_each(|element| {
                        let (key,value) = element
                            .split(":")
                            .collect_tuple()
                            .unwrap();
                        match key {
                            "byr" => {
                                passport.birth_year = Some(value.to_string().parse().unwrap())
                            },
                            "iyr" => {
                                passport.issue_year = Some(value.to_string().parse().unwrap())
                            },
                            "eyr" => {
                                passport.expiration_year = Some(value.to_string().parse().unwrap())
                            },
                            "hgt" => {
                                passport.height = Some(Height::from_str(&value));
                            },
                            "hcl" => {
                                passport.hair_color = Some(value.to_string().parse().unwrap())
                            },
                            "ecl" => {
                                passport.eye_color = Some(value.to_string().parse().unwrap())
                            },
                            "pid" => {
                                passport.passport_id = Some(value.to_string().parse().unwrap())
                            },
                            _ => {}
                        }
                    })
            });
        passport
    }
    
    fn is_valid(self) -> bool {
        if let (
            Some(birth_year),
            Some(issue_year),
            Some(expiration_year),
            Some(height),
            Some(hair_color),
            Some(eye_color),
            Some(passport_id)
        ) = (
            self.birth_year,
            self.issue_year,
            self.expiration_year,
            self.height,
            self.hair_color,
            self.eye_color,
            self.passport_id
        ) {
            (birth_year >= 1920 && birth_year <= 2002) &&
            (issue_year >= 2010 && issue_year <= 2020) &&
            (expiration_year >= 2020 && expiration_year <= 2030) &&
            height.is_valid() &&
            RE_HCL.is_match(&hair_color) &&
            RE_ECL.is_match(&eye_color) &&
            RE_PID.is_match(&passport_id)
        } else {
            false
        }
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|l| l.to_string())
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<String>) -> usize {
    let re_all: Regex = Regex::new(r"byr:|iyr:|eyr:|hgt:|hcl:|ecl:|pid:|cid:")
        .unwrap();
    let re_cid: Regex = Regex::new(r"cid:").unwrap();
    input
        .iter()
        .filter(|passport_line| {
            let el_count: usize = re_all
                .find_iter(passport_line)
                .count();
            let has_cid = re_cid.is_match(passport_line);
            el_count == 8 || (el_count == 7 && !has_cid)
        })
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<String>) -> usize {
    input
        .iter()
        .filter(|passport_str| {
            Passport::from_string(passport_str).is_valid()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
        byr:1937 iyr:2017 cid:147 hgt:183cm\n\
        \n\
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
        hcl:#cfa07d byr:1929\n\
        \n\
        hcl:#ae17e1 iyr:2013\n\
        eyr:2024\n\
        ecl:brn pid:760753108 byr:1931\n\
        hgt:179cm\n\
        \n\
        hcl:#cfa07d eyr:2025 pid:166559648\n\
        iyr:2011 ecl:brn hgt:59in";
        assert_eq!(part1(&input_generator(input)), 2)
    }

    #[test]
    fn test2_invalid() {
        let input = "eyr:1972 cid:100\n\
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\
        \n\
        iyr:2019\n\
        hcl:#602927 eyr:1967 hgt:170cm\n\
        ecl:grn pid:012533040 byr:1946\n\
        \n\
        hcl:dab227 iyr:2012\n\
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\
        \n\
        hgt:59cm ecl:zzz\n\
        eyr:2038 hcl:74454a iyr:2023\n\
        pid:3556412378 byr:2007\n";
        assert_eq!(part2(&input_generator(input)), 0)
    }
    
    #[test]
    fn test2_valid() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 \
        eyr:2030 byr:1980\n\
        hcl:#623a2f\n\
        \n\
        eyr:2029 ecl:blu cid:129 byr:1989\n\
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
        \n\
        hcl:#888785\n\
        hgt:164cm byr:2001 iyr:2015 cid:88\n\
        pid:545766238 ecl:hzl\n\
        eyr:2022\n\
        \n\
        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 \
        eyr:2021 pid:093154719\n";
        assert_eq!(part2(&input_generator(input)), 4)
    }
}