pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn decode(content: String) {
    let iteratable = content.chars().collect::<Vec<char>>();

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

            let typ = *iteratable.get(traveller).expect("Cannot unwrap");

            // println!("{} {}", typ, index);

            let g = iteratable[traveller + 1..index - 1].to_vec();

            if typ == 'd' {
                println!("{{");
            } else if typ == 'l' {
                println!("[");
            }

            if typ == 'd' || typ == 'l' {
                let mut res = String::new();

                for c in g.clone() {
                    res.push(c);
                }

                decode(res);
            }

            if typ == 'd' {
                println!("}}");
            } else if typ == 'l' {
                println!("]");
            }

            g
        // } else if iteratable.get(traveller).expect("Cannot unwrap") == &'l' {
        //     let mut index = traveller;
        //
        //     while iteratable.get(index).expect("Cannot unwrap") != &'e' {
        //         index += 1;
        //     }
        //
        //     iteratable[traveller..index + 1].to_vec()
        // } else if iteratable.get(traveller).expect("Cannot unwrap") == &'i' {
        //     let mut index = traveller;
        //
        //     while iteratable.get(index).expect("Cannot unwrap") != &'e' {
        //         index += 1;
        //     }
        //
        //     iteratable[traveller..index + 1].to_vec()
        } else {
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
        };

        println!("{group:?}");

        traveller += group.len() + 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn tester() {
        decode("l4:spami42ee".to_string());

        panic!()
    }
}
