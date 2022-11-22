use std::{error, arch::global_asm, path::PathBuf};

use thiserror::Error;
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

#[derive(Debug, Error)]
enum ExportError {
    #[error("Failed to load library")]
    LoadLibrary,
    #[error("Failed to get function address")]
    GetProcAddress,
    #[error("Failed to get module path")]
    GetModulePath,
    #[error("Failed to get module name")]
    GetModuleName,
    #[error("Proxy has an invalid file name")]
    InvalidFileName,
}

pub trait ProxyDll {
    fn get_path(&self) -> Result<PathBuf, Box<dyn error::Error>>;
    fn is_compatible(&self) -> Result<bool, Box<dyn error::Error>>;
}

impl ProxyDll for HINSTANCE {
    fn get_path(&self) -> Result<PathBuf, Box<dyn error::Error>> {
        let mut path = [0u16; 260];
        let len = unsafe { 
            winapi::um::libloaderapi::GetModuleFileNameW(
                *self, 
                path.as_mut_ptr(), 
                path.len() as u32
            ) 
        };

        match len {
            0 => Err("GetModuleFileNameW failed".into()),
            _ => Ok(PathBuf::from(
                String::from_utf16_lossy(&path[..len as usize])
            ))
        }
    }

    fn is_compatible(&self) -> Result<bool, Box<dyn error::Error>> {
        let path = self.get_path()?;
        let file_name = path.file_stem().ok_or_else(|| ExportError::GetModuleName)?;
        let file_name = file_name.to_str().ok_or_else(|| ExportError::GetModuleName)?;
        let file_name = file_name.to_lowercase();

        Ok(file_name.eq("version.dll") || file_name.eq("winhttp.dll") || file_name.eq("winmm.dll"))
    }
}

pub fn initialize(module: HINSTANCE) -> Result<(), Box<dyn error::Error>> {
    if module.is_null() {
        return Err(Box::new(ExportError::LoadLibrary));
    }

    if !module.is_compatible()? {
        return Err(Box::new(ExportError::InvalidFileName));
    }

    Ok(())
}