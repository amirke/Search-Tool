// memmap_line_reader.rs
// This file is used to read a file in chunks
// It is used to read the file in chunks and return the lines in a vector


use memmap2::Mmap;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

/// A structure to index lines in a file
pub struct LineIndex {
    mmap: Mmap,
    line_offsets: Vec<usize>,
}


// LineIndex is a structure that indexes lines in a file
// It is used to read the file in chunks and return the lines in a vector

impl LineIndex {
    /// Build line index from a file
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(&path)?;
        let mmap = unsafe { Mmap::map(&file)? };

        let mut line_offsets = vec![0]; // first line starts at 0
        for (i, &byte) in mmap.iter().enumerate() {
            if byte == b'\n' {
                line_offsets.push(i + 1); // next line starts after '\n'
            }
        }

        Ok(Self { mmap, line_offsets })
    }

    /// Return the total number of lines in the file
    pub fn line_count(&self) -> usize {
        self.line_offsets.len()
    }

    /// Get a chunk of lines from [start, start+count)
    pub fn get_lines(&self, start: usize, count: usize) -> Vec<String> {
        let end = (start + count).min(self.line_count());
        let mut result = Vec::with_capacity(count);

        for i in start..end {
            let start_byte = self.line_offsets[i];
            let end_byte = if i + 1 < self.line_offsets.len() {
                self.line_offsets[i + 1]
            } else {
                self.mmap.len()
            };

            let line = &self.mmap[start_byte..end_byte];
            // Remove possible trailing \n or \r\n
            let line_str = String::from_utf8_lossy(line).trim_end().to_string();
            result.push(line_str);
        }

        result
    }
}
