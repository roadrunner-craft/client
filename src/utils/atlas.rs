use image::DynamicImage;
use std::collections::HashMap;
use std::path::Path;
use std::ptr;
use std::vec::Vec;

macro_rules! printerr {
    ($path:expr, $msg:expr) => {
        println!("<atlas> could not process image ({}): {}", $path, $msg)
    };
}

pub struct AtlasGenerator {}

impl AtlasGenerator {
    fn get_required_size(tile_count: usize) -> usize {
        let mut grid_size = 0;
        for i in 1..15 {
            grid_size = 2_usize.pow(i);
            if grid_size.pow(2) >= tile_count {
                break;
            }
        }

        grid_size
    }

    pub fn generate(files: HashMap<usize, &str>) -> (Vec<u8>, u32) {
        let tile_count = files.len();
        let tile_size: usize = 16;
        let grid_size: usize = AtlasGenerator::get_required_size(tile_count);
        let img_size = grid_size * tile_size;
        let buffer_size = img_size.pow(2) * 4;

        let mut img = Vec::new();
        img.resize(buffer_size, 255);

        for (index, filename) in files.iter() {
            if *index >= grid_size.pow(2) {
                printerr!(filename, format!("{} {}", "invalid index", index));
                continue;
            }

            let filepath = Path::new(filename).to_str().unwrap();

            match image::open(filepath) {
                Err(err) => {
                    printerr!(filepath, err);
                    continue;
                }
                Ok(current_img) => {
                    let current_img = match current_img {
                        DynamicImage::ImageRgba8(x) => x,
                        x => x.to_rgba(),
                    };

                    let width = current_img.width() as usize;
                    let height = current_img.height() as usize;

                    if width != tile_size || height != tile_size {
                        printerr!(filepath, "invalid size");
                        continue;
                    }

                    let grid_x = index % grid_size;
                    let grid_y = index / grid_size;

                    let current_img = current_img.into_raw();

                    for y in 0..tile_size {
                        let line = grid_y * tile_size + y;
                        let line_width = tile_size * 4;
                        let index = line * img_size * 4 + grid_x * line_width;

                        unsafe {
                            ptr::copy_nonoverlapping(
                                &current_img[y * line_width],
                                &mut img[index],
                                line_width,
                            );
                        }

                        // for x in 0..tile_size {
                        //     let index = index + x * 4;
                        //     let pixel = current_img.get_pixel(x, y).0;

                        //     img[index as usize] = pixel[0];
                        //     img[(index + 1) as usize] = pixel[1];
                        //     img[(index + 2) as usize] = pixel[2];
                        //     img[(index + 3) as usize] = pixel[3];
                        // }
                    }
                }
            }
        }

        // image::save_buffer(
        //     "./atlas.png",
        //     img.as_slice(),
        //     img_size as u32,
        //     img_size as u32,
        //     image::ColorType::Rgba8,
        // );

        (img, img_size as u32)
    }
}
