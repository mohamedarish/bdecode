use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

pub struct Bencode;

#[derive(PartialEq, Eq, Debug)]
pub enum Types {
    Dictionary(HashMap<Types, Types>),
    List(Vec<Types>),
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
                for item in list {
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
        let start = 0;
        let end = Self::find_end(iterable);

        let mut index = start + 1;

        let mut return_value = HashMap::<Types, Types>::new();

        while index < end - 1 {
            let mut char_to_check = *iterable.get(index).expect("Cannot unwrap");

            let key = match char_to_check {
                'i' => {
                    let num_end = index + Self::find_end(&iterable[index..end]);

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

            char_to_check = *iterable.get(index).expect("Cannot unwrap");

            let value = match char_to_check {
                'd' => {
                    let dict_end = index + Self::find_end(&iterable[index..end]);

                    let old_index = index;
                    index = dict_end + 1;
                    Self::decode_dictionary(&iterable[old_index..=dict_end])
                }
                'l' => {
                    let list_end = index + Self::find_end(&iterable[index..end]);

                    let old_index = index;
                    index = list_end + 1;

                    Self::decode_list(&iterable[old_index..=list_end])
                }
                'i' => {
                    let num_end = index + Self::find_end(&iterable[index..end]);

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

            return_value.insert(key, value);
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

    pub fn decode_list(iterable: &[char]) -> Types {
        let mut index = 1;
        let end = iterable.len() - 1;

        let mut return_value = Vec::<Types>::new();

        while index < end {
            let mut char_to_check = *iterable.get(index).expect("Cannot unwrap");

            let element = match char_to_check {
                'd' => {
                    let dict_end = index + Self::find_end(&iterable[index..end]);

                    let old_index = index;
                    index = dict_end + 1;
                    Self::decode_dictionary(&iterable[old_index..=dict_end])
                }
                'l' => {
                    let list_end = index + Self::find_end(&iterable[index..end]);

                    let old_index = index;
                    index = list_end + 1;

                    Self::decode_list(&iterable[old_index..=list_end])
                }
                'i' => {
                    let num_end = index + Self::find_end(&iterable[index..end]);

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

            return_value.push(element);
        }

        Types::List(return_value)
    }

    pub fn find_end(iterable: &[char]) -> usize {
        let mut index = 1;

        let mut num_e = 1;

        while num_e > 0 {
            let mut char_being_checked = *iterable.get(index).expect("Cannot unwrap");

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

            index += 1;
        }

        index - 1
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::{Bencode, Types};

    #[test]
    fn check_dict_initialization() {
        let str_to_check =
            String::from("d3:bar4:spam3:fooi42ei43edi59e4:reedi45ei1ee1:rli44e2:asee");

        let iterable = str_to_check.chars().collect::<Vec<char>>();

        let Types::Dictionary(final_dictionary) = Bencode::decode_dictionary(&iterable) else {
            panic!("Cannot parse valid dictionary")
        };

        let keys = final_dictionary.keys();

        let expected_keys = [
            Types::StringType(String::from("foo")),
            Types::StringType(String::from("bar")),
            Types::Integer(43),
            Types::StringType(String::from("r")),
        ];

        for key in keys {
            assert!(expected_keys.contains(key));
        }

        let values = final_dictionary.values();

        let mut inner_dict = HashMap::<Types, Types>::new();

        inner_dict.insert(Types::Integer(59), Types::StringType(String::from("reed")));

        inner_dict.insert(Types::Integer(45), Types::Integer(1));

        let expected_values = [
            Types::StringType(String::from("spam")),
            Types::Integer(42),
            Types::Dictionary(inner_dict),
            Types::List(vec![
                Types::Integer(44),
                Types::StringType(String::from("as")),
            ]),
        ];

        for value in values {
            assert!(expected_values.contains(value));
        }
    }

    #[test]
    fn check_list_initialization() {
        let str_to_check = String::from("li42e4:spam3:fooi199ed3:key5:valueeli1ei2ei3eee");

        let iterable = str_to_check.chars().collect::<Vec<char>>();

        let Types::List(final_list) = Bencode::decode_list(&iterable) else {
            panic!("Cannot parse the given list");
        };

        let mut inner_dict = HashMap::<Types, Types>::new();

        inner_dict.insert(
            Types::StringType(String::from("key")),
            Types::StringType(String::from("value")),
        );

        let expected_list = vec![
            Types::Integer(42),
            Types::StringType(String::from("spam")),
            Types::StringType(String::from("foo")),
            Types::Integer(199),
            Types::Dictionary(inner_dict),
            Types::List(vec![
                Types::Integer(1),
                Types::Integer(2),
                Types::Integer(3),
            ]),
        ];

        for element in final_list {
            assert!(expected_list.contains(&element));
        }
    }
}
