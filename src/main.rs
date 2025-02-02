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

use std::thread;
use std::time;

mod data;
mod rtl_power;

fn main() {
    // Proof of concept for rtl_power module.
    // Read 10 values, switch frequency and read another 10 values
    let data = data::RxDataHolder::new();
    let mut rtlpwr = rtl_power::RtlPower::new(data.clone());
    rtlpwr.start();
    for _ in 0..10 {
        thread::sleep(time::Duration::from_secs(1));
        println!("{}", data.get_dbfs());
    }
    data.set_frequency_khz(435000);
    for _ in 0..10 {
        thread::sleep(time::Duration::from_secs(1));
        println!("{}", data.get_dbfs());
    }
    rtlpwr.stop();
}
