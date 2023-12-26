#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schematic {
    number_lines: Vec<Vec<Number>>,
    symbol_lines: Vec<Vec<Symbol>>,
}

impl Schematic {
    pub fn new(number_lines: Vec<Vec<Number>>, symbol_lines: Vec<Vec<Symbol>>) -> Self {
        Self {
            number_lines,
            symbol_lines,
        }
    }

    pub fn part_numbers(&self) -> Vec<Number> {
        let mut part_numbers = Vec::new();

        for (line, numbers) in self.number_lines.iter().enumerate() {
            for number in numbers {
                // Try to find an adjacent symbol for the number
                'symbol_search: for symbol_line in [line.saturating_sub(1), line, line + 1] {
                    let Some(symbols) = self.symbol_lines.get(symbol_line) else {
                        continue;
                    };

                    for symbol in symbols {
                        if number.is_horizontally_adjacent_to(symbol) {
                            part_numbers.push(number.clone());
                            break 'symbol_search;
                        }
                    }
                }
            }
        }

        part_numbers
    }

    pub fn gear_ratios(&self) -> u32 {
        let mut gear_ratio = 0;

        for (line, symbols) in self.symbol_lines.iter().enumerate() {
            for symbol in symbols {
                if symbol.text != '*' {
                    continue;
                }

                let mut part_numbers = Vec::new();

                // Try to find an adjacent numbers for the symbol
                for num_line in [line.saturating_sub(1), line, line + 1] {
                    let Some(numbers) = self.number_lines.get(num_line) else {
                        continue;
                    };

                    for number in numbers {
                        if number.is_horizontally_adjacent_to(symbol) {
                            part_numbers.push(number.clone());
                        }
                    }
                }

                if part_numbers.len() == 2 {
                    gear_ratio += part_numbers[0].value * part_numbers[1].value;
                }
            }
        }

        gear_ratio
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    pub text: String,
    pub value: u32,
    pub offset: usize,
}

impl Number {
    pub fn new(text: &str, offset: usize) -> Self {
        Self {
            value: text.parse::<u32>().expect("Invalid number"),
            text: text.to_string(),
            offset,
        }
    }

    fn is_horizontally_adjacent_to(&self, symbol: &Symbol) -> bool {
        symbol.offset + 1 >= self.offset && symbol.offset <= self.offset + self.text.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub text: char,
    pub offset: usize,
}

impl Symbol {
    pub fn new(text: char, offset: usize) -> Self {
        Self { text, offset }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_horizontally_adjacent_to_true() {
        assert!(Number::new("378", 1).is_horizontally_adjacent_to(&Symbol::new('*', 0)));
        assert!(Number::new("378", 1).is_horizontally_adjacent_to(&Symbol::new('*', 4)));
    }

    #[test]
    fn test_is_horizontally_adjacent_to_false() {
        assert!(!Number::new("378", 2).is_horizontally_adjacent_to(&Symbol::new('*', 0)));
        assert!(!Number::new("378", 1).is_horizontally_adjacent_to(&Symbol::new('*', 5)));
    }
}
