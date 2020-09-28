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
import 'dart:async';
import 'dart:ffi';
import 'dart:typed_data';

import 'package:bridge_ffi/model/states_states_generated.dart' as State;
import 'package:ffi/ffi.dart';
import 'package:isolate/ports.dart';

import 'ffi.dart' as native;

typedef _wrappedPrint_C = Void Function(Pointer<Utf8> a);
final wrappedPrintPointer = Pointer.fromFunction<_wrappedPrint_C>(_wrappedPrint);

void _wrappedPrint(Pointer<Utf8> arg){
  print("FFI: ${Utf8.fromUtf8(arg)}");
}

class BridgeFfi {

  static StreamController<State.Root> _controller =
      StreamController<State.Root>();
  static Stream<State.Root> stream = _controller.stream.asBroadcastStream();

  static State.Root setup() {
    native.setup_post_cobject(NativeApi.postCObject);
    native.setup_logger(wrappedPrintPointer);
    _setupNextCall();
    return getCurrentState();
  }

  static log(String message){
    print(message);
  }

  static _setupNextCall() {
    print("_setupNextCall");
    final completer = Completer<Uint8List>();
    final sendPort = singleCompletePort(completer);

    native.next_port(sendPort.nativePort);

    var completeFuture =
        _controller.addStream(completer.future.asStream().map((ptr) {
          print('callback is here');
          print(ptr);
      return State.Root(ptr);
    }));
    completeFuture.whenComplete(() => _setupNextCall());
  }

  static State.Root getCurrentState() {
    final ptr = native.current_state();
    final root = State.Root(_listTypedData(ptr));
    native.drop_pointer(ptr);
    return root;
  }

  static doAction(Uint8List action) {
    final pointer = allocate<native.Event>(count: action.length);
    final typedData = pointer.ref;
    typedData.length = action.length;
    typedData.values = allocate<Uint8>(count: action.length);
    for (int i = 0; i < action.length; i++) {
      typedData.values[i] = action[i];
    }
    print('before doing the action');
    native.do_action(pointer);
    print('after doing the action');
    free(typedData.values);
    free(pointer);
  }

  static Uint8List _listTypedData(Pointer<native.Event> ptr) {
    print("_listTypedData");
    final typedData = ptr.ref;
    var list = typedData.values.asTypedList(typedData.length);
    print(list);
    return list;
  }
}
