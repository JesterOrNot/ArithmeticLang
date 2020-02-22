mod lex;
use crate::lex::Token;
fn main() {
    let tokens = &lex::lex(&String::from("5 55 3 32 432  34")).unwrap();
    let mut sum = 0;
    for i in tokens.iter() {
        match i {
            Token::Number(n) => {
                sum += n
            },
            _ => {}
        }
    }
    println!("{}", sum)
}
