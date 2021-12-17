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
use presentation_interface::action::page::main_page::MainPage;
use presentation_interface::action::page::PageType;
use presentation_interface::action::user::User;
use presentation_interface::action::Root;
use state_action::model::actions_generated::actions::get_root_as_root;
use state_action::model::actions_generated::actions::Page as Page_;
use state_action::model::actions_generated::actions::User as User_;

pub fn map_action(data: Vec<u8>) -> Root {
    let action = get_root_as_root(data.as_ref());
    let user = match action.user_type() {
        User_::NONE => User::None,
        User_::ChangeUserName => User::ChangeUserName {
            name: String::from(action.user_as_change_user_name().unwrap().name().unwrap()),
        },
        _ => User::None,
    };
    let page = match action.page_type() {
        Page_::NONE => PageType::None,
        Page_::SettingsPage => PageType::None,
        Page_::MainPage => {
            let page = match action
                .page_as_main_page()
                .unwrap()
                .action_as_button_clicked()
            {
                None => MainPage::None,
                Some(_) => MainPage::ButtonClicked,
            };
            PageType::MainPageType { page }
        }
        _ => PageType::None,
    };
    Root { user, page }
}
