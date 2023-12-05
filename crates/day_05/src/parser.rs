use nom::{
    bytes::complete::{tag, take_until, take_while1},
    combinator::map_res,
    multi::separated_list0,
    sequence::{preceded, terminated},
    IResult, Parser,
};

use crate::types::{Almanac, AlmanacMap, AlmanacMapEntry};

fn parse_num(input: &str) -> IResult<&str, u32> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |input: &str| {
        input.parse::<u32>()
    })
    .parse(input)
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(tag("seeds: "), separated_list0(tag(" "), parse_num)).parse(input)
}

fn parse_map_entry(input: &str) -> IResult<&str, AlmanacMapEntry> {
    let (input, destination_start) = terminated(parse_num, tag(" "))(input)?;
    let (input, source_start) = terminated(parse_num, tag(" "))(input)?;
    let (input, range) = parse_num(input)?;

    Ok((
        input,
        AlmanacMapEntry::new(destination_start, source_start, range),
    ))
}

fn parse_map(input: &str) -> IResult<&str, AlmanacMap> {
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = tag("\n")(input)?;

    let (input, entries) = separated_list0(tag("\n"), parse_map_entry)(input)?;
    Ok((input, AlmanacMap::new(entries)))
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = terminated(parse_seeds, tag("\n\n"))(input)?;
    let (input, raw_maps) = separated_list0(tag("\n\n"), parse_map)(input)?;
    let maps = raw_maps.try_into().expect("There must be exactly 7 maps");

    Ok((input, Almanac::new(seeds, maps)))
}

pub fn parse_full_almanac(input: &str) -> Almanac {
    let (input, almanac) = parse_almanac(input).expect("Failed to parse almanac");
    assert_eq!(input, "");
    almanac
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::{parse_almanac, parse_map, parse_map_entry, parse_seeds},
        types::{Almanac, AlmanacMap, AlmanacMapEntry},
    };

    #[test]
    fn test_parse_seeds() {
        let input = "seeds: 79 14 55 13";

        assert_eq!(parse_seeds(input), Ok(("", vec![79, 14, 55, 13])))
    }

    #[test]
    fn test_parse_map_entry() {
        let input = "49 53 8";

        assert_eq!(
            parse_map_entry(input),
            Ok(("", AlmanacMapEntry::new(49, 53, 8)))
        )
    }

    #[test]
    fn test_parse_map() {
        let input = "fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4";

        assert_eq!(
            parse_map(input),
            Ok((
                "",
                AlmanacMap::new(vec![
                    AlmanacMapEntry::new(49, 53, 8),
                    AlmanacMapEntry::new(0, 11, 42),
                    AlmanacMapEntry::new(42, 0, 7),
                    AlmanacMapEntry::new(57, 7, 4)
                ])
            ))
        )
    }

    #[test]
    fn test_parse_almanac() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(
            parse_almanac(input),
            Ok((
                "",
                Almanac::new(
                    vec![79, 14, 55, 13],
                    [
                        // seed-to-soil
                        AlmanacMap::new(vec![
                            AlmanacMapEntry::new(50, 98, 2),
                            AlmanacMapEntry::new(52, 50, 48),
                        ]),
                        // soil-to-fertilizer
                        AlmanacMap::new(vec![
                            AlmanacMapEntry::new(0, 15, 37),
                            AlmanacMapEntry::new(37, 52, 2),
                            AlmanacMapEntry::new(39, 0, 15),
                        ]),
                        // fertilizer-to-water
                        AlmanacMap::new(vec![
                            AlmanacMapEntry::new(49, 53, 8),
                            AlmanacMapEntry::new(0, 11, 42),
                            AlmanacMapEntry::new(42, 0, 7),
                            AlmanacMapEntry::new(57, 7, 4)
                        ]),
                        // water-to-light
                        AlmanacMap::new(vec![
                            AlmanacMapEntry::new(88, 18, 7),
                            AlmanacMapEntry::new(18, 25, 70),
                        ]),
                        // light-to-temperature
                        AlmanacMap::new(vec![
                            AlmanacMapEntry::new(45, 77, 23),
                            AlmanacMapEntry::new(81, 45, 19),
                            AlmanacMapEntry::new(68, 64, 13),
                        ]),
                        // temperature-to-humidity
                        AlmanacMap::new(vec![
                            AlmanacMapEntry::new(0, 69, 1),
                            AlmanacMapEntry::new(1, 0, 69),
                        ]),
                        // humidity-to-location
                        AlmanacMap::new(vec![
                            AlmanacMapEntry::new(60, 56, 37),
                            AlmanacMapEntry::new(56, 93, 4),
                        ])
                    ]
                )
            ))
        )
    }
}
