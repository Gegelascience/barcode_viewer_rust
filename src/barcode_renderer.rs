use crate::svg_parser;
use crate::png_writer;
use std::fs;

pub fn save_barcode_as_svg(barcode_data: String)-> std::io::Result<()> {
    let svg_content =  svg_parser::get_svg_string(barcode_data);
        
    fs::write("test.svg", svg_content.as_bytes())?;

    Ok(())
}

pub fn save_barcode_as_png(barcode_data: String) {
    let mut f = std::fs::File::create("test.png").unwrap();

        let image_width = 95 *10;
        let image_height = 50;
        let mut img_data: Vec<u8> = Vec::new();
        let mut line_index = 0;

        while line_index < image_height {
            
            // entête ligne blanche
            if line_index == 0 {
                for _ in 0..10 {
                    for _ in barcode_data.chars() {
                        for _ in 0..40 {
                            img_data.push(
                                0xff
                            );
                        }
                        
                    }
                }
                
            }

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
        

        match png_writer::write(&mut f, &img_data, image_width, image_height+10) {
            Ok(_) => {
                println!("Written image!");
            },
            Err(e) => println!("Error {:?}", e),
        }
}