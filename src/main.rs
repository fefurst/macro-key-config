slint::include_modules!();

mod hid_service;
mod keyvalues;

use std::{rc::Rc, sync::{Arc, Mutex}};
use keyvalues::KeyValues;
use hid_service::{ HidService, Devices };
use slint::PlatformError;

fn main() -> Result<(), PlatformError> {

    let service = Arc::new(Mutex::new(HidService::new()));

    let keyvalues = Arc::new(Mutex::new(KeyValues::new()));

    service.lock().unwrap().load_devices();

    let ui = match AppWindow::new() {
        Ok(ui) => ui,
        Err(err) => panic!("Failed to initiate AppWindow ({:?})", err)
    };


    let devs = Devices(service.lock().unwrap().list_devices());
    let devs = Rc::new(devs);    
    ui.set_list_devices(slint::ModelRc::from(devs.clone()));


    let keys = keyvalues.lock().unwrap().to_owned();
    let keys = Rc::new(keys);    
    ui.set_list_keys(slint::ModelRc::from(keys.clone()));


    ui.on_connect({
        let ui_handle = ui.as_weak();
        let service = Arc::clone(&service);
        move | idx | {
            let _ui = ui_handle.unwrap();

            service.lock().unwrap().connect(idx.try_into().unwrap());

            _ui.set_connected(true);
            
        }
    });

    
    ui.on_write_key_config({
        let ui_handle = ui.as_weak();
        let service = Arc::clone(&service);
        let keyvalues = Arc::clone(&keyvalues);
        move |idx, is_ctrl, is_shift, is_alt, key| {
            let _ui = ui_handle.unwrap();

            let byte_key = keyvalues.lock().unwrap().byte_key_by_desc(key);

            service.lock().unwrap().write(idx, is_ctrl, is_shift, is_alt, byte_key);
        }
    });

    ui.on_read_key_config({
        let ui_handle = ui.as_weak();
        let service = Arc::clone(&service);
        let keyvalues = Arc::clone(&keyvalues);
        move |idx| {
            let ui = ui_handle.unwrap();
            let read = service.lock().unwrap().read(idx);

            ui.set_is_ctrl(read.ctrl==0x80);
            ui.set_is_shift(read.shift==0x81);
            ui.set_is_alt(read.alt==0x82);

            ui.set_selected_key(keyvalues.lock().unwrap().byte_key_index(read.key.try_into().unwrap()).try_into().unwrap());
            ui.set_selected_value(keyvalues.lock().unwrap().desc_key_by_byte(read.key.try_into().unwrap()).clone());
        }

    });

    ui.run()
}

//2341:8037 