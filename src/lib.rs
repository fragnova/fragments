/* SPDX-License-Identifier: BUSL-1.1 */
/* Copyright © 2022 Fragcolor Pte. Ltd. */

/// list of compatible formats
pub enum AudioFormats {
	Ogg,
	Mp3,
	Wav,
}

pub struct AudioData {
	pub format: AudioFormats,
	pub data: Vec<u8>,
}

pub enum FragmentData {
	Chain(Vec<u8>),
	Audio(AudioData),
}

