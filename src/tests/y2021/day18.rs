use crate::y2021::day18::Pair;

#[test]
fn test_nothing()  -> Result<(),String> {
    Ok(())
}

#[test]
fn test_parse() -> Result<(), String> {
    let tests = vec![
        ("[1,2]", 4),
        ("[[1,2],3]", 7),
        ("[9,[8,7]]", 7),
        ("[[1,9],[8,5]]", 10),
        ("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]", 25),
        ("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]", 31),
        ("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]", 46),
    ];
    for (input, len) in tests {
        let got: Pair = input.into();
        assert_eq!(got.elements.len(), len);
        assert_eq!(got.to_string(), input);
    }
    Ok(())
}

#[test]
fn test_explode() ->Result<(), String> {
    let tests = vec![
        ("[1,2]", None),
        ("[[[[[9,8],1],2],3],4]", Some("[[[[0,9],2],3],4]")),
        ("[7,[6,[5,[4,[3,2]]]]]", Some("[7,[6,[5,[7,0]]]]")),
        ("[[6,[5,[4,[3,2]]]],1]", Some("[[6,[5,[7,0]]],3]")),
        ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", Some("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")),
        ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", Some("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")),
    ];
    for (input, want) in tests {
        let p: Pair = input.into();
        let got = p.explode();
        match (want,got) {
            (None,Some(got)) => panic!("Wanted no change got {}", got),
            (Some(want), Some(got)) => assert_eq!(got.to_string(), want),
            (Some(want), None)=>panic!("Wanted {}, but got no change", want),
            _ => ()
        };
    }
    Ok(())
}

// fn test_split() ->Result<(), String> {
//     let tests = vec![
//         ("[1,2]", None),
//         ("[[[[0,7],4],[15,[0,13]]],[1,1]]", Some("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")),//Doesn't parse 15 correctly
//         ("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", Some("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")),
//         ("[[6,[5,[4,[3,2]]]],1]", Some("[[6,[5,[7,0]]],3]")), //incorrect
//     ];
//     for (input, want) in tests {
//         let p: Pair = input.into();
//         let got = p.split();
//         match (want,got) {
//             (None,Some(got)) => panic!("Wanted no change got {}", got),
//             (Some(want), Some(got)) => assert_eq!(got.to_string(), want),
//             (Some(want), None)=>panic!("Wanted {}, but got no change", want),
//             _ => ()
//         };
//     } 
//     Ok(())
// }

#[test]
fn test_reduce() -> Result<(), String> {
    let input: Pair = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".into();
    let got = input.reduce();
    assert_eq!(got.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

    Ok(())
}

#[test]
fn test_magnitude() -> Result<(), String> {
    let tests = [
    ("[[1,2],[[3,4],5]]" , 143),
    ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]" , 1384),
    ("[[[[1,1],[2,2]],[3,3]],[4,4]]" , 445),
    ("[[[[3,0],[5,3]],[4,4]],[5,5]]" , 791),
    ("[[[[5,0],[7,4]],[5,5]],[6,6]]" , 1137),
    ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]" , 3488),
    ];

    for (input, mag) in tests {
        let input: Pair = input.into();
        assert_eq!(input.magnitude(), mag)
    }
    Ok(())
}