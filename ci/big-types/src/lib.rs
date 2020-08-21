#![no_std]

const INIT_COMMAND: u8 = 0;

pub struct Interface;

impl Interface {
    pub fn write_cmd(&mut self, cmd: u8) -> Result<(), ()> {

    }
}

pub enum DisplayError {
    IfaceError,
}

pub struct Object;

pub mod owned {
    pub struct Display {
        // Some hardware interface.
        iface: Interface,
        buffer: [u8; 1024],
        // ..
    }
    
    impl Display {
        /// Creates and initializes the display driver.
        pub fn new(mut iface: Interface) -> Result<Self, DisplayError> {
            // Send some init commands to the display
            iface.write_cmd(INIT_COMMAND).map_err(|_| DisplayError::IfaceError)?;
            // ..
    
            Self {
                iface,
                buffer: [0; 1024],
            }
        }
    
        /// Draw to the screen.
        pub fn draw(&mut self, obj: Object) -> Result<(), DisplayError> {
            // ..
        }
    }
}

pub mod borrowed {
    pub struct Display<'a> {
        // Some hardware interface.
        iface: Interface,
        buffer: &'a mut [u8; 1024],
        // ..
    }
    
    impl<'a> Display<'a> {
        /// Creates and initializes the display driver.
        pub fn new(mut iface: Interface, buffer: &'a mut [u8; 1024]) -> Result<Self, DisplayError> {
            // Send some init commands to the display
            iface.write_cmd(INIT_COMMAND).map_err(|_| DisplayError::IfaceError)?;
            // ..
    
            Self {
                iface,
                buffer,
            }
        }
    
        /// Draw to the screen.
        pub fn draw(&mut self, obj: Object) -> Result<(), DisplayError> {
            // ..
        }
    }
}
