use std::{ops::Range, str::FromStr};
use itertools::Itertools;

pub fn part01(input: &str) -> i64 {
    let res=part_one(input);
    res.map(|n| n as i64).unwrap()
}


pub fn part02(input: &str) -> i64 {
    let res=part_two(input);
    res.map(|n| n as i64).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let almanac = input.parse::<Almanac>().expect("parse error");
    almanac
        .seeds
        .iter()
        .map(|&seed| almanac.lookup_location(seed) as u32)
        .min()
}

pub fn part_two(input: &str) -> Option<u32> {
    let almanac = input.parse::<Almanac>().expect("parse error");
    almanac
        .seeds
        .chunks_exact(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .filter_map(|r| almanac.min_location(r))
        .map(|l| l as u32)
        .min()
}


#[derive(Clone, Debug, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<u64>,
    maps: [AlmanacMap; 7],
}

impl Almanac {
    pub fn lookup_location(&self, seed: u64) -> u64 {
        self.maps.iter().fold(seed, |value, map| map.map(value))
    }

    pub fn min_location(&self, input: Range<u64>) -> Option<u64> {
        let mut ranges = vec![input];
        let mut next_ranges = vec![];
        for mapping in &self.maps {
            for range in ranges.drain(..) {
                next_ranges.extend(mapping.map_range(range));
            }
            std::mem::swap(&mut ranges, &mut next_ranges);
        }
        ranges.into_iter().filter_map(|r| r.min()).min()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AlmanacMap {
    ranges: Vec<RangeMapping>,
}

impl AlmanacMap {
    pub fn map(&self, input: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|mapping| mapping.map(input))
            .unwrap_or(input)
    }

    pub fn map_range(&self, input: Range<u64>) -> impl IntoIterator<Item = Range<u64>> + '_ {
        AlmanacRangeIterator {
            almanac: self,
            input,
            mapping_index: 0,
        }
    }
}

struct AlmanacRangeIterator<'a> {
    almanac: &'a AlmanacMap,
    input: Range<u64>,
    mapping_index: usize,
}

impl<'a> Iterator for AlmanacRangeIterator<'a> {
    type Item = Range<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.end <= self.input.start {
            // There is nothing left in the input, so we are done.
            return None;
        }
        while self.mapping_index < self.almanac.ranges.len() {
            let mapping = &self.almanac.ranges[self.mapping_index];

            // If any of the input precedes this mapping, then return that portion first as-is.
            if self.input.start < mapping.source_range.start {
                let start = self.input.start;
                let end = std::cmp::min(self.input.end, mapping.source_range.start);
                self.input.start = end;
                return Some(start..end);
            }

            // If any of the input overlaps the mapping, return the intersection with the offset
            // added.
            if self.input.start < mapping.source_range.end {
                let start = std::cmp::max(self.input.start, mapping.source_range.start);
                let end = std::cmp::min(self.input.end, mapping.source_range.end);
                self.input.start = end;
                self.mapping_index += 1;
                return Some(
                    (start as i64 + mapping.offset) as u64..(end as i64 + mapping.offset) as u64,
                );
            } else {
                self.mapping_index += 1;
            }
        }

        // If there is anything leftover in the input, return it as-is.
        if self.input.end > self.input.start {
            let result = Some(self.input.clone());
            self.input.start = self.input.end;
            return result;
        }

        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RangeMapping {
    source_range: Range<u64>,
    offset: i64,
}

impl RangeMapping {
    pub fn map(&self, input: u64) -> Option<u64> {
        if !self.source_range.contains(&input) {
            return None;
        }
        Some(((input as i64) + self.offset) as u64)
    }
}

#[derive(Debug)]
struct ParseAlmanacErr;

impl FromStr for Almanac {
    type Err = ParseAlmanacErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("\n\n").collect::<Vec<_>>();
        if parts.len() != 8 {
            return Err(ParseAlmanacErr);
        }
        let seeds = parts[0]
            .split_once(':')
            .ok_or(ParseAlmanacErr)?
            .1
            .split_whitespace()
            .map(|s| s.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseAlmanacErr)?;
        let maps = parts
            .into_iter()
            .skip(1)
            .filter_map(|s| {
                s.split_once(":\n")
                    .ok_or(ParseAlmanacErr)
                    .and_then(|(_, s)| s.parse::<AlmanacMap>())
                    .ok()
            })
            .collect_tuple::<(_, _, _, _, _, _, _)>()
            .ok_or(ParseAlmanacErr)?
            .into();
        Ok(Self { seeds, maps })
    }
}

impl FromStr for AlmanacMap {
    type Err = ParseAlmanacErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges = s
            .lines()
            .map(|line| line.parse::<RangeMapping>())
            .collect::<Result<Vec<_>, _>>()?;
        ranges.sort_by_key(|r| r.source_range.start);
        Ok(Self { ranges })
    }
}

impl FromStr for RangeMapping {
    type Err = ParseAlmanacErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: [u64; 3] = s
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect_tuple::<(_, _, _)>()
            .ok_or(ParseAlmanacErr)?
            .into();
        Ok(Self {
            source_range: parts[1]..(parts[1] + parts[2]),
            offset: (parts[0] as i64) - (parts[1] as i64),
        })
    }
}
