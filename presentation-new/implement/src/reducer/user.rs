/// Copyright 2021 Hadi Lashkari Ghouchani
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
/// http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.
use presentation_interface::action::user::User as UserAction;
use presentation_interface::reducer::user::UserReducer;
use presentation_interface::state::user::User as UserState;
use std::option::Option::Some;

pub struct UserReducerStruct;

impl UserReducer for UserReducerStruct {
    fn reduce(self: &Self, state: &UserState, action: &UserAction) -> UserState {
        let name = if let Some(name) = &state.name {
            match action {
                UserAction::None => Some(String::from(name)),
                UserAction::ChangeUserName { name } => Some(String::from(name)),
            }
        } else {
            match action {
                UserAction::None => None,
                UserAction::ChangeUserName { name } => Some(String::from(name)),
            }
        };
        UserState { name }
    }
}
