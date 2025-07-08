// Apache License
// Version 2.0, January 2004
//
// Copyright (c) 2025 wchar.net
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

mod server;
mod constants;
mod controller;
mod route;
mod dns;

use crate::constants::LOG4RS_INIT_FILE;
use crate::server::create_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    log4rs::init_file(LOG4RS_INIT_FILE, Default::default()).unwrap();

    let _server = create_server().await?;
    Ok(())
}
