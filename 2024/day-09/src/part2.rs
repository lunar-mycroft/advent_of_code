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
}
