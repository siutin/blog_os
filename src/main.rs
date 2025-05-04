#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use blog_os::println;
use blog_os::task::{Task, executor::Executor, keyboard};
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use x86_64::{structures::paging::{Page, PhysFrame, Size4KiB, FrameAllocator, Mapper, PageTableFlags, mapper::MapToError}, PhysAddr, VirtAddr};

mod framebuffer;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::allocator;
    use blog_os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    println!("Initializing framebuffer...");
        
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // Map the framebuffer memory region
    const FRAMEBUFFER_ADDR: u64 = framebuffer::FB_ADDR;
    const FRAMEBUFFER_SIZE: usize = framebuffer::BUFFER_SIZE;
    
    println!("Mapping framebuffer memory at 0x{:X}...", FRAMEBUFFER_ADDR);
    
    // Map each page in the framebuffer range
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_CACHE;
    
    for page_addr in (0..FRAMEBUFFER_SIZE).step_by(4096) {
        let page = Page::<Size4KiB>::containing_address(VirtAddr::new(FRAMEBUFFER_ADDR + page_addr as u64));
        let frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(FRAMEBUFFER_ADDR + page_addr as u64));
        unsafe {
            mapper.map_to(page, frame, flags, &mut frame_allocator)
                  .expect("failed to map framebuffer memory")
                  .flush();
        }
    }
    
    println!("Framebuffer memory mapped successfully");

    // Initialize framebuffer
    let mut fb = framebuffer::FrameBufferWriter::new();
    
    println!("Framebuffer initialized: {}x{}", fb.width(), fb.height());
    
    // Clear the screen
    fb.clear_screen();
    
    println!("Drawing test pattern...");
    
    // Draw some test patterns to verify it's working
    // Draw red, green, blue vertical bars to make it obvious when it's working
    let bar_width = fb.width() / 3;
    
    fb.draw_rect(0, 0, bar_width, fb.height(), &framebuffer::RED);
    fb.draw_rect(bar_width, 0, bar_width, fb.height(), &framebuffer::GREEN);
    fb.draw_rect(bar_width * 2, 0, bar_width, fb.height(), &framebuffer::BLUE);
    
    // Draw a white rectangle in the middle
    let rect_width = 300;
    let rect_height = 200;
    let x = (fb.width() - rect_width) / 2;
    let y = (fb.height() - rect_height) / 2;
    fb.draw_rect(x, y, rect_width, rect_height, &framebuffer::BLACK);
    
    // draw a circle with red border and filled with blue
    let circle_x = fb.width() / 2;
    let circle_y = fb.height() / 2;
    let circle_radius = 50;
    fb.draw_circle(circle_x, circle_y, circle_radius, &framebuffer::BLUE, &framebuffer::RED);
    
    // Draw some text
    fb.draw_text(x + 20, y + 20, "Hello from MyOS!", &framebuffer::BLACK, Some(&framebuffer::WHITE));
    fb.draw_text(x + 20, y + 40, "Framebuffer text rendering", &framebuffer::ORANGE, None);
    fb.draw_text(x + 20, y + 60, "ABCDEFGHIJKLMNOPQRSTUVWXYZ", &framebuffer::YELLOW, None);
    fb.draw_text(x + 20, y + 80, "abcdefghijklmnopqrstuvwxyz", &framebuffer::CYAN, None);
    fb.draw_text(x + 20, y + 100, "0123456789!@#$%^&*()", &framebuffer::MAGENTA, None);

    println!("Test pattern complete");

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    // blog_os::hlt_loop();
    
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
