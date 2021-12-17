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
use presentation_interface::reducer::root::RootReducer;
use presentation_interface::state::Root as RootState;

pub trait Application<RootReducerType: RootReducer> {

    fn new(root_reducer: RootReducerType) -> Self;

    fn log(self, message: String);

    fn dispatch(&mut self, action: RootAction);

    fn subscribe_for_updates(&mut self, callback: fn(&RootState));

    fn subscribe_for_logger(&mut self, callback: fn(String));

    fn dispatch_subscriptions(&self);

    fn state(&self) -> &RootState;
}
