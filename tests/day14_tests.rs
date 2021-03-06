extern crate aoc17;

use aoc17::day14::*;

#[test]
fn test_num_used_grids() {
    assert_eq!(num_used_grids("flqrgnkx"), 8108);
}

#[test]
fn test_num_used_regions() {
    assert_eq!(num_used_regions("flqrgnkx"), 1242);
}
