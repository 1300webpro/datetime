//! Exact points on a timeline.

use std::fmt;

use system::sys_time;


/// An **instant** is an exact point on the timeline, irrespective of time
/// zone or calendar format, with millisecond precision.
///
/// Internally, this is represented by a 64-bit integer of seconds, and a
/// 16-bit integer of milliseconds. This means that it will overflow (and thus
/// be unsuitable for) instants past GMT 15:30:08, Sunday 4th December,
/// 292,277,026,596 (yes, that’s a year)
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Instant {
    seconds: i64,
    milliseconds: i16,
}

impl Instant {

    /// Creates a new Instant set to the number of seconds since the Unix
    /// epoch, and zero milliseconds.
    pub fn at(seconds: i64) -> Instant {
        Instant::at_ms(seconds, 0)
    }

    /// Creates a new Instant set to the number of seconds since the
    /// Unix epoch, along with the number of milliseconds so far this
    /// second.
    pub fn at_ms(seconds: i64, milliseconds: i16) -> Instant {
        Instant { seconds: seconds, milliseconds: milliseconds }
    }

    /// Creates a new Instant set to the computer’s current time.
    pub fn now() -> Instant {
        let (s, ms) = unsafe { sys_time() };
        Instant { seconds: s, milliseconds: ms }
    }

    /// Creates a new Instant set to the Unix epoch.
    pub fn at_epoch() -> Instant {
        Instant::at(0)
    }

    /// Returns the number of seconds at this instant
    pub fn seconds(&self) -> i64 {
        self.seconds
    }

    /// Returns the number of milliseconds at this instant
    pub fn milliseconds(&self) -> i16 {
        self.milliseconds
    }
}

impl fmt::Debug for Instant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instant({}s/{}ms)", self.seconds, self.milliseconds)
    }
}


#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn seconds() {
        assert_eq!(Instant::at(3), Instant::at_ms(3, 0))
    }

    #[test]
    fn milliseconds() {
        assert_eq!(Instant::at_ms(3, 333).milliseconds(), 333)
    }

    #[test]
    fn epoch() {
        assert_eq!(Instant::at_epoch().seconds(), 0)
    }

    #[test]
    fn sanity() {
        // Test that the system call has worked at all.
        // If this fails then you have gone back in time, or something?
        assert!(Instant::now().seconds() != 0)
    }
}
