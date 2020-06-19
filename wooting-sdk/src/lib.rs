#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    missing_docs,
    unused_extern_crates,
    unused_qualifications,
    unused_results
)]

//! # rust-wooting-sdk
//! This crate provides Rust bindings to the Analog and RGB SDKs provided by Wooting for the Wooting
//! One and Wooting Two.
//!
//! ## Example
//! See the [`wooting-sdk/src/examples/`][examples] directory for more examples.
//!
//! [examples]: https://github.com/davidtwco/rust-wooting-sdk/tree/master/wooting-sdk/examples
//!
//! ```rust,no_run
//! # fn test() -> Result<(), wooting_sdk::WootingError> {
//! use wooting_sdk::{
//!     analog::read_analog_key,
//!     rgb::RgbKeyboard,
//!     Key
//! };
//!
//! let min = u8::min_value();
//! let max = u8::max_value();
//!
//! // Check how far down W has been pressed..
//! match read_analog_key(Key::W)? {
//!     min => { /* ..not pressed. */ },
//!     max => { /* ..completely pressed. */ },
//!     _ => { /* ..partially pressed. */ },
//! }
//!
//! let mut keyboard = RgbKeyboard::default();
//!
//! // Modify the keyboard array so QWERTY will be set to white..
//! keyboard.array_set_full(&[
//!     (Key::Q, (255, 255, 255)),
//!     (Key::W, (255, 255, 255)),
//!     (Key::E, (255, 255, 255)),
//!     (Key::R, (255, 255, 255)),
//!     (Key::T, (255, 255, 255)),
//!     (Key::Y, (255, 255, 255)),
//! ]);
//!
//! // ..and apply the change.
//! keyboard.array_update();
//! # Ok(())
//! # }
//! ```

use std::error::Error;
use std::fmt::{self, Display};

/// Represents an error that can occur when querying the state of a Wooting keyboard.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum WootingError {
    /// Indicates that the keyboard is disconnected.
    Disconnected,
    /// Indicates that the requested number of analog key values was invalid. Must be non-zero
    /// and less than sixteen.
    InvalidBufferSize,
}

impl Display for WootingError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            WootingError::Disconnected => write!(fmt, "Wooting keyboard is not connected"),
            WootingError::InvalidBufferSize => {
                write!(fmt, "Requested analog value of too many keys")
            }
        }
    }
}

impl Error for WootingError {}

/// Types that implement this trait can be transformed into a matrix row and column.
pub trait IntoMatrixRowColumn {
    /// Return a tuple `(row, column)` that represents the matrix row and column for this type.
    fn into_matrix_row_and_column(&self) -> (u8, u8);
}

/// Types that implement this trait can be associated with a scan index.
pub trait FromScanIndex: Sized {
    /// Return the instance of this type for the given scan index.
    fn from_scan_index(index: u8) -> Option<Self>;
}

/// Represents a key on the keyboard.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Key {
    /// Escape key (`Esc`). Generates the escape character (ASCII 27).
    Escape,
    /// Function key (`F1`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F1,
    /// Function key (`F2`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F2,
    /// Function key (`F3`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F3,
    /// Function key (`F4`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F4,
    /// Function key (`F5`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F5,
    /// Function key (`F6`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F6,
    /// Function key (`F7`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F7,
    /// Function key (`F8`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F8,
    /// Function key (`F9`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F9,
    /// Function key (`F10`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F10,
    /// Function key (`F11`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F11,
    /// Function key (`F12`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F12,
    /// Print screen key (`Prt Sc`). May share the same key as system request. Normally takes a
    /// screenshot.
    PrintScreen,
    /// Pause (or break) key. Has no well defined purpose.
    Pause,
    /// Scroll lock key (`Scr Lk`). Has different functions or purposes depending on the
    /// application or operating system. Originally intended to make arrow keys scroll the current
    /// window contents instead of moving the cursor.
    ScrollLock,
    /// Analog profile key (`A1`). Switches to analog profile one.
    A1,
    /// Analog profile key (`A2`). Switches to analog profile two.
    A2,
    /// Analog profile key (`A3`). Switches to analog profile three.
    A3,
    /// Mode key. Toggles between digital and analog modes.
    Mode,
    /// Tilde key (`~`).
    Tilde,
    /// Number one key (`1`).
    One,
    /// Number two key (`2`).
    Two,
    /// Number three key (`3`).
    Three,
    /// Number four key (`4`).
    Four,
    /// Number five key (`5`).
    Five,
    /// Number six key (`6`).
    Six,
    /// Number seven key (`7`).
    Seven,
    /// Number eight key (`8`).
    Eight,
    /// Number nine key (`9`).
    Nine,
    /// Number zero key (`0`).
    Zero,
    /// Dash or hyphen key (`-`).
    Dash,
    /// Equals key (`=`).
    Equals,
    /// Backspace key. Moves display cursor one position backwards, deleting the character at
    /// that position and shifting back the text after that position by one position.
    Backspace,
    /// Insert key (`Ins`). Switches between two text entry modes - overtype or insert. Overtype
    /// mode replaces the character present in the current location. Insert mode inserts a
    /// character at the current position, forcing all characters past it one position further.
    Insert,
    /// Home key. Has the opposite effect of the end key.
    Home,
    /// Page up key (`Pg Up`). Scrolls up in documents.
    PageUp,
    /// Number lock key (`Num`). Affects the function of the numeric keypad located to the right
    /// of the main keyboard.
    NumLock,
    /// Divide key on the numpad (`/`). Types a forward slash or acts as a divison key in
    /// calculator applications.
    NumDivide,
    /// Multiply key on the numpad (`*`). Types a star or acts as a multiplication key in
    /// calculator applications.
    NumMultiply,
    /// Subtract key on the numpad (`-`). Types a dash or acts as a subtraction key in
    /// calculator applications.
    NumSubtract,
    /// Tab key. Advances cursor to next tab stop.
    Tab,
    /// Letter `q` key.
    Q,
    /// Letter `w` key.
    W,
    /// Letter `e` key.
    E,
    /// Letter `r` key.
    R,
    /// Letter `t` key.
    T,
    /// Letter `y` key.
    Y,
    /// Letter `u` key.
    U,
    /// Letter `i` key.
    I,
    /// Letter `o` key.
    O,
    /// Letter `p` key.
    P,
    /// Left square bracket key (`[`).
    LeftBracket,
    /// Right square bracket key (`]`).
    RightBracket,
    /// Backslash key (`\`).
    Backslash,
    /// Delete key (`Del`). Deletes the character in the position after the cursor.
    Delete,
    /// End key. Has the opposite effect of the home key.
    End,
    /// Page down key (`Pg Dn`). Scrolls down in documents.
    PageDown,
    /// Number seven key (`7`) on the numpad.
    NumSeven,
    /// Number eight key (`8`) on the numpad.
    NumEight,
    /// Number nine key (`9`) on the numpad.
    NumNine,
    /// Addition key on the numpad (`+`). Types a plus or acts as a addition key in calculator
    /// applications.
    NumAddition,
    /// Capitalization lock key. Causes all letters in latin-based scripts to be generated in
    /// capitals.
    CapsLock,
    /// Letter `a` key.
    A,
    /// Letter `s` key.
    S,
    /// Letter `d` key.
    D,
    /// Letter `f` key.
    F,
    /// Letter `g` key.
    G,
    /// Letter `h` key.
    H,
    /// Letter `j` key.
    J,
    /// Letter `k` key.
    K,
    /// Letter `l` key.
    L,
    /// Semi-colon key (`;`).
    SemiColon,
    /// Apostrophe key (`'`).
    Apostrophe,
    /// Represents a key that is specific to the keyboard layout. This key is positioned above
    /// the return key or to the left of the return key. On UK layouts, this is a pound (`#`) key.
    /// On US layouts, this is a backslash key. This key has the same scan index as the backslash
    /// key.
    ISO1,
    /// Return (or enter) key.
    Return,
    /// Number four key (`4`) on the numpad.
    NumFour,
    /// Number five key (`5`) on the numpad.
    NumFive,
    /// Number six key (`6`) on the numpad.
    NumSix,
    /// Left shift modifier key. Used to type capital letters and other alternate "upper"
    /// characters.
    LeftShift,
    /// Represents a key that is specific to the keyboard layout. This key is positioned to the
    /// right of the left shift key. On UK layouts, this is a backslash key. On US layouts, this
    /// key does not exist.
    ISO2,
    /// Letter `z` key.
    Z,
    /// Letter `x` key.
    X,
    /// Letter `c` key.
    C,
    /// Letter `v` key.
    V,
    /// Letter `b` key.
    B,
    /// Letter `n` key.
    N,
    /// Letter `m` key.
    M,
    /// Comma key (`,`).
    Comma,
    /// Period key (`.`).
    Period,
    /// Forward slash key (`/`).
    ForwardSlash,
    /// Right shift modifier key. Performs the same function as left shift.
    RightShift,
    /// Up arrow key. Moves the cursor in the upwards direction.
    UpArrow,
    /// Number one key (`1`) on the numpad.
    NumOne,
    /// Number two key (`2`) on the numpad.
    NumTwo,
    /// Number three key (`3`) on the numpad.
    NumThree,
    /// Return (or enter) key on the numpad. Performs the same function as the normal return key.
    NumReturn,
    /// Left control modifier key. Performs a special operation when pressed in conjunction with
    /// another key.
    LeftControl,
    /// Left mod (or Windows) modifier key. Normally invokes the operating system's start menu.
    LeftMod,
    /// Left alt modifier key. Used to change (alternate) the function of other pressed keys.
    LeftAlt,
    /// Space key (` `).
    Space,
    /// Right alt modifier key. Performs the same function as left alt.
    RightAlt,
    /// Right mod modifier key. Performs the same function as left mod.
    RightMod,
    /// Function key (`Fn`). Performs an alternative operation for some keys, normally defined
    /// by the keyboard and indicated by symbols on the key.
    Fn,
    /// Right control modifier key. Performs the same function as left control.
    RightControl,
    /// Left arrow key. Moves the cursor in the left direction.
    LeftArrow,
    /// Down arrow key. Moves the cursor in the down direction.
    DownArrow,
    /// Right arrow key. Moves the cursor in the right direction.
    RightArrow,
    /// Number zero key (`0`) on the numpad.
    NumZero,
    /// Delete key (`Del`) on the numpad. Performs the same function as the normal delete key.
    NumDelete,
}

impl Display for Key {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Key::*;
        write!(
            fmt,
            "{}",
            match self {
                Escape => "Esc",
                F1 => "F1",
                F2 => "F2",
                F3 => "F3",
                F4 => "F4",
                F5 => "F5",
                F6 => "F6",
                F7 => "F7",
                F8 => "F8",
                F9 => "F9",
                F10 => "F10",
                F11 => "F11",
                F12 => "F12",
                PrintScreen => "Print Screen",
                Pause => "Pause",
                ScrollLock => "Scroll Lock",
                A1 => "A1",
                A2 => "A2",
                A3 => "A3",
                Mode => "Mode",
                Tilde => "~",
                One => "1",
                Two => "2",
                Three => "3",
                Four => "4",
                Five => "5",
                Six => "6",
                Seven => "7",
                Eight => "8",
                Nine => "9",
                Zero => "0",
                Dash => "-",
                Equals => "=",
                Backspace => "Backspace",
                Insert => "Insert",
                Home => "Home",
                PageUp => "Page Up",
                NumLock => "Num Lock",
                NumDivide => "\\",
                NumMultiply => "*",
                NumSubtract => "-",
                Tab => "Tab",
                Q => "Q",
                W => "W",
                E => "E",
                R => "R",
                T => "T",
                Y => "Y",
                U => "U",
                I => "I",
                O => "O",
                P => "P",
                LeftBracket => "[",
                RightBracket => "]",
                Backslash => "\\",
                Delete => "Delete",
                End => "End",
                PageDown => "Page Down",
                NumSeven => "7",
                NumEight => "8",
                NumNine => "9",
                NumAddition => "+",
                CapsLock => "Caps Lock",
                A => "A",
                S => "S",
                D => "D",
                F => "F",
                G => "G",
                H => "H",
                J => "J",
                K => "K",
                L => "L",
                SemiColon => ";",
                Apostrophe => "'",
                ISO1 => "ISO",
                Return => "Return",
                NumFour => "4",
                NumFive => "5",
                NumSix => "6",
                LeftShift => "Left Shift",
                ISO2 => "ISO",
                Z => "Z",
                X => "X",
                C => "C",
                V => "V",
                B => "B",
                N => "N",
                M => "M",
                Comma => "Comma",
                Period => "Period",
                ForwardSlash => "/",
                RightShift => "Right Shift",
                UpArrow => "Up Arrow",
                NumOne => "1",
                NumTwo => "2",
                NumThree => "3",
                NumReturn => "Return",
                LeftControl => "Left Control",
                LeftMod => "Left Mod",
                LeftAlt => "Left Alt",
                Space => "Space",
                RightAlt => "Right Alt",
                RightMod => "Right Mod",
                Fn => "Fn",
                RightControl => "Right Control",
                LeftArrow => "Left Arrow",
                DownArrow => "Down Arrow",
                RightArrow => "Right Arrow",
                NumZero => "0",
                NumDelete => "Delete",
            }
        )
    }
}

impl FromScanIndex for Key {
    /// Return the key that corresponds to the provided scan index, if any.
    fn from_scan_index(index: u8) -> Option<Self> {
        use Key::*;
        Some(match index {
            0 => Escape,
            1 => F1,
            2 => F2,
            3 => F3,
            4 => F4,
            5 => F5,
            6 => F6,
            7 => F7,
            8 => F8,
            9 => F9,
            10 => F10,
            11 => F11,
            12 => F12,
            13 => PrintScreen,
            14 => Pause,
            15 => ScrollLock,
            16 => Tilde,
            17 => One,
            18 => Two,
            19 => Three,
            20 => Four,
            21 => Five,
            22 => Six,
            23 => Seven,
            24 => Eight,
            25 => Nine,
            26 => Zero,
            27 => Dash,
            28 => Equals,
            29 => Backspace,
            30 => Insert,
            31 => Home,
            32 => Tab,
            33 => Q,
            34 => W,
            35 => E,
            36 => R,
            37 => T,
            38 => Y,
            39 => U,
            40 => I,
            41 => O,
            42 => P,
            43 => LeftBracket,
            44 => RightBracket,
            // 45 is also associated with `Key::Backslash`
            45 => ISO1,
            46 => Delete,
            47 => End,
            48 => CapsLock,
            49 => A,
            50 => S,
            51 => D,
            52 => F,
            53 => G,
            54 => H,
            55 => I,
            56 => J,
            57 => K,
            58 => L,
            59 => SemiColon,
            60 => Return,
            61 => PageUp,
            62 => PageDown,
            63 => UpArrow,
            64 => LeftShift,
            65 => Z,
            66 => X,
            67 => C,
            68 => V,
            69 => B,
            70 => N,
            71 => M,
            72 => Comma,
            73 => Period,
            74 => ForwardSlash,
            75 => RightShift,
            76 => LeftArrow,
            77 => DownArrow,
            78 => RightArrow,
            79 => RightControl,
            80 => LeftControl,
            81 => LeftMod,
            82 => LeftAlt,
            83 => Space,
            84 => RightAlt,
            85 => RightMod,
            86 => Fn,
            87 => ISO2,
            90 => NumOne,
            91 => NumTwo,
            92 => NumThree,
            93 => NumReturn,
            94 => NumDelete,
            95 => NumZero,
            96 => NumSix,
            97 => NumFive,
            98 => NumFour,
            99 => NumAddition,
            100 => NumNine,
            101 => NumEight,
            102 => NumSeven,
            103 => NumSubtract,
            104 => NumMultiply,
            105 => NumDivide,
            106 => NumLock,
            107 => A1,
            108 => A2,
            109 => A3,
            110 => Mode,
            // Invalid index should only ever be one of: 88 | 89 | 111 ..= 255
            _ => return None,
        })
    }
}

impl IntoMatrixRowColumn for Key {
    /// Returns a tuple `(row, column)` that represents the matrix row and column of the key.
    fn into_matrix_row_and_column(&self) -> (u8, u8) {
        use Key::*;
        match self {
            Escape => (0, 0),
            F1 => (0, 2),
            F2 => (0, 3),
            F3 => (0, 4),
            F4 => (0, 5),
            F5 => (0, 6),
            F6 => (0, 7),
            F7 => (0, 8),
            F8 => (0, 9),
            F9 => (0, 10),
            F10 => (0, 11),
            F11 => (0, 12),
            F12 => (0, 13),
            PrintScreen => (0, 14),
            Pause => (0, 15),
            ScrollLock => (0, 16),
            A1 => (0, 17),
            A2 => (0, 18),
            A3 => (0, 19),
            Mode => (0, 20),
            Tilde => (1, 0),
            One => (1, 1),
            Two => (1, 2),
            Three => (1, 3),
            Four => (1, 4),
            Five => (1, 5),
            Six => (1, 6),
            Seven => (1, 7),
            Eight => (1, 8),
            Nine => (1, 9),
            Zero => (1, 10),
            Dash => (1, 11),
            Equals => (1, 12),
            Backspace => (1, 13),
            Insert => (1, 14),
            Home => (1, 15),
            PageUp => (1, 16),
            NumLock => (1, 17),
            NumDivide => (1, 18),
            NumMultiply => (1, 19),
            NumSubtract => (1, 20),
            Tab => (2, 0),
            Q => (2, 1),
            W => (2, 2),
            E => (2, 3),
            R => (2, 4),
            T => (2, 5),
            Y => (2, 6),
            U => (2, 7),
            I => (2, 8),
            O => (2, 9),
            P => (2, 10),
            LeftBracket => (2, 11),
            RightBracket => (2, 12),
            Backslash => (2, 13),
            Delete => (2, 14),
            End => (2, 15),
            PageDown => (2, 16),
            NumSeven => (2, 17),
            NumEight => (2, 18),
            NumNine => (2, 19),
            NumAddition => (2, 20),
            CapsLock => (3, 0),
            A => (3, 1),
            S => (3, 2),
            D => (3, 3),
            F => (3, 4),
            G => (3, 5),
            H => (3, 6),
            J => (3, 7),
            K => (3, 8),
            L => (3, 9),
            SemiColon => (3, 10),
            Apostrophe => (3, 11),
            ISO1 => (3, 12),
            Return => (3, 13),
            NumFour => (3, 17),
            NumFive => (3, 18),
            NumSix => (3, 19),
            LeftShift => (4, 0),
            ISO2 => (4, 1),
            Z => (4, 2),
            X => (4, 3),
            C => (4, 4),
            V => (4, 5),
            B => (4, 6),
            N => (4, 7),
            M => (4, 8),
            Comma => (4, 9),
            Period => (4, 10),
            ForwardSlash => (4, 11),
            RightShift => (4, 13),
            UpArrow => (4, 15),
            NumOne => (4, 17),
            NumTwo => (4, 18),
            NumThree => (4, 19),
            NumReturn => (4, 20),
            LeftControl => (5, 0),
            LeftMod => (5, 1),
            LeftAlt => (5, 2),
            Space => (5, 6),
            RightAlt => (5, 10),
            RightMod => (5, 11),
            Fn => (5, 12),
            RightControl => (5, 13),
            LeftArrow => (5, 14),
            DownArrow => (5, 15),
            RightArrow => (5, 16),
            NumZero => (5, 18),
            NumDelete => (5, 19),
        }
    }
}

/// Contains functions from Wooting's Analog SDK.
#[cfg(feature = "analog")]
pub mod analog {
    use super::{FromScanIndex, IntoMatrixRowColumn, WootingError};

    use std::sync::Mutex;

    use lazy_static::lazy_static;
    use wooting_analog_sdk_sys;

    lazy_static! {
        static ref CALLBACK: Mutex<Option<Box<Fn() + Send>>> = Default::default();
    }

    /// Is there a Wooting keyboard connected?
    ///
    /// ```rust,no_run
    /// // Assert that a Wooting keyboard is connected..
    /// assert!(wooting_sdk::analog::is_wooting_keyboard_connected());
    /// ```
    pub fn is_wooting_keyboard_connected() -> bool {
        unsafe { wooting_analog_sdk_sys::wooting_kbd_connected() }
    }

    /// This is a trampoline function that is provided to the C function to be invoked which will
    /// in turn invoke the user provided callback. The user provided callback would normally be
    /// stored in userdata but due to the lack of any, we use a static instead.
    extern "C" fn set_disconnected_callback_handler() {
        if let Some(ref mut callback) = *CALLBACK.lock().unwrap() {
            callback();
        } else {
            panic!("Callback static has not been set");
        }
    }

    /// Set a callback to be invoked when a keyboard is disconnected. Currently only happens on a
    /// failed read.
    ///
    /// See [`analog_disconnected_callback`][example] example for usage.
    ///
    /// [example]: https://github.com/davidtwco/rust-wooting-sdk/blob/master/wooting-sdk/examples/analog_set_disconnected.rs
    pub fn set_disconnected_callback<F: 'static + Fn() + Send>(callback: F) {
        *CALLBACK.lock().unwrap() = Some(Box::new(callback));
        unsafe {
            wooting_analog_sdk_sys::wooting_set_disconnected_cb(Some(
                set_disconnected_callback_handler,
            ));
        }
    }

    /// Read the analog value, represented by a `u8`, of the requested key.
    ///
    /// ```rust,no_run
    /// # fn test() -> Result<(), wooting_sdk::WootingError> {
    /// use wooting_sdk::{analog::read_analog_key, Key};
    ///
    /// let min = u8::min_value();
    /// let max = u8::max_value();
    ///
    /// // Check how far down W has been pressed..
    /// match read_analog_key(Key::W)? {
    ///     min => { /* ..not pressed. */ },
    ///     max => { /* ..completely pressed. */ },
    ///     _ => { /* ..partially pressed. */ },
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_analog_key<K: IntoMatrixRowColumn>(key: K) -> Result<u8, WootingError> {
        let (row, column) = key.into_matrix_row_and_column();
        let ret = unsafe { wooting_analog_sdk_sys::wooting_read_analog(row, column) };
        if ret == 0 && !is_wooting_keyboard_connected() {
            Err(WootingError::Disconnected)
        } else {
            Ok(ret)
        }
    }

    /// Read the analog value, represented by a `u8`, of pressed keys, up to a maximum of
    /// `n` keys (maximum of sixteen).
    ///
    /// This function will return `Err(WootingError::InvalidBufferSize)` if `n` is zero or larger
    /// than sixteen.
    ///
    /// ```rust,no_run
    /// # fn test() -> Result<(), wooting_sdk::WootingError> {
    /// use wooting_sdk::{analog::read_analog_keys, Key};
    ///
    /// // Read the value of two pressed keys and check if they are CTRL and A..
    /// if let &[(Key::LeftControl, ctrl), (Key::A, a)] = read_analog_keys(2)?.as_slice() {
    ///     // ..if so, check if they are at least half pressed..
    ///     let is_half_pressed = |v: u8| v >= u8::max_value() / 2;
    ///     if is_half_pressed(ctrl) && is_half_pressed(a) {
    ///         // ..and if they are, select all.
    ///         select_all();
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// # fn select_all() {}
    /// ```
    pub fn read_analog_keys<K: FromScanIndex>(n: u8) -> Result<Vec<(K, u8)>, WootingError> {
        if n == 0 || n > 16 {
            return Err(WootingError::InvalidBufferSize);
        }

        let buffer_length = n as usize * 2;
        let mut buffer: Vec<u8> = vec![0; buffer_length];
        let ret: i32 = unsafe {
            wooting_analog_sdk_sys::wooting_read_full_buffer(
                buffer.as_mut_ptr(),
                buffer_length as u32,
            )
        };

        if ret == -1 {
            Err(WootingError::Disconnected)
        } else if ret < -1 {
            panic!("Invalid return code from Wooting Analog SDK");
        } else {
            Ok(buffer
                .chunks(2)
                .take(ret as usize)
                .filter_map(|chunk| match chunk {
                    &[scan_index, analog_value] => {
                        K::from_scan_index(scan_index).map(|key| (key, analog_value))
                    }
                    _ => unreachable!(),
                })
                .collect())
        }
    }
}

/// Contains functions from Wooting's RGB SDK.
#[cfg(feature = "rgb")]
pub mod rgb {
    use super::IntoMatrixRowColumn;

    use std::sync::Mutex;

    use lazy_static::lazy_static;
    use wooting_rgb_sdk_sys;

    /// How many columns are there?
    const COLUMNS: usize = 21;
    /// How many rows are there?
    const ROWS: usize = 6;
    /// How many components are there in a color?
    const COMPONENTS: usize = 3;

    lazy_static! {
        static ref CALLBACK: Mutex<Option<Box<Fn() + Send>>> = Default::default();
    }

    /// Is there a Wooting keyboard connected?
    ///
    /// ```rust,no_run
    /// // Assert that a Wooting keyboard is connected..
    /// assert!(wooting_sdk::rgb::is_wooting_keyboard_connected());
    /// ```
    pub fn is_wooting_keyboard_connected() -> bool {
        unsafe { wooting_rgb_sdk_sys::wooting_rgb_kbd_connected() }
    }

    /// This is a trampoline function that is provided to the C function to be invoked which will
    /// in turn invoke the user provided callback. The user provided callback would normally be
    /// stored in userdata but due to the lack of any, we use a static instead.
    extern "C" fn set_disconnected_callback_handler() {
        if let Some(ref mut callback) = *CALLBACK.lock().unwrap() {
            callback();
        } else {
            panic!("Callback static has not been set");
        }
    }

    /// Set a callback to be invoked when a keyboard is disconnected. Currently only happens on a
    /// failed read.
    ///
    /// See [`rgb_disconnected_callback`][example] example for usage.
    ///
    /// [example]: https://github.com/davidtwco/rust-wooting-sdk/blob/master/wooting-sdk/examples/rgb_set_disconnected.rs
    pub fn set_disconnected_callback<F: 'static + Fn() + Send>(callback: F) {
        *CALLBACK.lock().unwrap() = Some(Box::new(callback));
        unsafe {
            wooting_rgb_sdk_sys::wooting_rgb_set_disconnected_cb(Some(
                set_disconnected_callback_handler,
            ));
        }
    }

    /// Represents the connected keyboard to perform RGB operations. This struct is empty and
    /// only exists to enforce that `reset` is called on drop.
    #[derive(Clone, Debug, Default)]
    pub struct RgbKeyboard;

    impl RgbKeyboard {
        /// Set the color of a single key. This will not influence the keyboard color array. Use
        /// this function for simple amplifications, like a notification. Use the array functions
        /// if you want to change the entire keyboard. Returns `true` if the color is set.
        ///
        /// ```rust,no_run
        /// use wooting_sdk::{rgb::RgbKeyboard, Key};
        ///
        /// let mut keyboard = RgbKeyboard::default();
        /// // Set the A key to white...
        /// keyboard.direct_set_key(Key::A, 255, 255, 255);
        /// ```
        pub fn direct_set_key<K: IntoMatrixRowColumn>(
            &mut self,
            key: K,
            red: u8,
            green: u8,
            blue: u8,
        ) -> bool {
            let (row, column) = key.into_matrix_row_and_column();
            unsafe {
                wooting_rgb_sdk_sys::wooting_rgb_direct_set_key(row, column, red, green, blue)
            }
        }

        /// Directly reset the color of a single key on the keyboard. This will not influence the
        /// keyboard color array. Use this function for simple amplifications, like a notification.
        /// Use the array functions if you want to change the entire keyboard. Returns `true` if
        /// the color is reset.
        ///
        /// ```rust,no_run
        /// use wooting_sdk::{rgb::RgbKeyboard, Key};
        ///
        /// let mut keyboard = RgbKeyboard::default();
        /// // Set the A key to white...
        /// keyboard.direct_set_key(Key::A, 255, 255, 255);
        /// // ..and then reset it back!
        /// keyboard.direct_reset_key(Key::A);
        /// ```
        pub fn direct_reset_key<K: IntoMatrixRowColumn>(&mut self, key: K) -> bool {
            let (row, column) = key.into_matrix_row_and_column();
            unsafe { wooting_rgb_sdk_sys::wooting_rgb_direct_reset_key(row, column) }
        }

        /// Apply any updates made by the `array_set_single` and `array_set_full` functions.
        /// Returns `true` if the colors are updated.
        ///
        /// ```rust,no_run
        /// use wooting_sdk::{rgb::RgbKeyboard, Key};
        ///
        /// let mut keyboard = RgbKeyboard::default();
        /// // Modify keyboard array so A will be set to white..
        /// keyboard.array_set_single(Key::A, 255, 255, 255);
        /// // ..and apply the change.
        /// keyboard.array_update();
        /// ```
        pub fn array_update(&mut self) -> bool {
            unsafe { wooting_rgb_sdk_sys::wooting_rgb_array_update_keyboard() }
        }

        /// Set an auto-update trigger after every change with the `array_set_single` and
        /// `array_set_full` functions. By default, no auto-update trigger is set.
        ///
        /// ```rust,no_run
        /// use wooting_sdk::{rgb::RgbKeyboard, Key};
        ///
        /// let mut keyboard = RgbKeyboard::default();
        /// // Make keyboard array changes apply automatically..
        /// keyboard.array_auto_update(true);
        /// // ..and then modify the array so QWERTY are set to white...
        /// // ..with no need for a call to `array_update`!
        /// keyboard.array_set_full(&[
        ///     (Key::Q, (255, 255, 255)),
        ///     (Key::W, (255, 255, 255)),
        ///     (Key::E, (255, 255, 255)),
        ///     (Key::R, (255, 255, 255)),
        ///     (Key::T, (255, 255, 255)),
        ///     (Key::Y, (255, 255, 255)),
        /// ]);
        /// ```
        pub fn array_auto_update(&mut self, auto_update: bool) {
            unsafe { wooting_rgb_sdk_sys::wooting_rgb_array_auto_update(auto_update) }
        }

        /// Set a single color in the color array. This will not directly update the keyboard
        /// unless the auto update flag is set (see `array_auto_update`), so it can be called
        /// frequently (i.e. in a loop that updates the entire keyboard). Returns `true` if the
        /// colors have changed.
        ///
        /// ```rust,no_run
        /// use wooting_sdk::{rgb::RgbKeyboard, Key};
        ///
        /// let mut keyboard = RgbKeyboard::default();
        /// // Modify the keyboard array so QWERTY will be set to white..
        /// keyboard.array_set_single(Key::Q, 255, 255, 255);
        /// keyboard.array_set_single(Key::W, 255, 255, 255);
        /// keyboard.array_set_single(Key::E, 255, 255, 255);
        /// keyboard.array_set_single(Key::R, 255, 255, 255);
        /// keyboard.array_set_single(Key::T, 255, 255, 255);
        /// keyboard.array_set_single(Key::Y, 255, 255, 255);
        /// // ..and apply the change.
        /// keyboard.array_update();
        /// ```
        pub fn array_set_single<K: IntoMatrixRowColumn>(
            &mut self,
            key: K,
            red: u8,
            green: u8,
            blue: u8,
        ) -> bool {
            let (row, column) = key.into_matrix_row_and_column();
            unsafe {
                wooting_rgb_sdk_sys::wooting_rgb_array_set_single(row, column, red, green, blue)
            }
        }

        /// Set a complete color array. This will not directly update the keyboard unless the auto
        /// update flag is set (see `array_auto_update`). Returns `true` if the colors have
        /// changed.
        ///
        /// ```rust,no_run
        /// use wooting_sdk::{rgb::RgbKeyboard, Key};
        ///
        /// let mut keyboard = RgbKeyboard::default();
        /// // Modify the keyboard array so QWERTY will be set to white..
        /// keyboard.array_set_full(&[
        ///     (Key::Q, (255, 255, 255)),
        ///     (Key::W, (255, 255, 255)),
        ///     (Key::E, (255, 255, 255)),
        ///     (Key::R, (255, 255, 255)),
        ///     (Key::T, (255, 255, 255)),
        ///     (Key::Y, (255, 255, 255)),
        /// ]);
        /// // ..and apply the change.
        /// keyboard.array_update();
        /// ```
        pub fn array_set_full<K: IntoMatrixRowColumn>(
            &mut self,
            array: &[(K, (u8, u8, u8))],
        ) -> bool {
            let mut flattened: [u8; COMPONENTS * COLUMNS * ROWS] = [0; COMPONENTS * COLUMNS * ROWS];
            for (key, (red, green, blue)) in array {
                let (row, column) = key.into_matrix_row_and_column();
                let index: usize =
                    (row as usize) * (COLUMNS * COMPONENTS) + (column as usize) * COMPONENTS;
                flattened[index] = *red;
                flattened[index + 1] = *green;
                flattened[index + 2] = *blue;
            }
            unsafe { wooting_rgb_sdk_sys::wooting_rgb_array_set_full(flattened.as_ptr()) }
        }

        /// Restore all colors to those that were originally on the keyboard. Must be called when
        /// application is closed (this will be invoked when this struct is dropped).
        ///
        /// ```rust,no_run
        /// use wooting_sdk::{rgb::RgbKeyboard, Key};
        ///
        /// let mut keyboard = RgbKeyboard::default();
        /// // Set ABC to white..
        /// keyboard.direct_set_key(Key::A, 255, 255, 255);
        /// keyboard.direct_set_key(Key::B, 255, 255, 255);
        /// keyboard.direct_set_key(Key::C, 255, 255, 255);
        /// // ..and then reset the entire keyboard back to how it was previously.
        /// keyboard.reset_all();
        /// ```
        pub fn reset_all(&mut self) -> bool {
            unsafe { wooting_rgb_sdk_sys::wooting_rgb_reset() }
        }
    }

    impl Drop for RgbKeyboard {
        fn drop(&mut self) {
            // By restricting all rgb functions to get performed on a struct then we can ensure
            // that there is something to be dropped and therefore force a reset.
            let _ = self.reset_all();
            // Also, make sure that the auto update has been reset.
            self.array_auto_update(false);
        }
    }
}
