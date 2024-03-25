use scanner::Scanner;

mod scanner;
mod token_type;

fn main() {
    let mut scan = Scanner::new(
        "
    let a  = 10;
    let c = 10.10;
    let b = \"hello world\";
    println b;
",
    );

    match scan.scan_tokens() {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => println!("{:?}", e),
    }
}
