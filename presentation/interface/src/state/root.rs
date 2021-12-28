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
use crate::state::page::main_page::main_page::MainPage;
use crate::state::user::User;

use crate::state::page::page_type::PageType;
use crate::state::page::page_type::PageType::MainPageType;

#[derive(PartialEq, Debug, Clone)]
pub struct Root {
    pub user: User,
    pub page: PageType,
}

impl Default for Root {
    fn default() -> Self {
        Root {
            user: User::default(),
            page: MainPageType {
                page: MainPage::default(),
            },
        }
    }
}
