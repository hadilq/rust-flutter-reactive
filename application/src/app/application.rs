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
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use redux_rs::Store;

use presentation::action::Root as RootAction;
use presentation::reducer::RootReducer;
use presentation::state::Root as RootState;

lazy_static! {
     static ref APP: Arc<Mutex<App>> = Arc::new(Mutex::new(App::new()));
}

macro_rules! app {
    () => { APP.clone().as_ref().lock().unwrap() };
}

pub struct App {
    store: Store<RootState, RootAction>,
    logger: Option<fn(String)>,
}

impl App {
    pub fn new() -> Self {
        App {
            store: Store::new(RootReducer::reduce, RootState::default()),
            logger: None,
        }
    }

    pub fn act(action: RootAction) {
        app!().dispatch(action);
    }

    pub fn current_state() -> RootState {
        app!().state()
    }

    pub fn updates(cb: fn(&RootState)) {
        app!().register_for_updates(cb)
    }

    pub fn logger(cb: fn(String)) {
        app!().register_for_logger(cb);
    }

    pub fn log(self, message: String) {
        if let Some(func) = self.logger {
            func(message);
        }
    }

    pub fn dispatch(&mut self, action: RootAction) {
        self.store.dispatch(action);
    }

    pub fn register_for_updates(&mut self, cb: fn(&RootState)) {
        self.store.subscribe(cb);
    }

    pub fn register_for_logger(&mut self, cb: fn(String)) {
        self.logger = Some(cb);
    }

    pub fn state(
        &self,
    ) -> RootState {
        self.store.state().clone()
    }
}

