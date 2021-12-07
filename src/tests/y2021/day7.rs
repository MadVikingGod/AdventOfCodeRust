use crate::y2021::day7;

#[test]
fn test_distance() -> Result<(), String> {
    let dist: i64 = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
        .iter()
        .map(|&pos| day7::distance(pos, 2))
        .sum();
    assert_eq!(dist, 37);
    Ok(())
}

#[test]
fn test_find_min_distance() -> Result<(), String> {
    assert_eq!(
        day7::find_min_distance(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]),
        (2, 37)
    );
    Ok(())
}

#[test]
fn test_find_min_advance_distance() -> Result<(), String> {
    assert_eq!(
        day7::find_min_advance_distance(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]),
        (5, 168)
    );
    Ok(())
}
