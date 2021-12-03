use crate::y2021::day3;

#[test]
fn test_calculat_gama()  -> Result<(),String> {
    let input = vec![
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
    ];
    assert_eq!(day3::calculat_gama(input), "10110");
    Ok(())
}

#[test]
fn test_calculate_epsilon()  -> Result<(),String> {
    let input ="10110";
    assert_eq!(day3::calculate_epsilon(input.to_string()), "10110");
    Ok(())
}

#[test]
fn test_calculate_oxygen()  -> Result<(),String> {
    let input = vec![
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
    ];
    assert_eq!(day3::calculate_oxygen(input), "10111");
    Ok(())
}

#[test]
fn test_calculate_co2()  -> Result<(),String> {
    let input = vec![
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
    ];
    assert_eq!(day3::calculate_co2(input), "01010");
    Ok(())
}