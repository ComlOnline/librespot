#[macro_use] extern crate log;
#[macro_use] extern crate futures;

extern crate bit_set;
extern crate byteorder;
extern crate crypto;
extern crate num_traits;
extern crate num_bigint;
extern crate tempfile;

extern crate librespot_core as core;

mod fetch;
mod decrypt;

#[cfg(not(any(feature = "with-tremor", feature = "with-vorbis")))]
mod lewton_decoder;
#[cfg(any(feature = "with-tremor", feature = "with-vorbis"))]
mod libvorbis_decoder;

pub use fetch::{AudioFile, AudioFileOpen};
pub use decrypt::AudioDecrypt;

#[cfg(not(any(feature = "with-tremor", feature = "with-vorbis")))]
pub use lewton_decoder::{VorbisDecoder, VorbisPacket, VorbisError};
#[cfg(any(feature = "with-tremor", feature = "with-vorbis"))]
pub use libvorbis_decoder::{VorbisDecoder, VorbisPacket, VorbisError};
