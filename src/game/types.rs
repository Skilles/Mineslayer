#[derive(Clone, Copy)]
pub(crate) enum CellType {
    Mine,
    VisitedMine,
    Goal,
    Empty,
    None,
}

pub(crate) enum MoveType {
    Up,
    Down,
    Left,
    Right,
}
