// Copyright (c) Istvan Fehervari

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use std::fs::File;
use std::io::Read;
use std::error;

use super::{CrnFormat, CrunchedData};

#[test]
fn decompress_dxt1() {

    let file_path = "testdata/copyright_2048_compressed_dxt1.dat";
    let mut compressed_file = match File::open(&file_path) {
        Ok(f) => f,
        Err(err) => panic!(
            "Failed to open test data file at {}: {}",
            file_path,
            error::Error::description(&err)
        ),
    };

    let mut compressed_data = Vec::new();
    match compressed_file.read_to_end(&mut compressed_data) {
        Ok(_) => {}
        Err(err) => panic!(
            "Failed to read test data at {}: {}",
            file_path,
            error::Error::description(&err)
        ),
    };

    let c_data = CrunchedData::new(&compressed_data);
    let level_info = c_data.level_info(0);

    assert_eq!(level_info.height, 2048);
    assert_eq!(level_info.width, 2048);
    assert_eq!(level_info.faces, 1);
    assert_eq!(level_info.blocks_x, 512);
    assert_eq!(level_info.blocks_y, 512);
    assert_eq!(level_info.bytes_per_block, 8);
    assert_eq!(level_info.format, CrnFormat::Dxt1);

    let texture_info = c_data.texture_info();

    assert_eq!(texture_info.height, 2048);
    assert_eq!(texture_info.width, 2048);
    assert_eq!(texture_info.faces, 1);
    assert_eq!(texture_info.userdata0, 0);
    assert_eq!(texture_info.userdata1, 0);
    assert_eq!(texture_info.bytes_per_block, 8);
    assert_eq!(texture_info.format, CrnFormat::Dxt1);

    let decompressed_data = match c_data.decode_level(0) {
        None => {
            panic!("Decode failed");
        }
        Some(res) => res,
    };

    let file_path = "testdata/copyright_2048_decompressed_dxt1.dat";
    let mut decompressed_file = match File::open(&file_path) {
        Ok(f) => f,
        Err(err) => panic!(
            "Failed to open test data file at {}: {}",
            file_path,
            error::Error::description(&err)
        ),
    };

    let mut correct_decompressed_data = Vec::new();
    match decompressed_file.read_to_end(&mut correct_decompressed_data) {
        Ok(_) => {}
        Err(err) => panic!(
            "Failed to read test data at {}: {}",
            file_path,
            error::Error::description(&err)
        ),
    };

    assert_eq!(decompressed_data.len(), correct_decompressed_data.len());
    assert_eq!(decompressed_data, correct_decompressed_data);
}
