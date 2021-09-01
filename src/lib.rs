//! Detect if dark mode or light mode is enabled.
//!
//! # Examples
//!
//! ```
//! fn main() {
//!     let mode = dark_light::detect();
//!
//!     match mode {
//!         dark_light::Mode::Dark => {},
//!         dark_light::Mode::Light => {},
//!     }
//! }
//! ```

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos as platform;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as platform;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux as platform;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
mod platform {
    pub fn detect() -> crate::Mode {
        Mode::Light
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mode {
    Dark,
    Light,
}

/// Detect if light mode or dark mode is enabled. If the mode can’t be detected, fall back to [`Mode::Light`].
pub fn detect() -> Mode {
    platform::detect()
}

/// Watch if light mode or dark mode and return as soon as it changes.
pub fn watch() -> crate::Mode {
    let handle = std::thread::spawn(|| {
        loop {
            let mode = detect();
            println!("Current mode: {:?}", mode);
            loop {
                return if detect() != mode {
                    println!("New mode: {:?}", detect());
                    detect()
                } else {
                    mode
                }
            }
        }
    });
    handle.join().unwrap()
}
