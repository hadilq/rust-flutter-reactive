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
extern crate flatc_rust;

use std::path::Path;

fn main() {
    flatc_rust::run(flatc_rust::Args {
        lang: "rust",
        inputs: &[
            Path::new("../state-action/src/flat/actions.fbs"),
            Path::new("../state-action/src/flat/states.fbs"),
        ],
        out_dir: Path::new("src/model/"),
        ..Default::default()
    })
    .expect("flatc");

    flatc_rust::run(flatc_rust::Args {
        lang: "dart",
        inputs: &[
            Path::new("../state-action/src/flat/actions.fbs"),
            Path::new("../state-action/src/flat/states.fbs"),
        ],
        out_dir: Path::new("../ui/plugins/bridge_ffi/lib/model/"),
        ..Default::default()
    })
    .expect("flatc");
}
