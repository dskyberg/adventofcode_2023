#[derive(Hash, Clone, Debug, Default, PartialEq)]
pub enum Direction {
    #[default]
    East,
    West,
    North,
    South,
}
