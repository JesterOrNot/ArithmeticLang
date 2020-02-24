use lex::Token::CloseParenth;
use lex::Token::Number;
use lex::Token::OpenParenth;
use lex::Token::Operator;
use lex::Token::Whitespace;

#[test]
fn test_lex_1() {
    println!("Test lex()");
    assert_eq!(
        lex::lex(&String::from("   ")).unwrap(),
        vec![
            lex::Token::Whitespace(' '),
            lex::Token::Whitespace(' '),
            lex::Token::Whitespace(' ')
        ]
    );
}

#[test]
fn test_lex_2() {
    assert_eq!(
        lex::lex(&String::from("535+5444  - 6 (4)")).unwrap(),
        vec![
            Number(535),
            Operator('+'),
            Number(5444),
            Whitespace(' '),
            Whitespace(' '),
            Operator('-'),
            Whitespace(' '),
            Number(6),
            Whitespace(' '),
            OpenParenth('('),
            Number(4),
            CloseParenth(')')
        ]
    )
}

