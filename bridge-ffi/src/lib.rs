/**
* Copyright 2020 Hadi Lashkari Ghouchani

* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at

* http://www.apache.org/licenses/LICENSE-2.0

* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/
use std::ffi::CString;
use std::mem;
use std::mem::ManuallyDrop;
use std::os::raw::c_char;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::app::{act, register_for_logger, register_for_updates, state};
use crate::ffi::{
    DartCObject, DartCObjectType, DartCObjectValue, DartLoggerFn, DartNativeTypedData, DartPort,
    DartPostCObjectFn, DartTypedDataType,
};

mod app;
mod ffi;
mod test;

lazy_static! {
    static ref POST_COBJECT: Arc<Mutex<Option<DartPostCObjectFn>>> = Arc::new(Mutex::new(None));
}

lazy_static! {
    static ref NEXT_PORT: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(None));
}

lazy_static! {
    static ref LOGGER: Arc<Mutex<Option<DartLoggerFn>>> = Arc::new(Mutex::new(None));
}

pub fn notify_for_update(data: Vec<u8>) {
    let mut next = NEXT_PORT.lock().unwrap();
    if let Some(port) = mem::replace(&mut *next, None) {
        notify(port, data);
    };
}

pub fn notify(port: i64, data: Vec<u8>) -> bool {
    unsafe {
        if let Some(func) = POST_COBJECT.lock().unwrap().as_ref() {
            let mut vec = ManuallyDrop::new(data);
            let data = DartCObject {
                ty: DartCObjectType::DartCObjectKTypedData,
                value: DartCObjectValue {
                    as_typed_data: DartNativeTypedData {
                        ty: DartTypedDataType::DartTypedDataKUint8,
                        length: vec.len() as isize,
                        values: vec.as_mut_ptr() as *mut _,
                    },
                },
            };
            let mut boxed = Box::new(data);
            let result = func(port, boxed.as_mut() as *mut DartCObject);
            drop(boxed);
            drop(vec);
            result
        } else {
            false
        }
    }
}

pub fn log(msg: String) -> bool {
    unsafe {
        if let Some(func) = LOGGER.lock().unwrap().as_ref() {
            let message = ManuallyDrop::new(CString::new(msg).unwrap());
            let data = message.as_ptr() as *mut c_char;
            let result = func(data);
            drop(message);
            result
        } else {
            false
        }
    }
}

fn create_pointer(data: Vec<u8>) -> *mut Event {
    let boxed_msg = Box::new(into_dart(data));
    Box::into_raw(boxed_msg)
}

fn into_dart(data: Vec<u8>) -> Event {
    let mut vec = ManuallyDrop::new(data);
    Event {
        length: vec.len() as u32,
        values: vec.as_mut_ptr() as *mut _,
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Event {
    pub length: u32,
    pub values: *mut u8,
}

#[no_mangle]
pub unsafe extern "C" fn setup_post_cobject(ptr: Option<DartPostCObjectFn>) {
    let mut post = POST_COBJECT.lock().unwrap();
    *post = ptr;
    register_for_updates();
}

#[no_mangle]
pub unsafe extern "C" fn setup_logger(ptr: Option<DartLoggerFn>) {
    let mut post = LOGGER.lock().unwrap();
    *post = ptr;
    register_for_logger();
}

#[no_mangle]
pub unsafe extern "C" fn next_port(port_id: DartPort) {
    let mut next = NEXT_PORT.lock().unwrap();
    *next = Some(port_id)
}

#[no_mangle]
pub unsafe extern "C" fn do_action(data: *mut Event) {
    if let Some(data) = data.as_mut() {
        let length = (*data).length as usize;
        let vec = Vec::from_raw_parts((*data).values, length, length);
        act(vec.clone().to_vec());
        let _ = ManuallyDrop::new(vec);
    }
}

#[no_mangle]
pub unsafe extern "C" fn current_state() -> *mut Event {
    create_pointer(state())
}

#[no_mangle]
pub unsafe extern "C" fn drop_pointer(ptr: *mut DartCObject) {
    let boxed_obj = Box::from_raw(ptr);
    drop(boxed_obj);
}

#[no_mangle]
pub unsafe extern "C" fn dummy_method_for_dummy_xcode() -> i32 {
    0
}
