
mod ean_checker;
mod svg_parser;

use std::{char, fs,io};


fn calculate_barcode_ean13(ean:&str) -> String {
    let mut barcode = "101".to_string();

    let first_part = &ean[1..7];
    let second_part = &ean[7..13];

    println!("{}",first_part);

    println!("{}",second_part);

    let prefix = ean.chars().nth(0).unwrap();

    for i in 0..6 {
        let set_to_affect =  find_set_by_prefix(prefix,i);
        if set_to_affect == 'A' {
            barcode = barcode + map_set_a(first_part.chars().nth(i.try_into().unwrap()).unwrap())

        } else if set_to_affect == 'B' {
            barcode = barcode + map_set_b(first_part.chars().nth(i.try_into().unwrap()).unwrap())
        } else {
            panic!("Invalid set")
        }
    }

    barcode = barcode + "01010";

    for i in 0..6 {

        barcode = barcode + map_set_c(second_part.chars().nth(i.try_into().unwrap()).unwrap())
    }

    barcode = barcode + "101";


    return barcode

}

fn map_set_a(raw_char:char) -> &'static str {

    if raw_char == '0' {
		return "0001101"
    } else if raw_char == '1' {
        return "0011001"
    } else if raw_char == '2' {
        return "0010011"
    } else if raw_char == '3' {
        return "0111101"
    } else if raw_char == '4' {
        return "0100011"
    } else if raw_char == '5' {
        return "0110001"
    } else if raw_char == '6' {
        return "0101111"
    } else if raw_char == '7' {
        return "0111011"
    } else if raw_char == '8' {
        return "0110111"
    } else if raw_char == '9' {
        return "0001011"
    } else {
        return ""
    }

	
}

fn map_set_b(raw_char:char) -> &'static str {

    if raw_char == '0' {
		return "0100111"
    } else if raw_char == '1' {
        return "0110011"
    } else if raw_char == '2' {
        return "0011011"
    } else if raw_char == '3' {
        return "0100001"
    } else if raw_char == '4' {
        return "0011101"
    } else if raw_char == '5' {
        return "0111001"
    } else if raw_char == '6' {
        return "0000101"
    } else if raw_char == '7' {
        return "0010001"
    } else if raw_char == '8' {
        return "0001001"
    } else if raw_char == '9' {
        return "0010111"
    } else {
        return ""
    }


}

fn map_set_c(raw_char:char) -> &'static str {
	if raw_char == '0' {
		return "1110010"
    } else if raw_char == '1' {
        return "1100110"
    } else if raw_char == '2' {
        return "1101100"
    } else if raw_char == '3' {
        return "1000010"
    } else if raw_char == '4' {
        return "1011100"
    } else if raw_char == '5' {
        return "1001110"
    } else if raw_char == '6' {
        return "1010000"
    } else if raw_char == '7' {
        return "1000100"
    } else if raw_char == '8' {
        return "1001000"
    } else if raw_char == '9' {
        return "1110100"
    } else {
        return ""
    }

}

fn find_set_by_prefix(prefix:char, index:i32) -> char {
    if index == 0 ||  prefix == '0' {
        return 'A'
    } else {
        if prefix == '1' {
            if index ==1 || index == 3 {
                return 'A'
            } else {
                return 'B'
            }
             
        } else if prefix == '2' {
            if index ==1 || index == 4 {
                return 'A'
            } else {
                return 'B'
            }
        } else if prefix == '3' {
            if index ==1 || index == 5 {
                return 'A'
            } else {
                return 'B'
            }
        } else if prefix == '4' {
            if index ==2 || index == 3 {
                return 'A'
            } else {
                return 'B'
            }
        } else if prefix == '5' {
            if index ==3 || index == 4 {
                return 'A'
            } else {
                return 'B'
            }
        } else if prefix == '6' {
            if index ==4 || index == 5 {
                return 'A'
            } else {
                return 'B'
            }
        } else if prefix == '7' {
            if index ==2 || index == 4 {
                return 'A'
            } else {
                return 'B'
            }
        } else if prefix == '8' {
            if index ==2 || index == 5 {
                return 'A'
            } else {
                return 'B'
            }
        } else if prefix == '9' {
            if index ==3 || index == 5 {
                return 'A'
            } else {
                return 'B'
            }
        } else {
            print!("error");
            return 'X'
        }
    }
}

fn main() -> std::io::Result<()> {
    println!("Enter an ean:");

    //let ean_test = "3666154117284";

    let mut ean_test =  String::new();
    
    io::stdin().read_line(&mut ean_test).expect("no ean given");

    if let Some('\n')=ean_test.chars().next_back() {
        ean_test.pop();
    }
    if let Some('\r')=ean_test.chars().next_back() {
        ean_test.pop();
    }

    println!("test {}",ean_test);
    
    let result = ean_checker::is_correct_ean(ean_test.as_str());
    println!("{}",result);

    if result == true && ean_test.len() == 13 {
        let barcode_data = calculate_barcode_ean13(&ean_test);
        println!("{}",barcode_data);

        let svg_content =  svg_parser::get_svg_string(barcode_data);
        
        fs::write("test.svg", svg_content.as_bytes())?;
    }

    Ok(())

    
}
