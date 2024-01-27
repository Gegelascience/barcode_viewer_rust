
mod ean_checker;

use std::fs;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let result = ean_checker::is_correct_ean("3666154117284");
    println!("{}",result);

    fs::write("foo.txt", b"Lorem ipsum")?;
    Ok(())
}
