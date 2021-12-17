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
use flatbuffers::FlatBufferBuilder;

use presentation_interface::state::page::PageType;
use presentation_interface::state::Root as Root_;
use state_action::model::states_generated::states::{
    finish_root_buffer, MainPage, MainPageArgs, Page, Root, RootArgs, Text, TextArgs, User,
    UserArgs,
};

pub fn map_state(state: &Root_) -> Vec<u8> {
    let builder = &mut FlatBufferBuilder::new();

    let name = match &state.user.name {
        None => None,
        Some(n) => Some(builder.create_string(n.as_str())),
    };
    let user = User::create(builder, &UserArgs { name });
    let page_type = match state.page {
        PageType::MainPageType { .. } => Page::MainPage,
        PageType::SettingsType => Page::NONE,
    };
    let page = match &state.page {
        PageType::MainPageType { page } => {
            let text = Some(Text::create(
                builder,
                &TextArgs {
                    count: page.text.count,
                },
            ));
            Some(MainPage::create(builder, &MainPageArgs { text }).as_union_value())
        }
        PageType::SettingsType => None,
    };

    let root = Root::create(
        builder,
        &RootArgs {
            user: Some(user),
            page_type,
            page,
        },
    );

    finish_root_buffer(builder, root);
    builder.finished_data().to_owned()
}

#[cfg(test)]
mod test {
    use presentation_interface::action::page::main_page::MainPage::ButtonClicked;
    use presentation_interface::action::page::PageType::MainPageType as MainPageTypeAction;
    use presentation_interface::action::user::User as UserAction;
    use presentation_interface::action::Root as RootAction;
    use presentation_interface::state::Root;

    use crate::adapter::state_adapter::map_state;

    #[test]
    fn state_adapter_maps() {
        let serialized_root = [
            16, 0, 0, 0, 0, 0, 10, 0, 18, 0, 8, 0, 7, 0, 12, 0, 10, 0, 0, 0, 0, 0, 0, 1, 32, 0, 0,
            0, 12, 0, 0, 0, 0, 0, 6, 0, 8, 0, 4, 0, 6, 0, 0, 0, 4, 0, 0, 0, 252, 255, 255, 255, 4,
            0, 4, 0, 4, 0, 0, 0,
        ];
        assert_eq!(map_state(&Root::default()), serialized_root.to_vec())
    }
}
