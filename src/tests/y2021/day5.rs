use crate::y2021::day5::{Line, Point, parse_lines};


#[test]
fn test_nothing()  -> Result<(),String> {
    let input = vec![
        "0,9 -> 5,9",
        "8,0 -> 0,8",
    ];
    let want = vec![
        Line{p1:Point{x:0,y:9}, p2:Point{x:5,y:9}},
        Line{p1:Point{x:8,y:0}, p2:Point{x:0,y:8}},
    ];
    assert_eq!(parse_lines(input), want);
    Ok(())
}

#[test]
fn test_line_iter() -> Result<(),String> {
    let inputs = vec![
        Line{p1:Point{x:0,y:1}, p2:Point{x:0,y:5}},
        Line{p1:Point{x:1,y:1}, p2:Point{x:-3,y:-3}},
        
    ];
    let wants = vec![
        vec![Point{x:0,y:1},Point{x:0,y:2},Point{x:0,y:3},Point{x:0,y:4},Point{x:0,y:5}],
        vec![Point{x:1,y:1},Point{x:0,y:0},Point{x:-1,y:-1},Point{x:-2,y:-2},Point{x:-3,y:-3}],
    ];
    for (input, want) in inputs.iter().zip(wants) {
        assert_eq!(input.iter().collect::<Vec<Point>>(), want);
    }
    Ok(())
}