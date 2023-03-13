use crate::{EventType, SimulateError};
use std::sync::Mutex;
use uinput::device::*;
use uinput::event::*;
use crate::rdev::Key as RdevKey;
use uinput::event::keyboard::Key as EvdevKey;

// to-do: try remove unwrap() here
// use Option<Device> instead
lazy_static::lazy_static! {
    pub static ref DEVICE: Mutex<Device> = Mutex::new(uinput::default().unwrap()
    .name("AccessPro").unwrap()
    .event(uinput::event::Keyboard::All).unwrap()
    .create().unwrap());
}

fn rdev_key_to_evdev_key(key: RdevKey) -> EvdevKey{
    match key {
        RdevKey::Alt => EvdevKey::LeftAlt,
        RdevKey::AltGr => EvdevKey::RightAlt,
        RdevKey::Backspace => EvdevKey::BackSpace,
        RdevKey::CapsLock => EvdevKey::CapsLock,

        RdevKey::Num1 => EvdevKey::_1,
        RdevKey::KeyA => EvdevKey::A,
        RdevKey::ShiftLeft => EvdevKey::LeftShift,
        RdevKey::KeyB => EvdevKey::B,
        _ => EvdevKey::A
    }
}

// to-do: try remove unwrap() here
pub fn simulate_wayland(event_type: &EventType) -> Result<(), SimulateError> {
    match event_type {
        EventType::KeyPress(key) => {
            let uinput_event = rdev_key_to_evdev_key(*key);
            println!("KeyPress {:?}", uinput_event.code());

            DEVICE.lock().unwrap().press(&uinput_event);
            DEVICE.lock().unwrap().synchronize().unwrap();
        }
        EventType::KeyRelease(key) => {
            let uinput_event = rdev_key_to_evdev_key(*key);
            println!("KeyRelease {:?}", uinput_event.code());
            
            DEVICE.lock().unwrap().release(&uinput_event);
            DEVICE.lock().unwrap().synchronize().unwrap();
        }
        _ => {}
    }
    Ok(())
}
