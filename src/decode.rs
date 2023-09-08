pub fn decode(content: String) -> String {
    let iteratable = content.chars().collect::<Vec<char>>();

    let mut return_val = String::new();
    // println!("{iteratable:?}");

    let mut traveller = 0;

    while traveller < iteratable.len() {
        let group = if ['d', 'l', 'i'].contains(iteratable.get(traveller).expect("Cannot unwrap")) {
            let mut index = traveller + 1;

            let mut num_e = 1;

            while num_e > 0 {
                let char = *iteratable.get(index).expect("Cannot unwrap");

                if ['d', 'l', 'i'].contains(&char) {
                    num_e += 1;
                }

                if char == 'e' {
                    num_e -= 1;
                }

                // println!("{num_e}");

                index += 1;
            }

            iteratable[traveller + 1..index - 1].to_vec()
        // } else if *iteratable.get(traveller).expect("Cannot unwrap") == 'l' {
        //     let mut index = traveller + 1;
        //
        //     let mut num_e = 1;
        //
        //     while num_e > 0 {
        //         let char = *iteratable.get(index).expect("Cannot unwrap");
        //
        //         if ['d', 'l', 'i'].contains(&char) {
        //             num_e += 1;
        //         }
        //
        //         if char == 'e' {
        //             num_e -= 1;
        //         }
        //
        //         // println!("{num_e}");
        //
        //         index += 1;
        //     }
        //     iteratable[traveller + 1..index - 1].to_vec()
        // } else if *iteratable.get(traveller).expect("Cannot unwrap") == 'i' {
        //     let mut index = traveller;
        //
        //     while *iteratable.get(index).expect("Cannot unwrap") != 'e' {
        //         index += 1;
        //     }
        //
        //     iteratable[traveller + 1..index].to_vec()
        } else if iteratable
            .get(traveller)
            .expect("Cannot unwrap")
            .is_ascii_digit()
        {
            let mut index = traveller;
            let mut len = 0;

            while iteratable
                .get(index)
                .expect("Cannot unwrap")
                .is_ascii_digit()
            {
                len *= 10;
                len += *iteratable.get(index).expect("Cannot unwrap") as usize - 48;
                index += 1;
            }

            // println!(
            //     "{} {} {}",
            //     iteratable.get(traveller).expect("Cannot unwrap"),
            //     index,
            //     len
            // );

            iteratable[index + 1..index + 1 + len].to_vec()
        } else {
            let res = vec![iteratable[traveller]];

            res
        };

        // print!("{group:?}");

        let mut res = String::new();

        for c in group.clone() {
            res.push(c);
        }

        // println!(
        //     "{res} {}",
        //     *iteratable.get(traveller).expect("Cannot unwrap")
        // );

        if *iteratable.get(traveller).expect("Cannot unwrap") == 'd' {
            return_val.push('{');
            return_val.push_str(&decode(res));
        } else if *iteratable.get(traveller).expect("Cannot unwrap") == 'l' {
            return_val.push('[');
            return_val.push_str(&decode(res));
        } else {
            return_val.push_str(&res);
        }

        if *iteratable.get(traveller).expect("Cannot unwrap") == 'd' {
            return_val.push('}');
        } else if *iteratable.get(traveller).expect("Cannot unwrap") == 'l' {
            return_val.push(']');
        }

        traveller += group.len() + 2;

        if traveller < content.len() {
            return_val.push_str(", ");
        }
    }

    return_val
}

pub fn decode_object(content: Vec<char>) -> String {
    let mut traveller = 0;

    let mut return_value = String::new();

    while traveller < content.len() {
        let char_to_check = *content.get(traveller).expect("Cannot unwrap");
        if ['d', 'l'].contains(&char_to_check) {
            let end = traveller + get_end_index(content[traveller + 1..].to_vec());

            let old_traveller = traveller;

            traveller += end + 1 - traveller;

            return_value.push_str(&decode_object(content[old_traveller + 1..end].to_vec()));
        } else {
            let group = if char_to_check.is_ascii_digit() {
                let mut len = 0;
                let mut index = traveller;

                let mut char_being_processed = *content.get(index).expect("Cannot unwrap");

                while char_being_processed != ':' {
                    len *= 10;
                    len += char_being_processed as usize - 48;

                    index += 1;
                    char_being_processed = *content.get(index).expect("Cannot unwrap");
                }

                traveller += index + len - traveller + 1;

                content[index + 1..index + len + 1].to_vec()
            } else if char_to_check == 'i' {
                let end = traveller + get_end_index(content[traveller..].to_vec());

                let old_traveller = traveller;
                traveller = end + 1;

                content[old_traveller + 1..end - 1].to_vec()
            } else {
                traveller += 1;

                vec![char_to_check]
            };

            for &c in &group {
                return_value.push(c);
            }
        }
    }

    return_value
}

fn get_end_index(content: Vec<char>) -> usize {
    let mut index = 1;

    let mut num_e = 1;

    while num_e > 0 {
        let mut char_being_checked = *content.get(index).expect("Cannot unwrap");
        // println!("char_being_checked = {}", char_being_checked);

        if char_being_checked == ':' {
            let old_index = index;
            let mut length = 0;
            let mut multiplier = 1;

            while char_being_checked.is_ascii_digit() && index > 0 {
                index -= 1;
                char_being_checked = *content.get(index).expect("Cannot unwrap");

                length += (char_being_checked as usize - 48) * multiplier;

                multiplier *= 10;
            }

            index = old_index + length + 1;
            // println!(
            //     "{} {} {} {} {}",
            //     index,
            //     old_index,
            //     length,
            //     char_being_checked,
            //     content.len()
            // );
            char_being_checked = *content.get(index).expect("Cannot unwrap");
        }

        if ['d', 'l', 'i'].contains(&char_being_checked) {
            num_e += 1;
        }

        if char_being_checked == 'e' {
            num_e -= 1;
        }

        index += 1;
    }

    index
}

pub fn decoder(content: String, start: usize) -> (String, usize) {
    // println!("{content}");
    let temp_iterable = content.chars().collect::<Vec<char>>();
    let iterable = temp_iterable[start..].to_vec();

    let traveller = start;

    let char_to_check = *iterable.get(traveller).expect("Cannot unwrap");
    let end;

    if ['d', 'l'].contains(&char_to_check) {
        let mut index = traveller + 1;
        let mut num_e = 1;

        while num_e > 0 {
            let mut char_being_checked = *iterable.get(index).expect("Cannot unwrap");
            // println!("char_being_checked = {}", char_being_checked);

            if char_being_checked == ':' {
                let old_index = index;
                index -= 1;
                char_being_checked = *iterable.get(index).expect("Cannot unwrap");
                let mut length = 0;
                let mut multiplier = 1;

                while char_being_checked.is_ascii_digit() && index > traveller {
                    // println!(
                    //     "{length} {multiplier} {char_being_checked} this {}",
                    //     char_being_checked.is_ascii_digit()
                    // );

                    length += (char_being_checked as usize - 48) * multiplier;

                    // println!("{length} {multiplier} {char_being_checked} this");
                    multiplier *= 10;
                    index -= 1;
                    char_being_checked = *iterable.get(index).expect("Cannot unwrap");
                }

                index = old_index + length + 1;
                // println!(
                //     "{} {} {} {} {} this not",
                //     index,
                //     old_index,
                //     length,
                //     char_being_checked,
                //     iterable.len()
                // );
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

        end = index - 1;

        let mut return_val = String::from("{ ");
        let mut checker = 1;

        let mut check;
        let mut offset = traveller + checker;
        let mut i = 0;

        while offset < end - 1 {
            (check, checker) = decoder(content[offset..end].to_string(), 0);
            offset += checker;

            return_val.push_str(&check);

            if i % 2 == 0 {
                return_val.push_str(": ");
            } else if offset < end - 1 {
                return_val.push_str(", ");
            }
            i += 1;
        }

        return_val.push_str(" }");

        (return_val, end + 1)
    } else if char_to_check == 'i' {
        let mut index = traveller + 1;
        let mut num = 0;

        let mut char_being_checked = *iterable.get(index).expect("Cannot unwrap");
        while char_being_checked != 'e' {
            num *= 10;
            num += char_being_checked as usize - 48;

            index += 1;

            char_being_checked = *iterable.get(index).expect("Cannot unwrap");
        }

        (format!("{num}"), index + 1)
    } else {
        let mut num = char_to_check as usize - 48;
        //

        let mut index = traveller + 1;
        let mut char_being_checked = *iterable.get(index).expect("Cannot unwrap");
        while char_being_checked != ':' {
            num *= 10;
            num += char_being_checked as usize - 48;

            index += 1;
            char_being_checked = *iterable.get(index).expect("Cannot unwrap");
        }

        let group = iterable[index + 1..index + 1 + num].to_vec();

        let mut return_val = String::new();

        for c in group {
            return_val.push(c);
        }

        end = index + 1 + num;

        (return_val, end)
    }
}

#[cfg(test)]
mod test {
    use crate::decode::decode;

    #[test]
    fn tester() {
        let res = decode("l3:bar4:spam3:fooi42ee".to_string());

        println!("{res}");

        panic!()
    }
}
