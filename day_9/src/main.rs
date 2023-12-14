/// Thank you to https://github.com/timvisee/advent-of-code-2023/blob/master/day09b/src/main.rs
/// for the guidance on using Pascal's Triangle
use anyhow::Result;

fn pascal(size: usize) -> Vec<Vec<isize>> {
    let mut triangle: Vec<Vec<isize>> = vec![vec![1]];

    for i in 0..size {
        let mut next = vec![1isize];
        next.extend(triangle[i].windows(2).map(|w| w[0] + w[1]).chain([1]));
        triangle.push(next);
    }
    (0..=size)
        .flat_map(|row| (0..=row).step_by(2).map(move |col| (row, col)))
        .for_each(|(row, col)| triangle[row][col] *= -1);

    triangle
}

fn reduce_one(nums: &[Vec<isize>], triangle: &[Vec<isize>]) -> isize {
    nums.iter()
        .map(|nums| {
            let row = nums.len();
            nums.iter()
                .enumerate()
                .map(|(col, val)| triangle[row][col] * val)
                .sum::<isize>()
                * if row % 2 == 0 { 1 } else { -1 }
        })
        .sum()
}

fn reduce_two(nums: &[Vec<isize>], triangle: &[Vec<isize>]) -> isize {
    let mut answer: isize = 0;
    for nums in nums {
        let row = nums.len();
        answer += nums
            .iter()
            .enumerate()
            .map(|(col, n)| triangle[row][col + 1] * n)
            .sum::<isize>();
    }
    answer
}

fn part_one(input: &str) -> Result<()> {
    let nums = parse_input(input)?;
    let start = std::time::Instant::now();
    let triangle = pascal(nums[0].len());
    let result = reduce_one(&nums, &triangle);
    println!("Part One: {} -- {:?}", result, start.elapsed());

    Ok(())
}

fn part_two(input: &str) -> Result<()> {
    let nums = parse_input(input)?;
    let start = std::time::Instant::now();
    let triangle = pascal(nums[0].len());
    let result = reduce_two(&nums, &triangle);
    println!("Part Two: {} -- {:?}", result, start.elapsed());

    Ok(())
}

fn main() -> Result<()> {
    let input = include_str!("../../data/day_9.txt");
    let _matrix = parse_input(input)?;

    part_one(input)?;
    part_two(input)?;

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<Vec<isize>>> {
    let mut result: Vec<Vec<isize>> = Vec::new();
    for line in input.lines() {
        let values = line
            .split(' ')
            .map(|s| s.parse::<isize>().map_err(|e| e.into()))
            .collect::<Result<Vec<isize>>>()?;
        result.push(values);
    }

    Ok(result)
}
