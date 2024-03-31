
mod ean_checker;
mod barcode_data;
mod svg_parser;
mod png_writer;

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

        let svg_content =  svg_parser::get_svg_string(barcode_data.clone());
        
        fs::write("test.svg", svg_content.as_bytes())?;

        let mut f = std::fs::File::create("test.png").unwrap();

        let image_width = 95 *10;
        let image_height = 50;
        let mut img_data: Vec<u8> = Vec::new();
        let mut line_index = 0;

        while line_index < image_height {
            // ecriture d une ligne
            for char in barcode_data.chars() {
                if char == '1' {
                    for _n in 0..10 {
                        for _i in 0..4 {
                            img_data.push(
                                0x00
                            );
                        }
                        
                    }
                
                } else {
                    for _n in 0..10 {
                        for _i in 0..4 {
                            img_data.push(
                                0xff
                            );
                        }
                        
                    }
                
                }
    
            }

            line_index +=1
        }

        match png_writer::write(&mut f, &img_data, image_width, image_height) {
            Ok(_) => println!("Written image!"),
            Err(e) => println!("Error {:?}", e),
        }
    }

    Ok(())

    
}
