use lm::{interpreter::Interpreter, parser::Parser, scanner::Scanner};

fn main() {
    let mut scan = Scanner::new(
        "
    println 1 + 10 * 10 + 1;
",
    );

    match scan.scan_tokens() {
        Ok(tokens) => {
            // for token in &tokens {
            //     println!("{:?}", token);
            // }

            let mut parser = Parser::new(tokens);
            // println!("{:#?}", parser.parse());
            match &parser.parse() {
                Ok(stmts) => {
                    let mut interpreter = Interpreter::new();
                    let _ = interpreter.interpret(stmts.iter().collect());
                }
                Err(e) => println!("gen ast fail: {:?}", e),
            }
        }
        Err(e) => println!("gen token fail: {:?}", e),
    }
}
