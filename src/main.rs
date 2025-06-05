use moon_driver_loader::{
    ioctl::{FILE_DEVICE_UNKNOWN, METHOD_BUFFERED},
    loader::{mapper_loader::MapperDriverLoader, DriverLoader},
    CTL_CODE,
};

#[repr(C)]
struct DeviceIoTestOut {
    length: u16,         // version
    maximum_length: u16, // vmx abort reason. vmx abort:vmexit fault
}

const IOCTL_DEVICE_IO_CONTROL_TEST: u32 =
    CTL_CODE!(FILE_DEVICE_UNKNOWN, 0x2000, METHOD_BUFFERED, 0);

fn main() {
    let mut mapper = MapperDriverLoader::new();
    let r = mapper.load("file");
    if r.is_ok() {
        println!("成功");
    } else {
        println!("失败，{:?}", r.unwrap_err());
    }

    // let mut unload = false;
    // let args: Vec<String> = env::args().collect();
    // if args.contains(&String::from("unload")) {
    //     unload = true;
    // }

    // let mut loader = ServiceDriverLoader::new();
    // let file_path = "C:\\Users\\Administrator\\Desktop\\rust_driver.sys";

    // if unload {
    //     let r = loader.unload(&file_path);
    //     if r.is_err() {
    //         println!("error to unload:{:?}", r.err().unwrap());
    //         return;
    //     }

    //     println!("success to unload");
    // } else {
    //     let r = loader.load(file_path);
    //     if r.is_err() {
    //         println!("error to load:{:?}", r.err().unwrap());
    //         return;
    //     }

    //     let input = DeviceIoTestOut {
    //         length: 3,
    //         maximum_length: 4,
    //     };
    //     let mut output = DeviceIoTestOut {
    //         length: 0,
    //         maximum_length: 0,
    //     };

    //     let mut cc = Communication {
    //         code: IOCTL_DEVICE_IO_CONTROL_TEST,
    //         input: &input,
    //         output: &mut output,
    //     };

    //     let r = io_ctl("\\\\.\\20240703", &mut cc);
    //     if r.is_err() {
    //         println!("ioctl error:{}", r.err().unwrap());
    //         return;
    //     } else {
    //         println!("ioctl success:{},{}", output.length, output.maximum_length);
    //     }

    //     println!("success to load");
    // }
}
