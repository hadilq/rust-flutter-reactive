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

use relative_path::RelativePath;
use std::ffi::OsStr;
use std::env;

fn main() {
    let current = env::current_exe().unwrap();
    let mut target = current.as_path();
    for ancestor in current.ancestors() {
        if ancestor.file_name() == Some(OsStr::new("target")) {
            target = ancestor;
            break;
        }
    }
    let root = target.parent().unwrap();

    let actions = RelativePath::new("state-action/src/flat/actions.fbs").to_path(root);
    let states = RelativePath::new("state-action/src/flat/states.fbs").to_path(root);
    let rust_model = RelativePath::new("state-action/src/model/").to_path(root);
    let dart_model = RelativePath::new("ui/plugins/bridge_ffi/lib/model/").to_path(root);

    println!("actions location: {}", actions.display());
    println!("states location: {}", states.display());
    println!("rust generated location: {}", rust_model.display());
    println!("dart generated location: {}", dart_model.display());

    flatc_rust::run(flatc_rust::Args {
        lang: "rust",
        inputs: &[
            actions.as_path(),
            states.as_path(),
        ],
        out_dir: rust_model.as_path(),
        ..Default::default()
    })
    .expect("flatc");

    flatc_rust::run(flatc_rust::Args {
        lang: "dart",
        inputs: &[
            actions.as_path(),
            states.as_path(),
        ],
        out_dir: dart_model.as_path(),
        ..Default::default()
    })
    .expect("flatc");
}
