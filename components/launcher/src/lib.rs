// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate biome_core as core;
use biome_launcher_protocol as protocol;
#[macro_use]
extern crate log;
#[cfg(windows)]
extern crate winapi;

pub mod error;
pub mod server;
pub mod service;
mod sys;

pub const SUP_CMD: &str = "bio-sup";
pub const SUP_PACKAGE_IDENT: &str = "biome/bio-sup";
