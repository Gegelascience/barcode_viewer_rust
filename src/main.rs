
mod ean_checker;
mod barcode_data;
mod svg_parser;

use std::{fs,io};


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
        let barcode_data = barcode_data::calculate_barcode_ean13(&ean_test);
        println!("{}",barcode_data);

        let svg_content =  svg_parser::get_svg_string(barcode_data);
        
        fs::write("test.svg", svg_content.as_bytes())?;
    }

    Ok(())

    
}
