extern crate block_macro;
use block_macro::block_definition;
use filesize::PathExt;
use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

block_definition!("blockspecs/id.blockspec.yml");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_mf4() {
        let path = PathBuf::from("assets/test.mf4");
        let mdf4_result = get_mdf4(&path);
        assert!(mdf4_result.is_ok());
        let mut mdf4 = mdf4_result.unwrap();
        // print the size of the file
        println!("size: {}", &mdf4.size);
        // load the file identification
        mdf4.load_file_id();
        // print the version of the file
        let optional_header = &mdf4.header;
        assert!(optional_header.is_some());
        let header = &mdf4.header.unwrap();
        println!("version: {}", &header.version);
    }

    #[test]
    fn create_dummy_id_block() {
        let id_block: IDBlock = IDBlock {
            fileIdentifier: "".to_string(),
            formatIdentifier: "".to_string(),
            programIdentifier: "".to_string(),
            defaultByteOrder: 0,
            defaultFloatingPointFormat: 0,
            versionNumber: 420,
            codePageNumber: 0,
            reservedBlockA: ' ',
            reservedBlockB: ' ',
            standardUnfinalizedFlags: 0,
            customUnfinalizedFlags: 0,
        };

        println!("{:?}", id_block);
    }
}

pub struct MDF4Identification {
    version: String,
}

pub struct MDF4 {
    pub file_path: PathBuf,
    pub size: u64,
    pub header: Option<MDF4Identification>,
}

pub fn get_mdf4(file_path: &PathBuf) -> Result<MDF4, io::Error> {
    let metadata = file_path.symlink_metadata()?;
    // throw an error if the file does not exist
    let size = file_path.size_on_disk_fast(&metadata)?;
    let mdf4 = MDF4 {
        file_path: file_path.clone(),
        size,
        header: None,
    };
    return Ok(mdf4);
}

// implement trait load_header for MDF4
impl MDF4 {
    pub fn load_file_id(&mut self) {
        println!("loading file identification");
        // read first 16 bytes of the file
        // check if the first 3 bytes are "MDF"
        let chunk_size = 0x10;
        let mut chunk = Vec::with_capacity(chunk_size);
        let mut file = File::open(&self.file_path).unwrap();
        let n = file
            .by_ref()
            .take(chunk_size as u64)
            .read_to_end(&mut chunk)
            .unwrap();
        if n != chunk_size {
            panic!("could not read the first 16 bytes of the file");
        }
        // convert chunk to string
        let identification_string = String::from_utf8(chunk).unwrap();
        // panic if the first 3 bytes are not "MDF"
        if &identification_string[0..3] != "MDF" {
            panic!("file is not a valid MDF4 file");
        }
        // take the last 8 bytes as string
        let version_string = &identification_string[8..16];
        // print the version string
        println!("version: {}", version_string);

        self.header = Some(MDF4Identification {
            version: version_string.to_string(),
        });
    }
}
