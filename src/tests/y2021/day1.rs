#[test]
fn test_count_increase() -> Result<(), String> {
    let input: Vec<i64> = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
        .iter()
        .cloned()
        .collect();
    assert_eq!(crate::y2021::day1::count_increase(&input), 7);
    Ok(())
}

#[test]
fn test_count_window_increase() -> Result<(), String> {
    let input: Vec<i64> = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
        .iter()
        .cloned()
        .collect();
    assert_eq!(crate::y2021::day1::count_window_increase(&input, 3), 5);
    Ok(())
}
