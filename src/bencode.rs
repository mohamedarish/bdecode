use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

pub struct Bencode;

#[derive(PartialEq, Eq, Debug)]
pub enum Types {
    Dictionary(HashMap<Types, Types>),
    List(Box<[Types]>),
    Integer(i32),
    StringType(String),
}

impl Hash for Types {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Dictionary(dictionary) => {
                for (key, value) in dictionary {
                    key.hash(state);
                    value.hash(state);
                }
            }
            Self::List(list) => {
                for item in list.iter() {
                    item.hash(state);
                }
            }
            Self::Integer(i) => {
                i.hash(state);
            }
            Self::StringType(s) => {
                s.hash(state);
            }
        }
    }
}

impl Bencode {
    pub fn decode_dictionary(iterable: &[char]) -> Types {
        println!("{:?}", iterable);
        let start = 0;
        let end = Self::find_end(iterable);
        let mut index = start + 1;

        let mut return_value = HashMap::<Types, Types>::new();

        while index < end - 1 {
            let mut char_to_check = *iterable.get(index).expect("Cannot unwrap");

            println!("{char_to_check}");

            println!("{} {} {:?}", index, end, return_value);

            let key = match char_to_check {
                'i' => {
                    let num_end = index + Self::find_end(&iterable[index..end]);

                    println!("{} {} num", end, num_end);

                    let num = Self::parse_num(&iterable[index..num_end]);

                    index = num_end + 1;

                    Types::Integer(num.try_into().expect("Cannot unwrap"))
                }
                '0'..='9' => {
                    let mut len = 0;

                    while char_to_check != ':' {
                        len *= 10;
                        len += char_to_check as usize - 48;

                        index += 1;
                        char_to_check = *iterable.get(index).expect("Cannot unwrap");
                    }

                    let mut resultant = String::new();

                    for &c in &iterable[index + 1..index + 1 + len] {
                        resultant.push(c);
                    }

                    index += len + 1;

                    Types::StringType(resultant)
                }
                _ => Types::StringType(String::new()),
            };

            char_to_check = *iterable.get(index).expect("Cannot unwrap");

            println!("{char_to_check} middle");

            println!("{} {} {:?} {:?} middle", index, end, return_value, key);

            let value = match char_to_check {
                'd' => {
                    let dict_end = Self::find_end(&iterable[index + 1..end]);

                    Types::StringType(String::from("dictionary"))
                }
                'l' => Types::StringType(String::from("list")),
                'i' => {
                    let num_end = index + Self::find_end(&iterable[index..end]);

                    println!("{} {}", index, num_end);

                    let num = Self::parse_num(&iterable[index + 1..num_end]);

                    index = num_end + 1;

                    Types::Integer(num.try_into().expect("Cannot unwrap"))
                }
                '0'..='9' => {
                    let mut len = 0;

                    while char_to_check != ':' {
                        len *= 10;
                        len += char_to_check as usize - 48;

                        index += 1;
                        char_to_check = *iterable.get(index).expect("Cannot unwrap");
                    }

                    let mut resultant = String::new();

                    for &c in &iterable[index + 1..index + 1 + len] {
                        resultant.push(c);
                    }

                    index += len + 1;

                    Types::StringType(resultant)
                }
                _ => Types::StringType(String::new()),
            };

            println!("{} {} after", index, end);

            return_value.insert(key, value);

            println!("{} {} {:?} after", index, end, return_value);
        }

        Types::Dictionary(return_value)
    }

    fn parse_num(iterable: &[char]) -> usize {
        let mut num = 0;

        for &c in iterable {
            num *= 10;
            num += c as usize - 48;
        }

        num
    }

    pub fn find_end(iterable: &[char]) -> usize {
        let mut index = 1;

        let mut num_e = 1;

        while num_e > 0 {
            let mut char_being_checked = *iterable.get(index).expect("Cannot unwrap");

            println!("{} {} {} inside end", num_e, index, char_being_checked);

            if char_being_checked == ':' {
                let old_index = index;
                index -= 1;
                char_being_checked = *iterable.get(index).expect("Cannot unwrap");
                let mut length = 0;
                let mut multiplier = 1;

                while char_being_checked.is_ascii_digit() && index > 0 {
                    length += (char_being_checked as usize - 48) * multiplier;

                    multiplier *= 10;
                    index -= 1;
                    char_being_checked = *iterable.get(index).expect("Cannot unwrap");
                }

                index = old_index + length + 1;

                char_being_checked = *iterable.get(index).expect("Cannot unwrap");
            }

            if ['d', 'l', 'i'].contains(&char_being_checked) {
                num_e += 1;
            }

            if char_being_checked == 'e' {
                num_e -= 1;
            }

            println!("{} {} {} inside end2", num_e, index, char_being_checked);

            index += 1;
        }

        index - 1
    }
}
