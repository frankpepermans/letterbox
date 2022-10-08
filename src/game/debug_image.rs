use raster::{Color, Image};

use crate::Matrix;

use super::node::Node;

pub trait DebugImage {
    fn debug_image(&self, file_path: &str, path: Vec<(usize, usize)>);
}

impl DebugImage for Matrix<Node> {
    fn debug_image(&self, file_path: &str, path: Vec<(usize, usize)>) {
        let multiplier = (800 / (self.cols - 1)) as i32;
        let w = self.cols as i32 * multiplier + self.cols as i32;
        let h = self.rows as i32 * multiplier + self.rows as i32;
        let mut canvas = Image::blank(w, h);

        println!("output size = ({}, {}) [multiplier {}]", w, h, multiplier);

        for row in 0..self.rows {
            for col in 0..self.cols {
                let node = self[(row, col)].clone();
                let val: u8 = node.into();
                let color = match val {
                    0b1111 => Color::rgba(0x80, 0x80, 0x80, 0xff),
                    _ => Color::rgba(0xff, 0, 0, 0xff),
                };

                for r in 0..multiplier {
                    for s in 0..multiplier {
                        let x = (col as i32 + r) + (col as i32 * multiplier);
                        let y = (row as i32 + s) + (row as i32 * multiplier);

                        let _success = match canvas.set_pixel(x, y, color.clone()) {
                            Ok(_) => true,
                            Err(_e) => {
                                //println!("error! ({}, {})", x, y);
                                false
                            }
                        };
                    }
                }
            }
        }

        for index in path {
            let row = index.0 as i32;
            let col = index.1 as i32;
            let color = Color::rgba(0, 0xff, 0, 0xff);

            for r in 0..multiplier {
                for s in 0..multiplier {
                    let x = (col + r) + (col * multiplier);
                    let y = (row + s) + (row * multiplier);

                    canvas.set_pixel(x, y, color.clone()).unwrap();
                }
            }
        }

        raster::save(&canvas, file_path);
    }
}
