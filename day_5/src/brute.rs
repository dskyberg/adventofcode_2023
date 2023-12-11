use anyhow::Result;
#[derive(Debug)]
struct MapEntry {
    destination: usize,
    source: usize,
    range: usize,
}

impl std::fmt::Display for MapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", &self.destination, &self.source, &self.range)
    }
}
impl MapEntry {
    fn map_value(&self, value: usize) -> usize {
        if value >= self.source && value <= self.source + self.range {
            return self.destination + (value - self.source);
        }
        value
    }
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
struct AlmanacEntry {
    seed: usize,
    soil: usize,
    fertilizer: usize,
    water: usize,
    light: usize,
    temperature: usize,
    humidity: usize,
    location: usize,
}

impl std::fmt::Display for AlmanacEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Seed {}, soil {}, fertilizer {}, water {}, light {}, temerature {}, humidity {}, location {}",
            self.seed,
            self.soil,
            self.fertilizer,
            self.water,
            self.light,
            self.temperature,
            self.humidity,
            self.location
        )
    }
}

fn map_maps(value: usize, entries: &Vec<MapEntry>) -> usize {
    for entry in entries {
        let tmp_value = entry.map_value(value);
        if tmp_value != value {
            return tmp_value;
        }
    }
    value
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

impl std::fmt::Display for Almanac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("seeds: {}\n", nums_to_str(&self.seeds)));

        result.push_str("\nseed-to-soil\n");
        for entry in &self.seed_to_soil {
            result.push_str(&format!("{}\n", entry));
        }

        result.push_str("\nsoil-to-fertilizer\n");
        for entry in &self.soil_to_fertilizer {
            result.push_str(&format!("{}\n", entry));
        }

        result.push_str("\nfertilizer-to-water\n");
        for entry in &self.fertilizer_to_water {
            result.push_str(&format!("{}\n", entry));
        }

        result.push_str("\nwater-to-light\n");
        for entry in &self.water_to_light {
            result.push_str(&format!("{}\n", entry));
        }

        result.push_str("\nlight-to-temperature\n");
        for entry in &self.light_to_temperature {
            result.push_str(&format!("{}\n", entry));
        }

        result.push_str("\ntemperature-to-humidity\n");
        for entry in &self.temperature_to_humidity {
            result.push_str(&format!("{}\n", entry));
        }

        result.push_str("\nhumidity-to-location\n");
        for entry in &self.humidity_to_location {
            result.push_str(&format!("{}\n", entry));
        }

        write!(f, "{}", result)
    }
}
impl Almanac {
    fn entry(&self, seed: usize) -> AlmanacEntry {
        let soil = map_maps(seed, &self.seed_to_soil);
        let fertilizer = map_maps(soil, &self.soil_to_fertilizer);
        let water = map_maps(fertilizer, &self.fertilizer_to_water);
        let light = map_maps(water, &self.water_to_light);
        let temperature = map_maps(light, &self.light_to_temperature);
        let humidity = map_maps(temperature, &self.temperature_to_humidity);
        let location = map_maps(humidity, &self.humidity_to_location);

        AlmanacEntry {
            seed,
            soil,
            fertilizer,
            water,
            light,
            temperature,
            humidity,
            location,
        }
    }

    fn entries(&self) -> Vec<AlmanacEntry> {
        let mut entries: Vec<AlmanacEntry> = Vec::new();

        for seed in &self.seeds {
            entries.push(self.entry(*seed))
        }
        entries
    }
}

pub fn part_one(input: &str) -> Result<()> {
    let almanac = parse_almanac(input)?;

    let entries = almanac.entries();
    let mut min_loc = entries[0].location;
    for entry in &entries {
        min_loc = std::cmp::min(min_loc, entry.location);
    }
    println!("Part One: {}", min_loc);
    Ok(())
}
pub fn part_two(_input: &str) -> Result<()> {
    println!("Part Two:");
    Ok(())
}

fn nums_to_str(v: &[usize]) -> String {
    let nums = v.iter().map(|v| v.to_string()).collect::<Vec<String>>();
    nums.join(", ")
}

fn parse_almanac(input: &str) -> Result<Almanac> {
    let mut almanac = Almanac::default();
    let lines = input.split('\n').collect::<Vec<&str>>();

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
        let entry = MapEntry::try_from(lines[cursor])?;
        almanac.seed_to_soil.push(entry);
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

    Ok(almanac)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maps() {
        let maps = vec![
            MapEntry {
                destination: 50,
                source: 98,
                range: 2,
            },
            MapEntry {
                destination: 52,
                source: 50,
                range: 48,
            },
        ];

        assert_eq!(0, map_maps(0, &maps));
        assert_eq!(52, map_maps(50, &maps));
        assert_eq!(53, map_maps(51, &maps));
        assert_eq!(50, map_maps(98, &maps));
    }
}
