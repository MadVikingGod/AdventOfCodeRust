use crate::y2021::day6;
use std::collections::HashMap;

#[test]
fn test_iterator() -> Result<(), String> {
    let mut input = day6::School {
        fish: HashMap::from([(3, 2), (4, 1), (1, 1), (2, 1)]),
    };

    let mut got = input.next().unwrap();
    assert_eq!(got.fish, HashMap::from([(2, 2), (3, 1), (0, 1), (1, 1),]));
    got = input.next().unwrap();
    assert_eq!(
        got.fish,
        HashMap::from([(1, 2), (2, 1), (6, 1), (0, 1), (8, 1),])
    );
    Ok(())
}

#[test]
fn test_skip() -> Result<(), String> {
    let school = day6::parse_school("3,4,3,1,2");
    assert_eq!(school.clone().skip(79).next().unwrap().count_fish(), 5934);
    assert_eq!(school.skip(255).next().unwrap().count_fish(), 26984457539);
    Ok(())
}
