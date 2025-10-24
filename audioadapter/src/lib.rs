#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

/// The traits for accessing samples in buffers.
mod traits;
pub use traits::{Adapter, AdapterMut};

/// Calculate statistics for adapters with numerical sample types
pub mod stats;

/// Read-only iterators
mod iterators;

pub use iterators::AdapterIterators;

#[cfg(feature = "audio")]
pub mod audio;

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
        unsafe fn write_sample_unchecked(
            &mut self,
            channel: usize,
            frame: usize,
            value: &T,
        ) -> bool {
            let index = frame * self.channels + channel;
            *self.buf.get_unchecked_mut(index) = value.clone();
            false
        }
    }
}
