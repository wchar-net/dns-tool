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

use crate::controller;
use actix_web::web;
use actix_web::web::{get, post};

//路由配置
pub fn route(config: &mut web::ServiceConfig) {
    config
        .route("/", get().to(controller::index))
        .route("/v1/query", post().to(controller::v1_query))
        .route("/v1/query_dnssec", post().to(controller::v1_query_dnssec));
}
