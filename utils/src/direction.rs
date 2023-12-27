#[derive(Hash, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    #[default]
    East,
    West,
    North,
    South,
}
