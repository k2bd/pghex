use std::ops::{Add, Sub};

use crate::Hex;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct CubeCoord {
    q: i32,
    r: i32,
    s: i32,
}

impl From<Hex> for CubeCoord {
    fn from(value: Hex) -> Self {
        Self {
            q: value.q,
            r: value.r,
            s: -value.q - value.r,
        }
    }
}

impl From<CubeCoord> for Hex {
    fn from(value: CubeCoord) -> Self {
        Hex {
            q: value.q,
            r: value.r,
        }
    }
}

impl Add for CubeCoord {
    type Output = CubeCoord;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
            s: self.s + rhs.s,
        }
    }
}

impl Sub for CubeCoord {
    type Output = CubeCoord;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
            s: self.s - rhs.s,
        }
    }
}

const NEIGHBOR_DIRS: [CubeCoord; 6] = [
    CubeCoord { q: 1, r: 0, s: -1 },
    CubeCoord { q: 1, r: -1, s: 0 },
    CubeCoord { q: 0, r: -1, s: 1 },
    CubeCoord { q: -1, r: 0, s: 1 },
    CubeCoord { q: -1, r: 1, s: 0 },
    CubeCoord { q: 0, r: 1, s: -1 },
];

const DIAGONAL_DIRS: [CubeCoord; 6] = [
    CubeCoord { q: 2, r: -1, s: -1 },
    CubeCoord { q: 1, r: -2, s: 1 },
    CubeCoord { q: -1, r: -1, s: 2 },
    CubeCoord { q: -2, r: 1, s: 1 },
    CubeCoord { q: -1, r: 2, s: -1 },
    CubeCoord { q: 1, r: 1, s: -2 },
];

impl CubeCoord {
    pub fn new(q: i32, r: i32, s: i32) -> Self {
        Self { q, r, s }
    }

    pub fn neighbors(&self) -> Vec<CubeCoord> {
        NEIGHBOR_DIRS.iter().map(|&d| *self + d).collect()
    }

    pub fn diagonals(&self) -> Vec<CubeCoord> {
        DIAGONAL_DIRS.iter().map(|&d| *self + d).collect()
    }

    /// Get the hex distance to the origin
    pub fn abs(&self) -> i32 {
        [self.q.abs(), self.r.abs(), self.s.abs()]
            .into_iter()
            .max()
            .unwrap_or_default()
    }

    /// Get the hex distance to the other hex
    pub fn dist(&self, other: CubeCoord) -> i32 {
        (*self - other).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(CubeCoord{q: 1, r: 2, s: -3}, CubeCoord{q: 1, r: 2, s: -3},CubeCoord{q: 2, r: 4, s: -6} )]
    #[case(CubeCoord{q: 0, r: 0, s: 0}, CubeCoord{q: 0, r: 0, s: 0},CubeCoord{q: 0, r: 0, s: 0} )]
    #[case(CubeCoord{q: 1, r: 2, s: -3}, CubeCoord{q: -1, r: -2, s: 3},CubeCoord{q: 0, r: 0, s: 0} )]
    fn test_add(#[case] left: CubeCoord, #[case] right: CubeCoord, #[case] expected: CubeCoord) {
        assert_eq!(left + right, expected);
    }

    #[rstest]
    #[case(CubeCoord{q: 1, r: 2, s: -3}, CubeCoord{q: 1, r: 2, s: -3},CubeCoord{q: 0, r: 0, s: 0} )]
    #[case(CubeCoord{q: 0, r: 0, s: 0}, CubeCoord{q: 0, r: 0, s: 0},CubeCoord{q: 0, r: 0, s: 0} )]
    #[case(CubeCoord{q: 1, r: 2, s: -3}, CubeCoord{q: -1, r: -2, s: 3},CubeCoord{q: 2, r: 4, s: -6} )]
    fn test_subtract(
        #[case] left: CubeCoord,
        #[case] right: CubeCoord,
        #[case] expected: CubeCoord,
    ) {
        assert_eq!(left - right, expected);
    }

    #[rstest]
    fn test_neighbors() {
        assert_eq!(
            CubeCoord { q: 1, r: 2, s: -3 }.neighbors(),
            vec![
                CubeCoord { q: 2, r: 2, s: -4 },
                CubeCoord { q: 2, r: 1, s: -3 },
                CubeCoord { q: 1, r: 1, s: -2 },
                CubeCoord { q: 0, r: 2, s: -2 },
                CubeCoord { q: 0, r: 3, s: -3 },
                CubeCoord { q: 1, r: 3, s: -4 },
            ]
        )
    }

    #[rstest]
    fn test_diagonals() {
        assert_eq!(
            CubeCoord { q: 1, r: 2, s: -3 }.diagonals(),
            vec![
                CubeCoord { q: 3, r: 1, s: -4 },
                CubeCoord { q: 2, r: 0, s: -2 },
                CubeCoord { q: 0, r: 1, s: -1 },
                CubeCoord { q: -1, r: 3, s: -2 },
                CubeCoord { q: 0, r: 4, s: -4 },
                CubeCoord { q: 2, r: 3, s: -5 },
            ]
        )
    }

    #[rstest]
    #[case(CubeCoord::new(0, 0, 0), 0)]
    #[case(CubeCoord::new(1, -1, 0), 1)]
    #[case(CubeCoord::new(1, -3, 2), 3)]
    fn test_abs(#[case] hex: CubeCoord, #[case] expected: i32) {
        assert_eq!(hex.abs(), expected)
    }

    #[rstest]
    #[case(CubeCoord::new(0, 0, 0), CubeCoord::new(0, 0, 0), 0)]
    #[case(CubeCoord::new(1, -1, 0), CubeCoord::new(2, -1, 1), 1)]
    #[case(CubeCoord::new(1, -3, 2), CubeCoord::new(2, 2, -4), 6)]
    fn test_distance(#[case] left: CubeCoord, #[case] right: CubeCoord, #[case] expected: i32) {
        assert_eq!(left.dist(right), expected);
        assert_eq!(right.dist(left), expected);
    }
}
