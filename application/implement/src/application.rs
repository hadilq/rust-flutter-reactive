/// Copyright 2020 Hadi Lashkari Ghouchani
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

use application_interface::application::Application;
use presentation_interface::action::Root as RootAction;
use presentation_interface::reducer::root::RootReducer;
use presentation_interface::state::Root as RootState;

use crate::store::{Reducer, Store};

struct AppReducer<RootReducerType: RootReducer> {
    root_reducer: RootReducerType,
}

impl<RootReducerType: RootReducer> Reducer<RootState, RootAction> for AppReducer<RootReducerType> {
    fn reduce(self: &Self, state: &RootState, action: &RootAction) -> RootState {
        return self.root_reducer.reduce(state, action);
    }
}

pub struct App<RootReducerType: RootReducer> {
    store: Store<RootState, RootAction, AppReducer<RootReducerType>>,
    logger: Option<fn(String)>,
}

impl<RootReducerType: RootReducer> Application<RootReducerType> for App<RootReducerType> {
    fn new(root_reducer: RootReducerType) -> Self {
        Self {
            store: Store::new(AppReducer { root_reducer }, RootState::default()),
            logger: None,
        }
    }

    fn log(self, message: String) {
        if let Some(func) = self.logger {
            func(message);
        }
    }

    fn dispatch(&mut self, action: RootAction) {
        self.store.dispatch(action);
    }

    fn subscribe_for_updates(&mut self, callback: fn(&RootState)) {
        self.store.subscribe(callback);
    }

    fn subscribe_for_logger(&mut self, callback: fn(String)) {
        self.logger = Some(callback);
    }

    fn dispatch_subscriptions(&self) {
        self.store.dispatch_subscriptions()
    }

    fn state(&self) -> &RootState {
        &self.store.state()
    }
}

#[cfg(test)]
mod test {
    use application_interface::application::Application;
    use presentation_interface::action::page::main_page::MainPage::ButtonClicked;
    use presentation_interface::action::page::PageType::MainPageType as MainPageTypeAction;
    use presentation_interface::action::Root as RootAction;
    use presentation_interface::action::user::User as UserAction;
    use presentation_interface::reducer::root::RootReducer;
    use presentation_interface::state::page::main_page::MainPage;
    use presentation_interface::state::page::main_page::text::Text;
    use presentation_interface::state::page::PageType;
    use presentation_interface::state::page::PageType::MainPageType as MainPageTypeState;
    use presentation_interface::state::Root as RootState;
    use presentation_interface::state::user::User;

    use crate::application::App;

    #[test]
    fn apply_click_action_on_default_state() {
        let root_reducer = FakeRootReducer;
        let app: App<FakeRootReducer> = Application::new(root_reducer);
        let root = app.state();
        assert!(matches!(&root.page, MainPageTypeState { page } if page.text.count == 0 ));
    }

    #[test]
    fn apply_click_action_on_default_state_by_registration() {
        let root_reducer = FakeRootReducer;
        let mut app: App<FakeRootReducer> = Application::new(root_reducer);
        let listener = |state: &RootState| {
            assert!(matches!(state.clone().page, MainPageTypeState { page } if page.text.count == 1 ));
        };
        app.subscribe_for_updates(listener);
        let root = app.state();
        assert!(matches!(&root.page, MainPageTypeState { page } if page.text.count == 0 ));
        app.dispatch(click_action());
    }

    fn click_action() -> RootAction {
        RootAction {
            user: UserAction::None,
            page: MainPageTypeAction {
                page: ButtonClicked,
            },
        }
    }

    struct FakeRootReducer;

    impl RootReducer for FakeRootReducer {
        fn reduce(self: &Self, _state: &RootState, _action: &RootAction) -> RootState {
            RootState {
                user: User { name: Some("some name".to_string()) },
                page: PageType::MainPageType { page: MainPage { text: Text { count: 1 } } },
            }
        }
    }
}
