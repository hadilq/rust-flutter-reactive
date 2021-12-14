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
use domain::successor::AddOne;

use crate::action::page::main_page::MainPage;
use crate::action::page::PageType;
use crate::action::Root as RootAction;
use crate::reducer::page::main_page::MainPageReducer;
use crate::reducer::user::UserReducer;
use crate::state::page::PageType::{MainPageType, SettingsType};
use crate::state::Root as RootState;

pub mod user;
pub mod page;

pub struct RootReducer;

impl RootReducer {

    pub fn reduce<'a>(
        state: &RootState,
        action: &RootAction,
    ) -> RootState {
        let page = match &state.page {
            MainPageType { page: p } => {
                let page = match &action.page {
                    PageType::None => { p.clone() }
                    PageType::MainPageType { page } => {
                        match page {
                            MainPage::None => { p.clone() }
                            MainPage::ButtonClicked => {
                                MainPageReducer::reduce(
                                    p,
                                    &MainPage::ButtonClicked,
                                    AddOne
                                )
                            }
                        }
                    }
                };
                MainPageType { page }
            }
            SettingsType => SettingsType,
        };

        RootState {
            user: UserReducer::reduce(&state.user, &action.user),
            page,
        }
    }
}


