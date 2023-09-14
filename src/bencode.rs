use std::{collections::HashMap, rc::Rc};

use crate::{string_from_collection, Error, Result};

#[derive(Debug, Clone)]
pub enum Bencode {
    Dict(HashMap<String, Bencode>),
    List(Rc<[Bencode]>),
    Int(i64),
    Str(Rc<str>),
}

impl Bencode {
    /// This function converts the given string to a Bencode enum
    /// This method is generally not preferred
    /// Use [``Torrent::from``](./torrent/struct.Torrent.html#method.from) instead
    ///
    /// # Examples
    /// ```
    /// use bendecode::Bencode;
    ///
    /// let content = "d9:announce1:a4:infod12:piecelengthi1e6:pieces1:a4:name1:a6:lengthi1eee";
    ///
    /// let bencode = Bencode::decode(content);
    /// ```
    ///
    /// # Errors
    /// - [``NoEndMarker``](./enum.Error.html#variant.NoEndMarker)
    /// - [``WrongStringLength``](./enum.Error.html#variant.WrongStringLength)
    /// - [``InvalidTokenFound``](./enum.Error.html#variant.InvalidTokenFound)
    /// - [``NoValueForKey``](./enum.Error.html#variant.NoValueForKey)
    pub fn decode(content: &str) -> Result<Self> {
        let iterable = content.chars().collect::<Box<[char]>>();
        let (element, index) = match Self::matcher(&iterable) {
            Ok(t) => t,
            Err(e) => {
                return Err(e);
            }
        };

        if index < iterable.len() {
            return Err(Error::NoEndMarker {
                found: iterable[index],
                index,
            });
        }

        Ok(element)
    }

    fn decode_dictionary(iterable: &[char]) -> Result<Self> {
        let mut index = 1;

        let mut final_dictionary = HashMap::<String, Self>::new();

        while index < iterable.len() {
            let key_type_checker = iterable[index];

            if !key_type_checker.is_ascii_digit() {
                return Err(Error::InvalidTokenFound {
                    found: key_type_checker,
                    expected: "digit",
                    index,
                });
            }

            let (new_index, length) = Self::find_length_forwards(&iterable[index..]);

            index += new_index;

            let key = string_from_collection(&iterable[index..index + length]);

            index += length;

            if index >= iterable.len() {
                return Err(Error::NoValueForKey { key, index });
            }

            let (value, new_index) = match Self::matcher(&iterable[index..]) {
                Ok(tup) => tup,
                Err(e) => {
                    return Err(e);
                }
            };

            index += new_index;

            final_dictionary.insert(key, value);
        }

        Ok(Self::Dict(final_dictionary))
    }

    fn decode_list(iterable: &[char]) -> Result<Self> {
        let mut index = 1;

        let mut final_list = Vec::new();

        while index < iterable.len() {
            let (el, new_index) = match Self::matcher(&iterable[index..]) {
                Ok(tup) => tup,
                Err(e) => {
                    return Err(e);
                }
            };

            index += new_index;

            final_list.push(el);
        }

        let res = Rc::from(final_list);

        Ok(Self::List(res))
    }

    fn matcher(iterable: &[char]) -> Result<(Self, usize)> {
        let mut index = 0;
        let character_to_check = iterable[index];

        let bencoding = match character_to_check {
            'd' => {
                let dict_end = match Self::find_end_marker(&iterable[index..]) {
                    Ok(end) => end,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let old_index = index;
                index += dict_end + 1;
                match Self::decode_dictionary(&iterable[old_index..index - 1]) {
                    Ok(dictionary) => dictionary,
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            'l' => {
                let list_end = match Self::find_end_marker(&iterable[index..]) {
                    Ok(end) => end,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let old_index = index;
                index += list_end + 1;

                match Self::decode_list(&iterable[old_index..index - 1]) {
                    Ok(list) => list,
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            'i' => {
                let int_end = match Self::find_end_marker(&iterable[index..]) {
                    Ok(end) => end,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let old_index = index;
                index += int_end + 1;

                match Self::decode_integer(&iterable[old_index..index - 1]) {
                    Ok(int) => int,
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            '0'..='9' => {
                let (new_index, length) = Self::find_length_forwards(&iterable[index..]);

                index += new_index;

                if index + length > iterable.len() {
                    return Err(Error::WrongStringLength { length, index });
                }

                let string = string_from_collection(&iterable[index..index + length]);

                index += length;

                let v = Rc::from(string.as_str());

                Self::Str(v)
            }
            _ => {
                return Err(Error::InvalidTokenFound {
                    found: character_to_check,
                    expected: "'d', 'l', 'i', or digit",
                    index,
                });
            }
        };

        Ok((bencoding, index))
    }

    fn decode_integer(iterable: &[char]) -> Result<Self> {
        let mut integer = 0;

        let skip = if iterable[1] == '-' { 2 } else { 1 };

        for &c in iterable.iter().skip(skip) {
            let new_num = c as i64 - 48;

            if !(0..=9).contains(&new_num) {
                return Err(Error::InvalidTokenFound {
                    found: c,
                    expected: "digit",
                    index: 0,
                });
            }
            integer = integer * 10 + new_num;
        }

        if skip > 1 {
            integer = -integer;
        }

        Ok(Self::Int(integer))
    }

    const fn find_end_marker(iterable: &[char]) -> Result<usize> {
        let mut index = 1;
        let mut number_of_markers = 1;
        let mut latest_string_ending = 0;

        while number_of_markers > 0 {
            if index >= iterable.len() {
                return Err(Error::NoEndMarker { found: '$', index });
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
                        return Err(Error::WrongStringLength { length, index });
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

            if index >= iterable.len() {
                break;
            }

            character_to_check = iterable[index];
        }

        (index + 1, length)
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use crate::{error::Error, Bencode};

    #[test]
    fn test_valid_bencode() {
        let string_to_check = String::from(
            "d3:key5:value4:key1i365e4:key2i-365e4:key3li1ei2e4:spame4:key4d1:a1:b1:c1:dee",
        );

        let Ok(Bencode::Dict(dictionary)) = Bencode::decode(string_to_check.as_str()) else {
            panic!("Expected dictionary");
        };

        let keys = [
            String::from("key"),
            String::from("key1"),
            String::from("key2"),
            String::from("key3"),
            String::from("key4"),
        ];

        for (key, value) in dictionary {
            assert!(keys.contains(&key));

            match value {
                Bencode::Int(int) => {
                    assert!(int == 365 || int == -365);
                }
                Bencode::Str(string) => {
                    assert!(&*string == "value");
                }
                Bencode::Dict(dict) => {
                    let expected_keys = [String::from("a"), String::from("c")];
                    let expected_values = [Rc::from("b"), Rc::from("d")];
                    for (k, v) in dict {
                        assert!(expected_keys.contains(&k));

                        let Bencode::Str(s) = v else {
                            panic!("Expected string as value for dictionary");
                        };

                        assert!(expected_values.contains(&s));
                    }
                }
                Bencode::List(list) => {
                    for elem in &*list {
                        match elem {
                            Bencode::Int(integer) => {
                                assert!(*integer == 1 || *integer == 2);
                            }
                            Bencode::Str(s) => {
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
        let invalid_long_sequence = "d3:key5:valuei42e3:wowe";

        assert_eq!(
            Bencode::decode(no_value).expect_err("No value for key"),
            Error::NoValueForKey {
                key: String::from("key"),
                index: 6
            }
        );

        assert_eq!(
            Bencode::decode(invalid_length).expect_err("invalid length"),
            Error::WrongStringLength {
                length: 3,
                index: 2
            }
        );

        assert_eq!(
            Bencode::decode(invalid_num).expect_err("invalid char"),
            Error::InvalidTokenFound {
                found: 'r',
                expected: "digit",
                index: 0
            }
        );

        assert_eq!(
            Bencode::decode(missing_end_marker).expect_err("No end marker"),
            Error::NoEndMarker {
                found: '$',
                index: 5
            }
        );

        assert_eq!(
            Bencode::decode(invalid_long_sequence).expect_err("Invalid key type"),
            Error::InvalidTokenFound {
                found: 'i',
                expected: "digit",
                index: 13
            }
        );
    }
}
