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
#[cfg(test)]
mod test {
    use presentation::action::page::main_page::MainPage::ButtonClicked;
    use presentation::action::page::PageType::MainPageType as MainPageTypeAction;
    use presentation::action::user::User as UserAction;
    use presentation::action::Root as RootAction;
    use presentation::reducer::RootReducer;
    use presentation::state::Root;

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

    #[test]
    fn apply_click_action_on_default_state() {
        let root = RootReducer::reduce(&Root::default(), &click_action());
        let serialized_root = [
            16, 0, 0, 0, 0, 0, 10, 0, 18, 0, 8, 0, 7, 0, 12, 0, 10, 0, 0, 0, 0, 0, 0, 1, 44, 0, 0,
            0, 12, 0, 0, 0, 0, 0, 6, 0, 10, 0, 4, 0, 6, 0, 0, 0, 12, 0, 0, 0, 0, 0, 6, 0, 8, 0, 4,
            0, 6, 0, 0, 0, 1, 0, 0, 0, 4, 0, 4, 0, 4, 0, 0, 0,
        ];
        assert_eq!(map_state(&root), serialized_root.to_vec());
    }

    fn click_action() -> RootAction {
        RootAction {
            user: UserAction::None,
            page: MainPageTypeAction {
                page: ButtonClicked,
            },
        }
    }
}
