pub mod grid;
pub mod position_map;

#[macro_export]
macro_rules! read_input {
    ($filename:literal) => {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("inputs")
                .join($filename),
        )?
    };
}
