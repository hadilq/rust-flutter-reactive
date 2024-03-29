use presentation_interface::state::root::Root;
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
use root::thread_safe_app::ThreadSafeApp;

use crate::adapter::action_adapter::map_action;
use crate::adapter::state_adapter::map_state;
use crate::{log, notify_for_update};

pub mod action_adapter;
pub mod state_adapter;

pub fn act(data: Vec<u8>) {
    let action = map_action(data);
    log(format!("action: {:?}", action));
    ThreadSafeApp::act(action)
}

pub fn dispatch_state() {
    ThreadSafeApp::dispatch_current_state()
}

pub fn register_for_updates() {
    ThreadSafeApp::updates(|state: &Root| {
        let root = state.clone();
        log(format!("state: {:?}", root));
        notify_for_update(map_state(&root));
    });
}

pub fn register_for_logger() {
    ThreadSafeApp::logger(|message: String| {
        log(message);
    })
}
