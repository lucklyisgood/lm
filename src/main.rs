use lm::{parser::Parser, scanner::Scanner};

fn main() {
    let mut scan = Scanner::new(
        "
    10 * 10 - 1
",
    );

    match scan.scan_tokens() {
        Ok(tokens) => {
            for token in &tokens {
                println!("{:?}", token);
            }

            let mut parser = Parser::new(tokens);
            println!("{:?}", parser.parse());
        }
        Err(e) => println!("{:?}", e),
    }
}
