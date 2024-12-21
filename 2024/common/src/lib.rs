pub mod grid;
pub mod min_heap;
pub mod position_map;

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
