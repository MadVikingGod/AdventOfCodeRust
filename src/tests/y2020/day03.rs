use crate::y2020::day03::{Map, Slope};

fn test_map() -> Map {
    Map::new(
        r###"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"###,
    )
}

#[test]
fn test_is_tree() {
    assert_eq!(test_map().is_tree(0, 0), Ok(false));
    assert_eq!(test_map().is_tree(0, 1), Ok(true));
}

#[test]
fn test_slope() {
    let mut slope = test_map().slope(3, 1);
    assert_eq!(slope.next().unwrap(), false);
    assert_eq!(slope.next().unwrap(), true);
    assert_eq!(slope.next().unwrap(), false);
    assert_eq!(slope.next().unwrap(), true);

    let slope = test_map().slope(3, 1);
    assert_eq!(slope.filter(|x| *x).count(), 7)
}

#[test]
fn test_slope_p2() {
    let map = test_map();
    let slopes: Vec<Slope> = vec![
        map.slope(1, 1),
        map.slope(3, 1),
        map.slope(5, 1),
        map.slope(7, 1),
        map.slope(1, 2),
    ];
    assert_eq!(
        slopes
            .iter()
            .map(|s| s.clone().filter(|x| *x).count())
            .product::<usize>(),
        336
    )
}
