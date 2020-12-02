

use std::collections::HashSet;

#[test]
fn test_find_pair() -> Result<(),String> {
    let input: HashSet<i64> = [1721, 979, 366, 299, 675, 1456].iter().cloned().collect();
    assert_eq!(crate::y2020::day1::find_pair(2020, &input), Some(514579));
    Ok(())
}

#[test]
fn test_find_tripple() {
    let input: HashSet<i64> = [1721, 979, 366, 299, 675, 1456].iter().cloned().collect();
    assert_eq!(crate::y2020::day1::find_tripple(&input), Some(241861950));
}