pub const MAP_COUNT: usize = 7;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Almanac {
    pub seeds: Vec<u32>,
    maps: [AlmanacMap; MAP_COUNT],
}

impl Almanac {
    pub fn new(seeds: Vec<u32>, maps: [AlmanacMap; MAP_COUNT]) -> Self {
        Self { seeds, maps }
    }

    pub fn seed_location(&self, seed: u32) -> u32 {
        self.maps.iter().fold(seed, |res, map| map.get(res))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlmanacMap {
    entries: Vec<AlmanacMapEntry>,
}

impl AlmanacMap {
    pub fn new(entries: Vec<AlmanacMapEntry>) -> Self {
        Self { entries }
    }

    pub fn get(&self, source: u32) -> u32 {
        self.entries
            .iter()
            .find_map(|entry| entry.get(source))
            .unwrap_or(source)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlmanacMapEntry {
    destination_start: u32,
    source_start: u32,
    range: u32,
}

impl AlmanacMapEntry {
    pub fn new(destination_start: u32, source_start: u32, range: u32) -> Self {
        Self {
            destination_start,
            source_start,
            range,
        }
    }

    pub fn get(&self, source: u32) -> Option<u32> {
        if source >= self.source_start && source < self.source_start.saturating_add(self.range) {
            Some(self.destination_start + (source - self.source_start))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_almanac_map_entry() {
        let entry = AlmanacMapEntry::new(50, 98, 2);

        assert_eq!(entry.get(97), None);
        assert_eq!(entry.get(98), Some(50));
        assert_eq!(entry.get(99), Some(51));
        assert_eq!(entry.get(100), None);
    }

    #[test]
    fn test_almanac_map() {
        let map = AlmanacMap::new(vec![
            AlmanacMapEntry::new(50, 98, 2),
            AlmanacMapEntry::new(52, 50, 48),
        ]);

        assert_eq!(map.get(0), 0);
        assert_eq!(map.get(1), 1);
        assert_eq!(map.get(48), 48);
        assert_eq!(map.get(49), 49);
        assert_eq!(map.get(50), 52);
        assert_eq!(map.get(51), 53);
        assert_eq!(map.get(96), 98);
        assert_eq!(map.get(97), 99);
        assert_eq!(map.get(98), 50);
        assert_eq!(map.get(99), 51);
    }
}
