use std::ops::{Add, AddAssign, Mul, Sub};

use crate::Hex;

#[derive(PartialEq, Debug, Copy, Clone, Eq, Hash)]
pub struct CubeCoord {
    q: i32,
    r: i32,
    s: i32,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct FloatCubeCoord {
    q: f64,
    r: f64,
    s: f64,
}

const EPSILON_HEX: FloatCubeCoord = FloatCubeCoord {
    q: 1e-6,
    r: 2e-6,
    s: -3e-6,
};

pub struct HexLineDrawIter {
    start: FloatCubeCoord,
    end: FloatCubeCoord,
    distance: i32,
    t_unit: f64,
    next_i: i32,
}

impl HexLineDrawIter {
    fn new(start: CubeCoord, end: CubeCoord) -> Self {
        let distance = start.dist(end);
        Self {
            start: start.into(),
            end: FloatCubeCoord::from(end) + EPSILON_HEX,
            distance,
            t_unit: 1f64 / (distance as f64),
            next_i: 0,
        }
    }
}

impl Iterator for HexLineDrawIter {
    type Item = CubeCoord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_i > self.distance {
            return None;
        }

        let next_coord = CubeCoord::from(
            self.start
                .cube_lerp(self.end, (self.next_i as f64) * self.t_unit),
        );

        self.next_i += 1;

        Some(next_coord)
    }
}

pub struct HexRangeIter {
    center: CubeCoord,
    dist: i32,
    q: i32,
    r: i32,
}

impl HexRangeIter {
    fn new(center: CubeCoord, dist: i32) -> Self {
        Self {
            center,
            dist,
            q: -dist,
            r: 0,
        }
    }
}

impl Iterator for HexRangeIter {
    type Item = CubeCoord;

    fn next(&mut self) -> Option<Self::Item> {
        let r_max = self.dist.min(-self.q + self.dist);
        if self.q > self.dist {
            return None;
        }

        let new_coord = self.center + CubeCoord::new(self.q, self.r, -self.q - self.r);

        self.r += 1;
        if self.r > r_max {
            self.q += 1;
            self.r = (-self.dist).max(-self.q - self.dist);
        }

        Some(new_coord)
    }
}

pub struct HexRingPathIter {
    center: CubeCoord,
    radius: i32,

    current_position: CubeCoord,

    direction_index: usize,
    radius_index: i32,

    /// Special-case flag for raidus=0
    emitted_self: bool,
}

impl HexRingPathIter {
    fn new(center: CubeCoord, radius: i32) -> Self {
        Self {
            center,
            radius,
            current_position: center + (NEIGHBOR_DIRS[4] * radius),
            direction_index: 0,
            radius_index: 0,
            emitted_self: false,
        }
    }
}

impl Iterator for HexRingPathIter {
    type Item = CubeCoord;

    fn next(&mut self) -> Option<Self::Item> {
        // Special case for radius = 0 - only emit self
        if self.radius == 0 {
            if self.emitted_self {
                return None;
            } else {
                self.emitted_self = true;
                return Some(self.center);
            }
        }

        if self.direction_index >= 6 {
            return None;
        }

        let next_hex = self.current_position;

        self.current_position += NEIGHBOR_DIRS[self.direction_index];

        self.radius_index += 1;
        if self.radius_index >= self.radius {
            self.radius_index = 0;
            self.direction_index += 1;
        }

        Some(next_hex)
    }
}

pub struct HexSpiralPathIter {
    center: CubeCoord,
    current_radius: i32,
    max_radius: i32,
    current_iterator: HexRingPathIter,
}

impl HexSpiralPathIter {
    fn new(center: CubeCoord, radius: i32) -> Self {
        Self {
            center,
            max_radius: radius,
            current_radius: 0,
            current_iterator: HexRingPathIter::new(center, 0),
        }
    }
}

impl Iterator for HexSpiralPathIter {
    type Item = CubeCoord;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.current_iterator.next() {
            return Some(n);
        }
        self.current_radius += 1;
        if self.current_radius > self.max_radius {
            return None;
        }
        self.current_iterator = HexRingPathIter::new(self.center, self.current_radius);

        self.current_iterator.next()
    }
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

impl AddAssign for CubeCoord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
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

impl Mul<i32> for CubeCoord {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            q: self.q * rhs,
            r: self.r * rhs,
            s: self.s * rhs,
        }
    }
}
impl Mul<CubeCoord> for i32 {
    type Output = CubeCoord;

    fn mul(self, rhs: CubeCoord) -> Self::Output {
        rhs * self
    }
}

impl Add for FloatCubeCoord {
    type Output = FloatCubeCoord;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
            s: self.s + rhs.s,
        }
    }
}

impl Sub for FloatCubeCoord {
    type Output = FloatCubeCoord;

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
    fn new(q: i32, r: i32, s: i32) -> Self {
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

    pub fn linedraw(&self, other: CubeCoord) -> HexLineDrawIter {
        HexLineDrawIter::new(*self, other)
    }

    pub fn range(&self, dist: i32) -> HexRangeIter {
        HexRangeIter::new(*self, dist)
    }

    pub fn ring(&self, radius: i32) -> HexRingPathIter {
        HexRingPathIter::new(*self, radius)
    }

    pub fn spiral(&self, radius: i32) -> HexSpiralPathIter {
        HexSpiralPathIter::new(*self, radius)
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

impl FloatCubeCoord {
    fn cube_lerp(&self, other: Self, t: f64) -> Self {
        Self {
            q: lerp(self.q, other.q, t),
            r: lerp(self.r, other.r, t),
            s: lerp(self.s, other.s, t),
        }
    }
}

impl From<FloatCubeCoord> for CubeCoord {
    /// Round a FloatCubeCoord to a CubeCoord
    fn from(value: FloatCubeCoord) -> Self {
        let mut q = value.q.round_ties_even() as i32;
        let mut r = value.r.round_ties_even() as i32;
        let mut s = value.s.round_ties_even() as i32;

        let q_diff = (q as f64 - value.q).abs();
        let r_diff = (r as f64 - value.r).abs();
        let s_diff = (s as f64 - value.s).abs();

        if (q_diff > r_diff) && (q_diff > s_diff) {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q - s;
        } else {
            s = -q - r;
        }

        CubeCoord::new(q, r, s)
    }
}

impl From<CubeCoord> for FloatCubeCoord {
    fn from(value: CubeCoord) -> Self {
        Self {
            q: value.q as f64,
            r: value.r as f64,
            s: value.s as f64,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use rstest::*;

    #[rstest]
    #[case(CubeCoord{q: 1, r: 2, s: -3}, CubeCoord{q: 1, r: 2, s: -3}, CubeCoord{q: 2, r: 4, s: -6} )]
    #[case(CubeCoord{q: 0, r: 0, s: 0}, CubeCoord{q: 0, r: 0, s: 0}, CubeCoord{q: 0, r: 0, s: 0} )]
    #[case(CubeCoord{q: 1, r: 2, s: -3}, CubeCoord{q: -1, r: -2, s: 3}, CubeCoord{q: 0, r: 0, s: 0} )]
    fn test_add(#[case] left: CubeCoord, #[case] right: CubeCoord, #[case] expected: CubeCoord) {
        assert_eq!(left + right, expected);
    }

    #[rstest]
    #[case(CubeCoord{q: 1, r: 2, s: -3}, CubeCoord{q: 1, r: 2, s: -3}, CubeCoord{q: 2, r: 4, s: -6} )]
    #[case(CubeCoord{q: 0, r: 0, s: 0}, CubeCoord{q: 0, r: 0, s: 0}, CubeCoord{q: 0, r: 0, s: 0} )]
    #[case(CubeCoord{q: 1, r: 2, s: -3}, CubeCoord{q: -1, r: -2, s: 3}, CubeCoord{q: 0, r: 0, s: 0} )]
    fn test_add_assign(
        #[case] left: CubeCoord,
        #[case] right: CubeCoord,
        #[case] expected: CubeCoord,
    ) {
        let mut result = left.clone();
        result += right;
        assert_eq!(result, expected)
    }

    #[rstest]
    #[case(CubeCoord{q: 1, r: 2, s: -3}, CubeCoord{q: 1, r: 2, s: -3}, CubeCoord{q: 0, r: 0, s: 0} )]
    #[case(CubeCoord{q: 0, r: 0, s: 0}, CubeCoord{q: 0, r: 0, s: 0}, CubeCoord{q: 0, r: 0, s: 0} )]
    #[case(CubeCoord{q: 1, r: 2, s: -3}, CubeCoord{q: -1, r: -2, s: 3}, CubeCoord{q: 2, r: 4, s: -6} )]
    fn test_subtract(
        #[case] left: CubeCoord,
        #[case] right: CubeCoord,
        #[case] expected: CubeCoord,
    ) {
        assert_eq!(left - right, expected);
    }

    #[rstest]
    #[case(CubeCoord{q: 1, r: 2, s: -3}, 0, CubeCoord{q: 0, r: 0, s: 0} )]
    #[case(CubeCoord{q: 0, r: 0, s: 0}, 100, CubeCoord{q: 0, r: 0, s: 0} )]
    #[case(CubeCoord{q: 1, r: 2, s: -3}, 2, CubeCoord{q: 2, r: 4, s: -6} )]
    fn test_multiply(#[case] left: CubeCoord, #[case] right: i32, #[case] expected: CubeCoord) {
        assert_eq!(left * right, expected);
        assert_eq!(right * left, expected);
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

    #[rstest]
    #[case(CubeCoord::new(0, 0, 0), CubeCoord::new(0, 0, 0), vec![CubeCoord::new(0, 0, 0)])]
    #[case(CubeCoord::new(-3, 0, 3), CubeCoord::new(3, -3, 0), vec![
        CubeCoord::new(-3, 0, 3),
        CubeCoord::new(-2, 0, 2),
        CubeCoord::new(-1, -1, 2),
        CubeCoord::new(0, -1, 1),
        CubeCoord::new(1, -2, 1),
        CubeCoord::new(2, -2, 0),
        CubeCoord::new(3, -3, 0),
    ])]
    #[case(CubeCoord::new(-3, 0, 3), CubeCoord::new(0, 0, 0), vec![
        CubeCoord::new(-3, 0, 3),
        CubeCoord::new(-2, 0, 2),
        CubeCoord::new(-1, 0, 1),
        CubeCoord::new(0, 0, 0),
    ])]
    fn test_linedraw(
        #[case] from: CubeCoord,
        #[case] to: CubeCoord,
        #[case] expected: Vec<CubeCoord>,
    ) {
        let line = from.linedraw(to).collect::<Vec<_>>();
        assert_eq!(line, expected);
    }

    #[rstest]
    #[case(CubeCoord::new(0, 0, 0))]
    #[case(CubeCoord::new(100, -5, -95))]
    fn test_range(#[case] center: CubeCoord) {
        let dist = 2;
        let expected = [
            CubeCoord::new(0, 0, 0),
            CubeCoord::new(-1, 1, 0),
            CubeCoord::new(0, 1, -1),
            CubeCoord::new(1, 0, -1),
            CubeCoord::new(1, -1, 0),
            CubeCoord::new(0, -1, 1),
            CubeCoord::new(-1, 0, 1),
            CubeCoord::new(-2, 2, 0),
            CubeCoord::new(-1, 2, -1),
            CubeCoord::new(0, 2, -2),
            CubeCoord::new(1, 1, -2),
            CubeCoord::new(2, 0, -2),
            CubeCoord::new(2, -1, -1),
            CubeCoord::new(2, -2, 0),
            CubeCoord::new(1, -2, 1),
            CubeCoord::new(0, -2, 2),
            CubeCoord::new(-1, -1, 2),
            CubeCoord::new(-2, 0, 2),
            CubeCoord::new(-2, 1, 1),
        ]
        .iter()
        .map(|&coord| coord + center)
        .collect::<HashSet<_>>();

        let range_vec = center.range(dist).collect::<Vec<_>>();
        assert_eq!(range_vec.len(), expected.len());

        let range = range_vec.into_iter().collect::<HashSet<_>>();
        assert_eq!(range, expected);
    }

    #[rstest]
    #[case(CubeCoord::new(0, 0, 0), 5)]
    #[case(CubeCoord::new(100, -5, -95), 5)]
    #[case(CubeCoord::new(100, -5, -95), 100)]
    /// A more dynamic test that the range function produces the same result as
    /// iteratively adding neighbors
    fn test_range_2(#[case] center: CubeCoord, #[case] dist: i32) {
        let mut expected = HashSet::from([center]);
        for _ in 0..dist {
            expected = expected
                .into_iter()
                .map(|t| t.neighbors())
                .flatten()
                .collect();
        }

        let range = center.range(dist).collect::<HashSet<_>>();
        assert_eq!(range, expected);
    }

    #[rstest]
    #[case(CubeCoord::new(0, 0, 0))]
    #[case(CubeCoord::new(100, -5, -95))]
    fn test_ring(#[case] center: CubeCoord) {
        let expected_0 = [CubeCoord::new(0, 0, 0)]
            .iter()
            .map(|&coord| coord + center)
            .collect::<HashSet<_>>();
        let expected_1 = [
            CubeCoord::new(1, -1, 0),
            CubeCoord::new(1, 0, -1),
            CubeCoord::new(-1, 1, 0),
            CubeCoord::new(-1, 0, 1),
            CubeCoord::new(0, -1, 1),
            CubeCoord::new(0, 1, -1),
        ]
        .iter()
        .map(|&coord| coord + center)
        .collect::<HashSet<_>>();
        let expected_2 = [
            CubeCoord::new(0, -2, 2),
            CubeCoord::new(1, -2, 1),
            CubeCoord::new(2, -2, 0),
            CubeCoord::new(-1, -1, 2),
            CubeCoord::new(2, -1, -1),
            CubeCoord::new(-2, 0, 2),
            CubeCoord::new(2, 0, -2),
            CubeCoord::new(-2, 1, 1),
            CubeCoord::new(1, 1, -2),
            CubeCoord::new(-2, 2, 0),
            CubeCoord::new(-1, 2, -1),
            CubeCoord::new(0, 2, -2),
        ]
        .iter()
        .map(|&coord| coord + center)
        .collect::<HashSet<_>>();

        let ring_vec_0 = center.ring(0).collect::<Vec<_>>();
        assert_eq!(ring_vec_0.len(), expected_0.len());
        let ring_0 = ring_vec_0.into_iter().collect::<HashSet<_>>();
        assert_eq!(ring_0, expected_0);

        let ring_vec_1 = center.ring(1).collect::<Vec<_>>();
        assert_eq!(ring_vec_1.len(), expected_1.len());
        let ring_1 = ring_vec_1.into_iter().collect::<HashSet<_>>();
        assert_eq!(ring_1, expected_1);

        let ring_vec_2 = center.ring(2).collect::<Vec<_>>();
        assert_eq!(ring_vec_2.len(), expected_2.len());
        let ring_2 = ring_vec_2.into_iter().collect::<HashSet<_>>();
        assert_eq!(ring_2, expected_2);
    }

    #[rstest]
    #[case(CubeCoord::new(0, 0, 0))]
    #[case(CubeCoord::new(100, -5, -95))]
    fn test_spiral(#[case] center: CubeCoord) {
        let expected_0 = [CubeCoord::new(0, 0, 0)]
            .iter()
            .map(|&coord| coord + center)
            .collect::<Vec<_>>();
        let expected_1 = [
            CubeCoord::new(0, 0, 0),
            CubeCoord::new(-1, 1, 0),
            CubeCoord::new(0, 1, -1),
            CubeCoord::new(1, 0, -1),
            CubeCoord::new(1, -1, 0),
            CubeCoord::new(0, -1, 1),
            CubeCoord::new(-1, 0, 1),
        ]
        .iter()
        .map(|&coord| coord + center)
        .collect::<Vec<_>>();
        let expected_2 = [
            CubeCoord::new(0, 0, 0),
            CubeCoord::new(-1, 1, 0),
            CubeCoord::new(0, 1, -1),
            CubeCoord::new(1, 0, -1),
            CubeCoord::new(1, -1, 0),
            CubeCoord::new(0, -1, 1),
            CubeCoord::new(-1, 0, 1),
            CubeCoord::new(-2, 2, 0),
            CubeCoord::new(-1, 2, -1),
            CubeCoord::new(0, 2, -2),
            CubeCoord::new(1, 1, -2),
            CubeCoord::new(2, 0, -2),
            CubeCoord::new(2, -1, -1),
            CubeCoord::new(2, -2, 0),
            CubeCoord::new(1, -2, 1),
            CubeCoord::new(0, -2, 2),
            CubeCoord::new(-1, -1, 2),
            CubeCoord::new(-2, 0, 2),
            CubeCoord::new(-2, 1, 1),
        ]
        .iter()
        .map(|&coord| coord + center)
        .collect::<Vec<_>>();

        let spiral_0 = center.spiral(0).collect::<Vec<_>>();
        assert_eq!(spiral_0, expected_0);

        let spiral_1 = center.spiral(1).collect::<Vec<_>>();
        assert_eq!(spiral_1, expected_1);

        let spiral_2 = center.spiral(2).collect::<Vec<_>>();
        assert_eq!(spiral_2, expected_2);
    }
}
