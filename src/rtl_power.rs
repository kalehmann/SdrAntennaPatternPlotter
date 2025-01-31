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

use std::io::{BufRead, BufReader};
use std::process::{Child, ChildStderr, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time;

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

fn start_rtl_power(freq: f64) -> Child {
    Command::new("rtl_power")
        .arg("-f")
        .arg(format!("{:.1}M:{:.1}M:1k", freq - 0.1, freq + 0.1))
        .arg("-i")
        .arg("1")
        .arg("-g")
        .arg("0")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Could not start 'rtl_power'. Is it in $PATH ?")
}

pub struct RtlPower {
    frequency_mhz: AtomicU32,
    latest_dbfs: Arc<AtomicU16>,
    should_stop: Arc<AtomicBool>,
    stderr_thread: Option<thread::JoinHandle<()>>,
    stdout_thread: Option<thread::JoinHandle<()>>,
}

impl RtlPower {
    pub fn new() -> RtlPower {
        RtlPower {
            frequency_mhz: AtomicU32::new(145_000),
            latest_dbfs: Arc::new(AtomicU16::new(0)),
            stderr_thread: None,
            stdout_thread: None,
            should_stop: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self) {
        self.latest_dbfs.store(0, Ordering::Relaxed);
        self.should_stop.store(false, Ordering::Relaxed);
        let freq = f64::from(self.frequency_mhz.load(Ordering::Relaxed)) / 100.0;
        let mut command = start_rtl_power(freq);
        self.start_stderr_thread(command.stderr.take().unwrap());

        let stdout = command.stdout.take().unwrap();
        self.start_stdout_thread(command, stdout);
    }

    pub fn dbfs(&self) -> f64 {
        let compressed_val = self.latest_dbfs.load(Ordering::Relaxed);

        (compressed_val as f64) * -0.01
    }

    pub fn set_frequency(&mut self, fmhz: f64) {
        let frequency = (fmhz * 100.0) as u32;
        self.stop();
        self.frequency_mhz.store(frequency, Ordering::Relaxed);
        self.start();
    }

    pub fn stop(&mut self) {
        self.should_stop.store(true, Ordering::Relaxed);

        if let Some(thread) = self.stderr_thread.take() {
            thread.join().unwrap();
        }
        if let Some(thread) = self.stdout_thread.take() {
            thread.join().unwrap();
        }
    }

    fn start_stderr_thread(&mut self, stderr: ChildStderr) {
        let should_stop = self.should_stop.clone();

        self.stderr_thread = Some(thread::spawn(move || {
            println!("Stderr thread started");
            let reader = BufReader::new(stderr);

            for line in reader.lines().filter_map(|l| l.ok()) {
                if should_stop.load(Ordering::Relaxed) {
                    break;
                }

                if line.starts_with("Error:") {
                    println!("'rtl_power' failed with {}. Exiting ...", line);
                    should_stop.store(true, Ordering::Relaxed);
                }
            }

            println!("Stderr thread exited");
        }));
    }

    fn start_stdout_thread(&mut self, mut command: Child, stdout: ChildStdout) {
        let latest_dbfs = self.latest_dbfs.clone();
        let should_stop = self.should_stop.clone();

        self.stdout_thread = Some(thread::spawn(move || {
            println!("Stdout thread started");
            let mut started = false;
            let reader = BufReader::new(stdout);

            for line in reader.lines().filter_map(|l| l.ok()) {
                if should_stop.load(Ordering::Relaxed) {
                    break;
                }

                // Skip date, time and so on ...
                let values = line
                    .split(", ")
                    .skip(5)
                    .filter_map(|v| v.parse::<f64>().ok())
                    .map(|v| (v * -100.0) as u16);
                if let Some(val) = values.max() {
                    latest_dbfs.store(val, Ordering::Relaxed);
                    started = true;
                }
            }

            if !started {
                println!("'rtl_power' did not successfully start up!");
            } else if !should_stop.load(Ordering::Relaxed) {
                println!("'rtl_power' exited early!");
            }

            end_process(&mut command);
            println!("Stdout thread exited");
        }));
    }
}
