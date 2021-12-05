use advent_of_code::y2021::day4::read_input;

fn main() {
    println!("Hello, world!");

    let (num, mut board) = read_input();

    for i in num {
        board = board.iter().map(|b| b.mark(i)).collect();
        if board.iter().any(|b| {
            if  b.is_winner() {
                println!("Winner {}, {}",i, b.sum()*i);
                return true
            };
            false
        }) {break}
    }
    
    let (num, mut board) = read_input();

    for i in num {
        board = board.iter().map(|b| b.mark(i)).filter_map(|b| {
            if b.is_winner() {
                println!("Winner {}, {}",i, b.sum()*i);
                return None
            }
            Some(b)
        }).collect();
        
    }
}

// 44415 too high
