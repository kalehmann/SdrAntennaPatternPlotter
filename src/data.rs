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

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::watch::Receiver;
use tokio_stream::wrappers::WatchStream;

#[derive(Clone)]
pub struct RxDataHolder {
    frequency_khz: Arc<AtomicU32>,
    rx: Receiver<f64>,
}

impl RxDataHolder {
    pub fn new(rx: Receiver<f64>) -> RxDataHolder {
        RxDataHolder {
            frequency_khz: Arc::new(AtomicU32::new(145_000)),
            rx: rx,
        }
    }

    pub fn dbfs_as_stream(&self) -> WatchStream<f64> {
        WatchStream::<f64>::from_changes(self.rx.clone())
    }

    pub fn get_frequency_khz(&self) -> u32 {
        self.frequency_khz.load(Ordering::Relaxed)
    }

    pub fn set_frequency_khz(&self, val: u32) {
        self.frequency_khz.store(val, Ordering::Relaxed);
    }
}
