use crate::y2020::day5::convert;

#[test]
fn test_convert() {
    assert_eq!(567, convert("BFFFBBFRRR"));
    assert_eq!(119, convert("FFFBBBFRRR"));
    assert_eq!(820, convert("BBFFBBFRLL"));
}
