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
                println!("{} {}", char_to_check, return_value);
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
        let char_to_check = *content.get(index).expect("Cannot unwrap");

        if ['d', 'l', 'i'].contains(&char_to_check) {
            num_e += 1;
        }

        if char_to_check == 'e' {
            num_e -= 1;
        }

        index += 1;
    }

    index
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
