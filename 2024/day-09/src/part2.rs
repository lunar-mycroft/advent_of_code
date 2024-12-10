use crate::Puzzle;

#[must_use]
pub fn process(mut puzzle: Puzzle) -> usize {
    let files = puzzle
        .chunks()
        .fold((Vec::new(), 0usize), |(mut v, total), chunk| {
            if let Some(id) = chunk[0] {
                v.push((total, (id, chunk.len())));
            }
            (v, total + chunk.len())
        })
        .0;
    let mut free_spaces = puzzle
        .chunks()
        .fold((Vec::new(), 0usize), |(mut v, total), chunk| {
            if chunk[0].is_none() {
                v.push((total, chunk.len()));
            }
            (v, total + chunk.len())
        })
        .0;
    for (file_pos, (_, file_size)) in files.into_iter().rev() {
        for (space_pos, space_size) in &mut free_spaces {
            if *space_pos >= file_pos {
                break;
            }
            if *space_size >= file_size && (*space_pos < file_pos) {
                puzzle.swap_chunks(file_pos, *space_pos, file_size);
                *space_size -= file_size;
                *space_pos += file_size;
                break;
            }
        }
    }
    puzzle.checksum()
}

#[must_use]
pub fn process_heap(mut puzzle: Puzzle) -> usize {
    let files = puzzle
        .chunks()
        .fold((Vec::new(), 0usize), |(mut v, total), chunk| {
            if let Some(id) = chunk[0] {
                v.push((total, (id, chunk.len())));
            }
            (v, total + chunk.len())
        })
        .0;
    let mut space_heaps = puzzle
        .chunks()
        .fold((spaces_heap::new(), 0usize), |(mut s, total), chunk| {
            if chunk[0].is_none() {
                spaces_heap::insert(&mut s, total, chunk.len());
            }
            (s, total + chunk.len())
        })
        .0;
    for (file_pos, (_, file_size)) in files.into_iter().rev() {
        match spaces_heap::left_index(&space_heaps, file_size) {
            Some((space_pos, space_size)) if space_pos < file_pos => {
                debug_assert!(space_pos < file_pos);
                debug_assert!(space_size >= file_size);
                spaces_heap::remove(&mut space_heaps, space_pos);
                spaces_heap::insert(
                    &mut space_heaps,
                    space_pos + file_size,
                    space_size - file_size,
                );

                puzzle.swap_chunks(file_pos, space_pos, file_size);
            }
            _ => (),
        }
    }
    puzzle.checksum()
}

#[must_use]
pub fn process_vecs(mut puzzle: Puzzle) -> usize {
    let files = puzzle
        .chunks()
        .fold((Vec::new(), 0usize), |(mut v, total), chunk| {
            if let Some(id) = chunk[0] {
                v.push((total, (id, chunk.len())));
            }
            (v, total + chunk.len())
        })
        .0;
    let mut spaces = puzzle
        .chunks()
        .fold((spaces_vecs::new(), 0usize), |(mut s, total), chunk| {
            if chunk[0].is_none() {
                spaces_vecs::insert(&mut s, total, chunk.len());
            }
            (s, total + chunk.len())
        })
        .0;
    for (file_pos, (_, file_size)) in files.into_iter().rev() {
        match spaces_vecs::left_index(&spaces, file_size) {
            Some((space_pos, space_size)) if space_pos < file_pos => {
                debug_assert!(space_pos < file_pos);
                debug_assert!(space_size >= file_size);
                spaces_vecs::remove(&mut spaces, space_pos);
                spaces_vecs::insert(&mut spaces, space_pos + file_size, space_size - file_size);

                puzzle.swap_chunks(file_pos, space_pos, file_size);
            }
            _ => (),
        }
    }
    puzzle.checksum()
}

#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
mod spaces_heap {
    use std::collections::BinaryHeap;

    use tap::prelude::*;

    pub type Spaces = [BinaryHeap<isize>; 9];

    pub const fn new() -> Spaces {
        [
            BinaryHeap::new(),
            BinaryHeap::new(),
            BinaryHeap::new(),
            BinaryHeap::new(),
            BinaryHeap::new(),
            BinaryHeap::new(),
            BinaryHeap::new(),
            BinaryHeap::new(),
            BinaryHeap::new(),
        ]
    }

    pub fn insert(spaces: &mut Spaces, pos: usize, len: usize) {
        for heap in spaces.iter_mut().take(len) {
            heap.push(-(pos as isize));
        }
    }

    pub fn left_index(spaces: &Spaces, len: usize) -> Option<(usize, usize)> {
        spaces[len - 1..]
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(idx, heap)| (len + idx, heap.peek()?).pipe(Some))
            .map(|(size, &i)| ((-i) as usize, size))
            .min_by_key(|(pos, _)| *pos)
    }

    pub fn remove(spaces: &mut Spaces, pos: usize) {
        for heap in &mut spaces[..] {
            heap.retain(|i| ((-i) as usize) != pos);
        }
    }
}

#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
mod spaces_vecs {
    use tap::prelude::*;

    pub type Spaces = [Vec<usize>; 9];

    pub const fn new() -> Spaces {
        [
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ]
    }

    pub fn insert(spaces: &mut Spaces, pos: usize, len: usize) {
        for v in spaces.iter_mut().take(len) {
            if let Err(idx) = v.binary_search(&pos) {
                v.insert(idx, pos);
            }
        }
    }

    pub fn left_index(spaces: &Spaces, len: usize) -> Option<(usize, usize)> {
        spaces[len - 1..]
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(idx, v)| (len + idx, v.first()?).pipe(Some))
            .map(|(size, &i)| (i, size))
            .min_by_key(|(pos, _)| *pos)
    }

    pub fn remove(spaces: &mut Spaces, pos: usize) {
        for v in &mut spaces[..] {
            if let Ok(idx) = v.binary_search(&pos) {
                v.remove(idx);
            }
        }
    }
}

impl Puzzle {
    fn chunks(&self) -> impl Iterator<Item = &[Option<usize>]> {
        self.ids.chunk_by(|a, b| *a == *b)
    }

    fn swap_chunks(&mut self, src: usize, dest: usize, len: usize) {
        for idx in 0..len {
            self.ids.swap(src + idx, dest + idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::Result;

    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = common::read_input!("example.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 2858);
        Ok(())
    }

    #[test]
    fn test_actual() -> Result<()> {
        let input = common::read_input!("part2.txt").parse()?;
        let output = process(input);
        assert_eq!(output, 6_221_662_795_602);
        Ok(())
    }

    // #[test]
    // fn test_heap() -> Result<()> {
    //     let input = common::read_input!("part2.txt").parse()?;
    //     let output = process_heap(input);
    //     assert_eq!(output, 6_221_662_795_602);
    //     Ok(())
    // }

    #[test]
    fn test_heap() -> Result<()> {
        let input = common::read_input!("part2.txt").parse()?;
        let output = process_vecs(input);
        assert_eq!(output, 6_221_662_795_602);
        Ok(())
    }
}
