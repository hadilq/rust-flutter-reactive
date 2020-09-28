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
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A Dart_CObject is used for representing Dart objects as native C
 * data outside the Dart heap. These objects are totally detached from
 * the Dart heap. Only a subset of the Dart objects have a
 * representation as a Dart_CObject.
 *
 * The string encoding in the 'value.as_string' is UTF-8.
 *
 * All the different types from dart:typed_data are exposed as type
 * kTypedData. The specific type from dart:typed_data is in the type
 * field of the as_typed_data structure. The length in the
 * as_typed_data structure is always in bytes.
 *
 * The data for kTypedData is copied on message send and ownership remains with
 * the caller. The ownership of data for kExternalTyped is passed to the VM on
 * message send and returned when the VM invokes the
 * Dart_WeakPersistentHandleFinalizer callback; a non-NULL callback must be
 * provided.
 */
typedef struct DartCObject DartCObject;
enum DartCObjectType
#ifdef __cplusplus
  : int32_t
#endif // __cplusplus
 {
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
};
#ifndef __cplusplus
typedef int32_t DartCObjectType;
#endif // __cplusplus

enum DartTypedDataType
#ifdef __cplusplus
  : int32_t
#endif // __cplusplus
 {
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
};
#ifndef __cplusplus
typedef int32_t DartTypedDataType;
#endif // __cplusplus

typedef struct Event {
  uint32_t length;
  uint8_t *values;
} Event;

/**
 * A port is used to send or receive inter-isolate messages
 */
typedef int64_t DartPort;

typedef struct DartNativeSendPort {
  DartPort id;
  DartPort origin_id;
} DartNativeSendPort;

typedef struct DartNativeCapability {
  int64_t id;
} DartNativeCapability;

typedef struct DartNativeArray {
  intptr_t length;
  DartCObject **values;
} DartNativeArray;

typedef struct DartNativeTypedData {
  DartTypedDataType ty;
  intptr_t length;
  uint8_t *values;
} DartNativeTypedData;

typedef union DartCObjectValue {
  bool as_bool;
  int32_t as_int32;
  int64_t as_int64;
  double as_double;
  char *as_string;
  DartNativeSendPort as_send_port;
  DartNativeCapability as_capability;
  DartNativeArray as_array;
  DartNativeTypedData as_typed_data;
} DartCObjectValue;

typedef struct DartCObject {
  DartCObjectType ty;
  DartCObjectValue value;
} DartCObject;

typedef bool (*DartPostCObjectFn)(DartPort port_id, DartCObject *message);

typedef bool (*DartLoggerFn)(const char *message);

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

Event *current_state(void);

void do_action(Event *data);

void drop_pointer(DartCObject *ptr);

void next_port(DartPort port_id);

void setup_logger(DartLoggerFn ptr);

void setup_post_cobject(DartPostCObjectFn ptr);

int32_t dummy_method_for_dummy_xcode(void);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
