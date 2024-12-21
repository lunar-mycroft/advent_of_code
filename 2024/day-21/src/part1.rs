use std::collections::HashMap;

use glam::IVec2;

pub use crate::initial::part1 as process;

pub(crate) fn routes<'a>(path: &'a str, pad: &'a HashMap<char, IVec2>) -> String {
    let mut start = 'A';
    path.chars()
        .filter_map(move |end| {
            let old = start;
            start = end;
            crate::initial::step(old, end, pad)
        })
        .collect()
}
