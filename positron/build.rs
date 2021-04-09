use positron_build::{
    Cargo,
    External
};

pub fn main() {
    if !External::is_valid() {
        panic!("External content is not present or valid. Run `e+ external fetch` to fix.")
    }
}
