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
use futures::stream::{self, Stream};
use std::convert::Infallible;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio_stream::StreamExt as _;
use tower_http::trace::TraceLayer;

pub fn run_web_app(data: RxDataHolder) {
    let runtime = Runtime::new().unwrap();

    runtime.block_on(async {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        tracing::info!("Listening on 0.0.0.0:3000");
        axum::serve(listener, router(data)).await.unwrap();
    });
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
        include_str!("../assets/index.html"),
    )
}

async fn sse_handler(
    State(data): State<RxDataHolder>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream =
        stream::repeat_with(move || Event::default().data(format!("{:.2}", data.get_dbfs())))
            .map(Ok)
            .throttle(Duration::from_secs(1));

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
