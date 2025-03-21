use crate::lang_parser::LangParser;


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;
    #[test]
    fn simple_function() {
        let input = "fn add(x, y) {
            print(\"Adding\");
        }";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");

        println!("{:?}", ast);
        // evaluate the ast
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn call_function() {
        let input = "fn add(x, y) {
            print(\"Adding\");
            let z = x + y;
            print(z);
        }
        add(5, 10);";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");

        // evaluate the ast
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn prep_for_binary_conversion() {
        let input = "
        let x = 5;
        for i in 0..10 {
            let y = x % 2;
            x = x / 2;
            print(y);
            if (x == 0) {
                break;
            }
        }";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");

        // evaluate the ast
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn binary_type() {
        let input = "\
        let b = bin(5);\
        print(b);";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn bitwise_and() {
        let input = "
        let a = bin(7);
        let b = bin(3);
        let c = a & b;
        print(c);
        ";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn bitwise_or() {
        let input = "let a = bin(7);
        let b = bin(3);
        let c = a | b;
        print(c);";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn bitwise_xor() {
        let input = "let a = bin(7);
        let b = bin(3);
        let c = a ^ b;
        print(c);";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn bitwise_not() {
        let input = "let a = bin(7);
        let b = ~a;
        print(b);";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn shift_left() {
        let input = "let a = bin(7);
        let b = a << 1;
        print(b);";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn shift_right() {
        let input = "let a = bin(7);
        let b = a >> 1;
        print(b);";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn matching_masks() {
        let input = "let x = bin(7) & bin(3);
        print(bin(7) & bin(3));";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn return_values() {
        let input = "fn add(x, y) {
            let z = x + y;
            return z;
        }
        let z = add(5, 10);
        print(z);";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
        println!("decls: {:?}", ast.declarations);
    }

    #[test]
    fn generate_mask() {
        let input = "let a = 1;
        let x = bin(a) & bin(3);
        print(x);";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn loop_mask() {
        let input = "
        let input = bin(110);
        for i in 0..8 {
            let mask = bin(i) << 5;
            let res = input & mask;
            if (res == mask) {
                print(33);
            }
        }";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn bit_ranges() {
        let value = 0b1101_0110_1001_0011_1110_0101_0111;
        let start_bit = 2u32;
        let end_bit = start_bit + 2;
        let range_width = end_bit - start_bit + 1;
        println!("range width: {}", range_width);
        let mask = (1 << range_width) - 1;
        let shifted_val = value >> start_bit;
        println!("res: {:b}", (shifted_val & mask));

        let mut i : i32 = 8;
        while i > 1 {
            let start = i - 2;
            let end = (i) as u32;
            // let range = end - start_bit + 1;
            let range = 3;
            println!("range: {}", range);
            let mask = (1 << range) - 1;
            let shifted = value >> i;
            println!("shifted: {:b}", shifted & mask);
            println!("bits: {:b}", shifted);
            i -= 1;
        }
        // iterate_3bit_slices(value)
    }

    #[test]
    fn stuff() {
        let mut i = 7;
        let rule = 110u8;
        let mut rules = HashMap::new(); 
        while i >= 0 {
            // println!("i: {:03b}", i);
            // println!("is_set: {}", (rule >> i) & 1);
            rules.insert(i, (rule >> i) & 1);
            i -= 1;
        }
        // for i in (0..8).rev() {
        //     let bit = (rule >> i) & 1;
        //     println!("bit: {}", bit);
        // }
        println!("rules: {:?}", rules);
        println!("rule: {:03b}", rules.get(&2).unwrap());
    }
}
