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
use std::option::Option::Some;

use crate::action::user::{User as UserAction, User};
use crate::state::user::User as UserState;

pub struct UserReducer;

impl UserReducer {

    pub fn reduce<'a>(
        state: &UserState,
        action: &UserAction,
    ) -> UserState {
        let name = if let Some(name) = &state.name {
            match action {
                User::None => { Some(String::from(name)) }
                User::ChangeUserName { name } => { Some(String::from(name)) }
            }
        } else {
            match action {
                User::None => { None }
                User::ChangeUserName { name } => { Some(String::from(name)) }
            }
        };
        UserState {
            name,
        }
    }
}
