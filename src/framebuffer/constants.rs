// Color constants
pub const RED: [u8; 3] = [255, 0, 0];
pub const ORANGE: [u8; 3] = [255, 165, 0];
pub const YELLOW: [u8; 3] = [255, 255, 0];
pub const GREEN: [u8; 3] = [0, 255, 0];
pub const CYAN: [u8; 3] = [0, 255, 255];
pub const BLUE: [u8; 3] = [0, 0, 255];
pub const MAGENTA: [u8; 3] = [255, 0, 255];
pub const WHITE: [u8; 3] = [255, 255, 255];
pub const BLACK: [u8; 3] = [0, 0, 0];

// Framebuffer configuration
pub const WIDTH: usize = 1024;
pub const HEIGHT: usize = 768;
pub const BPP: usize = 4;  // 32-bit color (BGRA)

// VBE Registers (Bochs VBE Extensions)
pub const VBE_DISPI_INDEX_ID: u16 = 0x0;
pub const VBE_DISPI_INDEX_XRES: u16 = 0x1;
pub const VBE_DISPI_INDEX_YRES: u16 = 0x2;
pub const VBE_DISPI_INDEX_BPP: u16 = 0x3;
pub const VBE_DISPI_INDEX_ENABLE: u16 = 0x4;
pub const VBE_DISPI_INDEX_BANK: u16 = 0x5;
pub const VBE_DISPI_INDEX_VIRT_WIDTH: u16 = 0x6;
pub const VBE_DISPI_INDEX_VIRT_HEIGHT: u16 = 0x7;
pub const VBE_DISPI_INDEX_X_OFFSET: u16 = 0x8;
pub const VBE_DISPI_INDEX_Y_OFFSET: u16 = 0x9;

// VBE Ports
pub const VBE_DISPI_IOPORT_INDEX: u16 = 0x01CE;
pub const VBE_DISPI_IOPORT_DATA: u16 = 0x01CF;

// VBE Flags
pub const VBE_DISPI_ENABLED: u16 = 0x01;
pub const VBE_DISPI_LFB_ENABLED: u16 = 0x40; 