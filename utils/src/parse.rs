use std::str::FromStr;

use anyhow::{anyhow, Result};

pub fn parse_nums<T: FromStr>(input: &str, delim: char) -> Result<Vec<T>> {
    let mut result = vec![];
    let parts = input.split(delim);
    for part in parts {
        result.push(
            part.trim()
                .parse()
                .map_err(|_| anyhow!("Failed to parse T for {}", &part))?,
        )
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32() {
        let nums = "-10, -1, 0, 1, 10";
        let expected: Vec<i32> = vec![-10, -1, 0, 1, 10];

        let result = parse_nums::<i32>(nums, ',').expect("oops!");
        assert_eq!(&result, &expected);
    }
}
