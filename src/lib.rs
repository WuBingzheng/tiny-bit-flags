//! Generate bit-flags struct and methods.
//!
//! It's very simple and easy to use. See the example below for details.
//!
//! # Usage
//!
//! Import this crate and `paste` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tiny-bit-flags = "0.1"
//! paste = "1.0"
//! ```
//!
//! Invoke the `tiny_bit_flags!` macro to define flags:
//!
//! ```rust
//! tiny_bit_flags::tiny_bit_flags! {
//!     struct PrimFlags: u32 { // FORMAT: struct <StructName>: <InnerType>
//!         // list flags below
//!         const WRITABLE   = 0b00000001;
//!         const EXECUTABLE = 0b00000010;
//!     }
//! }
//! ```
//!
//! This actually generates the following code:
//!
//! ```rust,ignore
//! // struct
//! struct PrimFlags(u32);
//!
//! impl PrimFlags {
//!     // constant values
//!     const WRITABLE: u32   = 0b00000001;
//!     const EXECUTABLE: u32 = 0b00000010;
//!     // checking methods
//!     const fn is_writable(&self) -> bool { ... }
//!     const fn is_executable(&self) -> bool { ... }
//!     // setting methods
//!     const fn set_writable(&mut self) { ... }
//!     const fn set_executable(&mut self) { ... }
//!     // clearing methods
//!     const fn clear_writable(&mut self) { ... }
//!     const fn clear_executable(&mut self) { ... }
//! }
//! ```
//!
//! Then you can use them in your program:
//!
//! ```rust,ignore
//! let mut f = PrimFlags(PrimFlags::WRITABLE); // initialize
//! assert!(f.is_writable()); // check flag
//! assert!(!f.is_executable());
//!
//! f.clear_writable(); // clear flag
//! assert!(!f.is_writable());
//!
//! f.set_executable(); // set flag
//! assert!(f.is_executable());
//! ```
//!
//! You can use `pub` before `struct` to make all above to be public:
//!
//! ```diff
//!  tiny_bit_flags! {
//! +    pub struct PrimFlags: u32 {
//! -    struct PrimFlags: u32 {
//! ```
//!
//! You can also derive some traits on the struct:
//!
//! ```diff
//!  tiny_bit_flags! {
//! +    #[derive(Copy, Clone, Debug, Default)]
//!      struct PrimFlags: u32 {
//! ```

/// Generate bit-flags struct and methods.
///
/// See module-level document for details.
///
/// Example:
///
/// ```rust
/// tiny_bit_flags::tiny_bit_flags! {
///     struct PrimFlags: u32 {
///         const WRITABLE   = 0b00000001;
///         const EXECUTABLE = 0b00000010;
///     }
/// }
///
/// let mut f = PrimFlags(PrimFlags::WRITABLE); // initialize
/// assert!(f.is_writable()); // check flag
/// assert!(!f.is_executable());
///
/// f.clear_writable(); // clear flag
/// assert!(!f.is_writable());
///
/// f.set_executable(); // set flag
/// assert!(f.is_executable());
/// ```
///
#[macro_export]
macro_rules! tiny_bit_flags {
    (
        $(#[$outer:meta])*
        $vis:vis struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:tt = $value:expr;
            )*
        }
    ) => {
        // struct
        $(#[$outer])*
        $vis struct $BitFlags($vis $T);

        impl $BitFlags {
            $(
                // constant values
                $(#[$inner $($args)*])*
                $vis const $Flag: $T = $value;

                // methods
                paste::paste! {
                    $vis const fn [<is_ $Flag:lower>](&self) -> bool {
                        self.0 & $value != 0
                    }
                    $vis const fn [<set_ $Flag:lower>](&mut self) {
                        self.0 |= $value
                    }
                    $vis const fn [<clear_ $Flag:lower>](&mut self) {
                        self.0 &= !$value
                    }
                }
            )*
        }
    };
}
