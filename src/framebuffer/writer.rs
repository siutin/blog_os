use core::fmt::{self, Write};
use x86_64::instructions::port::{Port, PortGeneric, ReadWriteAccess};

use super::constants::*;

// FrameBuffer writer implementation
pub struct FrameBufferWriter {
    buffer: &'static mut [u8],
}

impl FrameBufferWriter {
    pub fn new() -> Self {
        // Initialize VBE mode
        unsafe { Self::init_vbe_mode() };
        
        let buffer = unsafe { 
            core::slice::from_raw_parts_mut(
                super::FB_ADDR as *mut u8,
                super::BUFFER_SIZE
            )
        };
        
        Self { buffer }
    }
    
    unsafe fn init_vbe_mode() {
        let mut index_port: PortGeneric<u16, ReadWriteAccess> = Port::new(VBE_DISPI_IOPORT_INDEX);
        let mut data_port: PortGeneric<u16, ReadWriteAccess> = Port::new(VBE_DISPI_IOPORT_DATA);
        
        unsafe {
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
                let _xres = data_port.read();
                
                index_port.write(VBE_DISPI_INDEX_YRES);
                let _yres = data_port.read();
                
                index_port.write(VBE_DISPI_INDEX_BPP);
                let _bpp = data_port.read();
                
                index_port.write(VBE_DISPI_INDEX_ENABLE);
                let _enabled = data_port.read();
        }
    }

    pub fn width(&self) -> usize {
        WIDTH
    }

    pub fn height(&self) -> usize {
        HEIGHT
    }

    pub fn clear_screen(&mut self) {
        for i in (0..super::BUFFER_SIZE).step_by(BPP) {
            unsafe {
                *((super::FB_ADDR + i as u64) as *mut u32) = 0;
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
                    *((super::FB_ADDR + pixel_offset as u64) as *mut u32) = color;
                }
            }
        }
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, rgb: &[u8; 3]) {
        self.fill(x, y, width, height, rgb);
    }

    pub fn draw_circle(&mut self, x: usize, y: usize, radius: usize, fill_rgb: &[u8; 3], border_rgb: &[u8; 3]) {
        // First fill a slightly smaller circle
        if radius > 1 {
            let fill_radius = radius - 1; // Make the fill radius 1 pixel smaller
            for cy in y.saturating_sub(fill_radius)..=(y + fill_radius).min(HEIGHT-1) {
                for cx in x.saturating_sub(fill_radius)..=(x + fill_radius).min(WIDTH-1) {
                    let dx = if cx > x { cx - x } else { x - cx };
                    let dy = if cy > y { cy - y } else { y - cy };
                    let distance_squared = dx * dx + dy * dy;
                    
                    if distance_squared <= fill_radius * fill_radius {
                        self.set_pixel(cx, cy, fill_rgb);
                    }
                }
            }
        }
        
        // Draw the border using distance calculation for better visibility
        for cy in y.saturating_sub(radius)..=(y + radius).min(HEIGHT-1) {
            for cx in x.saturating_sub(radius)..=(x + radius).min(WIDTH-1) {
                let dx = if cx > x { cx - x } else { x - cx };
                let dy = if cy > y { cy - y } else { y - cy };
                let distance_squared = dx * dx + dy * dy;
                
                // Draw pixels that are close to the exact radius
                let inner_limit = (radius - 1).saturating_mul(radius - 1);
                let outer_limit = (radius + 1) * (radius + 1);
                
                if distance_squared >= inner_limit && distance_squared <= outer_limit {
                    self.set_pixel(cx, cy, border_rgb);
                }
            }
        }
    }
}

impl Write for FrameBufferWriter {
    fn write_str(&mut self, _s: &str) -> fmt::Result {
        Ok(())
    }
} 