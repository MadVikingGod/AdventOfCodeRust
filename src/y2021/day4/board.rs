use array2d::Array2D;



#[derive(Debug, Clone, PartialEq)]
pub struct Spot {
    pub value: i64,
    pub seen: bool,

}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub arry: Array2D<Spot>,
}

pub fn from_list(list: Vec<&str>) -> Result<Board, &str> {
    if list.len() != 5 {
        return Err("Board is to big")
    }

    Ok(Board{
        arry: Array2D::from_rows(list.iter().map(|&line| {
            line.split_whitespace()
                .map(|value| value.trim_end_matches(","))
                .map(|value| Spot{value: value.parse().unwrap(), seen: false} )
                .collect()
            }).collect::<Vec<Vec<Spot>>>().as_slice())
    })
}

impl Board {
    pub fn is_winner(self: &Self) -> bool {
        self.arry.as_rows().iter().any(|row| row.iter().all(|spot| spot.seen)) ||
        self.arry.as_columns().iter()
            .any(|col| col.iter().all(|spot| spot.seen))

    }

    pub fn sum(self: &Self) -> i64 {
        self.arry.elements_row_major_iter().filter_map(|spot| {
            if !spot.seen {Some(spot.value)} else {None}
        }).sum()
    }

    pub fn mark(self: &Self, num: i64) -> Self {
        Self{arry: Array2D::from_row_major(
            self.arry.elements_row_major_iter()
                .map(|spot| {
                    let mut s = spot.clone();
                    if spot.value == num { s.seen = true;};
                    s
                })
                .collect::<Vec<Spot>>()
                .as_slice()
        ,5,5)}
    }

}