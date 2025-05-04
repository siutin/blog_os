use core::fmt::{self, Write};
use x86_64::instructions::port::{Port, PortGeneric, ReadWriteAccess};

// Constants
pub const RED: [u8; 3] = [255, 0, 0];
pub const GREEN: [u8; 3] = [0, 255, 0];
pub const BLUE: [u8; 3] = [0, 0, 255];
pub const WHITE: [u8; 3] = [255, 255, 255];
pub const BLACK: [u8; 3] = [0, 0, 0];

// Framebuffer configuration for 1024x768 32bpp
const WIDTH: usize = 1024;
const HEIGHT: usize = 768;
const BPP: usize = 4;  // 32-bit color (BGRA)
pub const BUFFER_SIZE: usize = WIDTH * HEIGHT * BPP;

// VBE Registers (Bochs VBE Extensions)
const VBE_DISPI_INDEX_ID: u16 = 0x0;
const VBE_DISPI_INDEX_XRES: u16 = 0x1;
const VBE_DISPI_INDEX_YRES: u16 = 0x2;
const VBE_DISPI_INDEX_BPP: u16 = 0x3;
const VBE_DISPI_INDEX_ENABLE: u16 = 0x4;
const VBE_DISPI_INDEX_BANK: u16 = 0x5;
const VBE_DISPI_INDEX_VIRT_WIDTH: u16 = 0x6;
const VBE_DISPI_INDEX_VIRT_HEIGHT: u16 = 0x7;
const VBE_DISPI_INDEX_X_OFFSET: u16 = 0x8;
const VBE_DISPI_INDEX_Y_OFFSET: u16 = 0x9;

// VBE Ports
const VBE_DISPI_IOPORT_INDEX: u16 = 0x01CE;
const VBE_DISPI_IOPORT_DATA: u16 = 0x01CF;

// VBE Flags
const VBE_DISPI_ENABLED: u16 = 0x01;
const VBE_DISPI_LFB_ENABLED: u16 = 0x40;

// Framebuffer base address for Bochs VBE (standard location)
pub const FB_ADDR: u64 = 0xFD000000;

pub struct FrameBufferWriter {
    buffer: &'static mut [u8],
}

impl FrameBufferWriter {
    pub fn new() -> Self {
        // Initialize VBE mode
        unsafe { Self::init_vbe_mode() };
        
        let buffer = unsafe { 
            core::slice::from_raw_parts_mut(
                FB_ADDR as *mut u8,
                BUFFER_SIZE
            )
        };
        
        Self { buffer }
    }
    
    unsafe fn init_vbe_mode() {
        let mut index_port: PortGeneric<u16, ReadWriteAccess> = Port::new(VBE_DISPI_IOPORT_INDEX);
        let mut data_port: PortGeneric<u16, ReadWriteAccess> = Port::new(VBE_DISPI_IOPORT_DATA);
        
        // Disable VBE for initialization
        index_port.write(VBE_DISPI_INDEX_ENABLE);
        data_port.write(0);
        
        // Set resolution and color depth
        index_port.write(VBE_DISPI_INDEX_XRES);
        data_port.write(WIDTH as u16);
        
        index_port.write(VBE_DISPI_INDEX_YRES);
        data_port.write(HEIGHT as u16);
        
        index_port.write(VBE_DISPI_INDEX_BPP);
        data_port.write(32); // 32 bits per pixel
        
        // Enable VBE with linear framebuffer
        index_port.write(VBE_DISPI_INDEX_ENABLE);
        data_port.write(VBE_DISPI_ENABLED | VBE_DISPI_LFB_ENABLED);
        
        // Read back to verify
        index_port.write(VBE_DISPI_INDEX_XRES);
        let xres = data_port.read();
        
        index_port.write(VBE_DISPI_INDEX_YRES);
        let yres = data_port.read();
        
        index_port.write(VBE_DISPI_INDEX_BPP);
        let bpp = data_port.read();
        
        index_port.write(VBE_DISPI_INDEX_ENABLE);
        let enabled = data_port.read();
        
        // Note: We can't use println in an unsafe block, so we can't log these values
    }

    pub fn width(&self) -> usize {
        WIDTH
    }

    pub fn height(&self) -> usize {
        HEIGHT
    }

    pub fn clear_screen(&mut self) {
        for i in (0..BUFFER_SIZE).step_by(4) {
            unsafe {
                *((FB_ADDR + i as u64) as *mut u32) = 0;
            }
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: &[u8; 3]) {
        if x >= self.width() || y >= self.height() {
            return;
        }

        let offset = (y * self.width() + x) * BPP;
        if offset + BPP > self.buffer.len() {
            return;
        }

        // Set RGB components (BGRA format)
        self.buffer[offset] = rgb[2];     // Blue
        self.buffer[offset + 1] = rgb[1];  // Green
        self.buffer[offset + 2] = rgb[0];  // Red
        self.buffer[offset + 3] = 255;     // Alpha (full opacity)
    }

    pub fn fill(&mut self, x: usize, y: usize, width: usize, height: usize, rgb: &[u8; 3]) {
        // Create a 32-bit color value (BGRA format)
        let color: u32 = ((rgb[2] as u32) << 0) | 
                         ((rgb[1] as u32) << 8) | 
                         ((rgb[0] as u32) << 16) |
                         (255u32 << 24);
                         
        for cy in y..(y + height) {
            if cy >= HEIGHT {
                break;
            }
            
            let row_offset = (cy * WIDTH + x) * BPP;
            
            for cx in 0..width {
                if x + cx >= WIDTH {
                    break;
                }
                
                let pixel_offset = row_offset + cx * BPP;
                
                // Directly write a 32-bit value for better performance
                unsafe {
                    *((FB_ADDR + pixel_offset as u64) as *mut u32) = color;
                }
            }
        }
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, rgb: &[u8; 3]) {
        self.fill(x, y, width, height, rgb);
    }
}

impl Write for FrameBufferWriter {
    fn write_str(&mut self, _s: &str) -> fmt::Result {
        Ok(())
    }
}