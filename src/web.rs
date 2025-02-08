/* Copyright (c) 2025 by Karsten Lehmann <mail@kalehmann.de>
 *
 *   This file is part of sdr_gain_tool.
 *
 *   sdr_gain_tool is free software: you can redistribute it and/or modify it
 *   under the terms of the GNU Affero General Public License as published by
 *   the Free Software Foundation, either version 3 of the License, or (at your
 *   option) any later version.
 *
 *   sdr_gain_tool is distributed in the hope that it will be useful, but
 *   WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 *   or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public
 *   License for more details.
 *
 *   You should have received a copy of the GNU Affero General Public License
 *   along with sdr_gain_tool. If not, see <https://www.gnu.org/licenses/>. */

use crate::data::RxDataHolder;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::sse::{Event, Sse};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use axum_server::tls_rustls::RustlsConfig;
use clap_port_flag::Port;
use futures::stream::Stream;
use std::convert::Infallible;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio_stream::StreamExt as _;
use tower_http::trace::TraceLayer;

pub fn run_web_app(data: RxDataHolder, port: Port, with_tls: bool) {
    let runtime = Runtime::new().unwrap();

    runtime.block_on(async {
        let listener = port.bind_or(8000).unwrap();
        if with_tls {
            let certificate = gen_cert();
            let config = RustlsConfig::from_der(
                vec![certificate.cert.der().to_vec()],
                certificate.key_pair.serialize_der(),
            )
            .await
            .unwrap();
            tracing::info!("Listening on https://{}", listener.local_addr().unwrap());
            axum_server::from_tcp_rustls(listener, config)
                .serve(router(data).into_make_service())
                .await
                .unwrap();
        } else {
            let listener = tokio::net::TcpListener::from_std(listener).unwrap();
            tracing::info!("Listening on http://{}", listener.local_addr().unwrap());
            axum::serve(listener, router(data)).await.unwrap();
        }
    });
}

fn gen_cert() -> rcgen::CertifiedKey {
    let key_pair = rcgen::KeyPair::generate().unwrap();
    let mut cert_params = rcgen::CertificateParams::new([]).unwrap();
    cert_params
        .distinguished_name
        .push(rcgen::DnType::CommonName, env!("CARGO_PKG_NAME"));
    let cert = cert_params.self_signed(&key_pair).unwrap();

    rcgen::CertifiedKey { cert, key_pair }
}

async fn change_frequency(
    State(data): State<RxDataHolder>,
    frequency: String,
) -> Result<(), StatusCode> {
    let Ok(val) = frequency.parse::<u32>() else {
        return Err(StatusCode::BAD_REQUEST);
    };
    if val < 50_000 || val > 1_500_000 {
        return Err(StatusCode::BAD_REQUEST);
    }
    data.set_frequency_khz(val);

    Ok(())
}

async fn index() -> impl IntoResponse {
    (
        [("content-type", "html")],
        include_str!("../frontend/build/index.html"),
    )
}

async fn sse_handler(
    State(data): State<RxDataHolder>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = data
        .dbfs_as_stream()
        .map(|val: f64| Event::default().data(format!("{:.2}", val)))
        .map(Ok);

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

fn router(data: RxDataHolder) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/frequency", post(change_frequency))
        .route("/sse", get(sse_handler))
        .with_state(data)
        .layer(TraceLayer::new_for_http())
}
