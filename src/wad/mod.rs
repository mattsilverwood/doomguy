//! doomguy::wad
//!
//! This module pertains to the vanilla WAD format.

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
};

use crate::DoomguyError;

pub struct Wad {
    header: WadHeader,
    directory: Vec<WadDirectoryEntry>,
    map: HashMap<String, usize>,
    lumps: Vec<Lump>,
}
impl Default for Wad {
    fn default() -> Self {
        Self {
            header: WadHeader::default(),
            directory: Vec::new(),
            lumps: Vec::new(),
        }
    }
}

pub enum WadType {
    IWAD,
    PWAD,
}
pub struct WadHeader {
    pub wad_type: Option<WadType>,
    pub lump_count: i32,
    directory_offset: Option<i32>,
}
impl Default for WadHeader {
    fn default() -> Self {
        Self {
            wad_type: None,
            lump_count: 0,
            directory_offset: None,
        }
    }
}
pub struct WadDirectoryEntry;
impl Default for WadDirectoryEntry {
    fn default() -> Self {
        Self {}
    }
}
pub struct Lump;

impl Wad {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Wad, DoomguyError> {
        let path = path.as_ref().to_path_buf();

        let mut file = BufReader::new(File::open(&path)?);

        let mut wad = Wad::default();

        file.seek(SeekFrom::Start(0))?;
        wad.header = wad.parse_header(&mut file)?;

        // TODO: Seek to directory first.
        wad.directory = wad.parse_directory(&mut file)?;

        todo!();
    }

    fn parse_header<R: Read>(&self, reader: &mut R) -> Result<WadHeader, DoomguyError> {
        let mut header_buffer = [0u8; 12];

        reader.read_exact(&mut header_buffer)?;

        let _wad_type = String::from_utf8(header_buffer[0..4].into());

        todo!()
    }

    fn parse_directory<R: Read>(&self, reader: &mut R) -> Result<WadDirectory, DoomguyError> {
        todo!()
    }
}
