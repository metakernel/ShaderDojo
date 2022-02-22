

use std::error::Error;

#[cfg(target_os = "windows")]
use shaderdojo_native::windows_native::windows_main::win_main;

fn main() -> Result<(), Box<dyn Error>> {
    
    #[cfg(target_os = "windows")]
    win_main().expect("Failed to initialize window");

    #[cfg(not(target_os = "windows"))]
    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Not implemented")));

    Ok(())
}
