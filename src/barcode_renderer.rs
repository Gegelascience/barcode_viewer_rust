use crate::svg_parser;
use crate::png_writer;
use std::fs;

pub fn show_on_terminal(barcode_data: String) {
    for _ in 0..5 {
        let mut line: String = Default::default();
        for char in barcode_data.chars() {
            if char == '1' {
                line.push('#');
            } else {
                line.push(' ');
            }
        }
        println!("{}",line);
    }
}

pub fn save_barcode_as_svg(barcode_data: String)-> std::io::Result<()> {
    let svg_content =  svg_parser::get_svg_string(barcode_data);
        
    fs::write("test.svg", svg_content.as_bytes())?;

    Ok(())
}

pub fn save_barcode_as_png(barcode_data: String) {
    let mut f = std::fs::File::create("test.png").unwrap();

        let image_width = 95 *10;
        let size_header = 10;
        let image_height = 50 ;
        let mut img_data: Vec<u8> = Vec::new();
        let mut line_index = 0;

        for _ in 0..size_header {
            for _ in barcode_data.chars() {
                for _ in 0..40 {
                    img_data.push(
                        0xff
                    );
                }
                
            }
        }

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

        for _ in 0..size_header {
            for _ in barcode_data.chars() {
                for _ in 0..40 {
                    img_data.push(
                        0xff
                    );
                }
                
            }
        }
        
        match png_writer::write(&mut f, &img_data, image_width, image_height + 2*size_header) {
            Ok(_) => {
                println!("Written image!");
            },
            Err(e) => println!("Error {:?}", e),
        }
}