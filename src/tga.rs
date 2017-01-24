use image::{Image, Color};
use std::fs::File;
use std::io::Read;
use std::io::Error;
use byteorder::{ReadBytesExt, LittleEndian};

enum ImageType {
    NoImageData = 0,
    // Uncompressed
    RawColorMap = 1,
    RawTrueColor = 2,
    RawGrayScale = 3,
    // Run length encoded
    RunColorMap = 9,
    RunTrueColor = 10,
    RunGrayScale = 11,
    Unknown,
}

impl ImageType {
    fn new(img_type: u8) -> ImageType {
        match img_type {
            0 => ImageType::NoImageData,
            1 => ImageType::RawColorMap,
            2 => ImageType::RawTrueColor,
            3 => ImageType::RawGrayScale,
            9 => ImageType::RunColorMap,
            10 => ImageType::RunTrueColor,
            11 => ImageType::RunGrayScale,
            _ => ImageType::Unknown,
        }
    }

    fn is_color(&self) -> bool {
        match *self {
            ImageType::RawColorMap |
            ImageType::RawTrueColor |
            ImageType::RunTrueColor |
            ImageType::RunColorMap => true,
            _ => false,
        }
    }

    fn is_color_mapped(&self) -> bool {
        match *self {
            ImageType::RawColorMap |
            ImageType::RunColorMap => true,
            _ => false,
        }
    }

    fn is_encoded(&self) -> bool {
        match *self {
            ImageType::RunColorMap |
            ImageType::RunTrueColor |
            ImageType::RunGrayScale => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct TGAHeader {
    id_length: u8,
    color_map_type: u8,
    data_type_code: u8,
    color_map_origin: u16,
    color_map_length: u16,
    color_map_depth: u8,
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    bits_per_pixel: u8,
    image_descriptor: u8,
}

impl TGAHeader {
    fn from_reader(r: &mut Read) -> Result<TGAHeader, Error> {
        Ok(TGAHeader {
            id_length: try!(r.read_u8()),
            color_map_type: try!(r.read_u8()),
            data_type_code: try!(r.read_u8()),
            color_map_origin: try!(r.read_u16::<LittleEndian>()),
            color_map_length: try!(r.read_u16::<LittleEndian>()),
            color_map_depth: try!(r.read_u8()),
            x_origin: try!(r.read_u16::<LittleEndian>()),
            y_origin: try!(r.read_u16::<LittleEndian>()),
            width: try!(r.read_u16::<LittleEndian>()),
            height: try!(r.read_u16::<LittleEndian>()),
            bits_per_pixel: try!(r.read_u8()),
            image_descriptor: try!(r.read_u8()),
        })
    }
}

pub fn read_tga_file(filename: &str) -> Result<Image, Error> {
    let mut f = try!(File::open(filename));
    let header = TGAHeader::from_reader(&mut f).unwrap();
    println!("{:?}", header);
    // TODO: Don't assume there is no color map, or all the other assumptions I'm making
    let width = header.width as usize;
    let height = header.height as usize;
    let bytes_per_pixel = (header.bits_per_pixel as usize + 7) / 8;

    // Check the image type - we only currently handle run length encoded true color
    let image_type = ImageType::new(header.data_type_code);

    let num_bytes = (height * width * bytes_per_pixel);
    let mut image_buf = Vec::with_capacity(num_bytes);

    match image_type {
        ImageType::RunTrueColor => {
            while image_buf.len() < num_bytes {
                let run_packet = try!(f.read_u8());
                if (run_packet & 0x80) != 0 {
                    // The highest bit is an indicator to repeat pixels
                    let repeat_count = ((run_packet & !0x80) + 1) as usize;
                    let mut data = Vec::with_capacity(bytes_per_pixel);
                    try!(f.by_ref().take(bytes_per_pixel as u64).read_to_end(&mut data));
                    for _ in 0usize..repeat_count {
                        image_buf.extend(data.iter().map(|&c| c));
                    }
                } else {
                    // We're dealing with non-encoded pixels
                    let num_raw_bytes = (run_packet + 1) as usize * bytes_per_pixel;
                    try!(f.by_ref().take(num_raw_bytes as u64).read_to_end(&mut image_buf));
                }
            }
        },
        _ => panic!("Can't handle this image type")
    }

    let mut color_buf: Vec<Color> = Vec::with_capacity(width * height);

    for chunk in image_buf.chunks(3) {
        let color = Color(chunk[0], chunk[1], chunk[2]);
        color_buf.push(color);
    }

    let mut image = Image::new(width as i32, height as i32);
    image.set_data_buffer(color_buf);
    Ok(image)
}
