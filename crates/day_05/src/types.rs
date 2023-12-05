pub const MAP_COUNT: usize = 7;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Almanac {
    pub seeds: Vec<ValueRange>,
    maps: [AlmanacMap; MAP_COUNT],
}

impl Almanac {
    pub fn new(seeds: Vec<ValueRange>, maps: [AlmanacMap; MAP_COUNT]) -> Self {
        Self { seeds, maps }
    }

    pub fn lowest_seed_location(&self) -> u32 {
        self.maps
            .iter()
            .fold(self.seeds.clone(), |acc, map| map.get_many(acc))
            .iter()
            .filter_map(|source| {
                if source.is_empty() {
                    None
                } else {
                    Some(source.start)
                }
            })
            .min()
            .unwrap()
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

    pub fn get_many(&self, sources: Vec<ValueRange>) -> Vec<ValueRange> {
        let (mut hits, mut misses) =
            self.entries
                .iter()
                .fold((Vec::new(), sources), |(mut hit_acc, miss_acc), entry| {
                    let (mut hits, misses) = entry.get_multiple(miss_acc);
                    hit_acc.append(&mut hits);
                    (hit_acc, misses)
                });

        hits.append(&mut misses);
        hits.into_iter().filter(|hit| !hit.is_empty()).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlmanacMapEntry {
    destination_start: u32,
    source_range: ValueRange,
}

impl AlmanacMapEntry {
    pub fn new(destination_start: u32, source_range: ValueRange) -> Self {
        Self {
            destination_start,
            source_range,
        }
    }

    pub fn get_multiple(&self, sources: Vec<ValueRange>) -> (Vec<ValueRange>, Vec<ValueRange>) {
        let res: Vec<_> = sources.iter().map(|source| self.get(source)).collect();
        res.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut hit_acc, mut miss_acc), (hit, mut misses)| {
                if let Some(hit) = hit {
                    hit_acc.push(hit);
                }

                miss_acc.append(&mut misses);

                (hit_acc, miss_acc)
            },
        )
    }

    pub fn get(&self, source: &ValueRange) -> (Option<ValueRange>, Vec<ValueRange>) {
        let source_intersection = source.intersection(&self.source_range);

        let hit = source_intersection.map(|int| {
            int.map(self.destination_start + (source.start.saturating_sub(self.source_range.start)))
        });
        let miss = if let Some(int) = source_intersection {
            source.minus(&int)
        } else {
            vec![*source]
        };

        (hit, miss)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ValueRange {
    start: u32,
    length: u32,
}

impl ValueRange {
    pub fn new(start: u32, length: u32) -> Self {
        Self { start, length }
    }

    pub fn single(start: u32) -> Self {
        Self { start, length: 1 }
    }

    pub fn from_end(start: u32, end_exclusive: u32) -> Self {
        Self {
            start,
            length: end_exclusive.saturating_sub(start),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn end_exclusive(&self) -> u32 {
        self.start.saturating_add(self.length)
    }

    pub fn intersection(&self, other: &ValueRange) -> Option<ValueRange> {
        let start = self.start.max(other.start);
        let end = self.end_exclusive().min(other.end_exclusive());

        if start >= end {
            None
        } else {
            Some(ValueRange {
                start,
                length: end - start,
            })
        }
    }

    pub fn is_disjunct_with(&self, other: &ValueRange) -> bool {
        self.end_exclusive() < other.start || other.end_exclusive() < self.start
    }

    pub fn union(&self, other: &ValueRange) -> Vec<ValueRange> {
        if self.is_empty() && other.is_empty() {
            Vec::new()
        } else if other.is_empty() {
            vec![*self]
        } else if self.is_disjunct_with(other) {
            // Can't be merged
            vec![*self, *other]
        } else {
            let start = self.start.min(other.start);
            let end = self.end_exclusive().max(other.end_exclusive());
            // Merge ranges
            vec![ValueRange::new(start, end - start)]
        }
    }

    pub fn minus(&self, other: &ValueRange) -> Vec<ValueRange> {
        if other.is_empty() {
            return vec![*self];
        }

        let first = ValueRange::from_end(self.start, self.end_exclusive().min(other.start));
        let second = ValueRange::from_end(
            self.end_exclusive().min(other.end_exclusive()),
            self.end_exclusive(),
        );

        first.union(&second)
    }

    pub fn map(&self, new_start: u32) -> ValueRange {
        ValueRange {
            start: new_start,
            length: self.length,
        }
    }
}

impl Ord for ValueRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for ValueRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_range_is_disjunct_with() {
        assert!(ValueRange::new(1, 2).is_disjunct_with(&ValueRange::new(4, 5)));
        assert!(!ValueRange::new(1, 2).is_disjunct_with(&ValueRange::new(3, 5)));
    }

    #[test]
    fn test_almanac_map_entry() {
        let entry = AlmanacMapEntry::new(50, ValueRange::new(98, 2));

        assert_eq!(
            entry.get(&ValueRange::single(97)),
            (None, vec![ValueRange::single(97)])
        );
        assert_eq!(
            entry.get(&ValueRange::single(98)),
            (Some(ValueRange::single(50)), Vec::new())
        );
        assert_eq!(
            entry.get(&ValueRange::single(99)),
            (Some(ValueRange::single(51)), Vec::new())
        );
        assert_eq!(
            entry.get(&ValueRange::single(100)),
            (None, vec![ValueRange::single(100)])
        );
    }

    #[test]
    fn test_almanac_map() {
        let map = AlmanacMap::new(vec![
            AlmanacMapEntry::new(50, ValueRange::new(98, 2)),
            AlmanacMapEntry::new(52, ValueRange::new(50, 48)),
        ]);

        assert_eq!(
            map.get_many(vec![ValueRange::single(0)]),
            vec![ValueRange::single(0)]
        );
        assert_eq!(
            map.get_many(vec![ValueRange::single(1)]),
            vec![ValueRange::single(1)]
        );
        assert_eq!(
            map.get_many(vec![ValueRange::single(48)]),
            vec![ValueRange::single(48)]
        );
        assert_eq!(
            map.get_many(vec![ValueRange::single(49)]),
            vec![ValueRange::single(49)]
        );
        assert_eq!(
            map.get_many(vec![ValueRange::single(50)]),
            vec![ValueRange::single(52)]
        );
        assert_eq!(
            map.get_many(vec![ValueRange::single(51)]),
            vec![ValueRange::single(53)]
        );
        assert_eq!(
            map.get_many(vec![ValueRange::single(96)]),
            vec![ValueRange::single(98)]
        );
        assert_eq!(
            map.get_many(vec![ValueRange::single(97)]),
            vec![ValueRange::single(99)]
        );
        assert_eq!(
            map.get_many(vec![ValueRange::single(98)]),
            vec![ValueRange::single(50)]
        );
        assert_eq!(
            map.get_many(vec![ValueRange::single(99)]),
            vec![ValueRange::single(51)]
        );
    }
}
