use pgrx::prelude::*;
use serde::{Deserialize, Serialize};

::pgrx::pg_module_magic!();

mod hex_alg;

#[derive(PartialEq, Debug, Copy, Clone, PostgresType, Serialize, Deserialize)]
//#[pgvarlena_inoutfuncs]
/// A hex position in cubic coordinates
struct Hex {
    q: i32,
    r: i32,
}

// TODO: Custom repr
// impl PgVarlenaInOutFuncs for Hex {
//     fn input(input: &core::ffi::CStr) -> PgVarlena<Self> {
//         let mut iter = input.to_str().unwrap().split(',');
//         let (q, r) = (iter.next(), iter.next());
//
//         let mut result = PgVarlena::<Self>::new();
//         result.q =
//             i32::from_str(q.unwrap().trim()).expect(&format!("q {:?} is not a valid i32", q));
//         result.r =
//             i32::from_str(r.unwrap().trim()).expect(&format!("r {:?} is not a valid i32", r));
//
//         result
//     }
//
//     fn output(&self, buffer: &mut pgrx::StringInfo) {
//         buffer.push_str(&format!("{},{}", self.q, self.r));
//     }
// }

// Operators

#[pg_operator]
#[opname(=)]
fn hex_eq(left: Hex, right: Hex) -> bool {
    left == right
}

#[pg_operator]
#[opname(+)]
fn hex_add(left: Hex, right: Hex) -> Hex {
    (hex_alg::CubeCoord::from(left) + hex_alg::CubeCoord::from(right)).into()
}

#[pg_operator]
#[opname(-)]
fn hex_sub(left: Hex, right: Hex) -> Hex {
    (hex_alg::CubeCoord::from(left) - hex_alg::CubeCoord::from(right)).into()
}

// Functions
#[pg_extern]
fn neighbors(coord: Hex) -> SetOfIterator<'static, Hex> {
    SetOfIterator::new(
        hex_alg::CubeCoord::from(coord)
            .neighbors()
            .into_iter()
            .map(|cube| cube.into()),
    )
}

#[pg_extern]
fn diagonals(coord: Hex) -> SetOfIterator<'static, Hex> {
    SetOfIterator::new(
        hex_alg::CubeCoord::from(coord)
            .diagonals()
            .into_iter()
            .map(|cube| cube.into()),
    )
}

#[pg_extern]
fn dist(coord: Hex, other: Hex) -> i32 {
    hex_alg::CubeCoord::from(coord).dist(hex_alg::CubeCoord::from(other))
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use crate::*;

    #[pg_test]
    fn test_create_hex() {
        let value = Spi::get_one::<Hex>("select '[1,2]'::hex").unwrap().unwrap();
        assert_eq!(value, Hex { q: 1, r: 2 });
    }

    #[pg_test]
    fn test_add_hex() {
        let value = Spi::get_one::<Hex>("select '[1,2]'::hex + '[3,4]'::hex")
            .unwrap()
            .unwrap();
        assert_eq!(value, Hex { q: 4, r: 6 });
    }

    #[pg_test]
    /// N.B. unfortunately at the moment I can only work out how to get the first result...
    fn test_neighbors() {
        let result = Spi::get_one::<Hex>("select neighbors('[1,2]'::hex)")
            .unwrap()
            .unwrap();
        assert_eq!(result, Hex { q: 2, r: 2 })
    }

    #[pg_test]
    /// N.B. unfortunately at the moment I can only work out how to get the first result...
    fn test_diagonals() {
        let result = Spi::get_one::<Hex>("select diagonals('[1,2]'::hex)")
            .unwrap()
            .unwrap();
        assert_eq!(result, Hex { q: 3, r: 1 })
    }

    #[pg_test]
    fn test_dist() {
        let result = Spi::get_one::<i32>("select dist('[1,2]'::hex, '[3,-4]'::hex)")
            .unwrap()
            .unwrap();
        assert_eq!(result, 6)
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
