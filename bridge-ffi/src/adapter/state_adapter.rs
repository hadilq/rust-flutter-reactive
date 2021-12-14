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

use presentation::state::page::PageType;
use presentation::state::Root as Root_;
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
