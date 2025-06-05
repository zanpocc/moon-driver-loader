use windows::{
    core::{Error, PCWSTR},
    Win32::{
        Foundation::{CloseHandle, GENERIC_ALL, HANDLE},
        Storage::FileSystem::{CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_NONE, OPEN_EXISTING},
        System::IO::DeviceIoControl,
    },
};

#[macro_export]
macro_rules! CTL_CODE {
    ($DeviceType:expr, $Function:expr, $Method:expr, $Access:expr) => {
        ((($DeviceType as u32) << 16)
            | (($Access as u32) << 14)
            | (($Function as u32) << 2)
            | ($Method as u32))
    };
}

pub const FILE_DEVICE_UNKNOWN: u32 = 0x00000022;
pub const METHOD_BUFFERED: u32 = 0;

#[repr(C)]
#[derive(Debug)]
pub struct Communication<'a, T, U> {
    pub code: u32,
    pub input: &'a T,
    pub output: &'a mut U,
}

fn open_file(symbolic_link_path: &str) -> windows::core::Result<HANDLE> {
    let wide_path: Vec<u16> = symbolic_link_path.encode_utf16().chain(Some(0)).collect();

    let handle = unsafe {
        CreateFileW(
            PCWSTR(wide_path.as_ptr()),
            GENERIC_ALL.0,
            FILE_SHARE_NONE,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            HANDLE::default(),
        )
    };

    if handle.is_err() {
        return Err(handle.err().unwrap());
    }

    Ok(handle.unwrap())
}

pub fn is_driver_load_by_symlink(symbolic_link_path: &str) -> bool {
    match open_file(symbolic_link_path) {
        Ok(h) => {
            unsafe {
                let _ = CloseHandle(h);
            };
            return true;
        }
        _ => {
            return false;
        }
    }
}

pub fn io_ctl<T, U>(symbol_link: &str, cc: &mut Communication<T, U>) -> Result<(), Error> {
    let handle = open_file(symbol_link);
    if handle.is_err() {
        return Err(handle.err().unwrap());
    }

    let handle = handle.unwrap();

    let mut return_byte: u32 = 0;

    let r = unsafe {
        DeviceIoControl(
            handle,
            cc.code,
            Some(cc.input as *const _ as _),
            core::mem::size_of::<T>() as _,
            Some(cc.output as *mut _ as _),
            core::mem::size_of::<U>() as _,
            Some(&mut return_byte as *mut _),
            None,
        )
    };

    if r.is_err() {
        return Err(r.err().unwrap());
    }

    unsafe {
        let _ = CloseHandle(handle);
    };

    Ok(())
}
