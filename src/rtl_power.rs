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
use rustfft::{num_complex::Complex, FftPlanner};
use std::f64::consts::PI;
use std::ops::DerefMut;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;
use tokio::sync::watch;

const FFT_SIZE: usize = 1024;
const SAMPLE_RATE: u32 = 1_024_000;
const BIN_SIZE_HZ: usize = SAMPLE_RATE as usize / FFT_SIZE;

pub struct RtlPower {
    control_thread: Option<thread::JoinHandle<()>>,
    data: RxDataHolder,
    frequency_khz: Arc<AtomicU32>,
    gain: i16,
    should_stop: Arc<AtomicBool>,
    tx: watch::Sender<f64>,
}

impl RtlPower {
    pub fn new(gain: i16) -> RtlPower {
        let (tx, rx) = watch::channel(0.0);
        let data = RxDataHolder::new(rx);
        let frequency = data.get_frequency_khz();

        RtlPower {
            control_thread: None,
            data: data,
            frequency_khz: Arc::new(AtomicU32::new(frequency)),
            gain: gain,
            should_stop: Arc::new(AtomicBool::new(false)),
            tx: tx,
        }
    }

    pub fn data(&self) -> RxDataHolder {
        self.data.clone()
    }

    pub fn start(&mut self) {
        self.should_stop.store(false, Ordering::Relaxed);
        self.start_control_thread();
    }

    pub fn stop(&mut self) {
        self.should_stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.control_thread.take() {
            thread.join().unwrap();
        }
    }

    fn start_control_thread(&mut self) {
        let gain = self.gain;
        let data = self.data.clone();
        let frequency_khz = self.frequency_khz.clone();
        let should_stop = self.should_stop.clone();
        let tx = self.tx.clone();

        // No need to worry about DC correction. Just move the center
        // frequency a little bit about target.
        let offset = SAMPLE_RATE / 8;
        let center_frequency = frequency_khz.load(Ordering::Relaxed) as u32 * 1_000 + offset;

        let (mut ctl, reader) =
            rtlsdr_mt::open(0).expect("Could not open RTL-SDR device at index 0.");
        ctl.set_sample_rate(SAMPLE_RATE).unwrap();
        ctl.set_center_freq(center_frequency).unwrap();
        ctl.disable_agc().unwrap();
        ctl.set_tuner_gain(gain as i32).unwrap();

        self.control_thread = Some(thread::spawn(move || {
            tracing::info!("Control thread started");

            let dsp_thread = start_dsp_thread(reader, tx, should_stop.clone());

            while !should_stop.load(Ordering::Relaxed) {
                let current_freq = frequency_khz.load(Ordering::Relaxed);
                let new_freq = data.get_frequency_khz();
                thread::sleep(time::Duration::from_millis(100));
                if current_freq != new_freq {
                    frequency_khz.store(new_freq, Ordering::Relaxed);
                    tracing::info!(
                        "Changing frequency from {:.3} MHz to {:.3} MHz",
                        (current_freq as f64) / 1000.0,
                        (new_freq as f64) / 1000.0,
                    );
                    // No need to worry about DC correction. Just move the center
                    // frequency a little bit about target.
                    let center_frequency = new_freq as u32 * 1_000 + offset;
                    ctl.set_center_freq(center_frequency).unwrap();
                }
            }
            ctl.cancel_async_read();
            dsp_thread.join().unwrap();
            tracing::info!("Stopping control thread");
        }));
    }
}

fn apply_window(complex_signal: &mut [Complex<f64>], window: &[f64]) {
    for i in 0..window.len() {
        complex_signal[i] *= window[i];
    }
}

fn hann_window(window_buffer: &mut [f64]) {
    let m = window_buffer.len() as f64;
    for i in 0..window_buffer.len() {
        let n = i as f64;
        window_buffer[i] = 0.5 - 0.5 * (2.0 * PI * n / (m - 1.0)).cos();
    }
}

fn start_dsp_thread(
    mut reader: rtlsdr_mt::Reader,
    sender: watch::Sender<f64>,
    should_stop: Arc<AtomicBool>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        tracing::info!("DSP thread started");
        let mut planner: FftPlanner<f64> = FftPlanner::new();
        let fft = planner.plan_fft_forward(FFT_SIZE);
        let mut result = [0.0f64; FFT_SIZE];
        let mut window = [0.0f64; FFT_SIZE];
        hann_window(&mut window);
        let squared_size = (FFT_SIZE as f64).powi(2);

        while !should_stop.load(Ordering::Relaxed) {
            match reader.read_async(1, 2048, |iq_samples| {
                let mut complex_signal_vector = iq_samples
                    .chunks(2)
                    .map(|pair| Complex {
                        re: (f64::from(pair[0]) - 127.0) / 127.0,
                        im: (f64::from(pair[1]) - 127.0) / 127.0,
                    })
                    .collect::<Vec<Complex<f64>>>();
                let complex_signal = complex_signal_vector.deref_mut();
                apply_window(complex_signal, &window);
                fft.process(complex_signal);

                for (i, c) in complex_signal.into_iter().enumerate() {
                    // The signal is shifted by half of the FFT_SIZE.
                    let index = (i + FFT_SIZE / 2) % FFT_SIZE;
                    let normalized_magnitude = c.norm_sqr() / squared_size;
		    // Clip the signal between 0 and -120 dBFS
                    let logmag = normalized_magnitude.max(1e-12).log10().min(0.0);
                    let dbfs = 10.0 * logmag;
                    result[index] = dbfs;
                }
                // The center frequency is one eigthth above the frequency of
                // interest. So the bins to analyze are at 3/8ths of the
                // result slice.
                let center = FFT_SIZE / 8 * 3;
                // A range of 30 KHz is analyzed.
                let range = 15_000usize.div_ceil(BIN_SIZE_HZ);
                let mut peak: f64 = -120.0;
                for m in result[center - range..center + range].into_iter() {
                    if *m > peak {
                        peak = *m;
                    }
                }

                match sender.send(peak) {
                    Ok(..) => {}
                    Err(err) => {
                        tracing::warn!("Error sending peak value via channel: {err}");
                    }
                };

                thread::sleep(time::Duration::from_millis(100));
            }) {
                Ok(..) => {}
                Err(..) => {
                    tracing::warn!("Reading samples failed. Continuing...");
                }
            }
        }
        tracing::info!("Stopping DSP thread");
    })
}
