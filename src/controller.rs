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

use crate::constants::{ApiResponse, AppError, DnsQuery, DnsResult, DnsSecQuery, DnsSecResult};
use actix_web::{HttpResponse, web};

use crate::dns;
use tera::Tera;

//index
pub async fn index(tera: web::Data<Tera>) -> Result<HttpResponse, AppError> {
    let context = tera::Context::new();
    match tera.render("index.html", &context) {
        Ok(template) => Ok(HttpResponse::Ok().body(template)),
        Err(e) => Err(AppError::tera(e.into())),
    }
}

//query
pub async fn v1_query(request: web::Json<DnsQuery>) -> Result<ApiResponse<DnsResult>, AppError> {
    let query_result = dns::v1_query(request.into_inner()).await;
    match query_result {
        Ok(dns_query) => Ok(ApiResponse::okay(dns_query)),
        Err(e) => Err(e),
    }
}

//query_dnssec
pub async fn v1_query_dnssec(request: web::Json<DnsSecQuery>) -> Result<ApiResponse<DnsSecResult>, AppError> {
    let query_result = dns::v1_query_dnssec(request.into_inner()).await;
    match query_result {
        Ok(dns_query) => Ok(ApiResponse::okay(dns_query)),
        Err(e) => Err(e),
    }
}