use hidapi::{DeviceInfo, HidApi, HidDevice, HidError};
use slint::SharedString;

#[derive(Debug)]
pub struct KeyConfig {
    pub reserved: i32,
    pub ctrl: i32,
    pub shift: i32,
    pub alt: i32,
    pub key: i32
}

pub struct Devices(pub Vec<DeviceInfo>);

impl slint::Model for Devices {
    type Data = SharedString;

    fn row_count(&self) -> usize {
        self.0.len()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.0.get(row).map(|x| format!("{:#06x}:{:#06x} - {:?} ({:?})", x.vendor_id(), x.product_id(), x.product_string().unwrap(), x.path() ).try_into().unwrap() )
    }

    fn model_tracker(&self) -> &dyn slint::ModelTracker {
        &()
    }
}


pub struct HidService {
    hidapi: HidApi,
    device_list: Vec<DeviceInfo>,
    device: Result<HidDevice, HidError>
}

impl HidService {
    pub fn new() -> Self {
        Self {
            hidapi: match HidApi::new() {
                Ok(api) => api,
                Err(err) => panic!("Failed to initiate HidApi ({:?})", err)
            }, 
            device_list: vec![],
            device: Err(HidError::InitializationError)
        }
    }

    pub fn load_devices(&mut self) {
        self.device_list = self.hidapi.device_list().filter(|d| {
            d.vendor_id() == 0x2341 && d.product_id() == 0x8037
        }).map(|d| d.clone()).collect();
    }

    pub fn list_devices(&mut self) -> Vec<DeviceInfo>{
        <Vec<DeviceInfo> as Clone>::clone(&self.device_list)
    }

    pub fn connect(&mut self, idx: usize) {

        println!("2 Devices: {:?}", self.device_list);

        let device_info = <Vec<DeviceInfo> as Clone>::clone(&self.device_list).into_iter()
            .nth(idx)
            .expect("No devices are available!")
            .clone();

        self.device = match device_info.open_device(&self.hidapi) {
            Ok(ret) =>  Ok(ret),
            Err(err) => Err(err)
        }
    }

    pub fn write(&mut self, idx:i32, is_ctrl:bool, is_shift:bool, is_alt:bool, key:u8) {
        println!("Writing data on device ...\n");
            match &self.device { 
                Ok(dev) => {
                    match dev.write(&[
                        0xCC,                           // WRITE COMMAND
                        idx.try_into().unwrap(),        // INDEX
                        0x00,                           // RESERVED
                        if is_ctrl {0x80} else {0x00},  // CTRL
                        if is_shift {0x81} else {0x00}, // SHIFT
                        if is_alt {0x82} else {0x00},   // ALT
                        key               // KEY
                    ]) {
                        Ok(ret) => println!("Wrote data ({:?})", ret),
                        Err(err) => println!("Failed to write data ({:?})", err)
                    }
                },
                Err(err) => println!("Failed to write data ({:?})", err)
            };
    }

    pub fn read(&mut self, idx:i32) -> KeyConfig {
        println!("Reading data from device ...\n");
        let mut ret_buffer: [u8; 5] = [0; 5];
        match &self.device { 
            Ok(dev) => {
                match dev.write(&[
                    0xCB,                           // WRITE COMMAND
                    idx.try_into().unwrap(),        // INDEX
                ]) {
                    Ok(_ret) => {
                        
                        for n in 0..5 {
                            let mut buffer: [u8; 1] = [0; 1];  
                            let _ = dev.read(&mut buffer);
                            ret_buffer[n] = buffer[0];
                        }
                    },
                    Err(err) => println!("Failed to read data ({:?})", err)
                }

                
            },
            Err(err) => println!("Failed to write data ({:?})", err)
        };


        KeyConfig { reserved: ret_buffer[0].into(), ctrl: ret_buffer[1].into(), shift: ret_buffer[2].into(), alt: ret_buffer[3].into(), key: ret_buffer[4].into() }

    }
}