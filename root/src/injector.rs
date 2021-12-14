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
use lazy_static::lazy_static;

use application_implement::application::App;
use application_interface::application::Application;
use domain_implement::successor::AddOne;
use presentation_implement::reducer::page::main_page::MainPageReducerStruct;
use presentation_implement::reducer::root::RootReducerStruct;
use presentation_implement::reducer::user::UserReducerStruct;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref APP: Arc<Mutex<
        App<RootReducerStruct<MainPageReducerStruct<AddOne>, UserReducerStruct>>
    >> = Arc::new(Mutex::new(Application::new(provide_root_reducer())));
}

fn provide_root_reducer() -> RootReducerStruct<MainPageReducerStruct<AddOne>, UserReducerStruct> {
    return RootReducerStruct::<MainPageReducerStruct::<AddOne>, UserReducerStruct> {
        main_page_reducer: provide_main_reducer(),
        user_reducer: UserReducerStruct,
    };
}

fn provide_main_reducer() -> MainPageReducerStruct<AddOne> {
    return MainPageReducerStruct::<AddOne> {
        successor: AddOne
    };
}
