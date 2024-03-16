use wasm_bindgen::prelude::*;
mod ean_checker;
mod barcode_data;
use wasm_bindgen::JsCast;

extern crate js_sys;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn check_ean(possible_ean: &str) -> bool {
    ean_checker::is_correct_ean(possible_ean)
}

#[wasm_bindgen]
pub fn get_ean_13_barcode_data(possible_ean: &str) -> String {
    if ean_checker::is_correct_ean(possible_ean) {
        return barcode_data::calculate_barcode_ean13(possible_ean);
    }
    String::from("")
    
}


// webgl
#[wasm_bindgen]
pub fn draw_barcode(barcode_data: &str) -> Result<String, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("barcodeCanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let ctx = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    for index in 0..barcode_data.len() {
        //draw_line(ctx, [(25 + 5*index) as f64, 100.0], [(25 + 5*index) as f64, 350.0], Some("black"), Some(5.0));
        if barcode_data.chars().nth(index.try_into().unwrap()).unwrap() == '1' {
            ctx.set_stroke_style(&JsValue::from_str("black"));
            ctx.set_line_width(5.0);
            ctx.begin_path();
            ctx.move_to((25 + 5*index) as f64, 100.0);
            ctx.line_to((25 + 5*index) as f64, 350.0);
            ctx.stroke();
        }
        
    }
    
    Ok( String::from(barcode_data))

}