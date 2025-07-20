pub mod types {
    pub mod tex_1005;
    pub mod tex_1002;
}

use std::{env, fmt::Display, fs::File, io::Read};

use anyhow::{Error, Result};
use byteorder::{ReadBytesExt, LE};
use image::{DynamicImage, ImageFormat};
use types::{tex_1002::Tex1002, tex_1005::Tex1005};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const SUPPORTED: &str = "1002, 1005";
const USAGE: &str = "Usage:\n  tex_decode [files...]";

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();

    if arguments.is_empty() {
        println!("Not enough arguments provided");
        println!("{USAGE}");
        return
    }

    if arguments[0] == "-V" || arguments[0] == "--version" {
        println!("tex_decode v{VERSION}");
        return
    }

    if arguments[0] == "-h" || arguments[0] == "--help" {
        println!("Very simple Zachtronics .tex file decoder");
        println!("Supported .tex file versions: {SUPPORTED}");
        println!();
        println!("{USAGE}");
        return
    }

    for file in arguments {
        let input_filename = file.clone();

        let mut reader = File::open(&input_filename).unwrap();

        let version = reader.read_u32::<LE>().unwrap();

        println!("[{input_filename}; Version {version}]");

        let tex_file: Box<dyn TexImage> = match version {
            1002 => Box::new(Tex1002::decode(reader).unwrap()),
            1005 => Box::new(Tex1005::decode(reader).unwrap()),
            _ => {
                println!("Invalid file provided, version was {version}");
                return
            },
        };

        for image in tex_file.images() {
            let output_filename = if tex_file.count() == 1 {
                input_filename.clone() + ".png"
            } else {
                input_filename.clone() + "." + image.index.to_string().as_str() + ".png"
            };

            println!(
                "    [{}] - Width: {}, Height: {}, Color Format: {}",
                image.index,
                image.width,
                image.height,
                image.color_format,
            );

            let prelim_image: DynamicImage = match image.color_format {
                ColorFormat::Gray8 =>
                    image::GrayImage::from_raw(image.width, image.height, image.raw_image.clone()).unwrap().into(),
                ColorFormat::Rgba8 =>
                    image::RgbaImage::from_raw(image.width, image.height, image.raw_image.clone()).unwrap().into(),
            };

            // Flip it vertically because they're upside down otherwise
            let img = prelim_image.flipv();
            img.save_with_format(output_filename, ImageFormat::Png).unwrap();
        }

        println!()
    }
}

trait TexImage {
    fn decode(reader: impl Read) -> Result<Self> where Self: Sized;

    fn images(&self) -> Vec<TexSubImage>;

    fn count(&self) -> usize;
}


#[derive(Debug, Clone)]
struct TexSubImage {
    index: usize,
    width: u32,
    height: u32,

    color_format: ColorFormat,

    raw_image: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
enum ColorFormat {
    Gray8 = 1,
    Rgba8 = 4,
}

impl Display for ColorFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Gray8 => "GRAY 8",
            Self::Rgba8 => "RGBA 8",
        };

        write!(f, "{string}")
    }
}

impl TryFrom<u32> for ColorFormat {
    type Error = Error;

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Gray8,
            2 => Self::Rgba8,
            e => return Err(Error::msg(format!("Unknown color format {e}"))),
        })
    }
}
