use std::io::Cursor;
use macroquad::Error;
use symphonia::core::audio::Signal;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::{get_codecs, get_probe};
use hound::{WavWriter, WavSpec, SampleFormat};

/// Converts audio-formats to .wav, since the macroquad sound engine only supports .wav
/// 
pub fn ensure_wav(data: Vec<u8>) -> Result<Vec<u8>, Error> {
    let cursor = Cursor::new(data);
    let mss = MediaSourceStream::new(Box::new(cursor), Default::default());

    let probed = get_probe()
        .format(&Hint::new(), mss, &FormatOptions::default(), &MetadataOptions::default())
        .map_err(|e| {
            let s: &'static str = Box::leak(format!("Symphonia probe error: {}", e).into_boxed_str());
            Error::from(s)
        })?;

    let mut format = probed.format;

    let track = format
        .default_track()
        .ok_or_else(|| Error::from("No default audio track found"))?;

    let mut decoder = get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|_| Error::from("Decoder init error"))?;

    let mut wav_cursor = Cursor::new(Vec::new());

    let sample_rate = track
        .codec_params
        .sample_rate
        .ok_or_else(|| Error::from("No sample rate"))?;
    let channels = track
        .codec_params
        .channels
        .ok_or_else(|| Error::from("No channel info"))?
        .count() as u16;

    let spec = WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::new(&mut wav_cursor, spec)
        .map_err(|_| Error::from("WavWriter init error"))?;

    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(SymphoniaError::IoError(_)) => break, // End of stream
            Err(_) => return Err(Error::from("Packet read error")),
        };

        let decoded = match decoder.decode(&packet) {
            Ok(audio_buf) => audio_buf,
            Err(SymphoniaError::DecodeError(_)) => continue, // Skip corrupt frame
            Err(_) => return Err(Error::from("Decode error")),
        };

        use symphonia::core::audio::AudioBufferRef;
        match decoded {
            AudioBufferRef::F32(buf) => {
                for s in buf.chan(0) {
                    let sample = (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                    writer.write_sample(sample).map_err(|_| Error::from("Write sample error"))?;
                }
            }
            AudioBufferRef::S16(buf) => {
                for s in buf.chan(0) {
                    writer.write_sample(*s).map_err(|_| Error::from("Write sample error"))?;
                }
            }
            _ => return Err(Error::from("Unsupported audio buffer format")),
        }
    }

    writer.finalize().map_err(|_| Error::from("Finalize WAV error"))?;

    Ok(wav_cursor.into_inner())
}
