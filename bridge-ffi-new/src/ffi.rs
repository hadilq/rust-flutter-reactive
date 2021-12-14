#![allow(missing_docs)]
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
use std::os::raw::c_char;
use std::{ffi::CString, os::raw};

/// A port is used to send or receive inter-isolate messages
pub type DartPort = i64;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DartTypedDataType {
    DartTypedDataKByteData = 0,
    DartTypedDataKInt8 = 1,
    DartTypedDataKUint8 = 2,
    DartTypedDataKUint8Clamped = 3,
    DartTypedDataKInt16 = 4,
    DartTypedDataKUint16 = 5,
    DartTypedDataKInt32 = 6,
    DartTypedDataKUint32 = 7,
    DartTypedDataKInt64 = 8,
    DartTypedDataKUint64 = 9,
    DartTypedDataKFloat32 = 10,
    DartTypedDataKFloat64 = 11,
    DartTypedDataKInt32x4 = 12,
    DartTypedDataKFloat32x4 = 13,
    DartTypedDataFloat64x2 = 14,
    DartTypedDataKInvalid = 15,
}

/// A Dart_CObject is used for representing Dart objects as native C
/// data outside the Dart heap. These objects are totally detached from
/// the Dart heap. Only a subset of the Dart objects have a
/// representation as a Dart_CObject.
///
/// The string encoding in the 'value.as_string' is UTF-8.
///
/// All the different types from dart:typed_data are exposed as type
/// kTypedData. The specific type from dart:typed_data is in the type
/// field of the as_typed_data structure. The length in the
/// as_typed_data structure is always in bytes.
///
/// The data for kTypedData is copied on message send and ownership remains with
/// the caller. The ownership of data for kExternalTyped is passed to the VM on
/// message send and returned when the VM invokes the
/// Dart_WeakPersistentHandleFinalizer callback; a non-NULL callback must be
/// provided.
#[repr(i32)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum DartCObjectType {
    DartCObjectKNull = 0,
    DartCObjectKBool = 1,
    DartCObjectKInt32 = 2,
    DartCObjectKInt64 = 3,
    DartCObjectKDouble = 4,
    DartCObjectKString = 5,
    DartCObjectKTypedData = 7,
    DartCObjectKExternalTypedData = 8,
    DartCObjectKSendPort = 9,
    DartCObjectKCapability = 10,
    DartCObjectKUnsupported = 11,
    DartCObjectKNumberOfTypes = 12,
}

#[allow(missing_debug_implementations)]
#[repr(C)]
pub struct DartCObject {
    pub ty: DartCObjectType,
    pub value: DartCObjectValue,
}

#[allow(missing_debug_implementations)]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DartCObjectValue {
    pub as_bool: bool,
    pub as_int32: i32,
    pub as_int64: i64,
    pub as_double: f64,
    pub as_string: *mut raw::c_char,
    pub as_send_port: DartNativeSendPort,
    pub as_capability: DartNativeCapability,
    pub as_array: DartNativeArray,
    pub as_typed_data: DartNativeTypedData,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DartNativeSendPort {
    pub id: DartPort,
    pub origin_id: DartPort,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DartNativeCapability {
    pub id: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DartNativeArray {
    pub length: isize,
    pub values: *mut *mut DartCObject,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DartNativeTypedData {
    pub ty: DartTypedDataType,
    pub length: isize,
    pub values: *mut u8,
}

///  Posts a message on some port. The message will contain the
///  Dart_CObject object graph rooted in 'message'.
///
///  While the message is being sent the state of the graph of
///  Dart_CObject structures rooted in 'message' should not be accessed,
///  as the message generation will make temporary modifications to the
///  data. When the message has been sent the graph will be fully
///  restored.
///
///  `port_id` The destination port.
///  `message` The message to send.
///
///  return true if the message was posted.
pub type DartPostCObjectFn =
    unsafe extern "C" fn(port_id: DartPort, message: *mut DartCObject) -> bool;

pub type DartLoggerFn = unsafe extern "C" fn(message: *mut c_char) -> bool;

impl Drop for DartCObject {
    fn drop(&mut self) {
        if self.ty == DartCObjectType::DartCObjectKString {
            let _ = unsafe { CString::from_raw(self.value.as_string) };
        } else if self.ty == DartCObjectType::DartCObjectKTypedData {
            let v = unsafe { self.value.as_typed_data };
            let ty = v.ty;
            match ty {
                DartTypedDataType::DartTypedDataKInt8 => {
                    let _ = unsafe {
                        Vec::from_raw_parts(
                            v.values as *mut i8,
                            v.length as usize,
                            v.length as usize,
                        )
                    };
                }
                DartTypedDataType::DartTypedDataKUint8 => {
                    let _ = unsafe {
                        Vec::from_raw_parts(
                            v.values as *mut u8,
                            v.length as usize,
                            v.length as usize,
                        )
                    };
                }
                _ => {}
            };
        }
    }
}
