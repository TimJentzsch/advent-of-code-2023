use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::map_res,
    multi::separated_list0,
    sequence::delimited,
    IResult, Parser,
};

use crate::types::{Draw, Game, Reveal};

fn parse_num(input: &str) -> IResult<&str, u32> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |input: &str| {
        input.parse::<u32>()
    })
    .parse(input)
}

fn parse_red_draw(input: &str) -> IResult<&str, Draw> {
    let (input, count) = parse_num(input)?;
    let (input, _) = tag(" red")(input)?;
    Ok((input, Draw::Red(count)))
}

fn parse_green_draw(input: &str) -> IResult<&str, Draw> {
    let (input, count) = parse_num(input)?;
    let (input, _) = tag(" green")(input)?;
    Ok((input, Draw::Green(count)))
}

fn parse_blue_draw(input: &str) -> IResult<&str, Draw> {
    let (input, count) = parse_num(input)?;
    let (input, _) = tag(" blue")(input)?;
    Ok((input, Draw::Blue(count)))
}

fn parse_draw(input: &str) -> IResult<&str, Draw> {
    alt((parse_red_draw, parse_green_draw, parse_blue_draw)).parse(input)
}

fn parse_reveal(input: &str) -> IResult<&str, Reveal> {
    let (input, draws) = separated_list0(tag(", "), parse_draw).parse(input)?;

    Ok((input, Reveal { draws }))
}

pub fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = delimited(tag("Game "), parse_num, tag(": "))(input)?;
    let (input, reveals) = separated_list0(tag("; "), parse_reveal).parse(input)?;

    Ok((input, Game { id, reveals }))
}

pub fn parse_full_game(input: &str) -> Game {
    let (rest, game) = parse_game(input).unwrap();
    assert_eq!(rest, "");
    game
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn test_parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let res = parse_game(input);

        assert_eq!(
            res,
            Ok((
                "",
                Game {
                    id: 1,
                    reveals: vec![
                        Reveal {
                            draws: vec![Draw::Blue(3), Draw::Red(4)]
                        },
                        Reveal {
                            draws: vec![Draw::Red(1), Draw::Green(2), Draw::Blue(6)]
                        },
                        Reveal {
                            draws: vec![Draw::Green(2)]
                        }
                    ]
                }
            ))
        )
    }
}
