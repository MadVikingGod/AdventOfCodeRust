if [ -z $1 ]; then
    echo Please supply a day number
    exit 1
fi
day="$1"

if [ -z $2 ]; then
    year="2021"
else
    year="$2"
fi

bin_dir="src/bin/day$day-$year"
test_dir="src/tests/y$year"
lib_dir="src/y$year/day$day"

dirs="$bin_dir $test_dir $lib_dir"
mkdir -p $dirs

### Bins
touch $bin_dir/instructions.txt
grep -E "use advent_of_code::y$year::day$day::read_input;" $bin_dir/main.rs -q || cat >$bin_dir/main.rs <<EOF
use advent_of_code::y$year::day$day::*;
// Common tools
// use advent_of_code::util::Field;
// use advent_of_code::util::Point;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}
EOF

### Tests
grep -E "y$year" src/tests/mod.rs -q || echo "mod y$year;" >> src/tests/mod.rs
grep -E "day$day" $test_dir/mod.rs -q || echo "mod day$day;" >> $test_dir/mod.rs
grep -E "use crate::y$year::day$day;" $test_dir/day$day.rs -q || cat > $test_dir/day$day.rs <<EOF
use crate::y$year::day$day;

#[test]
fn test_nothing()  -> Result<(),String> {
    Ok(())
}
EOF

### Library
# Manually add `pub mod y$year;` ath the start of a new year
grep -E "y$year" src/lib.rs -q || echo "Need to add 'pub mod y$year;' to src/lib.rs"
grep -E "day$day" src/y$year/mod.rs -q || echo "pub mod day$day;" >> src/y$year/mod.rs
touch $lib_dir/input.txt
grep -E "pub fn read_input()" $lib_dir/mod.rs -q || cat >$lib_dir/mod.rs <<EOF
// Common tools
// use crate::util::Field;
// use crate::util::Point;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::collections::VecDeque;
// use itertools::Itertools;

pub fn read_input() -> Vec<&'static str> {
    let input = include_str!("input.txt");
    input.lines().collect()
}
EOF