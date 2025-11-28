#![cfg(unix)]

/// Utilities for configuring the alternate signal stack on Unix platforms.
///
/// Rust installs a small alternate signal stack for handling certain signals such as `SIGSEGV`.
/// When embedding or hosting the .NET CoreCLR inside Rust, this small stack may be insufficient
/// for the CLR exception-handling mechanisms. Increasing or disabling the alternate signal stack
/// may be necessary to avoid a segfault in such cases.
///
/// See https://github.com/OpenByteDev/netcorehost/issues/38 for more details.
pub mod altstack {
    use libc::{
        mmap, sigaltstack, stack_t, MAP_ANON, MAP_FAILED, MAP_PRIVATE, PROT_READ, PROT_WRITE,
        SS_DISABLE,
    };
    use std::{io, mem::MaybeUninit, ptr};

    /// Represents the desired configuration of the alternate signal stack.
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub enum State {
        /// Disables Rust's alternate signal stack.
        Disabled,

        /// Enables and sets the alternate signal stack to a given size in bytes.
        Enabled {
            /// Target altstack size
            size: usize,
        },
    }

    impl Default for State {
        fn default() -> Self {
            Self::Enabled { size: 8 * 1024 }
        }
    }

    /// Configures the alternate signal stack according to the provided status.
    pub fn set(state: State) -> io::Result<()> {
        match state {
            State::Disabled => {
                let ss = stack_t {
                    ss_flags: SS_DISABLE,
                    ss_sp: ptr::null_mut(),
                    ss_size: 0,
                };

                let result = unsafe { sigaltstack(&raw const ss, ptr::null_mut()) };
                if result != 0 {
                    return Err(io::Error::last_os_error());
                }
            }

            State::Enabled { size } => {
                let ptr = unsafe {
                    mmap(
                        ptr::null_mut(),
                        size,
                        PROT_READ | PROT_WRITE,
                        MAP_PRIVATE | MAP_ANON,
                        -1,
                        0,
                    )
                };

                if ptr == MAP_FAILED {
                    return Err(io::Error::last_os_error());
                }

                let ss = stack_t {
                    ss_sp: ptr,
                    ss_size: size,
                    ss_flags: 0,
                };

                let result = unsafe { sigaltstack(&raw const ss, ptr::null_mut()) };
                if result != 0 {
                    return Err(io::Error::last_os_error());
                }
            }
        }

        Ok(())
    }

    /// Returns the current altstack status.
    pub fn get() -> io::Result<State> {
        let mut current = MaybeUninit::uninit();
        let result = unsafe { sigaltstack(ptr::null(), current.as_mut_ptr()) };
        if result != 0 {
            return Err(io::Error::last_os_error());
        }

        let current = unsafe { current.assume_init() };
        let enabled = current.ss_flags & SS_DISABLE == 0;
        let state = if enabled {
            State::Enabled {
                size: current.ss_size,
            }
        } else {
            State::Disabled
        };

        Ok(state)
    }
}
