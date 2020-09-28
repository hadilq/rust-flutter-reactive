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
import 'dart:typed_data';

import 'package:bridge_ffi/BridgeFfi.dart';
import 'package:bridge_ffi/model/actions_actions_generated.dart';
import 'package:flat_buffers/flat_buffers.dart' as fb;

class Action {
  Action(this.serialized);

  final Uint8List serialized;

  void doAction() {
    BridgeFfi.doAction(serialized);
  }
}

class ActionFactory {
  static Action createMainPageButtonClick() {
    final builder = new fb.Builder(initialSize: 1024);
    final buttonClicked = new ButtonClickedObjectBuilder().finish(builder) ;

    final mainPageBuilder = MainPageBuilder(builder)
      ..begin()
      ..addActionType(MainPageActionTypeId.ButtonClicked)
      ..addActionOffset(buttonClicked);
    final mainPage = mainPageBuilder.finish();

    final rootBuilder = new RootBuilder(builder)
      ..begin()
      ..addPageType(PageTypeId.MainPage)
      ..addPageOffset(mainPage);
    final root = rootBuilder.finish();

    var serialized = builder.finish(root);

    return Action(serialized);
  }

  static Action createUserChangeUserName(String name) {
    var builder = new fb.Builder(initialSize: 1024);

    final changeUserNameBuilder = ChangeUserNameBuilder(builder)
      ..begin()
      ..addNameOffset(builder.writeString(name));
    var changeUserName = changeUserNameBuilder.finish();

    final rootBuilder = new RootBuilder(builder)
      ..begin()
      ..addUserType(UserTypeId.ChangeUserName)
      ..addUserOffset(changeUserName);
    var root = rootBuilder.finish();

    return Action(builder.finish(root));
  }
}
