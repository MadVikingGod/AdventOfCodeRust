use crate::y2020::day02::Rule;

#[test]
fn test_check_password() {
    assert_eq!(
        Rule::new("1", "3", "a").check_password("abcde".to_string()),
        true
    );
    assert_eq!(
        Rule::new("1", "3", "b").check_password("cdefg".to_string()),
        false
    );
    assert_eq!(
        Rule::new("2", "9", "c").check_password("ccccccccc".to_string()),
        true
    );
}

#[test]
fn test_check_toboggan_password() {
    assert_eq!(
        Rule::new("1", "3", "a").check_toboggan_password("abcde".to_string()),
        true
    );
    assert_eq!(
        Rule::new("1", "3", "b").check_toboggan_password("cdefg".to_string()),
        false
    );
    assert_eq!(
        Rule::new("2", "9", "c").check_toboggan_password("ccccccccc".to_string()),
        false
    );
}
