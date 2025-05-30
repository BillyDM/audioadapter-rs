#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]


/// The traits for accessing samples in buffers.
mod traits;
pub use traits::{Adapter, AdapterMut};

/// Calculate statistics for adapters with numerical sample types
pub mod stats;

/// Read-only iterators
mod iterators;

#[cfg(feature = "std")]
use std::error::Error;
#[cfg(feature = "std")]
use std::fmt;

pub use iterators::AdapterIterators;

#[cfg(feature = "audio")]
pub mod audio;


/// Error returned when the wrapped data structure has the wrong dimensions,
/// typically that it is too short.
#[derive(Debug)]
pub enum SizeError {
    Channel {
        index: usize,
        actual: usize,
        required: usize,
    },
    Frame {
        index: usize,
        actual: usize,
        required: usize,
    },
    Total {
        actual: usize,
        required: usize,
    },
    Mask {
        actual: usize,
        required: usize,
    },
}

#[cfg(feature = "std")]
impl Error for SizeError {}

#[cfg(feature = "std")]
impl fmt::Display for SizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = match self {
            SizeError::Channel {
                index,
                actual,
                required,
            } => format!(
                "Buffer for channel {} is too short, got: {}, required: {}",
                index, actual, required
            ),
            SizeError::Frame {
                index,
                actual,
                required,
            } => format!(
                "Buffer for frame {} is too short, got: {}, required: {}",
                index, actual, required
            ),
            SizeError::Total { actual, required } => format!(
                "Buffer is too short, got: {}, required: {}",
                actual, required
            ),
            SizeError::Mask { actual, required } => format!(
                "Mask is wrong length, got: {}, required: {}",
                actual, required
            ),
        };
        write!(f, "{}", &desc)
    }
}


#[cfg(test)]
mod tests {
    use crate::{Adapter, AdapterMut};

    // Minimal implementation of an Adapter based on a vec
    pub struct VecAdapter<U> {
        buf: Vec<U>,
        frames: usize,
        channels: usize,
    }


    impl<T> VecAdapter<T>
    where
        T: Clone,
    {
        pub fn new_from_vec(buf: Vec<T>, channels: usize, frames: usize) -> Self {
            Self {
                buf,
                frames,
                channels,
            }
        }

    }

    impl<'a, T> Adapter<'a, T> for VecAdapter<T>
    where
        T: Clone + 'a,
    {
        unsafe fn read_sample_unchecked(&self, channel: usize, frame: usize) -> T {
            let index = frame * self.channels + channel;
            self.buf.get_unchecked(index).clone()
        }

        fn channels(&self) -> usize {
            self.channels
        }

        fn frames(&self) -> usize {
            self.frames
        }
    }

    impl<'a, T> AdapterMut<'a, T> for VecAdapter<T>
    where
        T: Clone + 'a,
    {
        unsafe fn write_sample_unchecked(&mut self, channel: usize, frame: usize, value: &T) -> bool {
            let index = frame * self.channels + channel;
            *self.buf.get_unchecked_mut(index) = value.clone();
            false
        }
    }
}
