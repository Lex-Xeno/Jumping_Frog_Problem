use crate::frog::FrogTrait;

use super::frog::*;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

/// A single thread that makes a [`Frog`] jump until it returns to its starting point, returns that [`Frog`] via a [`mpsc::channel`] and repeats this until it is sent a stop signal via another [`mpsc::channel`]
pub struct Handler {
    thread: JoinHandle<()>,
    channel: Sender<Signal>,
}

/// A type that spawns and manages a thread that simulates a frog
impl Handler {
    /// creates a [`Vec`] of [`Handler`] with the size given to it and returns that [`Vec`] and a [`mpsc::Receiver`] to receive the frogs' results on
    pub fn new_group(size: usize) -> (Vec<Handler>, Receiver<(usize, String)>) {
        let (tx, rx) = mpsc::channel();

        (
            (0..size)
                .map(|id| Handler::new::<Frog>(tx.clone(), id))
                .collect::<Vec<Handler>>(),
            rx,
        )
    }

    /// Spawns a new thread with a separate [`mpsc::Receiver`] and [`mpsc::Sender`] for [`start`](Worker::start) and [`stop`](Worker::stop) operations, and [results](Workers<Active>::receive_result) respectively
    fn new<T: FrogTrait>(result_sender: Sender<(usize, String)>, id: usize) -> Self {
        let (channel, receiver) = mpsc::channel();

        let thread = thread::spawn(move || loop {
            match receiver.recv() {
                Ok(signal) => match signal {
                    Signal::Job => (),
                    Signal::Stop => break,
                },
                Err(_) => panic!("The main thread has hung up this handlers channel!"),
            }

            let mut frog = T::start(None);

            while frog.jump().is_none() {}
            result_sender
                .send((id, frog.result()))
                .unwrap_or_else(|error| {
                    panic!("A handler failed to send finished frog! Error: {}", error)
                });
        });

        Self { thread, channel }
    }

    /// Tells the handler to simulate a frog
    ///
    /// # Error
    /// Returns an [`Err`] if the handler has hung up its [`mpsc::Receiver`]
    pub fn issue(&self) -> Result<(), SendError> {
        let result = self.channel.send(Signal::Job);

        if result.is_ok() {
            Ok(())
        } else {
            Err(SendError)
        }
    }

    /// Sends the stop signal to the [`Handler`]
    ///
    /// # Error
    /// Returns an [`Err`] if the handler has hung up its [`mpsc::Receiver`]
    pub fn stop(&self) -> Result<(), SendError> {
        if self.channel.send(Signal::Stop).is_ok() {
            Ok(())
        } else {
            Err(SendError)
        }
    }

    /// Waits for the [`Handler`] to finish and it's thread to join
    ///
    /// # Error
    /// Returns an [`Err`] if the thread panicked
    pub fn wait(self) -> Result<(), Box<dyn std::any::Any + Send + 'static>> {
        self.thread.join()
    }
}

enum Signal {
    Job,
    Stop,
}

#[derive(Debug)]
pub struct SendError;

impl std::fmt::Display for SendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The line is closed")
    }
}

impl std::error::Error for SendError {}
