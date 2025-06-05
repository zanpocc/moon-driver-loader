use std::path::Path;

use moon_win32_utils::{
    ntdll_wrap::{adjust_privilege, nt_load_driver, nt_unload_driver},
    registry::{delete_registry_key, registry_key_exists},
};
use windows::Win32::{
    Foundation::{NTSTATUS, STATUS_INVALID_PARAMETER, STATUS_UNSUCCESSFUL},
    System::Registry::HKEY_LOCAL_MACHINE,
};

use crate::loader::{write_driver_info_to_registry, DriverLoader};

pub struct ServiceDriverLoader;

impl ServiceDriverLoader {
    pub fn new() -> Self {
        Self {}
    }
}

impl DriverLoader for ServiceDriverLoader {
    // service load driver
    fn load(&mut self, file: &str) -> Result<(), NTSTATUS> {
        let image_path = format!("\\\\?\\{}", file);
        match std::fs::metadata(image_path) {
            Ok(meta) => println!("Driver exists, size: {}", meta.len()),
            Err(err) => {
                println!("Failed to access driver file: {}", err);
                return Err(STATUS_UNSUCCESSFUL);
            }
        }

        // 1
        adjust_privilege(10)?;
        adjust_privilege(20)?;

        // 2
        let file_name = Path::new(file).file_stem().and_then(|stem| stem.to_str());
        if file_name.is_none() {
            return Err(STATUS_INVALID_PARAMETER);
        }

        let file_name = file_name.unwrap();

        let app_sub_key = format!("System\\CurrentControlSet\\Services\\{}", file_name);

        write_driver_info_to_registry(file, app_sub_key)?;

        nt_load_driver(file_name)?;

        return Ok(());
    }

    // service load driver
    fn unload(&mut self, file: &str) -> Result<(), NTSTATUS> {
        let image_path = format!("\\\\?\\{}", file);
        match std::fs::metadata(image_path) {
            Ok(meta) => println!("Driver exists, size: {}", meta.len()),
            Err(err) => {
                println!("Failed to access driver file: {}", err);
                return Err(STATUS_UNSUCCESSFUL);
            }
        }

        // 1
        adjust_privilege(10)?;
        adjust_privilege(20)?;

        // 2
        let file_name = Path::new(file).file_stem().and_then(|stem| stem.to_str());
        if file_name.is_none() {
            return Err(STATUS_INVALID_PARAMETER);
        }

        let app_sub_key = format!(
            "System\\CurrentControlSet\\Services\\{}",
            file_name.unwrap()
        );

        if !registry_key_exists(HKEY_LOCAL_MACHINE, &app_sub_key) {
            return Err(STATUS_UNSUCCESSFUL);
        }

        // 3
        nt_unload_driver(file_name.unwrap())?;

        let r = delete_registry_key(HKEY_LOCAL_MACHINE, &app_sub_key);
        if r.is_err() {
            return Err(STATUS_UNSUCCESSFUL);
        }

        return Ok(());
    }
}
