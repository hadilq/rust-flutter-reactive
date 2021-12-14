use application_interface::application::Application;
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
use presentation_interface::action::Root as RootAction;
use presentation_interface::state::Root as RootState;

use crate::injector::APP;

macro_rules! app {
    () => { APP.clone().as_ref().lock().unwrap() };
}

pub fn act(action: RootAction) {
    app!().dispatch(action);
}

pub fn dispatch_current_state() {
    app!().dispatch_subscriptions()
}

pub fn updates(cb: fn(&RootState)) {
    app!().subscribe_for_updates(cb)
}

pub fn logger(cb: fn(String)) {
    app!().subscribe_for_logger(cb);
}
