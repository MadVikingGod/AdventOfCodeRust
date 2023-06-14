if [ -z $1 ]; then
    echo Please supply a day number
    exit 1
fi
day="$(printf '%02d' $1)"

if [ -z $2 ]; then
    year="2022"
else
    year="$2"
fi

year_dir="y$year"
echo ${year_dir}
bin_dir="${year_dir}/src/bin/day${day}"

mkdir -p $bin_dir
touch $bin_dir/instructions.txt
touch $bin_dir/input.txt
cat >$bin_dir/main.rs <<EOF
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
#[test]
fn test_nothing() {
    ()
}
EOF