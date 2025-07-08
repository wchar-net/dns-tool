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

use actix_web::body::BoxBody;
use actix_web::{HttpResponse, Responder, ResponseError};
use hickory_client::proto::rr::RecordType;
use lazy_static::lazy_static;
use log::error;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt::Display;
use tera::Tera;

pub const LOG4RS_INIT_FILE: &str = "log4rs.yaml";
pub const TERA_TEMPLATE_PATH: &str = "html/templates/**/*";

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsRecordSecResult {
    pub value: String,
    #[serde(rename = "recordType")]
    pub record_type: String,
    pub ttl: u32,
}

impl DnsRecordSecResult {
    #[allow(dead_code)]
    pub fn new(record_type: RecordType, ttl: u32, value: String) -> Self {
        DnsRecordSecResult {
            record_type: record_type.to_string().to_uppercase(),
            ttl,
            value,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsQuery {
    pub domain: String,

    #[serde(rename = "recordType")]
    pub record_type: String,

    #[serde(rename = "dnsServer")]
    pub dns_server: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsSecQuery {
    pub domain: String,
    #[serde(rename = "dnsServer")]
    pub dns_server: String,

    #[serde(rename = "recordType")]
    pub record_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsRecordResult {
    pub value: String,
    pub ttl: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsResult {
    #[serde(rename = "dnsServer")]
    pub dns_server: String,

    #[serde(rename = "recordType")]
    pub record_type: String,

    #[serde(rename = "record")]
    pub dns_record: Vec<DnsRecordResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnsSecResult {
    #[serde(rename = "dnsServer")]
    pub dns_server: String,

    #[serde(rename = "record")]
    pub dns_record: Vec<DnsRecordSecResult>,
}

#[allow(dead_code)]
pub const STATIC_MOUNT_PREFIX: &str = "/static";

#[allow(dead_code)]
pub const STATIC_SERVE_FROM: &str = "html/static/";

#[allow(dead_code)]
pub const INTERNAL_SERVER_ERROR: &str = "Internal Server Error!";

#[allow(dead_code)]
pub const NOT_FOUND_ERROR: &str = "404 not found!";

#[allow(dead_code)]
pub const NOT_FOUND_HTML: &str = "404.html";

#[allow(dead_code)]
pub const INTERNAL_SERVER_HTML: &str = "500.html";

//template render exp code , AppError
#[allow(dead_code)]
pub const TERA_RENDER_EXP_CODE: &str = "TERA500";

//bus exp code , AppError
#[allow(dead_code)]
pub const BUS_EXP_CODE: &str = "BUS500";

#[allow(dead_code)]
pub const QUERY_DNS_TIMEOUT: &str = "QUERY_DNS_TIMEOUT";

#[allow(dead_code)]
pub const OKAY_CODE: &str = "1";

//错误
#[allow(dead_code)]
pub const ERR_CODE: &str = "0";

#[allow(dead_code)]
pub const EMPTY_STR: &str = "";

#[allow(dead_code)]
pub const OKAY_MSG: &str = "操作成功!";

#[allow(dead_code)]
pub const CONTENT_TYPE_JSON_VALUE: &str = "application/json; charset=utf-8";

//json序列化错误
#[allow(dead_code)]
pub const ERR_SERDE_CODE: &str = "sys_500_serde_json";

lazy_static! {
    pub static ref BIND_ADDRESS: String =
        env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    pub static ref BIND_PORT: u16 = env::var("BIND_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);
    pub static ref TEMPLATES: Tera = {
        let tera = Tera::new(TERA_TEMPLATE_PATH).unwrap();
        tera
    };
    pub static ref DNS_MAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("google".to_string(), "8.8.8.8".to_string());
        m.insert("open".to_string(), "8.8.4.4".to_string());
        m.insert("cloudflare".to_string(), "1.1.1.1".to_string());
        m.insert("ali".to_string(), "223.5.5.5".to_string());
        m.insert("114".to_string(), "114.114.114.114".to_string());
        m
    };
    pub static ref RECORD_TYPES: Vec<&'static str> = vec!["A", "AAAA", "CNAME", "NS", "TXT"];

    //domain
    pub static ref DOMAIN_REG: Regex =
        Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z0-9-]{2,}$")
            .unwrap();

    //ipv4
     pub static ref V4_REG  : Regex=  Regex::new(r"^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();

    //查询超时时间
    pub static ref QUERY_TIMEOUT : u64 =  env::var("QUERY_TIMEOUT")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .unwrap_or(10);
}

#[derive(Debug, Serialize)]
pub struct AppError {
    code: String,
    msg: String,
}

impl AppError {
    #[allow(dead_code)]
    pub fn new(code: impl Into<String>, msg: impl Into<String>) -> Self {
        AppError {
            code: code.into(),
            msg: msg.into(),
        }
    }
    //template render exp
    #[allow(dead_code)]
    pub fn tera(error: Box<dyn Error>) -> Self {
        error!("{:?}", error);
        AppError {
            code: TERA_RENDER_EXP_CODE.to_string(),
            msg: error.to_string(),
        }
    }

    //bus exp
    #[allow(dead_code)]
    pub fn bus(msg: impl Into<String>) -> Self {
        AppError {
            code: BUS_EXP_CODE.to_string(),
            msg: msg.into(),
        }
    }
    #[allow(dead_code)]
    pub fn query_dns_timeout(msg: String) -> Self {
        AppError {
            code: QUERY_DNS_TIMEOUT.to_string(),
            msg: msg.into(),
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppError: code = {} msg = {} ", self.code, self.msg)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        let json =
            ApiResponse::<String>::full(self.code.clone(), self.msg.clone(), EMPTY_STR.to_string());
        HttpResponse::Ok().json(json)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: String,
    pub msg: String,
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl<T> ApiResponse<T> {
    #[allow(dead_code)]
    pub fn okay(data: T) -> Self {
        Self {
            code: OKAY_CODE.to_string(),
            msg: OKAY_MSG.to_string(),
            data: Some(data),
            total: None,
        }
    }

    #[allow(dead_code)]
    pub fn full(code: String, msg: String, data: T) -> Self {
        Self {
            code,
            msg,
            data: Some(data),
            total: None,
        }
    }

    #[allow(dead_code)]
    pub fn full_not_data(code: String, msg: String) -> Self {
        Self {
            code,
            msg,
            data: None,
            total: None,
        }
    }

    #[allow(dead_code)]
    pub fn ok_page(total: i64, data: T) -> Self {
        Self {
            code: OKAY_CODE.to_string(),
            msg: OKAY_MSG.to_string(),
            data: Some(data),
            total: Some(total),
        }
    }
    #[allow(dead_code)]
    pub fn err_with_data(msg: String, data: T) -> Self {
        Self {
            code: ERR_CODE.to_string(),
            msg,
            data: Some(data),
            total: None,
        }
    }

    #[allow(dead_code)]
    pub fn err_without_data(msg: String) -> Self {
        Self {
            code: ERR_CODE.to_string(),
            msg,
            data: None,
            total: None,
        }
    }
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = BoxBody;
    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        match serde_json::to_string(&self) {
            Ok(json_body) => HttpResponse::Ok()
                .content_type(CONTENT_TYPE_JSON_VALUE)
                .body(json_body),
            Err(_) => {
                let error_response = ApiResponse {
                    code: ERR_SERDE_CODE.to_string(),
                    total: None,
                    msg: "格式化json错误!".to_string(),
                    data: None::<()>, // 使用 None 表示空数据
                };
                let error_json = serde_json::to_string(&error_response).unwrap_or_else(|_| {
                    // 如果再次序列化失败，返回一个固定的备用错误信息
                    return format!(
                        r#"{{"code":"{}","msg":"{}","data":null}}"#,
                        ERR_SERDE_CODE,
                        "格式化json错误!".to_string()
                    );
                });

                HttpResponse::Ok()
                    .content_type(CONTENT_TYPE_JSON_VALUE)
                    .body(error_json)
            }
        }
    }
}
