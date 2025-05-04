mod constants;
mod font;
mod writer;

pub use constants::*;
pub use writer::FrameBufferWriter;

pub const BUFFER_SIZE: usize = WIDTH * HEIGHT * BPP;
pub const FB_ADDR: u64 = 0xFD000000; 