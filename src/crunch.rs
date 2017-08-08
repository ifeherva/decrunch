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

use libc::{c_int, c_void, uint32_t, uint8_t};
use CrunchedData;
use LevelInfo;
use TextureInfo;

extern "C" {
    fn crnd_get_level_info(
        pData: *const uint8_t,
        data_size: uint32_t,
        level_index: uint32_t,
        pLevel_info: *mut LevelInfo,
    ) -> c_int;

    fn crnd_get_texture_info(
        pData: *const uint8_t,
        data_size: uint32_t,
        pTexture_info: *mut TextureInfo,
    ) -> c_int;

    fn crnd_unpack_begin(pData: *const uint8_t, data_size: uint32_t) -> *const c_void;

    fn crnd_unpack_end(ctx: *const c_void) -> c_int;

    fn crnd_unpack_level(
        pContext: *const c_void,
        ppDst: *const *const uint8_t,
        dst_size_in_bytes: uint32_t,
        row_pitch_in_bytes: uint32_t,
        level_index: uint32_t,
    ) -> c_int;
}

pub fn get_level_info(data: &CrunchedData, level: u32) -> LevelInfo {
    let mut level_info = LevelInfo::default();
    unsafe {
        crnd_get_level_info(
            data.buffer.as_ptr(),
            data.buffer.len() as uint32_t,
            level as uint32_t,
            &mut level_info as *mut LevelInfo,
        );
    }
    level_info
}

pub fn get_texture_info(data: &CrunchedData) -> TextureInfo {
    let mut texture_info = TextureInfo::default();
    unsafe {
        crnd_get_texture_info(
            data.buffer.as_ptr(),
            data.buffer.len() as uint32_t,
            &mut texture_info as *mut TextureInfo,
        );
    }
    texture_info
}

/// Decompresses the texture's decoder tables and endpoint/selector palettes.
pub fn unpack_begin(buffer: &[u8]) -> *const c_void {
    unsafe { crnd_unpack_begin(buffer.as_ptr(), buffer.len() as uint32_t) }
}

pub fn unpack_level(
    ctx: *const c_void,
    dst: &mut [u8],
    row_pitch_in_bytes: u32,
    level_index: u32,
) -> bool {
    unsafe {
        let ptr = dst.as_ptr();
        crnd_unpack_level(
            ctx,
            &ptr as *const *const uint8_t,
            dst.len() as uint32_t,
            row_pitch_in_bytes as uint32_t,
            level_index as uint32_t,
        ) > 0
    }
}

pub fn unpack_end(ctx: *const c_void) {
    unsafe {
        crnd_unpack_end(ctx);
    }
}
