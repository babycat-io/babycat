use ffmpeg::codec::packet::packet::Packet;
use ffmpeg::decoder::Audio as AudioDecoder;
use ffmpeg::format::context::input::PacketIter;
use ffmpeg::format::context::Input;

use crate::backend::decode::decoder_iter::DecoderIter;
use crate::backend::decode::ffmpeg::frame_iter::FrameIter;
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

pub struct FFmpegDecoderIter<'a> {
    decoder: &'a mut AudioDecoder,
    packet_iter: PacketIter<'a>,
    stream_index: usize,
    frame_rate_hz: u32,
    num_channels: u16,
    est_num_frames: Option<usize>,
    packet: Option<Packet>,
    frame: Option<FrameIter>,
    sent_eof: bool,
    current_sample: usize,
}

impl<'a> FFmpegDecoderIter<'a> {
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
        let frame = FrameIter::new(decoder);
        Self {
            decoder,
            packet_iter,
            stream_index,
            frame_rate_hz,
            num_channels,
            est_num_frames,
            packet,
            frame,
            sent_eof: false,
            current_sample: 0,
        }
    }
}

impl<'a> DecoderIter for FFmpegDecoderIter<'a> {}

impl<'a> Signal for FFmpegDecoderIter<'a> {
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

impl<'a> Iterator for FFmpegDecoderIter<'a> {
    type Item = f32;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.packet.is_none() {
                if !self.sent_eof {
                    let _ = self.decoder.send_eof();
                    self.sent_eof = true;
                }
                match &mut self.frame {
                    None => return None,
                    Some(frame) => match frame.next() {
                        None => {
                            self.frame = FrameIter::new(self.decoder);
                            continue;
                        }
                        Some(sample) => {
                            self.current_sample += 1;
                            return Some(sample);
                        }
                    },
                }
            }
            match &mut self.frame {
                None => {
                    self.packet =
                        next_packet(&mut self.packet_iter, self.decoder, self.stream_index);
                    self.frame = FrameIter::new(self.decoder);
                    continue;
                }
                Some(frame) => match frame.next() {
                    None => {
                        self.frame = FrameIter::new(self.decoder);
                        continue;
                    }
                    Some(sample) => {
                        self.current_sample += 1;
                        return Some(sample);
                    }
                },
            }
        }
    }
}
