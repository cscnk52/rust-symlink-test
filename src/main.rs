use std::fs;

fn main() {
    println!("{}", fs::symlink_metadata("src/fixture/this_is_a_symlink_file.txt").unwrap().is_symlink())
}
