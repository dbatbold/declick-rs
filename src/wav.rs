use std::fmt::{self, Display};
use std::io;

/* WAVE file header spec from http://soundfile.sapp.org/doc/WaveFormat/
 *
 * Offset  Size  Name             Description
 * The canonical WAVE format starts with the RIFF header:
 *
 * 0         4   ChunkID          Contains the letters "RIFF" in ASCII form
 *                                (0x52494646 big-endian form).
 * 4         4   ChunkSize        36 + SubChunk2Size, or more precisely:
 *                                4 + (8 + SubChunk1Size) + (8 + SubChunk2Size)
 *                                This is the size of the rest of the chunk
 *                                following this number.  This is the size of the
 *                                entire file in bytes minus 8 bytes for the
 *                                two fields not included in this count:
 *                                ChunkID and ChunkSize.
 * 8         4   Format           Contains the letters "WAVE"
 *                                (0x57415645 big-endian form).
 *
 * The "WAVE" format consists of two subchunks: "fmt " and "data":
 * The "fmt " subchunk describes the sound data's format:
 *
 * 12        4   Subchunk1ID      Contains the letters "fmt "
 *                                (0x666d7420 big-endian form).
 * 16        4   Subchunk1Size    16 for PCM.  This is the size of the
 *                                rest of the Subchunk which follows this number.
 * 20        2   AudioFormat      PCM = 1 (i.e. Linear quantization)
 *                                Values other than 1 indicate some
 *                                form of compression.
 * 22        2   NumChannels      Mono = 1, Stereo = 2, etc.
 * 24        4   SampleRate       8000, 44100, etc.
 * 28        4   ByteRate         == SampleRate * NumChannels * BitsPerSample/8
 * 32        2   BlockAlign       == NumChannels * BitsPerSample/8
 *                                The number of bytes for one sample including
 *                                all channels. I wonder what happens when
 *                                this number isn't an integer?
 * 34        2   BitsPerSample    8 bits = 8, 16 bits = 16, etc.
 *           2   ExtraParamSize   if PCM, then doesn't exist
 *           X   ExtraParams      space for extra parameters
 *
 * The "data" subchunk contains the size of the data and the actual sound:
 *
 * 36        4   Subchunk2ID      Contains the letters "data"
 *                                (0x64617461 big-endian form).
 * 40        4   Subchunk2Size    == NumSamples * NumChannels * BitsPerSample/8
 *                                This is the number of bytes in the data.
 *                                You can also think of this as the size
 *                                of the read of the subchunk following this
 *                                number.
 * 44        *   Data             The actual sound data.
 *
 */

#[allow(dead_code)]
pub struct WaveHeader {
    chunk_id: u32,   // "RIFF" ASCII
    chunk_size: u32, // RIFF chuck size
    format: u32,     // "WAVE" ASCII

    // fmt chunk
    sub_chunk1_id: u32,   // "fmt " ASCII
    sub_chunk1_size: u32, // 16 for PCM
    audio_format: u16,    // PCM = 1, any other value is compressed formats
    num_channels: u16,    // Mono = 1, Stereo = 2
    sample_rate: u32,     // 8000, 44100, etc.
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,

    // data chunk
    sub_chunk2_id: u32, // "data" ASCII
    sub_chunk2_size: u32,
}

pub fn parse_wave_header(stream: &mut dyn io::Read) -> Result<WaveHeader, String> {
    let mut buf = [0; 44];
    match stream.read(&mut buf) {
        Err(e) => return Err(e.to_string()),
        Ok(n) => {
            if n != 44 {
                return Err(format!(
                    "WAVE header size must be 44-bytes long, but got {n}."
                ));
            }
        }
    };

    let header = WaveHeader {
        chunk_id: slice_to_u32(&buf[0..4]),
        chunk_size: slice_to_u32(&buf[4..8]),
        format: slice_to_u32(&buf[8..12]),
        sub_chunk1_id: slice_to_u32(&buf[12..16]),
        sub_chunk1_size: slice_to_u32(&buf[16..20]),
        audio_format: slice_to_u16(&buf[20..22]),
        num_channels: slice_to_u16(&buf[22..24]),
        sample_rate: slice_to_u32(&buf[24..28]),
        byte_rate: slice_to_u32(&buf[28..32]),
        block_align: slice_to_u16(&buf[32..34]),
        bits_per_sample: slice_to_u16(&buf[34..36]),
        sub_chunk2_id: slice_to_u32(&buf[36..40]),
        sub_chunk2_size: slice_to_u32(&buf[40..44]),
    };

    if let Err(e) = header.is_valid() {
        return Err(e.to_string());
    }

    Ok(header)
}

impl WaveHeader {
    pub fn is_valid(&self) -> Result<(), String> {
        if self.chunk_id != 0x46464952 {
            return Err(format!(
                "Stream must have 'RIFF' header, but got 0x{:x}.",
                self.chunk_id
            ));
        }
        if self.format != 0x45564157 {
            return Err(format!(
                "Stream must have 'WAVE' format, but got 0x{:x}.",
                self.format
            ));
        }
        if self.sub_chunk1_id != 0x20746d66 {
            return Err(format!(
                "Stream must have 'fmt ' sub chuck, but got 0x{:x}.",
                self.sub_chunk1_id
            ));
        }
        if self.sub_chunk1_size != 16 {
            return Err(format!(
                "Stream 'fmt ' sub chuck size must be 16, but got {}.",
                self.sub_chunk1_size
            ));
        }
        if self.audio_format != 1 {
            return Err(format!(
                "Stream audio format must be 1 (PCM), but got {}.",
                self.audio_format
            ));
        }
        Ok(())
    }
}

impl Display for WaveHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            r#"
WaveHeader {{
    chunk_id: 0x{0:x}
    chunk_size: {1}
    format: {2}
    sub_chunk1_id: 0x{3:x}
    sub_chunk1_size: {4}
    audio_format: {5}
    num_channels: {6}
    sample_rate: {7}
    byte_rate: {8}
    block_align: {9}
    bits_per_sample:{10} 
    sub_chunk2_id: {11}
    sub_chunk2_size: {12}
}}"#,
            self.chunk_id,
            self.chunk_size,
            self.format,
            self.sub_chunk1_id,
            self.sub_chunk1_size,
            self.audio_format,
            self.num_channels,
            self.sample_rate,
            self.byte_rate,
            self.block_align,
            self.bits_per_sample,
            self.sub_chunk2_id,
            self.sub_chunk2_size,
        )
    }
}

fn slice_to_u32(array: &[u8]) -> u32 {
    u32::from_le_bytes(array.try_into().unwrap())
}

fn slice_to_u16(array: &[u8]) -> u16 {
    u16::from_le_bytes(array.try_into().unwrap())
}

#[test]
fn test_slice_to_u32() {
    let a = [1u8, 2u8, 3u8, 4u8];
    let n = slice_to_u32(&a[0..4]);
    assert_eq!(n, 0x04030201);
}

#[test]
fn test_slice_to_u16() {
    let a = [1u8, 2u8];
    let n = slice_to_u16(&a[0..2]);
    assert_eq!(n, 0x0201);
}
