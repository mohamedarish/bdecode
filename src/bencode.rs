use std::rc::Rc;

use crate::{Error, Result};

pub struct Bencoder;

#[derive(Debug)]
pub enum Bencoding {
    Dict(Rc<[(String, Bencoding)]>),
    List(Rc<[Bencoding]>),
    Int(i64),
    Str(Rc<str>),
}

fn string_from_collection(collection: &[char]) -> String {
    let mut string = String::new();

    for &c in collection {
        string.push(c);
    }

    string
}

impl Bencoder {
    /// # Errors
    pub fn decode(content: &str) -> Result<Bencoding> {
        let iterable = content.chars().collect::<Box<[char]>>();
        let (element, index) = match Self::matcher(&iterable) {
            Ok(t) => t,
            Err(e) => {
                return Err(Error::from(e));
            }
        };

        if index < iterable.len() {
            let error_message_bencode_file_not_valid = format!(
                "invalid bencode file: expected EOF found {character}",
                character = iterable[index]
            );
            return Err(Error::from(error_message_bencode_file_not_valid));
        }

        Ok(element)
    }

    fn decode_dictionary(iterable: &[char]) -> Result<Bencoding> {
        let mut index = 1;

        let mut final_dictionary = Vec::<(String, Bencoding)>::new();

        while index < iterable.len() {
            let key_type_checker = iterable[index];

            if !key_type_checker.is_ascii_digit() {
                let error_message_invalid_key =
                    format!("Expected string key ['0'..='9'] found {key_type_checker}");
                return Err(Error::from(error_message_invalid_key));
            }

            let (new_index, length) = Self::find_length_forwards(&iterable[index..]);

            index += new_index;

            let key = string_from_collection(&iterable[index..index + length]);

            index += length;

            if index >= iterable.len() {
                let error_message_no_value = format!("The provided key: {key} has no value");
                return Err(Error::from(error_message_no_value));
            }

            let (value, new_index) = match Self::matcher(&iterable[index..]) {
                Ok(t) => t,
                Err(e) => {
                    return Err(Error::from(e));
                }
            };

            index += new_index;

            final_dictionary.push((key, value));
        }

        let dict = Rc::from(final_dictionary);

        Ok(Bencoding::Dict(dict))
    }

    fn decode_list(iterable: &[char]) -> Result<Bencoding> {
        let mut index = 1;

        let mut final_list = Vec::new();

        while index < iterable.len() {
            let (el, new_index) = match Self::matcher(&iterable[index..]) {
                Ok(t) => t,
                Err(e) => {
                    return Err(Error::from(e));
                }
            };

            index += new_index;

            final_list.push(el);
        }

        let res = Rc::from(final_list);

        Ok(Bencoding::List(res))
    }

    fn matcher(iterable: &[char]) -> Result<(Bencoding, usize)> {
        let mut index = 0;
        let character_to_check = iterable[index];

        let bencoding = match character_to_check {
            'd' => {
                let dict_end = match Self::find_end_marker(&iterable[index..]) {
                    Ok(end) => end,
                    Err(e) => {
                        return Err(Error::from(e));
                    }
                };

                let old_index = index;
                index += dict_end + 1;
                match Self::decode_dictionary(&iterable[old_index..index - 1]) {
                    Ok(dictionary) => dictionary,
                    Err(e) => {
                        return Err(Error::from(e));
                    }
                }
            }
            'l' => {
                let list_end = match Self::find_end_marker(&iterable[index..]) {
                    Ok(end) => end,
                    Err(e) => {
                        return Err(Error::from(e));
                    }
                };

                let old_index = index;
                index += list_end + 1;

                match Self::decode_list(&iterable[old_index..index - 1]) {
                    Ok(list) => list,
                    Err(e) => {
                        return Err(Error::from(e));
                    }
                }
            }
            'i' => {
                let int_end = match Self::find_end_marker(&iterable[index..]) {
                    Ok(end) => end,
                    Err(e) => {
                        return Err(Error::from(e));
                    }
                };

                let old_index = index;
                index += int_end + 1;

                match Self::decode_integer(&iterable[old_index..index - 1]) {
                    Ok(int) => int,
                    Err(e) => {
                        return Err(Error::from(e));
                    }
                }
            }
            '0'..='9' => {
                let (new_index, length) = Self::find_length_forwards(&iterable[index..]);

                index += new_index;

                if index + length > iterable.len() {
                    let error_message_invalid_string_length_provided = format!("The provided string length is wrong, got length: {length}, but max length allowed is: {len}", len = iterable.len());
                    return Err(Error::from(error_message_invalid_string_length_provided));
                }

                let string = string_from_collection(&iterable[index..index + length]);

                index += length;

                let v = Rc::from(string.as_str());

                Bencoding::Str(v)
            }
            _ => {
                let error_message_invalid_character = format!("Unexpected character obtained at index: {index}, expected: ['d', 'l', 'i', '0'..='9'], found: {character_to_check}");
                return Err(Error::from(error_message_invalid_character));
            }
        };

        Ok((bencoding, index))
    }

    fn decode_integer(iterable: &[char]) -> Result<Bencoding> {
        let mut integer = 0;

        let skip = if iterable[1] == '-' { 2 } else { 1 };

        for &c in iterable.iter().skip(skip) {
            let new_num = c as i64 - 48;

            if !(0..=9).contains(&new_num) {
                let error_message_invalid_character_in_integer =
                    format!("Invalid character found: expected ['0'..='9'], found: {c}");
                return Err(Error::from(error_message_invalid_character_in_integer));
            }
            integer = integer * 10 + new_num;
        }

        if skip > 1 {
            integer = -integer;
        }

        Ok(Bencoding::Int(integer))
    }

    fn find_end_marker(iterable: &[char]) -> Result<usize> {
        let mut index = 1;
        let mut number_of_markers = 1;
        let mut latest_string_ending = 0;

        while number_of_markers > 0 {
            if index >= iterable.len() {
                let error_message_out_of_bounds = format!("No end Marker could be found for provided element: Index out of range ({index} / {length})", length = iterable.len());
                return Err(Error::from(error_message_out_of_bounds));
            }

            let character_being_checked = iterable[index];

            match character_being_checked {
                'd' | 'l' | 'i' => {
                    number_of_markers += 1;
                    index += 1;
                }
                'e' => {
                    number_of_markers -= 1;
                    index += 1;
                }
                ':' => {
                    let length = Self::find_length_backwards(iterable, index, latest_string_ending);

                    if length == 0 {
                        let error_message_invalid_string_length_provided = format!(
                            "The length mentioned near index {index} is wrong, found length = 0"
                        );
                        return Err(Error::from(error_message_invalid_string_length_provided));
                    }

                    index += length + 1;

                    latest_string_ending = index - 1;
                }
                _ => {
                    index += 1;
                }
            }
        }

        Ok(index - 1)
    }

    const fn find_length_backwards(
        iterable: &[char],
        semicolon: usize,
        previous_string: usize,
    ) -> usize {
        let mut index = semicolon - 1;
        let mut length = 0;
        let mut multiplier = 1;

        while index > previous_string && iterable[index].is_ascii_digit() {
            let character_being_checked = iterable[index];

            length += (character_being_checked as usize - 48) * multiplier;
            index -= 1;
            multiplier *= 10;
        }

        length
    }

    const fn find_length_forwards(iterable: &[char]) -> (usize, usize) {
        let mut index = 0;
        let mut character_to_check = iterable[index];
        let mut length = 0;

        while character_to_check != ':' {
            length = length * 10 + (character_to_check as usize - 48);

            index += 1;
            character_to_check = iterable[index];
        }

        (index + 1, length)
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::{Bencoder, Bencoding};

    #[test]
    fn test_valid_bencode() {
        let string_to_check = String::from(
            "d3:key5:value4:key1i365e4:key2i-365e4:key3li1ei2e4:spame4:key4d1:a1:b1:c1:dee",
        );

        let Ok(Bencoding::Dict(dictionary)) = Bencoder::decode(string_to_check.as_str()) else {
            panic!("Expected dictionary");
        };

        let keys = [
            String::from("key"),
            String::from("key1"),
            String::from("key2"),
            String::from("key3"),
            String::from("key4"),
        ];

        for (key, value) in &*dictionary {
            assert!(keys.contains(key));

            match value {
                Bencoding::Int(int) => {
                    assert!(*int == 365 || *int == -365);
                }
                Bencoding::Str(string) => {
                    assert!(&**string == "value");
                }
                Bencoding::Dict(dict) => {
                    let expected_keys = [String::from("a"), String::from("c")];
                    let expected_values = [Rc::from("b"), Rc::from("d")];
                    for (k, v) in &**dict {
                        assert!(expected_keys.contains(k));

                        let Bencoding::Str(s) = v else {
                            panic!("Expected string as value for dictionary");
                        };

                        assert!(expected_values.contains(s));
                    }
                }
                Bencoding::List(list) => {
                    for elem in &**list {
                        match elem {
                            Bencoding::Int(integer) => {
                                assert!(*integer == 1 || *integer == 2);
                            }
                            Bencoding::Str(s) => {
                                assert_eq!(&**s, "spam");
                            }
                            _ => {
                                panic!("Expected integer or string");
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_invalid_bencode() {
        let no_value = "d3:keye";
        let invalid_length = "3:as";
        let invalid_num = "i3re";
        let missing_end_marker = "li43e";

        assert!(Bencoder::decode(no_value).is_err());

        assert!(Bencoder::decode(invalid_length).is_err());

        assert!(Bencoder::decode(invalid_num).is_err());

        assert!(Bencoder::decode(missing_end_marker).is_err());
    }
}
