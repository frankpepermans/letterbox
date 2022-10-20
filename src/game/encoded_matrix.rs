use std::fs::File;
use std::io::{Read, Write};

use super::matrix::Matrix;
use super::node::Node;

#[derive(Debug)]
pub struct EncodedMatrix {
    pub cells: Vec<u8>,
    pub rows: usize,
    pub cols: usize,
}

impl EncodedMatrix {
    pub fn to_file(&self, file_name: &str) -> std::io::Result<()> {
        let mut data = vec![self.rows as u8, self.cols as u8];

        data.append(&mut self.cells.clone());

        let mut e = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());

        e.write_all(&data).expect("could not write to file");

        let data = e.finish().expect("could not zip bytes");

        let mut pos = 0;
        let mut buffer = File::create(file_name)?;

        while pos < data.len() {
            let bytes_written = buffer.write(&data[pos..])?;
            pos += bytes_written;
        }

        Ok(())
    }

    pub fn from_file(file_name: &str) -> Self {
        let raw = std::fs::read(file_name).expect("could not read from file");

        let mut z = flate2::read::ZlibDecoder::new(&raw[..]);
        let mut v: Vec<u8> = Vec::new();

        z.read_to_end(&mut v).expect("could not read to vector");

        let rows = v.remove(0);
        let cols = v.remove(0);

        Self {
            rows: rows as usize,
            cols: cols as usize,
            cells: v,
        }
    }
}

impl From<EncodedMatrix> for Matrix<Node> {
    fn from(encoded: EncodedMatrix) -> Self {
        Matrix {
            vec: encoded
                .cells
                .iter()
                .map(|it| it.to_owned().into())
                .collect(),
            rows: encoded.rows,
            cols: encoded.cols,
            entanglements: Vec::new(),
        }
    }
}

impl Into<EncodedMatrix> for Matrix<Node> {
    fn into(self) -> EncodedMatrix {
        EncodedMatrix {
            cells: self.vec.iter().map(|it| it.to_owned().into()).collect(),
            rows: self.rows,
            cols: self.cols,
        }
    }
}
