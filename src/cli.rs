/* Copyright (c) 2025 by Karsten Lehmann <mail@kalehmann.de>
 *
 *   This file is part of SdrAntennaPatternPlotter.
 *
 *   SdrAntennaPatternPlotter is free software: you can redistribute it and/or
 *   modify it under the terms of the GNU Affero General Public License as
 *   published by the Free Software Foundation, either version 3 of the License,
 *   or (at your option) any later version.
 *
 *   SdrAntennaPatternPlotter is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero
 *   General Public License for more details.
 *
 *   You should have received a copy of the GNU Affero General Public License
 *   along with SdrAntennaPatternPlotter. If not, see
 *   <https://www.gnu.org/licenses/>. */

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = 1)]
    pub gain: i16,
    #[clap(flatten)]
    pub port: clap_port_flag::Port,
    /// Enables TLS with self signed certificate. Only for testing!!!
    #[arg(short, long)]
    pub tls: bool,
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}
