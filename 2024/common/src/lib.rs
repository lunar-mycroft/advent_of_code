pub mod counter;
pub mod grid;
pub mod min_heap;
pub mod position_map;

/*
TODO:
- bitset (set of usizes where each item is stored as a bit)
- counter
- threading library
- parsing utils?
*/

#[macro_export]
macro_rules! read_input {
    ($filename:expr) => {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("inputs")
                .join($filename),
        )?
        .replace('\r', "")
    };
}
