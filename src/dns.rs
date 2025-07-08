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

use crate::constants;
use crate::constants::{
    AppError, DnsQuery, DnsRecordResult, DnsRecordSecResult, DnsResult, DnsSecQuery, DnsSecResult,
};
use hickory_client::client::{Client, ClientHandle, DnssecClient};
use hickory_client::proto::rr::{DNSClass, Name, RecordType};
use hickory_client::proto::runtime::TokioRuntimeProvider;
use hickory_client::proto::udp::UdpClientStream;
use log::info;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;

fn get_record_type_from_str(record_type: String) -> Result<RecordType, AppError> {
    let record_type_upper = record_type.to_uppercase();
    constants::RECORD_TYPES
        .iter()
        .position(|&r| r == record_type_upper.as_str())
        .map(|index| match index {
            0 => RecordType::A,
            1 => RecordType::AAAA,
            2 => RecordType::CNAME,
            3 => RecordType::NS,
            4 => RecordType::TXT,
            _ => unreachable!(),
        })
        .ok_or_else(|| AppError::bus(format!("Unsupported record type: {}", record_type)))
}

pub async fn v1_query_dnssec(dns_sec_query: DnsSecQuery) -> Result<DnsSecResult, AppError> {
    let domain = &dns_sec_query.domain;
    let record_type = &dns_sec_query.record_type;
    let dns_server_box = &dns_sec_query.dns_server;
    if domain.trim().is_empty() {
        return Err(AppError::bus("域名不能为空!".to_string()));
    }

    if !constants::DOMAIN_REG.is_match(domain) {
        return Err(AppError::bus("域名格式不正确!".to_string()));
    }

    if record_type.trim().is_empty() {
        return Err(AppError::bus("记录类型不能为空!".to_string()));
    }
    if !constants::RECORD_TYPES.contains(&record_type.to_uppercase().as_str()) {
        return Err(AppError::bus("不支持的记录类型!".to_string()));
    }

    if dns_server_box.trim().is_empty() {
        return Err(AppError::bus("dns供应商不能为空!".to_string()));
    }

    let dns_addr = constants::DNS_MAP
        .get(dns_server_box)
        .map(|v| v.clone())
        .unwrap_or_else(|| dns_server_box.clone());

    if !constants::V4_REG.is_match(&*dns_addr) {
        return Err(AppError::bus("dns 服务器 ip地址不正确!".to_string()));
    }

    let full_addr = format!("{}:53", dns_addr);
    info!("dns::v1_query_dnssec => full_addr: {}", full_addr);

    let socket_addr = SocketAddr::from_str(&full_addr)
        .map_err(|_| AppError::bus(format!("无效的地址格式: {}", dns_addr)))?;

    let conn = UdpClientStream::builder(socket_addr, TokioRuntimeProvider::default())
        .with_timeout(Option::from(Duration::from_secs(*constants::QUERY_TIMEOUT)))
        .build();
    let (mut client, bg) = DnssecClient::connect(conn)
        .await
        .map_err(|e| AppError::bus(e.to_string()))?;
    tokio::spawn(bg);

    let query_type = get_record_type_from_str(record_type.clone())?;
    let query = client.query(Name::from_str(domain).unwrap(), DNSClass::IN, query_type);
    let response = query.await.map_err(|e| {
        let debug_info = format!("{:?}", e);
        let display_info = e.to_string();

        let error_message = format!(
            " {},{}",
            debug_info, display_info
        );

        AppError::query_dns_timeout(error_message)
    })?;

    let mut arr: Vec<DnsRecordSecResult> = vec![];
    response.answers().iter().for_each(|record| {
        let ttl = record.ttl();
        let data = record.data();
        arr.push(DnsRecordSecResult::new(
            record.record_type(),
            ttl,
            data.to_string(),
        ));
    });

    let result = DnsSecResult {
        dns_server: dns_server_box.clone(),
        dns_record: arr,
    };
    Ok(result)
}

pub async fn v1_query(dns_query: DnsQuery) -> Result<DnsResult, AppError> {
    let domain = &dns_query.domain;
    let record_type = &dns_query.record_type;
    let dns_server_box = &dns_query.dns_server;

    if domain.trim().is_empty() {
        return Err(AppError::bus("域名不能为空!".to_string()));
    }
    if !constants::DOMAIN_REG.is_match(domain) {
        return Err(AppError::bus("域名格式不正确!".to_string()));
    }
    if record_type.trim().is_empty() {
        return Err(AppError::bus("记录类型不能为空!".to_string()));
    }
    if !constants::RECORD_TYPES.contains(&record_type.to_uppercase().as_str()) {
        return Err(AppError::bus("不支持的记录类型!".to_string()));
    }
    if dns_server_box.trim().is_empty() {
        return Err(AppError::bus("dns供应商不能为空!".to_string()));
    }

    let dns_addr = constants::DNS_MAP
        .get(dns_server_box)
        .map(|v| v.clone())
        .unwrap_or_else(|| dns_server_box.clone());

    if !constants::V4_REG.is_match(&*dns_addr) {
        return Err(AppError::bus("dns 服务器 ip地址不正确!".to_string()));
    }

    let full_addr = format!("{}:53", dns_addr);
    info!("dns::v1_query => full_addr: {}", full_addr);

    let socket_addr = SocketAddr::from_str(&full_addr)
        .map_err(|_| AppError::bus(format!("无效的地址格式: {}", dns_addr)))?;

    let conn = UdpClientStream::builder(socket_addr, TokioRuntimeProvider::default())
        .with_timeout(Option::from(Duration::from_secs(*constants::QUERY_TIMEOUT)))
        .build();
    let (mut client, bg) = Client::connect(conn)
        .await
        .map_err(|e| AppError::bus(e.to_string()))?;
    tokio::spawn(bg);

    // Get record type
    let query_type = get_record_type_from_str(record_type.clone())?;

    info!("dns::v1_query => query for domain: {}", domain);
    info!("dns::v1_query => query for type: {}", query_type);

    let query = client.query(Name::from_str(domain).unwrap(), DNSClass::IN, query_type);

    let response = query
        .await
        .map_err(|e| AppError::query_dns_timeout(e.to_string()))?;

    let records: Vec<DnsRecordResult> = response
        .answers()
        .iter()
        .filter(|record| record.record_type() == query_type)
        .map(|record| DnsRecordResult {
            value: record.data().to_string(),
            ttl: record.ttl(),
        })
        .collect();
    let result = DnsResult {
        dns_server: dns_server_box.clone(),
        record_type: dns_query.record_type.clone().to_uppercase(),
        dns_record: records,
    };
    Ok(result)
}
