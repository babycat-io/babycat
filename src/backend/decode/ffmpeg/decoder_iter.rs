use std::marker::PhantomData;

use ffmpeg::codec::packet::packet::Packet;
use ffmpeg::decoder::Audio as AudioDecoder;
use ffmpeg::format::context::input::PacketIter;
use ffmpeg::format::context::Input;
use ffmpeg::frame::Audio as Frame;

use crate::backend::decode::decoder_iter::DecoderIter;
use crate::backend::decode::ffmpeg::sample::Sample;
use crate::backend::signal::Signal;

#[inline(always)]
fn next_packet<'a>(
    packet_iter: &mut PacketIter<'a>,
    decoder: &mut AudioDecoder,
    stream_index: usize,
) -> Option<Packet> {
    for (packet_stream, packet) in packet_iter.by_ref() {
        if packet_stream.index() == stream_index && decoder.send_packet(&packet).is_ok() {
            return Some(packet);
        }
    }
    None
}

#[inline(always)]
fn next_decoded_frame_and_len(decoder: &mut AudioDecoder) -> (Option<Frame>, usize) {
    let mut frame = Frame::empty();
    if decoder.receive_frame(&mut frame).is_err() {
        return (None, 0);
    }
    let frame_len: usize = frame.samples();
    (Some(frame), frame_len)
}

#[inline(always)]
fn get_sample_packed<T: Sample>(
    frame: &Frame,
    num_channels: usize,
    frame_idx: usize,
    channel_idx: usize,
) -> f32 {
    let sample_idx: usize = frame_idx * num_channels + channel_idx;
    unsafe {
        // When audio is in a "packed" format, FFmpeg stores
        // each sample interleaved in the first data plane.
        let plane_ptr: *const T = (*frame.as_ptr()).data[0] as *const T;
        let sample: T = *plane_ptr.add(sample_idx);
        sample.as_f32_sample()
    }
}

#[inline(always)]
fn get_sample_planar<T: Sample>(frame: &Frame, frame_idx: usize, channel_idx: usize) -> f32 {
    unsafe {
        // When audio is stored in a "planar" format, FFmpeg
        // stores the first eight planes in the `.data` attribute.
        // If there are more than 8 planes, all of them are
        // available in the `.extended_data` attribute.
        // If there are not more than 8 planes, then
        // `.extended_data` just points to `.data`.
        let extended_data_ptr: *const *const T = (*frame.as_ptr()).extended_data as *const *const T;
        let plane_ptr: *const T = *extended_data_ptr.add(channel_idx);
        let sample: T = *plane_ptr.add(frame_idx);
        sample.as_f32_sample()
    }
}

pub struct FFmpegDecoderIter<'a, T: Sample, const PACKED: bool> {
    decoder: &'a mut AudioDecoder,
    packet_iter: PacketIter<'a>,
    stream_index: usize,
    frame_rate_hz: u32,
    num_channels: u16,
    num_channels_usize: usize,
    est_num_frames: Option<usize>,
    packet: Option<Packet>,
    frame: Option<Frame>,
    frame_len: usize,
    frame_idx: usize,
    channel_idx: usize,
    sent_eof: bool,
    _ph: PhantomData<T>,
}

impl<'a, T: Sample, const PACKED: bool> FFmpegDecoderIter<'a, T, PACKED> {
    pub fn new(
        input: &'a mut Input,
        decoder: &'a mut AudioDecoder,
        stream_index: usize,
        frame_rate_hz: u32,
        num_channels: u16,
        est_num_frames: Option<usize>,
    ) -> Self {
        let mut packet_iter = input.packets();
        let packet = next_packet(&mut packet_iter, decoder, stream_index);
        let (frame, frame_len) = next_decoded_frame_and_len(decoder);
        Self {
            decoder,
            packet_iter,
            stream_index,
            frame_rate_hz,
            num_channels,
            num_channels_usize: num_channels as usize,
            est_num_frames,
            packet,
            frame,
            frame_len,
            frame_idx: 0,
            channel_idx: 0,
            sent_eof: false,
            _ph: PhantomData,
        }
    }
}

impl<'a, T: Sample, const PACKED: bool> DecoderIter for FFmpegDecoderIter<'a, T, PACKED> {}

impl<'a, T: Sample, const PACKED: bool> Signal for FFmpegDecoderIter<'a, T, PACKED> {
    #[inline(always)]
    fn frame_rate_hz(&self) -> u32 {
        self.frame_rate_hz
    }

    #[inline(always)]
    fn num_channels(&self) -> u16 {
        self.num_channels
    }

    #[inline(always)]
    fn num_frames_estimate(&self) -> Option<usize> {
        self.est_num_frames
    }
}

impl<'a, T: Sample, const PACKED: bool> Iterator for FFmpegDecoderIter<'a, T, PACKED> {
    type Item = f32;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.packet.is_none() && !self.sent_eof {
                let _ = self.decoder.send_eof();
                self.sent_eof = true;
                continue;
            }
            match &mut self.frame {
                None => {
                    if self.sent_eof {
                        return None;
                    }
                    self.packet =
                        next_packet(&mut self.packet_iter, self.decoder, self.stream_index);
                    let (f, fl) = next_decoded_frame_and_len(self.decoder);
                    self.frame = f;
                    self.frame_len = fl;
                    self.channel_idx = 0;
                    self.frame_idx = 0;
                    continue;
                }
                Some(frame) => {
                    if self.channel_idx >= self.num_channels_usize {
                        self.channel_idx = 0;
                        self.frame_idx += 1;
                    }
                    if self.frame_idx >= self.frame_len {
                        let (f, fl) = next_decoded_frame_and_len(self.decoder);
                        self.frame = f;
                        self.frame_len = fl;
                        continue;
                    }
                    let sample: f32 = if PACKED {
                        get_sample_packed::<T>(
                            frame,
                            self.num_channels_usize,
                            self.frame_idx,
                            self.channel_idx,
                        )
                    } else {
                        get_sample_planar::<T>(frame, self.frame_idx, self.channel_idx)
                    };
                    self.channel_idx += 1;
                    return Some(sample);
                }
            }
        }
    }
}
