use crate::y2021::day04;
use array2d::Array2D;
use day04::board::{Board, Spot};

#[test]
fn test_from_list() -> Result<(), String> {
    let input = vec![
        "22 13 17 11  0",
        "8  2 23  4 24",
        "21  9 14 16  7",
        "6 10  3 18  5",
        "1 12 20 15 19",
    ];
    let want = Board {
        arry: Array2D::from_rows(&[
            vec![
                Spot {
                    value: 22,
                    seen: false,
                },
                Spot {
                    value: 13,
                    seen: false,
                },
                Spot {
                    value: 17,
                    seen: false,
                },
                Spot {
                    value: 11,
                    seen: false,
                },
                Spot {
                    value: 0,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 8,
                    seen: false,
                },
                Spot {
                    value: 2,
                    seen: false,
                },
                Spot {
                    value: 23,
                    seen: false,
                },
                Spot {
                    value: 4,
                    seen: false,
                },
                Spot {
                    value: 24,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 21,
                    seen: false,
                },
                Spot {
                    value: 9,
                    seen: false,
                },
                Spot {
                    value: 14,
                    seen: false,
                },
                Spot {
                    value: 16,
                    seen: false,
                },
                Spot {
                    value: 7,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 6,
                    seen: false,
                },
                Spot {
                    value: 10,
                    seen: false,
                },
                Spot {
                    value: 3,
                    seen: false,
                },
                Spot {
                    value: 18,
                    seen: false,
                },
                Spot {
                    value: 5,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 1,
                    seen: false,
                },
                Spot {
                    value: 12,
                    seen: false,
                },
                Spot {
                    value: 20,
                    seen: false,
                },
                Spot {
                    value: 15,
                    seen: false,
                },
                Spot {
                    value: 19,
                    seen: false,
                },
            ],
        ]),
    };
    assert_eq!(day04::board::from_list(input).unwrap(), want);
    Ok(())
}

#[test]
fn test_board_is_not_winner_row() -> Result<(), String> {
    let input = Board {
        arry: Array2D::from_rows(&[
            vec![
                Spot {
                    value: 22,
                    seen: false,
                },
                Spot {
                    value: 13,
                    seen: false,
                },
                Spot {
                    value: 17,
                    seen: false,
                },
                Spot {
                    value: 11,
                    seen: false,
                },
                Spot {
                    value: 0,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 8,
                    seen: false,
                },
                Spot {
                    value: 2,
                    seen: false,
                },
                Spot {
                    value: 23,
                    seen: false,
                },
                Spot {
                    value: 4,
                    seen: false,
                },
                Spot {
                    value: 24,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 21,
                    seen: false,
                },
                Spot {
                    value: 9,
                    seen: false,
                },
                Spot {
                    value: 14,
                    seen: false,
                },
                Spot {
                    value: 16,
                    seen: false,
                },
                Spot {
                    value: 7,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 6,
                    seen: false,
                },
                Spot {
                    value: 10,
                    seen: false,
                },
                Spot {
                    value: 3,
                    seen: false,
                },
                Spot {
                    value: 18,
                    seen: false,
                },
                Spot {
                    value: 5,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 1,
                    seen: false,
                },
                Spot {
                    value: 12,
                    seen: false,
                },
                Spot {
                    value: 20,
                    seen: false,
                },
                Spot {
                    value: 15,
                    seen: false,
                },
                Spot {
                    value: 19,
                    seen: false,
                },
            ],
        ]),
    };
    assert_eq!(input.is_winner(), false);
    Ok(())
}
#[test]
fn test_board_is_winner_row() -> Result<(), String> {
    let input = Board {
        arry: Array2D::from_rows(&[
            vec![
                Spot {
                    value: 22,
                    seen: false,
                },
                Spot {
                    value: 13,
                    seen: false,
                },
                Spot {
                    value: 17,
                    seen: false,
                },
                Spot {
                    value: 11,
                    seen: false,
                },
                Spot {
                    value: 0,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 8,
                    seen: true,
                },
                Spot {
                    value: 2,
                    seen: true,
                },
                Spot {
                    value: 23,
                    seen: true,
                },
                Spot {
                    value: 4,
                    seen: true,
                },
                Spot {
                    value: 24,
                    seen: true,
                },
            ],
            vec![
                Spot {
                    value: 21,
                    seen: false,
                },
                Spot {
                    value: 9,
                    seen: false,
                },
                Spot {
                    value: 14,
                    seen: false,
                },
                Spot {
                    value: 16,
                    seen: false,
                },
                Spot {
                    value: 7,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 6,
                    seen: false,
                },
                Spot {
                    value: 10,
                    seen: false,
                },
                Spot {
                    value: 3,
                    seen: false,
                },
                Spot {
                    value: 18,
                    seen: false,
                },
                Spot {
                    value: 5,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 1,
                    seen: false,
                },
                Spot {
                    value: 12,
                    seen: false,
                },
                Spot {
                    value: 20,
                    seen: false,
                },
                Spot {
                    value: 15,
                    seen: false,
                },
                Spot {
                    value: 19,
                    seen: false,
                },
            ],
        ]),
    };
    assert_eq!(input.is_winner(), true);
    Ok(())
}
#[test]
fn test_board_is_winner_column() -> Result<(), String> {
    let input = Board {
        arry: Array2D::from_rows(&[
            vec![
                Spot {
                    value: 22,
                    seen: false,
                },
                Spot {
                    value: 13,
                    seen: false,
                },
                Spot {
                    value: 17,
                    seen: true,
                },
                Spot {
                    value: 11,
                    seen: false,
                },
                Spot {
                    value: 0,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 8,
                    seen: false,
                },
                Spot {
                    value: 2,
                    seen: false,
                },
                Spot {
                    value: 23,
                    seen: true,
                },
                Spot {
                    value: 4,
                    seen: false,
                },
                Spot {
                    value: 24,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 21,
                    seen: false,
                },
                Spot {
                    value: 9,
                    seen: false,
                },
                Spot {
                    value: 14,
                    seen: true,
                },
                Spot {
                    value: 16,
                    seen: false,
                },
                Spot {
                    value: 7,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 6,
                    seen: false,
                },
                Spot {
                    value: 10,
                    seen: false,
                },
                Spot {
                    value: 3,
                    seen: true,
                },
                Spot {
                    value: 18,
                    seen: false,
                },
                Spot {
                    value: 5,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 1,
                    seen: false,
                },
                Spot {
                    value: 12,
                    seen: false,
                },
                Spot {
                    value: 20,
                    seen: true,
                },
                Spot {
                    value: 15,
                    seen: false,
                },
                Spot {
                    value: 19,
                    seen: false,
                },
            ],
        ]),
    };
    assert_eq!(input.is_winner(), true);
    Ok(())
}

#[test]
fn test_board_mark() -> Result<(), String> {
    let input = Board {
        arry: Array2D::from_rows(&[
            vec![
                Spot {
                    value: 22,
                    seen: false,
                },
                Spot {
                    value: 13,
                    seen: false,
                },
                Spot {
                    value: 17,
                    seen: true,
                },
                Spot {
                    value: 11,
                    seen: false,
                },
                Spot {
                    value: 0,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 8,
                    seen: false,
                },
                Spot {
                    value: 2,
                    seen: false,
                },
                Spot {
                    value: 23,
                    seen: true,
                },
                Spot {
                    value: 4,
                    seen: false,
                },
                Spot {
                    value: 24,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 21,
                    seen: false,
                },
                Spot {
                    value: 9,
                    seen: false,
                },
                Spot {
                    value: 14,
                    seen: true,
                },
                Spot {
                    value: 16,
                    seen: false,
                },
                Spot {
                    value: 7,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 6,
                    seen: false,
                },
                Spot {
                    value: 10,
                    seen: false,
                },
                Spot {
                    value: 3,
                    seen: true,
                },
                Spot {
                    value: 18,
                    seen: false,
                },
                Spot {
                    value: 5,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 1,
                    seen: false,
                },
                Spot {
                    value: 12,
                    seen: false,
                },
                Spot {
                    value: 20,
                    seen: true,
                },
                Spot {
                    value: 15,
                    seen: false,
                },
                Spot {
                    value: 19,
                    seen: false,
                },
            ],
        ]),
    };
    let want = Board {
        arry: Array2D::from_rows(&[
            vec![
                Spot {
                    value: 22,
                    seen: false,
                },
                Spot {
                    value: 13,
                    seen: false,
                },
                Spot {
                    value: 17,
                    seen: true,
                },
                Spot {
                    value: 11,
                    seen: false,
                },
                Spot {
                    value: 0,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 8,
                    seen: true,
                },
                Spot {
                    value: 2,
                    seen: false,
                },
                Spot {
                    value: 23,
                    seen: true,
                },
                Spot {
                    value: 4,
                    seen: false,
                },
                Spot {
                    value: 24,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 21,
                    seen: false,
                },
                Spot {
                    value: 9,
                    seen: false,
                },
                Spot {
                    value: 14,
                    seen: true,
                },
                Spot {
                    value: 16,
                    seen: false,
                },
                Spot {
                    value: 7,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 6,
                    seen: false,
                },
                Spot {
                    value: 10,
                    seen: false,
                },
                Spot {
                    value: 3,
                    seen: true,
                },
                Spot {
                    value: 18,
                    seen: false,
                },
                Spot {
                    value: 5,
                    seen: false,
                },
            ],
            vec![
                Spot {
                    value: 1,
                    seen: false,
                },
                Spot {
                    value: 12,
                    seen: false,
                },
                Spot {
                    value: 20,
                    seen: true,
                },
                Spot {
                    value: 15,
                    seen: false,
                },
                Spot {
                    value: 19,
                    seen: false,
                },
            ],
        ]),
    };
    assert_eq!(input.mark(8), want);
    Ok(())
}
