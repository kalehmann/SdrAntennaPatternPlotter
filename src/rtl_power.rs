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
use std::io::{BufRead, BufReader};
use std::process::{Child, ChildStderr, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;
use tokio::sync::watch;

fn end_process(proc: &mut Child) {
    // First try it the graceful way
    let mut kill = Command::new("kill")
        .args(["-s", "TERM", &proc.id().to_string()])
        .spawn()
        .expect("Could not launch 'kill' to send signal to process.");
    kill.wait().expect("Could not await process 'kill'.");

    for _ in 0..5 {
        if !process_running(proc) {
            return;
        }
        thread::sleep(time::Duration::from_secs(1));
    }

    // Omit the result here as the process may have ended in the mean time.
    let _ = proc.kill();
}

fn process_running(proc: &mut Child) -> bool {
    match proc.try_wait() {
        Ok(Some(_)) => false,
        Ok(None) => true,
        Err(e) => panic!("Error checking child process status: {}", e),
    }
}

fn start_rtl_power(freq_khz: u32, gain: i16) -> Child {
    Command::new("rtl_power")
        .arg("-f")
        .arg(format!("{}K:{}K:1k", freq_khz - 100, freq_khz + 100))
        .arg("-i")
        .arg("1")
        .arg("-g")
        .arg(format!("{}", gain))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Could not start 'rtl_power'. Is it in $PATH ?")
}

fn start_stderr_thread(
    should_stop: Arc<AtomicBool>,
    stderr: ChildStderr,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        tracing::info!("Stderr thread started");
        let reader = BufReader::new(stderr);

        for line in reader.lines().filter_map(|l| l.ok()) {
            tracing::debug!("rtl_power: {}", line);
            if should_stop.load(Ordering::Relaxed) {
                break;
            }

            if line.starts_with("Error:") {
                tracing::warn!("'rtl_power' failed with {}", line);
                should_stop.store(true, Ordering::Relaxed);
            } else if line.starts_with("No supported devices found") {
                tracing::warn!("'rtl_power' failed with {}", line);
                should_stop.store(true, Ordering::Relaxed);
            }
        }

        tracing::info!("Stopping stderr thread");
    })
}

fn start_stdout_thread(
    should_stop: Arc<AtomicBool>,
    mut command: Child,
    stdout: ChildStdout,
    tx: watch::Sender<f64>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        tracing::info!("Stdout thread started");
        let mut started = false;
        let reader = BufReader::new(stdout);

        for line in reader.lines().filter_map(|l| l.ok()) {
            if should_stop.load(Ordering::Relaxed) {
                break;
            }

            // Skip date, time and so on ...
            let values = line
                .split(", ")
                .skip(6)
                .filter_map(|v| v.parse::<f64>().ok())
                // Integer cast is need here as floats are not ordered.
                .map(|v| (v * 100.0) as i32);
            if let Some(val) = values.max() {
                tx.send((val as f64) / 100.0).unwrap();
                started = true;
            }
        }

        if !started {
            tracing::warn!("'rtl_power' did not successfully start up!");
        } else if !should_stop.load(Ordering::Relaxed) {
            tracing::warn!("'rtl_power' exited early!");
        }

        end_process(&mut command);
        tracing::info!("Stopping stdout thread");
    })
}

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

    #[allow(dead_code)]
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
        let should_io_stop = Arc::new(AtomicBool::new(false));
        let should_stop = self.should_stop.clone();
        let tx = self.tx.clone();

        self.control_thread = Some(thread::spawn(move || {
            tracing::info!("Control thread started");
            let mut command = start_rtl_power(data.get_frequency_khz(), gain);
            let mut stderr_thread =
                start_stderr_thread(should_io_stop.clone(), command.stderr.take().unwrap());
            let mut stdout = command.stdout.take().unwrap();
            let mut stdout_thread =
                start_stdout_thread(should_io_stop.clone(), command, stdout, tx.clone());

            while !should_stop.load(Ordering::Relaxed) {
                let current_freq = frequency_khz.load(Ordering::Relaxed);
                let new_freq = data.get_frequency_khz();
                thread::sleep(time::Duration::from_millis(100));
                if current_freq != new_freq {
                    should_io_stop.store(true, Ordering::Relaxed);
                    stderr_thread.join().unwrap();
                    stdout_thread.join().unwrap();
                    frequency_khz.store(new_freq, Ordering::Relaxed);
                    should_io_stop.store(false, Ordering::Relaxed);
                    tracing::info!(
                        "Changing frequency from {:.3} MHz to {:.3} MHz",
                        (current_freq as f64) / 1000.0,
                        (new_freq as f64) / 1000.0,
                    );
                    command = start_rtl_power(new_freq, gain);
                    stderr_thread =
                        start_stderr_thread(should_io_stop.clone(), command.stderr.take().unwrap());
                    stdout = command.stdout.take().unwrap();
                    stdout_thread =
                        start_stdout_thread(should_io_stop.clone(), command, stdout, tx.clone());
                }
            }
            should_io_stop.store(true, Ordering::Relaxed);
            stderr_thread.join().unwrap();
            stdout_thread.join().unwrap();
            tracing::info!("Stopping control thread");
        }));
    }
}
