//! Background slideshow worker and image selection logic.

use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

use crate::image_ops::process_image;
use crate::wallpaper::{set_wallpaper, set_wallpaper_style, StyleMode};

/// Command messages sent to the slideshow worker.
#[derive(Debug, Clone, Copy)]
pub enum SlideshowCommand {
    /// Stop the worker loop.
    Stop,
    /// Advance to the next image immediately.
    Next,
}

/// Events emitted by the worker to inform the UI.
#[derive(Debug)]
pub enum SlideshowEvent {
    /// Informational status text (last image set).
    Info(String),
    /// Terminal error that stops the worker.
    Error(String),
}

/// Handle to a background slideshow worker thread.
#[derive(Debug)]
pub struct SlideshowWorker {
    cmd_tx: Sender<SlideshowCommand>,
    event_rx: Receiver<SlideshowEvent>,
    join: Option<thread::JoinHandle<()>>,
}

impl SlideshowWorker {
    /// Spawn a slideshow worker and return a handle for control/event polling.
    pub fn start(
        images: Vec<PathBuf>,
        auto_rotate: bool,
        style: StyleMode,
        interval: Duration,
        random_order: bool,
    ) -> Result<Self> {
        set_wallpaper_style(style)?;

        let (cmd_tx, cmd_rx) = mpsc::channel();
        let (evt_tx, evt_rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let _ = run_worker(images, auto_rotate, interval, random_order, cmd_rx, evt_tx);
        });

        Ok(Self {
            cmd_tx,
            event_rx: evt_rx,
            join: Some(handle),
        })
    }

    /// Send a request to advance to the next image.
    pub fn request_next(&self) {
        let _ = self.cmd_tx.send(SlideshowCommand::Next);
    }

    /// Stop the worker thread without blocking the UI thread.
    pub fn stop(mut self) {
        let _ = self.cmd_tx.send(SlideshowCommand::Stop);
        if let Some(join) = self.join.take() {
            thread::spawn(move || {
                let _ = join.join();
            });
        }
    }

    /// Drain any pending events into the provided buffer.
    pub fn drain_events(&self, out: &mut Vec<SlideshowEvent>) {
        while let Ok(evt) = self.event_rx.try_recv() {
            out.push(evt);
        }
    }
}

/// Main worker loop that processes images and applies wallpapers.
fn run_worker(
    images: Vec<PathBuf>,
    auto_rotate: bool,
    interval: Duration,
    random_order: bool,
    cmd_rx: Receiver<SlideshowCommand>,
    evt_tx: Sender<SlideshowEvent>,
) -> Result<()> {
    let mut last: Option<PathBuf> = None;
    let mut rng = ChaChaRng::from_entropy();
    let mut skip_wait = false;

    loop {
        while let Ok(cmd) = cmd_rx.try_recv() {
            match cmd {
                SlideshowCommand::Stop => return Ok(()),
                SlideshowCommand::Next => {
                    // Skip the next sleep so the slideshow advances immediately.
                    skip_wait = true;
                }
            }
        }

        let next = if random_order {
            pick_random_with_rng(&images, last.as_ref(), &mut rng)?
        } else {
            sequential_pick(&images, last.as_ref())
        };

        let processed = process_image(&next, auto_rotate)?;
        if let Err(err) = set_wallpaper(&processed) {
            let _ = evt_tx.send(SlideshowEvent::Error(err.to_string()));
            break;
        }
        let _ = evt_tx.send(SlideshowEvent::Info(format!("Set: {}", next.display())));
        last = Some(next);

        if skip_wait {
            skip_wait = false;
            continue;
        }

        // Allow Next/Stop commands to interrupt the sleep interval.
        match wait_or_command(&cmd_rx, interval) {
            Some(SlideshowCommand::Stop) => break,
            Some(SlideshowCommand::Next) => continue,
            None => {}
        }
    }

    Ok(())
}

/// Block until the interval elapses or a command arrives.
fn wait_or_command(
    cmd_rx: &Receiver<SlideshowCommand>,
    interval: Duration,
) -> Option<SlideshowCommand> {
    let step = Duration::from_millis(500);
    let start = Instant::now();
    while start.elapsed() < interval {
        match cmd_rx.try_recv() {
            // Stop or advance immediately when requested.
            Ok(SlideshowCommand::Stop) => return Some(SlideshowCommand::Stop),
            Ok(SlideshowCommand::Next) => return Some(SlideshowCommand::Next),
            Err(_) => {}
        }
        let remaining = interval.saturating_sub(start.elapsed());
        thread::sleep(step.min(remaining));
    }
    None
}

/// Select the next image in order, wrapping at the end.
fn sequential_pick(images: &[PathBuf], last: Option<&PathBuf>) -> PathBuf {
    if images.is_empty() {
        return PathBuf::new();
    }
    if let Some(last) = last {
        if let Some(pos) = images.iter().position(|p| p == last) {
            return images[(pos + 1) % images.len()].clone();
        }
    }
    images[0].clone()
}

/// Select a random image, avoiding repeats when possible.
fn pick_random_with_rng(
    images: &[PathBuf],
    last: Option<&PathBuf>,
    rng: &mut ChaChaRng,
) -> Result<PathBuf> {
    if images.len() == 1 {
        return Ok(images[0].clone());
    }
    for _ in 0..5 {
        let candidate = images
            .choose(rng)
            .ok_or_else(|| anyhow::anyhow!("no images available"))?;
        if Some(candidate) != last {
            return Ok(candidate.clone());
        }
    }
    images
        .choose(rng)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("no images available"))
}
