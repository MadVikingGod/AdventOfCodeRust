use crate::y2020::day6::{convert, convert_everyone};

#[test]
fn test_convert() {
    let inputs = vec![
        r"abc",
        r"a
        b
        c",
        r"ab
        ac",
        r"a
        a
        a
        a",
        r"b",
    ];

    assert_eq!(convert(inputs[0]).len(), 3);
    assert_eq!(convert(inputs[1]).len(), 3);
    assert_eq!(convert(inputs[2]).len(), 3);
    assert_eq!(convert(inputs[3]).len(), 1);
    assert_eq!(convert(inputs[4]).len(), 1);
}

#[test]
fn test_convert_everyone() {
    let inputs = vec![
        r"abc",
        r"a
        b
        c",
        r"ab
        ac",
        r"a
        a
        a
        a",
        r"b",
    ];

    assert_eq!(convert_everyone(inputs[0]).len(), 3);
    assert_eq!(convert_everyone(inputs[1]).len(), 0);
    assert_eq!(convert_everyone(inputs[2]).len(), 1);
    assert_eq!(convert_everyone(inputs[3]).len(), 1);
    assert_eq!(convert_everyone(inputs[4]).len(), 1);
}
