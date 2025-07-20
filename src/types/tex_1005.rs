use std::io::Read;

use anyhow::Result;
use byteorder::{LE, ReadBytesExt};
use lz4_flex::block;

use crate::{ColorFormat, TexImage, TexSubImage};

#[derive(Debug)]
pub struct Tex1005 {
    count: usize,
    images: Vec<TexSubImage>,
}

impl TexImage for Tex1005 {
    fn decode(mut reader: impl Read) -> Result<Self> {
        let count = reader.read_u32::<LE>().unwrap();

        let mut images = Vec::new();

        for index in 0..count as usize {
            let width = reader.read_u32::<LE>().unwrap();
            let height = reader.read_u32::<LE>().unwrap();

            let color_format = ColorFormat::try_from(reader.read_u32::<LE>().unwrap())?;

            let mut unknown = [0u8; 52];
            reader.read_exact(unknown.as_mut_slice()).unwrap();

            let lz4_count = reader.read_u32::<LE>().unwrap();

            let mut lz4_bytes = vec![0u8; lz4_count as usize];
            reader.read_exact(&mut lz4_bytes).unwrap();

            // Get the uncompressed size and pass it into the lz4 decoder
            let decompressed_size = width as usize * height as usize * color_format as usize;
            let image_bytes = block::decompress(&lz4_bytes, decompressed_size).unwrap();

            let image_info = TexSubImage {
                index,
                width,
                height,
                color_format,
                raw_image: image_bytes,
            };

            images.push(image_info);
        }

        Ok(Tex1005 {
            count: count as usize,
            images
        })
    }

    fn images(&self) -> Vec<TexSubImage> {
        self.images.clone()
    }

    fn count(&self) -> usize {
        self.count
    }
}
