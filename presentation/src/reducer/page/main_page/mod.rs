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
use domain::successor::Successor;

use crate::action::page::main_page::{MainPage as MainAction, MainPage};
use crate::state::page::main_page::MainPage as MainState;
use crate::state::page::main_page::text::Text;

pub struct MainPageReducer;

impl MainPageReducer {
    pub fn reduce<'a>(
        state: &MainState,
        action: &MainAction,
        successor: impl Successor,
    ) -> MainState {
        let count = match action {
            MainPage::None => { state.text.count }
            MainPage::ButtonClicked => { successor.add_one(state.text.count) }
        };
        MainState {
            text: Text {
                count,
            },
        }
    }
}

