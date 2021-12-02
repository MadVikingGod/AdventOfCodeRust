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
echo >$bin_dir/main.rs <<EOF
use advent_of_code::y$year::day$day::read_input;

fn main() {
    println!("Hello, world!");
}
EOF

### Tests
grep -E "y$year" src/tests/mod.rs -q || echo "mod y$year;" >> src/tests/mod.rs
grep -E "day$day" $test_dir/mod.rs -q || echo "mod day$day;" >> $test_dir/mod.rs

### Library
# Manually add `pub mod y$year;` ath the start of a new year
grep -E "y$year" src/lib.rs -q || echo "Need to add 'pub mod y$year;' to src/lib.rs"
grep -E "day$day" src/y$year/mod.rs -q || echo "pub mod day$day;" >> src/y$year/mod.rs
touch $lib_dir/input.txt
echo >$lib_dir/mod.rs <<EOF
pub fn read_input() -> Map {
    let input = include_str!("input.txt");
    Map::new(input)
}
EOF