/* SPDX-License-Identifier: BUSL-1.1 */
/* Copyright © 2022 Fragcolor Pte. Ltd. */

use parity_scale_codec::{Decode, Encode};
use std::collections::btree_map::BTreeMap;

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
pub struct EdnData {
  pub text: String,
}

#[derive(Encode, Decode, Clone, scale_info::TypeInfo, PartialEq, Debug, Eq)]
pub enum FragmentData {
  Edn(EdnData),
  Audio(AudioData),
  /// Nested types
  Sequence(Vec<FragmentData>),
  Table(BTreeMap<String, FragmentData>),
}

#[derive(Encode, Decode, Clone, scale_info::TypeInfo, PartialEq, Debug, Eq)]
pub struct FragmentMetadata {
  pub name: String,
}

#[derive(Encode, Decode, Clone, scale_info::TypeInfo, PartialEq, Debug, Eq)]
pub struct Fragment {
  pub metadata: FragmentMetadata,
  pub data: FragmentData,
}

#[test]
fn test_encode_decode_fragment_audio() {
  let data = FragmentData::Audio(AudioData {
    format: AudioFormats::Ogg,
    data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
  });
  let encoded = data.encode();
  let decoded = FragmentData::decode(&mut &encoded[..]).unwrap();
  assert_eq!(data, decoded);
}

#[test]
fn test_encode_decode_fragment_edn() {
  let data = FragmentData::Edn(EdnData {
    text: "hello world".to_string(),
  });
  let encoded = data.encode();
  let decoded = FragmentData::decode(&mut &encoded[..]).unwrap();
  assert_eq!(data, decoded);
}

#[test]
fn test_encode_decode_fragment() {
  let data = FragmentData::Edn(EdnData {
    text: "hello world".to_string(),
  });
  let metadata = FragmentMetadata {
    name: "test".to_string(),
  };
  let fragment = Fragment { metadata, data };
  let encoded = fragment.encode();
  let decoded = Fragment::decode(&mut &encoded[..]).unwrap();
  assert_eq!(fragment, decoded);
}

#[test]
fn test_encode_decode_fragment_sequence() {
  let data = FragmentData::Sequence(vec![
    FragmentData::Edn(EdnData {
      text: "hello world".to_string(),
    }),
    FragmentData::Audio(AudioData {
      format: AudioFormats::Ogg,
      data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    }),
  ]);
  let encoded = data.encode();
  let decoded = FragmentData::decode(&mut &encoded[..]).unwrap();
  assert_eq!(data, decoded);
}

#[test]
fn test_encode_decode_fragment_table() {
  let data = FragmentData::Table(BTreeMap::from_iter(vec![
    (
      "a".to_string(),
      FragmentData::Edn(EdnData {
        text: "hello world".to_string(),
      }),
    ),
    (
      "b".to_string(),
      FragmentData::Audio(AudioData {
        format: AudioFormats::Ogg,
        data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
      }),
    ),
  ]));
  let encoded = data.encode();
  let decoded = FragmentData::decode(&mut &encoded[..]).unwrap();
  assert_eq!(data, decoded);
}
