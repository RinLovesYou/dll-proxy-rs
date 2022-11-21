use std::{error, arch::global_asm};

use winapi::shared::minwindef::{HINSTANCE, FARPROC};

#[no_mangle]
static mut OriginalFuncs_version: [FARPROC; 17] = [0 as FARPROC; 17];

#[no_mangle]
static mut OriginalFuncs_winhttp: [FARPROC; 65] = [0 as FARPROC; 65];

#[no_mangle]
static mut OriginalFuncs_winmm: [FARPROC; 181] = [0 as FARPROC; 181];

#[cfg(target_pointer_width = "64")]
global_asm!(include_str!("deps/version.x64.S"));
#[cfg(target_pointer_width = "32")]
global_asm!(include_str!("deps/version.x86.S"));

#[cfg(target_pointer_width = "64")]
global_asm!(include_str!("deps/winhttp.x64.S"));
#[cfg(target_pointer_width = "32")]
global_asm!(include_str!("deps/winhttp.x86.S"));

#[cfg(target_pointer_width = "64")]
global_asm!(include_str!("deps/winmm.x64.S"));
#[cfg(target_pointer_width = "32")]
global_asm!(include_str!("deps/winmm.x86.S"));

pub fn initialize(module: HINSTANCE) -> Result<(), Box<dyn error::Error>> {
    Ok(())
}