use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::none_of,
    combinator::map,
    multi::{many0, many1_count, separated_list0},
    IResult, Parser,
};

use crate::types::{Number, Schematic, Symbol};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Num(String),
    Sym(char),
    Period(usize),
}

fn parse_number(input: &str) -> IResult<&str, Token> {
    map(take_while1(|c: char| c.is_ascii_digit()), |text: &str| {
        Token::Num(text.to_string())
    })
    .parse(input)
}

fn parse_symbol(input: &str) -> IResult<&str, Token> {
    map(none_of(".0123456789\n"), Token::Sym).parse(input)
}

fn parse_periods(input: &str) -> IResult<&str, Token> {
    map(many1_count(tag(".")), Token::Period).parse(input)
}

fn parse_token(input: &str) -> IResult<&str, Token> {
    alt((parse_periods, parse_number, parse_symbol)).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, (Vec<Number>, Vec<Symbol>)> {
    let (input, tokens) = many0(parse_token)(input)?;

    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let mut offset = 0;

    for token in tokens {
        match token {
            Token::Period(count) => offset += count,
            Token::Num(text) => {
                numbers.push(Number::new(&text, offset));
                offset += text.len();
            }
            Token::Sym(ch) => {
                symbols.push(Symbol::new(ch, offset));
                offset += 1;
            }
        }
    }

    Ok((input, (numbers, symbols)))
}

fn parse_schematic(input: &str) -> IResult<&str, Schematic> {
    let (input, lines) = separated_list0(tag("\n"), parse_line)(input)?;
    let (number_lines, symbol_lines) = lines.into_iter().unzip();
    Ok((input, Schematic::new(number_lines, symbol_lines)))
}

pub fn parse_full_schematic(input: &str) -> Schematic {
    let (input, schematic) = parse_schematic(input).unwrap();
    assert!(input.is_empty());
    schematic
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    use super::parse_schematic;

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("467"), Ok(("", Token::Num("467".to_string()))));
    }

    #[test]
    fn test_parse_symbol() {
        assert_eq!(parse_symbol("*"), Ok(("", Token::Sym('*'))));
    }

    #[test]
    fn test_parse_periods() {
        assert_eq!(parse_periods("....."), Ok(("", Token::Period(5))));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("617*......"),
            Ok((
                "",
                (vec![Number::new("617", 0),], vec![Symbol::new('*', 3)])
            ))
        )
    }

    #[test]
    fn test_parse_token() {
        println!("{:?}", parse_token("\n"));
        assert!(parse_token("\n").is_err());
    }

    #[test]
    fn test_parse_schematic() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let res = parse_schematic(input);

        assert_eq!(
            res,
            Ok((
                "",
                Schematic::new(
                    vec![
                        vec![Number::new("467", 0), Number::new("114", 5)],
                        vec![],
                        vec![Number::new("35", 2), Number::new("633", 6)],
                        vec![],
                        vec![Number::new("617", 0)],
                        vec![Number::new("58", 7)],
                        vec![Number::new("592", 2)],
                        vec![Number::new("755", 6)],
                        vec![],
                        vec![Number::new("664", 1), Number::new("598", 5)]
                    ],
                    vec![
                        vec![],
                        vec![Symbol::new('*', 3)],
                        vec![],
                        vec![Symbol::new('#', 6)],
                        vec![Symbol::new('*', 3)],
                        vec![Symbol::new('+', 5)],
                        vec![],
                        vec![],
                        vec![Symbol::new('$', 3), Symbol::new('*', 5)],
                        vec![]
                    ]
                )
            ))
        );
    }
}
