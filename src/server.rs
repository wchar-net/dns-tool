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

use actix_cors::Cors;
use crate::constants::AppError;
use crate::{constants, route};
use actix_web::dev::ServiceResponse;
use actix_web::middleware::TrailingSlash::Trim;
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers, Logger};
use actix_web::mime::TEXT_HTML_UTF_8;
use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer};
use log::{error, info};
use tera::Tera;

pub async fn create_server() -> std::io::Result<()> {
    let bind_address = (*constants::BIND_ADDRESS).clone();
    let bind_port = (*constants::BIND_PORT).clone();



    //服务
    let server = match HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default().allowed_origin("http://127.0.0.1:5500")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS", "PATCH"]) // 允许所有常用方法
                    .allowed_headers(vec!["Content-Type", "Authorization", "X-Requested-With", "Accept"]) // 允许常用请求头
                    .max_age(3600),  // 缓存预检请求的时间，单位秒
            )
            .app_data(Data::new((*constants::TEMPLATES).clone()))
            .wrap(Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(actix_web::middleware::NormalizePath::new(Trim))
            .configure(route::route)
            .wrap(ErrorHandlers::new().default_handler(error_handler))
            .service(
                actix_files::Files::new(
                    constants::STATIC_MOUNT_PREFIX,
                    constants::STATIC_SERVE_FROM,
                )
                .use_last_modified(true),
            )
    })
    .bind((bind_address, bind_port))
    {
        Ok(server) => {
            info!(
                "📢 Listening on: http://{}:{}",
                (*constants::BIND_ADDRESS).clone(),
                (*constants::BIND_PORT).clone()
            );
            info!("✅  okay run site!");
            server
        }
        Err(e) => {
            error!(
                "!!! FAILED TO BIND A SERVER !!! \n {} {}",
                (*constants::BIND_ADDRESS).clone(),
                e
            );
            return Err(e);
        }
    };
    server.run().await?;
    Ok(())
}

fn error_handler<B>(res: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    let (request, response) = res.into_parts();

    let tera = request
        .app_data::<Data<Tera>>()
        .cloned()
        .ok_or_else(|| AppError::bus("error_handler: 从 app_data 中获取 Tera 失败!"))?;

    let mut context = tera::Context::new();
    context.insert("path", request.path());
    context.insert("method", request.method().as_str());
    context.insert("uri", request.uri().to_string().as_str());
    context.insert("full_url", request.full_url().to_string().as_str());
    context.insert("query_string", request.query_string());
    let error = response.error().map_or_else(|| "".to_string(), |err| err.to_string());
    context.insert("error", &error);

    //这里拦截了所有异常  .wrap(ErrorHandlers::new().default_handler(error_handler)) 包括404
    if response.status() == 404 {
        return match tera.render(constants::NOT_FOUND_HTML, &context) {
            Ok(template) => Ok(ErrorHandlerResponse::Response(
                ServiceResponse::new(
                    request,
                    HttpResponse::NotFound()
                        .content_type(TEXT_HTML_UTF_8)
                        .body(template),
                )
                .map_into_right_body(),
            )),
            Err(_) => Ok(ErrorHandlerResponse::Response(
                ServiceResponse::new(
                    request,
                    HttpResponse::NotFound().body(constants::NOT_FOUND_ERROR),
                )
                .map_into_right_body(),
            )),
        };
    }

    //其他
    let err = response.error();
    error!("error_handler: {:?}", err);

    match tera.render(constants::INTERNAL_SERVER_HTML, &context) {
        Ok(template) => {
            let response = HttpResponse::InternalServerError()
                .content_type(TEXT_HTML_UTF_8)
                .body(template);
            let rs = ServiceResponse::new(request, response).map_into_right_body();
            Ok(ErrorHandlerResponse::Response(rs))
        }
        Err(_) => {
            let response =
                HttpResponse::InternalServerError().body(constants::INTERNAL_SERVER_ERROR);
            let rs = ServiceResponse::new(request, response).map_into_right_body();
            Ok(ErrorHandlerResponse::Response(rs))
        }
    }
}
