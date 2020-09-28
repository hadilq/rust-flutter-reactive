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
    use crate::action::page::main_page::MainPage::ButtonClicked;
    use crate::action::page::PageType::MainPageType as MainPageTypeAction;
    use crate::action::Root as RootAction;
    use crate::action::user::User as UserAction;
    use crate::reducer::RootReducer;
    use crate::state::page::main_page::MainPage;
    use crate::state::page::PageType::MainPageType as MainPageTypeState;
    use crate::state::Root as RootState;
    use crate::state::user::User as UserState;

    #[test]
    fn create_default_state() {
        let root = RootState::default();
        assert_eq!(root.user, UserState::default());
        assert!(matches!(root.page, MainPageTypeState { page } if page == MainPage::default() ));
    }

    #[test]
    fn apply_click_action_on_default_state() {
        let root = RootState::default();
        assert!(matches!(root.page, MainPageTypeState {ref page} if page.text.count == 0 ));
        let new_root = RootReducer::reduce(&root, &click_action());
        assert!(matches!(new_root.page, MainPageTypeState {page} if page.text.count == 1 ));
    }

    fn click_action() -> RootAction {
        RootAction {
            user: UserAction::None,
            page: MainPageTypeAction { page: ButtonClicked },
        }
    }
}
