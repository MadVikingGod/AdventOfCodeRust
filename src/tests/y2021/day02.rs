#[test]
fn test_instruction_parse() -> Result<(), String> {
    let input = "forward 2";
    let got: crate::y2021::day02::Instruction = input.parse().unwrap();
    assert_eq!(
        got,
        crate::y2021::day02::Instruction {
            dir: crate::y2021::day02::Direction::Forward,
            size: 2,
        }
    );
    Ok(())
}

#[test]
fn test_calculate_position() -> Result<(), String> {
    use crate::y2021::day02::{Direction, Instruction};
    let input = vec![
        Instruction {
            dir: Direction::Forward,
            size: 5,
        },
        Instruction {
            dir: Direction::Down,
            size: 5,
        },
        Instruction {
            dir: Direction::Forward,
            size: 8,
        },
        Instruction {
            dir: Direction::Up,
            size: 3,
        },
        Instruction {
            dir: Direction::Down,
            size: 8,
        },
        Instruction {
            dir: Direction::Forward,
            size: 2,
        },
    ];
    assert_eq!(crate::y2021::day02::calculate_position(input), (10, 15));
    Ok(())
}

#[test]
fn test_calculate_advance_position() -> Result<(), String> {
    use crate::y2021::day02::{Direction, Instruction};
    let input = vec![
        Instruction {
            dir: Direction::Forward,
            size: 5,
        },
        Instruction {
            dir: Direction::Down,
            size: 5,
        },
        Instruction {
            dir: Direction::Forward,
            size: 8,
        },
        Instruction {
            dir: Direction::Up,
            size: 3,
        },
        Instruction {
            dir: Direction::Down,
            size: 8,
        },
        Instruction {
            dir: Direction::Forward,
            size: 2,
        },
    ];
    assert_eq!(
        crate::y2021::day02::calculate_advance_position(input),
        (60, 15)
    );
    Ok(())
}
