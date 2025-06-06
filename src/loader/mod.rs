use moon_win32_utils::{
    registry::{create_registry_key, registry_key_exists, set_registry_value},
    string::string_to_u16_slice_u8,
};
use windows::Win32::{
    Foundation::{NTSTATUS, STATUS_ALREADY_REGISTERED, STATUS_UNSUCCESSFUL},
    System::Registry::{HKEY_LOCAL_MACHINE, REG_DWORD, REG_EXPAND_SZ},
};

pub mod mapper_loader;
pub mod service_loader;

pub trait DriverLoader {
    fn load(&mut self, file: &str) -> Result<(), NTSTATUS>;
    fn unload(&mut self, file: &str) -> Result<(), NTSTATUS>;
}

fn write_driver_info_to_registry(file: &str, app_sub_key: String) -> Result<(), NTSTATUS> {
    if registry_key_exists(HKEY_LOCAL_MACHINE, &app_sub_key) {
        return Err(STATUS_ALREADY_REGISTERED);
    }

    let hkey =
        create_registry_key(HKEY_LOCAL_MACHINE, &app_sub_key).map_err(|_| STATUS_UNSUCCESSFUL)?;

    let image_path = format!("\\??\\{}", file);

    set_registry_value(
        &hkey,
        "ImagePath",
        REG_EXPAND_SZ,
        string_to_u16_slice_u8(&image_path, true).as_slice(),
    )
    .map_err(|_| STATUS_UNSUCCESSFUL)?;

    set_registry_value(&hkey, "Type", REG_DWORD, &1u32.to_le_bytes())
        .map_err(|_| STATUS_UNSUCCESSFUL)?;

    set_registry_value(&hkey, "Start", REG_DWORD, &3u32.to_le_bytes())
        .map_err(|_| STATUS_UNSUCCESSFUL)?;

    Ok(())
}
