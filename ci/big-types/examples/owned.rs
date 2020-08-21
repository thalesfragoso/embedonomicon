#![no_std]

use core::{cell::RefCell, sync::atomic::{compiler_fence, Ordering}};
use cortex_m_rt::entry;
use bare_metal::Mutex;
use big_types::{Interface, Object, owned::Display};
use cortex_m::interrupt::{free as interrupt_free};


static mut DISPLAY: Mutex<RefCell<Option<Display>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // Get the interface.
    let mut iface = interface {};

    interrupt_free(move |cs| {
        let display = DISPLAY.borrow(cs).borrow_mut();
        *display = Some(Display::new(iface)).unwrap();
    });

    loop {
        // Do some stuff.
        compiler_fence(Ordering::SeqCst);
    }
}

#[interrupt]
fn USART1() {
    // We got some command to draw an object, draw it.
    let object = Object {};
    interrupt_free(|cs| {
        let some_display = DISPLAY.borrow(cs).borrow_mut();
        if let Some(display) = some_display {
            display.draw(object);
        }
    })
}
