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
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:ffi/ffi.dart' as ffi;

// ignore_for_file: unused_import, camel_case_types, non_constant_identifier_names
final DynamicLibrary _dl = _open();

/// Reference to the Dynamic Library, it should be only used for low-level access
final DynamicLibrary dl = _dl;

DynamicLibrary _open() {
  if (Platform.isAndroid) return DynamicLibrary.open('libbridge_ffi.so');
  if (Platform.isIOS) return DynamicLibrary.executable();
  throw UnsupportedError('This platform is not supported.');
}

/// C struct `DartNativeTypedData`.
class Event extends Struct {

  @Uint32()
  int length;

  Pointer<Uint8> values;

  static Pointer<Event> allocate() {
    return ffi.allocate<Event>();
  }

  static Event from(int ptr) {
    return Pointer<Event>.fromAddress(ptr).ref;
  }
}

/// C function `current_state`.
Pointer<Event> current_state() {
  return _current_state();
}
final _current_state_Dart _current_state = _dl.lookupFunction<_current_state_C, _current_state_Dart>('current_state');
typedef _current_state_C = Pointer<Event> Function();
typedef _current_state_Dart = Pointer<Event> Function();

/// C function `do_action`.
void do_action(
  Pointer<Event> data,
) {
  _do_action(data);
}
final _do_action_Dart _do_action = _dl.lookupFunction<_do_action_C, _do_action_Dart>('do_action');
typedef _do_action_C = Void Function(
  Pointer<Event> data,
);
typedef _do_action_Dart = void Function(
  Pointer<Event> data,
);

/// C function `drop_pointer`.
void drop_pointer(
  Pointer<Event> ptr,
) {
  _drop_pointer(ptr);
}
final _drop_pointer_Dart _drop_pointer = _dl.lookupFunction<_drop_pointer_C, _drop_pointer_Dart>('drop_pointer');
typedef _drop_pointer_C = Void Function(
  Pointer<Event> ptr,
);
typedef _drop_pointer_Dart = void Function(
    Pointer<Event> ptr,
    );

/// C function `next_port`.
void next_port(int port_id,) {
  _next_port(port_id);
}

final _next_port_Dart _next_port =
_dl.lookupFunction<_next_port_C, _next_port_Dart>('next_port');

typedef _next_port_C = Void Function(
    Int64 port_id,
    );
typedef _next_port_Dart = void Function(
    int port_id,
    );

/// C function `setup_logger`.
final void Function(Pointer) setup_logger =
_dl.lookup<NativeFunction<Void Function(Pointer)>>("setup_logger")
    .asFunction<void Function(Pointer)>();

/// C function `setup_post_cobject`.
void setup_post_cobject(
    Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>> ptr,) {
  _setup_post_cobject(ptr);
}

final _setup_post_cobject_Dart _setup_post_cobject =
_dl.lookupFunction<_setup_post_cobject_C, _setup_post_cobject_Dart>(
    'setup_post_cobject');

typedef _setup_post_cobject_C = Void Function(
    Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>>,
    );
typedef _setup_post_cobject_Dart = void Function(
    Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>>,
    );
