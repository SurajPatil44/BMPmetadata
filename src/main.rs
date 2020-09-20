use byteorder::{ByteOrder, LittleEndian};
use std::{
    fs::File,
    io::prelude::*,
    //error::Error,
};
use thiserror::Error;

#[allow(non_snake_case)]
#[derive(Debug)]
struct BmpHeader {
    Signature: [u8; 2],
    FileSize: u32,
    Reserved: u32,
    DataOffset: u32,
}

#[derive(Error, Debug)]
enum HeaderError {
    #[error("found invalid signature in header")]
    InvalidSignature { msg: String },
}

impl BmpHeader {
    fn get_header(buffer: &[u8]) -> Result<BmpHeader, HeaderError> {
        let sig = &buffer[..2];
        match (sig[0], sig[1]) {
            (66, 77) => (),
            (_, _) => {
                return Err(HeaderError::InvalidSignature {
                    msg: String::from("Not a BMP file"),
                })
            }
        }

        let fs = LittleEndian::read_u32(&buffer[2..6]);
        let res = LittleEndian::read_u32(&buffer[6..10]);
        let doff = LittleEndian::read_u32(&buffer[10..14]);
        Ok(BmpHeader {
            Signature: [sig[0], sig[1]],
            FileSize: fs,
            Reserved: res,
            DataOffset: doff,
        })
    }
}

#[repr(u16)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
enum BitsPerPixel {
    monochrome = 1,
    four_bit_pallet = 4,
    eight_bit_pallet = 8,
    sixteen_bit_pallet = 16,
    twenty_four_bit_pallet = 24,
}

impl BitsPerPixel {
    fn new(num: u16) -> BitsPerPixel {
        match num {
            1 => BitsPerPixel::monochrome,
            4 => BitsPerPixel::four_bit_pallet,
            8 => BitsPerPixel::eight_bit_pallet,
            16 => BitsPerPixel::sixteen_bit_pallet,
            n if (n == 24) | (n == 32) => BitsPerPixel::twenty_four_bit_pallet,
            //n if n > 24 => BitsPerPixel::twenty_four_bit_palleti,
            _ => todo!(),
        }
    }
}

#[repr(u32)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
enum CompressionType {
    ///No Compression
    BI_RGB = 0,
    ///8bit RLE encoding
    BI_RLE8 = 1,
    ///4bit RLR encoding
    BI_RLE4 = 2,
    ///Bitfields need to get information on this
    BI_BITFIELDS = 3,
}

impl CompressionType {
    fn new(num: u32) -> CompressionType {
        match num {
            0 => CompressionType::BI_RGB,
            1 => CompressionType::BI_RLE8,
            2 => CompressionType::BI_RLE4,
            3 => CompressionType::BI_BITFIELDS,
            n => {
                println!("{}", n);
                todo!();
            }
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
struct BmpInfoHeader {
    Size: u32,
    Width: u32,
    Height: u32,
    Planes: u16,
    BitPerPixel: BitsPerPixel,
    Compression: CompressionType,
    ImageSize: u32,
    HorizontalRes: u32,
    VerticalRes: u32,
    // for 8 bit it 100H to 256
    Colors: u32,
    // 0 means all
    ImportantColors: u32,
}

impl BmpInfoHeader {
    fn create_bmp_info_header(buffer: &[u8]) -> BmpInfoHeader {
        // let mut load_part = size| { get_slice(&buffer,offset,size) };
        let size = LittleEndian::read_u32(&buffer[..4]);
        let width = LittleEndian::read_u32(&buffer[4..8]);
        let height = LittleEndian::read_u32(&buffer[8..12]);
        let planes = LittleEndian::read_u16(&buffer[12..14]);
        let bpp = BitsPerPixel::new(LittleEndian::read_u16(&buffer[14..16]));
        let compression = CompressionType::new(LittleEndian::read_u32(&buffer[16..20]));
        let image_size = LittleEndian::read_u32(&buffer[20..24]);
        let x = LittleEndian::read_u32(&buffer[24..28]);
        let y = LittleEndian::read_u32(&buffer[28..32]);
        let colors = LittleEndian::read_u32(&buffer[32..36]);
        let impcol = LittleEndian::read_u32(&buffer[36..40]);

        BmpInfoHeader {
            Size: size,
            Width: width,
            Height: height,
            Planes: planes,
            BitPerPixel: bpp,
            Compression: compression,
            ImageSize: image_size,
            HorizontalRes: x,
            VerticalRes: y,
            Colors: colors,
            ImportantColors: impcol,
        }
    }
}

fn main() -> Result<(), HeaderError> {
    let mut bmpfile = File::open("file.bmp").unwrap();
    let mut buffer: Vec<u8> = Vec::with_capacity(1024);
    bmpfile.read_to_end(&mut buffer).unwrap();
    let header = BmpHeader::get_header(&buffer[..14])?;
    let bmpinfoheader = BmpInfoHeader::create_bmp_info_header(&buffer[14..54]);
    println!("{:?}", header);
    println!("{:?}", bmpinfoheader);
    Ok(())
}
