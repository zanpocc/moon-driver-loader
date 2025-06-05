use moon_win32_utils::{file_utils::get_temp_path, file_utils::random_file_name};
use windows::Win32::Foundation::NTSTATUS;

use crate::{
    binary::write_to_disk,
    loader::{service_loader::ServiceDriverLoader, DriverLoader},
};

pub struct MapperDriverLoader {
    service_loader: ServiceDriverLoader,
    vulner_drvier_path: Option<String>,
}

impl MapperDriverLoader {
    pub fn new() -> Self {
        Self {
            service_loader: ServiceDriverLoader::new(),
            vulner_drvier_path: None,
        }
    }

    fn release_driver_file(&self) -> String {
        let temp = get_temp_path();
        let file_name = random_file_name(None);
        let file_path = format!("{}{}.sys", temp, file_name);
        write_to_disk(&file_path);
        file_path
    }
}

impl DriverLoader for MapperDriverLoader {
    fn load(&mut self, _file: &str) -> Result<(), NTSTATUS> {
        let vulner_driver_path = self.release_driver_file();

        self.service_loader.load(&vulner_driver_path)?;
        self.vulner_drvier_path = Some(vulner_driver_path);

        Ok(())
    }

    fn unload(&mut self, _file: &str) -> Result<(), NTSTATUS> {
        todo!()
    }
}
