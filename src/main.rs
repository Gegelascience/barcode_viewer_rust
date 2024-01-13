
mod ean_checker;

fn main() {
    println!("Hello, world!");
    let result = ean_checker::is_correct_ean("3666154117284");
    println!("{}",result)
}
