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

use std::sync::atomic::{AtomicU16, AtomicU32, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct RxDataHolder {
    frequency_khz: Arc<AtomicU32>,
    latest_dbfs: Arc<AtomicU16>,
}

impl RxDataHolder {
    pub fn new() -> RxDataHolder {
        RxDataHolder {
            frequency_khz: Arc::new(AtomicU32::new(145_000)),
            latest_dbfs: Arc::new(AtomicU16::new(0)),
        }
    }

    pub fn get_dbfs(&self) -> f64 {
        let compressed_val = self.latest_dbfs.load(Ordering::Relaxed);

        (compressed_val as f64) * -0.01
    }

    pub fn get_frequency_khz(&self) -> u32 {
        self.frequency_khz.load(Ordering::Relaxed)
    }

    pub fn set_dbfs(&self, val: f64) {
        let compressed_val = (val * -100.0) as u16;

        self.latest_dbfs.store(compressed_val, Ordering::Relaxed);
    }

    pub fn set_frequency_khz(&self, val: u32) {
        self.frequency_khz.store(val, Ordering::Relaxed);
    }
}
