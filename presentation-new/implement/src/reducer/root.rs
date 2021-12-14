use presentation_interface::action::page::main_page::MainPage;
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
use presentation_interface::action::page::PageType;
use presentation_interface::action::Root as RootAction;
use presentation_interface::reducer::page::main_page::MainPageReducer;
use presentation_interface::reducer::root::RootReducer;
use presentation_interface::reducer::user::UserReducer;
use presentation_interface::state::page::PageType::MainPageType as MainPageTypeState;
use presentation_interface::state::page::PageType::SettingsType;
use presentation_interface::state::Root as RootState;

pub struct RootReducerStruct<
    MainPageReducerType: MainPageReducer,
    UserReducerType: UserReducer,
> {
    pub main_page_reducer: MainPageReducerType,
    pub user_reducer: UserReducerType,
}

impl<
    MainPageReducerType: MainPageReducer,
    UserReducerType: UserReducer,
> RootReducer for RootReducerStruct<
    MainPageReducerType,
    UserReducerType,
> {
    fn reduce(
        self: &Self,
        state: &RootState,
        action: &RootAction,
    ) -> RootState {
        let page = match &state.page {
            MainPageTypeState { page: p } => {
                let page = match &action.page {
                    PageType::None => p.clone(),
                    PageType::MainPageType { page } => match page {
                        MainPage::None => p.clone(),
                        MainPage::ButtonClicked => {
                            self.main_page_reducer.reduce(p, &MainPage::ButtonClicked)
                        }
                    },
                };
                MainPageTypeState { page }
            }
            SettingsType => SettingsType,
        };

        RootState {
            user: self.user_reducer.reduce(&state.user, &action.user),
            page,
        }
    }
}

#[cfg(test)]
mod test {
    use presentation_interface::action::page::main_page::MainPage;
    use presentation_interface::action::page::PageType;
    use presentation_interface::action::Root as RootAction;
    use presentation_interface::action::user::User as UserAction;
    use presentation_interface::reducer::page::main_page::MainPageReducer;
    use presentation_interface::reducer::root::RootReducer;
    use presentation_interface::reducer::user::UserReducer;
    use presentation_interface::state::page::main_page::MainPage as MainPageState;
    use presentation_interface::state::page::main_page::text::Text;
    use presentation_interface::state::page::PageType::MainPageType as MainPageTypeState;
    use presentation_interface::state::Root as RootState;
    use presentation_interface::state::user::User as UserState;

    use crate::reducer::root::RootReducerStruct;

    #[test]
    fn create_default_state() {
        let root = RootState::default();
        assert_eq!(root.user, UserState::default());
        assert!(matches!(root.page, MainPageTypeState { page } if page == MainPageState::default() ));
    }

    #[test]
    fn apply_click_action_on_default_state() {
        let root = RootState::default();
        assert!(matches!(root.page, MainPageTypeState {ref page} if page.text.count == 0 ));
        let reducer = RootReducerStruct {
            main_page_reducer: FakeMainPageReducer,
            user_reducer: FakeUserReducer,

        };
        let new_root = reducer.reduce(
            &root,
            &click_action(),
        );
        assert!(matches!(new_root.page, MainPageTypeState {page} if page.text.count == 1 ));
    }

    fn click_action() -> RootAction {
        RootAction {
            user: UserAction::None,
            page: PageType::MainPageType {
                page: MainPage::ButtonClicked,
            },
        }
    }

    struct FakeMainPageReducer;

    impl MainPageReducer for FakeMainPageReducer {
        fn reduce(self: &Self, _state: &MainPageState, _action: &MainPage) -> MainPageState {
            MainPageState {
                text: Text { count: 1 }
            }
        }
    }

    struct FakeUserReducer;

    impl UserReducer for FakeUserReducer {
        fn reduce(self: &Self, _state: &UserState, _action: &UserAction) -> UserState {
            UserState {
                name: None
            }
        }
    }
}
