/* SPDX-License-Identifier: BUSL-1.1 */
/* Copyright © 2022 Fragcolor Pte. Ltd. */

use parity_scale_codec::{Decode, Encode};

/// list of compatible formats
#[derive(Encode, Decode, Clone, scale_info::TypeInfo, PartialEq, Debug, Eq)]
pub enum AudioFormats {
    Ogg,
    Mp3,
    Wav,
}

#[derive(Encode, Decode, Clone, scale_info::TypeInfo, PartialEq, Debug, Eq)]
pub struct AudioData {
    pub format: AudioFormats,
    pub data: Vec<u8>,
}

#[derive(Encode, Decode, Clone, scale_info::TypeInfo, PartialEq, Debug, Eq)]
pub enum FragmentData {
    Chain(Vec<u8>),
    Audio(AudioData),
}

#[test]
fn test_encode_decode() {
    let data = AudioData {
        format: AudioFormats::Ogg,
        data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    };
    let encoded = data.encode();
    let decoded = AudioData::decode(&mut &encoded[..]).unwrap();
    assert_eq!(data, decoded);
}

#[test]
fn test_encode_decode_fragment() {
    let data = FragmentData::Audio(AudioData {
        format: AudioFormats::Ogg,
        data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    });
    let encoded = data.encode();
    let decoded = FragmentData::decode(&mut &encoded[..]).unwrap();
    assert_eq!(data, decoded);
}
