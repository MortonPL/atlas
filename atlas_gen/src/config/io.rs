use std::{
    fs::{self, File},
    path::Path,
};

use png::ColorType;

use crate::config::AtlasGenConfig;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Deser(#[from] toml::de::Error),
    #[error("{0}")]
    Serde(#[from] toml::ser::Error),
    #[error("{0}")]
    PngDecode(#[from] png::DecodingError),
    #[error("{0}")]
    PngEncode(#[from] png::EncodingError),
    #[error("Image resolution {0}x{1} doesn't match world size {2}x{3}")]
    ResolutionMismatch(u32, u32, u32, u32),
    #[error("Image byte per pixel value is {0}, but only 4 is accepted")]
    InvalidBytePerPixel(usize),
    #[error("Image color type is not greyscale")]
    InvalidColorTypeGrey(ColorType),
    #[error("Image color type is not RGBA")]
    InvalidColorTypeRgba(ColorType),
}

/// Load a generator config from a TOML file.
pub fn load_config(path: impl AsRef<Path>) -> Result<AtlasGenConfig> {
    let text = fs::read_to_string(path)?;
    let config = toml::from_str(&text)?;
    Ok(config)
}

/// Save a generator config to a TOML file.
pub fn save_config(config: &AtlasGenConfig, path: impl AsRef<Path>) -> Result<()> {
    let text = toml::to_string(config)?;
    fs::write(path, text)?;
    Ok(())
}

/// Load a generator image (layer) from a PNG file.
pub fn load_image(path: impl AsRef<Path>, width: u32, height: u32) -> Result<Vec<u8>> {
    let decoder = png::Decoder::new(File::open(path)?);
    let mut reader = decoder.read_info()?;
    let info = reader.info();

    if (info.width != width) || (info.height != height) {
        return Err(Error::ResolutionMismatch(info.width, info.height, width, height));
    }

    let bypp = info.bytes_per_pixel();
    if bypp != 4 {
        return Err(Error::InvalidBytePerPixel(bypp));
    }

    match info.color_type {
        png::ColorType::Rgba => Ok(()),
        x => Err(Error::InvalidColorTypeRgba(x)),
    }?;

    let mut buf = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut buf)?;

    Ok(buf)
}

/// Load layer data from a greyscale PNG file.
pub fn load_image_grey(path: impl AsRef<Path>, width: u32, height: u32) -> Result<Vec<u8>> {
    let decoder = png::Decoder::new(File::open(path)?);
    let mut reader = decoder.read_info()?;
    let info = reader.info();

    if (info.width != width) || (info.height != height) {
        return Err(Error::ResolutionMismatch(info.width, info.height, width, height));
    }

    let bypp = info.bytes_per_pixel();
    if bypp != 1 {
        return Err(Error::InvalidBytePerPixel(bypp));
    }

    match info.color_type {
        png::ColorType::Grayscale => Ok(()),
        x => Err(Error::InvalidColorTypeGrey(x)),
    }?;

    let mut buf = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut buf)?;

    Ok(buf)
}

/// Save a generator image (layer) to a PNG file.
pub fn save_image(path: impl AsRef<Path>, data: &[u8], width: u32, height: u32) -> Result<()> {
    let mut encoder = png::Encoder::new(File::create(path)?, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_srgb(png::SrgbRenderingIntent::AbsoluteColorimetric); // TODO check what that means
    let mut writer = encoder.write_header()?;
    writer.write_image_data(data)?;

    Ok(())
}

/// Save layer data as a greyscale PNG file.
pub fn save_image_grey(path: impl AsRef<Path>, data: &[u8], width: u32, height: u32) -> Result<()> {
    let mut encoder = png::Encoder::new(File::create(path)?, width, height);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(data)?;

    Ok(())
}
