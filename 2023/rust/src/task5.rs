use rust::read_data;
use std::collections::HashSet;

#[allow(unused)]
const TEST_DATA: &str = "seeds: 79 14 55 13

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
56 93 4
";

pub fn run5_1()
{
    let data = read_data("data/data5.txt");
    // let data = TEST_DATA;
    let mut total = 0;
    let (maps, seeds) = parse_data(&data);

    let mut min_location = u64::MAX;
    for seed in seeds {
        let location = process_seed(&seed, &maps);
        min_location = std::cmp::min(min_location, location);
    }

    total = min_location;
    println!("Task 5_1 Solution:\nTotal: {}", total);
}

pub fn run5_2()
{
    let data = read_data("data/data5.txt");
    // let data = TEST_DATA;
    let mut total = 0;
    let (maps, seeds) = parse_data(&data);

    let mut min_location = u64::MAX;
    // work with ranges
    let mut seed_ranges = get_seed_ranges(&seeds);

    let mut i = 0;
    for mut map in maps {

        map.values.sort_by_key(|x| x.source);
        i += 1;
        if i == 10 {
            break;
        }
        seed_ranges = map_ranges(&seed_ranges, &map);
    }

    for seed_range in seed_ranges {
        min_location = std::cmp::min(min_location, seed_range.from);
    }

    total = min_location;

    println!("Task 5_2 Solution:\nTotal: {}", total);
}

#[derive(Debug)]
struct Range {
    from: u64,
    to: u64,
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    values: Vec<Value>,
}

#[derive(Debug)]
struct Value {
    destination: u64,
    source: u64,
    range: u64,
}

fn parse_data(data: &str) -> (Vec<Map>, Vec<u64>) {

    let maps = parse_maps(data);
    let seeds = parse_seeds(data);

    (maps, seeds)
}

fn parse_seeds(data: &str) -> Vec<u64> {
    let mut seeds = Vec::new();

    let line1 = data.lines().next().unwrap();
    for seed in line1.split(":").last().unwrap().split_whitespace() {
        seeds.push(seed.parse::<u64>().unwrap());
    }

    seeds
}

fn parse_maps(data: &str) -> Vec<Map> {
    let mut maps = Vec::new();

    for map in data.split("\n\n").skip(1) {
        maps.push(parse_map(map.trim()));
    }
        
    maps
}

fn parse_map(data: &str) -> Map {
    let mut map = Map {
        from: String::new(),
        to: String::new(),
        values: Vec::new(),
    };

    let line1 = data.lines().next().unwrap();
    map.from = line1.split("-to-").next().unwrap().to_string();
    map.to = line1.split("-to-").last().unwrap().split_whitespace()
        .next().unwrap().to_string();
    let mut values = Vec::new();
    for line in data.lines().skip(1) {
        values.push(parse_value(line));
    }
    map.values = values;
    map
}

fn parse_value (line: &str) -> Value {
    let mut value = Value {
        destination: 0,
        source: 0,
        range: 0,
    };

    value.destination = line.split_whitespace().nth(0).unwrap().parse::<u64>().unwrap();
    value.source = line.split_whitespace().nth(1).unwrap().parse::<u64>().unwrap();
    value.range = line.split_whitespace().nth(2).unwrap().parse::<u64>().unwrap();

    value
}


fn process_seed(seed: &u64, maps: &Vec<Map>) -> u64 {

    let mut value = seed.clone();

    for map in maps {
        value = process_map(value, map);
    }

    value
    }

fn process_map(value: u64, map: &Map) -> u64 {

    for mapping in &map.values {
        if mapping.source <= value && value <= mapping.source + mapping.range {
            return mapping.destination + value - mapping.source;
        }
    }
    value
}

fn get_seed_ranges(seeds: &Vec<u64>) -> Vec<Range> {
    let mut ranges = Vec::new();

    for i in (0..seeds.len()).step_by(2) {
        let v = Range {
            from: seeds[i],
            to: seeds[i] + seeds[i + 1],
        };
        ranges.push(v);
    }

    ranges
}

fn map_ranges(seed_ranges: &Vec<Range>, map: &Map) -> Vec<Range> {
    let mut ranges = Vec::new();

    for seed_range in seed_ranges {
        ranges.extend(map_range(seed_range, &map));
    }

    ranges
}

fn map_range(range: &Range, map: &Map) -> Vec<Range> {

    let mut ranges = Vec::new();
    let mut start_range = range.from;
    let mut end_range = range.to;

    for map in &map.values {
        let start_map = map.source;
        let end_map = map.source + map.range;
        let dest = map.destination;
 
        if start_range < start_map {

            if end_range < start_map {
                // no intersection
                ranges.push(Range {
                    from: start_range,
                    to: end_range,
                });
                break;
            } else if end_range < end_map {
                // intersection
                ranges.push(Range {
                    from: start_range,
                    to: start_map - 1,
                });
                ranges.push(Range {
                    from: dest,
                    to: dest + end_range - start_map,
                });
                break;
            } else {
                // end_range > end_map
                ranges.push(Range {
                    from: start_range,
                    to: start_map - 1,
                });
                ranges.push(Range {
                    from: dest,
                    to: dest + end_map - start_map,
                });
                start_range = end_map;
            }

        } else if start_range < end_map {

            if end_range < end_map {
                // fully in map
                ranges.push(Range {
                    from: dest + start_range - start_map,
                    to: dest + end_range - start_map,
                });
                break;
            } else {
                // end_range > end_map
                ranges.push(Range {
                    from: dest + start_range - start_map,
                    to: dest + map.range,
                });
                start_range = end_map;
            }
        }
    }
    let last_map_end = map.values.last().unwrap().source +
        map.values.last().unwrap().range;

    // handle the case where the range is after the map
    if start_range > last_map_end {
        ranges.push(Range {
            from: start_range,
            to: end_range,
        });
    } else if end_range > last_map_end {
        ranges.push(Range {
            from: last_map_end + 1,
            to: end_range,
        });
    }
    ranges
}
