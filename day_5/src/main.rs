#[derive(Debug, Default)]
struct AlmanacEntry {
    seed: usize,
    soil: usize,
    fertilizer: usize,
    water: usize,
    light: usize,
    temerature: usize,
    humidity: usize,
    location: usize,
}

#[derive(Debug)]
struct MapEntry {
    destination: usize,
    source: usize,
    range: usize,
}

impl TryFrom<&str> for MapEntry {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(' ');
        let destination = parts
            .next()
            .ok_or("No desitionation".to_string())?
            .parse::<usize>()
            .map_err(|e| e.to_string())?;
        let source = parts
            .next()
            .ok_or("No source".to_string())?
            .parse::<usize>()
            .map_err(|e| e.to_string())?;
        let range = parts
            .next()
            .ok_or("No range".to_string())?
            .parse::<usize>()
            .map_err(|e| e.to_string())?;

        Ok(MapEntry {
            destination,
            source,
            range,
        })
    }
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: Vec<MapEntry>,
    soil_to_fertilizer: Vec<MapEntry>,
    fertilizer_to_water: Vec<MapEntry>,
    water_to_light: Vec<MapEntry>,
    light_to_temperature: Vec<MapEntry>,
    temperature_to_humidity: Vec<MapEntry>,
    humidity_to_location: Vec<MapEntry>,
}

fn part_one(almanac: &Almanac) -> Result<(), String> {
    println!("Part One:");
    Ok(())
}
fn part_two(_almanac: &Almanac) -> Result<(), String> {
    println!("Part Two:");
    Ok(())
}

fn main() -> Result<(), String> {
    let input = include_str!("../../data/day_5.txt");
    let almanac = parse_almanac(input)?;
    part_one(&almanac)?;
    part_two(&almanac)?;
    Ok(())
}

fn parse_almanac(input: &str) -> Result<Almanac, String> {
    let mut almanac = Almanac::default();
    let mut lines = input.split('\n').collect::<Vec<&str>>();

    let mut cursor = 0;

    let seeds_line = lines[cursor];
    let seeds_parts = seeds_line.split(':').collect::<Vec<&str>>();
    let seeds = seeds_parts[1].trim().split(' ').collect::<Vec<&str>>();
    almanac.seeds = seeds
        .iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    // Skip the empty line and title line
    cursor += 2;

    assert_eq!(lines[cursor], "seed-to-soil map:");
    cursor += 1;
    // Process the seed-to-soil map
    while !lines[cursor].is_empty() {
        cursor += 1
    }
    cursor += 1;
    assert_eq!(lines[cursor], "soil-to-fertilizer map:");
    cursor += 1;

    // Process soil-to-fertilizer
    while !lines[cursor].is_empty() {
        let entry = MapEntry::try_from(lines[cursor])?;
        almanac.soil_to_fertilizer.push(entry);
        cursor += 1
    }
    cursor += 1;
    assert_eq!(lines[cursor], "fertilizer-to-water map:");
    cursor += 1;

    // Process fertilizer-to-water
    while !lines[cursor].is_empty() {
        let entry = MapEntry::try_from(lines[cursor])?;
        almanac.fertilizer_to_water.push(entry);

        cursor += 1
    }
    cursor += 1;
    assert_eq!(lines[cursor], "water-to-light map:");
    cursor += 1;

    // Process water-to-light
    while !lines[cursor].is_empty() {
        let entry = MapEntry::try_from(lines[cursor])?;
        almanac.water_to_light.push(entry);

        cursor += 1
    }
    cursor += 1;
    assert_eq!(lines[cursor], "light-to-temperature map:");
    cursor += 1;

    // Process light-to-temperature
    while !lines[cursor].is_empty() {
        let entry = MapEntry::try_from(lines[cursor])?;
        almanac.light_to_temperature.push(entry);

        cursor += 1
    }
    cursor += 1;
    assert_eq!(lines[cursor], "temperature-to-humidity map:");
    cursor += 1;

    // Process temerature-to-humidity
    while !lines[cursor].is_empty() {
        let entry = MapEntry::try_from(lines[cursor])?;
        almanac.temperature_to_humidity.push(entry);
        cursor += 1
    }
    cursor += 1;
    assert_eq!(lines[cursor], "humidity-to-location map:");
    cursor += 1;

    // Process humidity-to-location
    while cursor < lines.len() {
        let entry = MapEntry::try_from(lines[cursor])?;
        almanac.humidity_to_location.push(entry);

        cursor += 1
    }

    println!("{:?}", &almanac);
    Ok(almanac)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let v = vec![10, 20, 30];
        let first = v.first().unwrap();
        println!("{}: {:?}", &first, &v);
    }
}
