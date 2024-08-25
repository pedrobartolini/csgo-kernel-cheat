pub use winapi::um::winuser::*;

// Mouse buttons
// pub const LMOUSE: i32 = VK_LBUTTON;
// pub const RMOUSE: i32 = VK_RBUTTON;
// pub const MMOUSE: i32 = VK_MBUTTON;

// // Function keys
// pub const F1: i32 = VK_F1;
// pub const F2: i32 = VK_F2;
// pub const F3: i32 = VK_F3;
// pub const F4: i32 = VK_F4;
// pub const F5: i32 = VK_F5;
// pub const F6: i32 = VK_F6;
// pub const F7: i32 = VK_F7;
// pub const F8: i32 = VK_F8;
// pub const F9: i32 = VK_F9;
// pub const F10: i32 = VK_F10;
// pub const F11: i32 = VK_F11;
// pub const F12: i32 = VK_F12;

// // Alphabet keys
// pub const A: i32 = 'A' as i32;
// pub const B: i32 = 'B' as i32;
// pub const C: i32 = 'C' as i32;
// pub const D: i32 = 'D' as i32;
// pub const E: i32 = 'E' as i32;
// pub const F: i32 = 'F' as i32;
// pub const G: i32 = 'G' as i32;
// pub const H: i32 = 'H' as i32;
// pub const I: i32 = 'I' as i32;
// pub const J: i32 = 'J' as i32;
// pub const K: i32 = 'K' as i32;
// pub const L: i32 = 'L' as i32;
// pub const M: i32 = 'M' as i32;
// pub const N: i32 = 'N' as i32;
// pub const O: i32 = 'O' as i32;
// pub const P: i32 = 'P' as i32;
// pub const Q: i32 = 'Q' as i32;
// pub const R: i32 = 'R' as i32;
// pub const S: i32 = 'S' as i32;
// pub const T: i32 = 'T' as i32;
// pub const U: i32 = 'U' as i32;
// pub const V: i32 = 'V' as i32;
// pub const W: i32 = 'W' as i32;
// pub const X: i32 = 'X' as i32;
// pub const Y: i32 = 'Y' as i32;
// pub const Z: i32 = 'Z' as i32;

// // Number keys
// pub const N0: i32 = '0' as i32;
// pub const N1: i32 = '1' as i32;
// pub const N2: i32 = '2' as i32;
// pub const N3: i32 = '3' as i32;
// pub const N4: i32 = '4' as i32;
// pub const N5: i32 = '5' as i32;
// pub const N6: i32 = '6' as i32;
// pub const N7: i32 = '7' as i32;
// pub const N8: i32 = '8' as i32;
// pub const N9: i32 = '9' as i32;

// // Control keys
// pub const ESC: i32 = VK_ESCAPE;
// pub const ENTER: i32 = VK_RETURN;
// pub const TAB: i32 = VK_TAB;
// pub const SHIFT: i32 = VK_SHIFT;
// pub const CTRL: i32 = VK_CONTROL;
// pub const ALT: i32 = VK_MENU;

// // Arrow keys
// pub const UP: i32 = VK_UP;
// pub const DOWN: i32 = VK_DOWN;
// pub const LEFT: i32 = VK_LEFT;
// pub const RIGHT: i32 = VK_RIGHT;

// // Other common keys
// pub const SPACE: i32 = VK_SPACE;
// pub const BACKSPACE: i32 = VK_BACK;
// pub const DELETE: i32 = VK_DELETE;
// pub const INSERT: i32 = VK_INSERT;
// pub const HOME: i32 = VK_HOME;
// pub const END: i32 = VK_END;
// pub const PAGEUP: i32 = VK_PRIOR;
// pub const PAGEDOWN: i32 = VK_NEXT;
// pub const CAPSLOCK: i32 = VK_CAPITAL;
// pub const NUMLOCK: i32 = VK_NUMLOCK;
// pub const SCROLLLOCK: i32 = VK_SCROLL;

pub fn key_state(key_code: i32) -> bool {
   unsafe { GetAsyncKeyState(key_code) != 0 }
}
