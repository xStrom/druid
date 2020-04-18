// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Common types for representing mouse events and state

use crate::kurbo::Point;

use crate::keyboard::KeyModifiers;

/// Information about the mouse move event.
#[derive(Debug, Clone, PartialEq)]
pub struct MoveEvent {
    /// The location of the mouse in the current window.
    ///
    /// This is in px units not device pixels, that is, adjusted for hi-dpi.
    pub pos: Point,
    /// Mouse buttons being held down at the time of the event.
    pub buttons: MouseButtons,
    /// Keyboard modifiers at the time of the event.
    pub mods: KeyModifiers,
}

/// Information about the mouse click event.
#[derive(Debug, Clone, PartialEq)]
pub struct ClickEvent {
    /// The location of the mouse in the current window.
    ///
    /// This is in px units not device pixels, that is, adjusted for hi-dpi.
    pub pos: Point,
    /// Mouse buttons being held down after the event.
    /// Thus it will contain the `button` that triggered a mouse-down event,
    /// and it will not contain the `button` that triggered a mouse-up event.
    pub buttons: MouseButtons,
    /// Keyboard modifiers at the time of the event.
    pub mods: KeyModifiers,
    /// The number of mouse clicks associated with this event. This will always
    /// be `0` for a mouse-up event.
    pub count: u8,
    /// The button that was pressed down in the case of mouse-down,
    /// or the button that was released in the case of mouse-up.
    pub button: MouseButton,
}

/// An indicator of which mouse button was pressed.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum MouseButton {
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
    /// First X button.
    X1,
    /// Second X button.
    X2,
    /// Some other unknown button.
    Other,
}

impl MouseButton {
    /// Returns `true` if this is [`MouseButton::Left`].
    ///
    /// [`MouseButton::Left`]: #variant.Left
    #[inline]
    pub fn is_left(self) -> bool {
        self == MouseButton::Left
    }

    /// Returns `true` if this is [`MouseButton::Right`].
    ///
    /// [`MouseButton::Right`]: #variant.Right
    #[inline]
    pub fn is_right(self) -> bool {
        self == MouseButton::Right
    }

    /// Returns `true` if this is [`MouseButton::Middle`].
    ///
    /// [`MouseButton::Middle`]: #variant.Middle
    #[inline]
    pub fn is_middle(self) -> bool {
        self == MouseButton::Middle
    }

    /// Returns `true` if this is [`MouseButton::X1`].
    ///
    /// [`MouseButton::X1`]: #variant.X1
    #[inline]
    pub fn is_x1(self) -> bool {
        self == MouseButton::X1
    }

    /// Returns `true` if this is [`MouseButton::X2`].
    ///
    /// [`MouseButton::X2`]: #variant.X2
    #[inline]
    pub fn is_x2(self) -> bool {
        self == MouseButton::X2
    }

    /// Returns `true` if this is [`MouseButton::Other`].
    ///
    /// [`MouseButton::Other`]: #variant.Other
    #[inline]
    pub fn is_other(self) -> bool {
        self == MouseButton::Other
    }
}

/// A set of [`MouseButton`]s.
///
/// [`MouseButton`]: enum.MouseButton.html
#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub struct MouseButtons(u8);

impl MouseButtons {
    /// Create a new empty set.
    #[inline]
    pub fn new() -> MouseButtons {
        MouseButtons(0)
    }

    /// Add the `button` to the set.
    #[inline]
    pub fn add(&mut self, button: MouseButton) {
        self.0 |= 1 << button as u8;
    }

    /// Remove the `button` from the set.
    #[inline]
    pub fn remove(&mut self, button: MouseButton) {
        self.0 &= !(1 << button as u8);
    }

    /// Builder-style method for adding the `button` to the set.
    #[inline]
    pub fn with(mut self, button: MouseButton) -> MouseButtons {
        // TODO: Does this compile down well enough or should we do the bit work here?
        self.add(button);
        self
    }

    /// Builder-style method for removing the `button` from the set.
    #[inline]
    pub fn without(mut self, button: MouseButton) -> MouseButtons {
        self.remove(button);
        self
    }

    /// Returns `true` if the `button` is in the set.
    #[inline]
    pub fn has(self, button: MouseButton) -> bool {
        (self.0 & (1 << button as u8)) != 0
    }

    /// Returns `true` if any button is in the set.
    #[inline]
    pub fn has_any(self) -> bool {
        self.0 != 0
    }

    /// Returns `true` if the set is empty.
    #[inline]
    pub fn has_none(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if [`MouseButton::Left`] is in the set.
    ///
    /// [`MouseButton::Left`]: enum.MouseButton.html#variant.Left
    #[inline]
    pub fn has_left(self) -> bool {
        self.has(MouseButton::Left)
    }

    /// Returns `true` if [`MouseButton::Right`] is in the set.
    ///
    /// [`MouseButton::Right`]: enum.MouseButton.html#variant.Right
    #[inline]
    pub fn has_right(self) -> bool {
        self.has(MouseButton::Right)
    }

    /// Returns `true` if [`MouseButton::Middle`] is in the set.
    ///
    /// [`MouseButton::Middle`]: enum.MouseButton.html#variant.Middle
    #[inline]
    pub fn has_middle(self) -> bool {
        self.has(MouseButton::Middle)
    }

    /// Returns `true` if [`MouseButton::X1`] is in the set.
    ///
    /// [`MouseButton::X1`]: enum.MouseButton.html#variant.X1
    #[inline]
    pub fn has_x1(self) -> bool {
        self.has(MouseButton::X1)
    }

    /// Returns `true` if [`MouseButton::X2`] is in the set.
    ///
    /// [`MouseButton::X2`]: enum.MouseButton.html#variant.X2
    #[inline]
    pub fn has_x2(self) -> bool {
        self.has(MouseButton::X2)
    }

    /// Returns `true` if [`MouseButton::Other`] is in the set.
    ///
    /// [`MouseButton::Other`]: enum.MouseButton.html#variant.Other
    #[inline]
    pub fn has_other(self) -> bool {
        self.has(MouseButton::Other)
    }

    /// Adds all the [`MouseButton`]s in `other` to the set.
    ///
    /// [`MouseButton`]: enum.MouseButton.html
    pub fn union(&mut self, other: MouseButtons) {
        self.0 |= other.0;
    }

    /// Builder-style method for adding all the [`MouseButton`]s in `other`.
    ///
    /// [`MouseButton`]: enum.MouseButton.html
    #[inline]
    pub fn with_other(mut self, other: MouseButtons) -> MouseButtons {
        self.union(other);
        self
    }

    /// Clear the set.
    #[inline]
    pub fn clear(&mut self) {
        self.0 = 0;
    }
}

impl std::fmt::Debug for MouseButtons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MouseButtons({:06b})", self.0)
    }
}

//NOTE: this currently only contains cursors that are included by default on
//both Windows and macOS. We may want to provide polyfills for various additional cursors,
//and we will also want to add some mechanism for adding custom cursors.
/// Mouse cursors.
#[derive(Clone)]
pub enum Cursor {
    /// The default arrow cursor.
    Arrow,
    /// A vertical I-beam, for indicating insertion points in text.
    IBeam,
    Crosshair,
    OpenHand,
    NotAllowed,
    ResizeLeftRight,
    ResizeUpDown,
}
