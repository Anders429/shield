use bitflags::bitflags;
use derive_more::Add;
use std::ops::{BitOr, BitOrAssign};

bitflags! {
    struct Flags: u8 {
        const SYS_EXIT = 0b0000_0001;
        const INPUT = 0b0000_0010;
        const SCREEN_CHUNK_CHANGE = 0b0000_0100;
    }
}

impl Default for Flags {
    fn default() -> Self {
        Self::empty()
    }
}

bitflags! {
    pub(crate) struct Input : u8 {
        const UP = 0b0000_0001;
        const RIGHT = 0b0000_0010;
        const DOWN = 0b0000_0100;
        const LEFT = 0b0000_1000;
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::empty()
    }
}

impl Input {
    pub(crate) fn has_up(&self) -> bool {
        self.contains(Input::UP)
    }

    pub(crate) fn has_right(&self) -> bool {
        self.contains(Input::RIGHT)
    }

    pub(crate) fn has_down(&self) -> bool {
        self.contains(Input::DOWN)
    }

    pub(crate) fn has_left(&self) -> bool {
        self.contains(Input::LEFT)
    }
}

#[derive(Add, Clone, Copy, Default)]
pub(crate) struct ChunkChange {
    pub(crate) x: i8,
    pub(crate) y: i8,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct Events {
    flags: Flags,
    input: Input,
    screen_chunk_change: ChunkChange,
}

impl BitOr for Events {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            flags: self.flags | rhs.flags,
            input: self.input | rhs.input,
            screen_chunk_change: self.screen_chunk_change + rhs.screen_chunk_change
        }
    }
}

impl BitOrAssign for Events {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl Events {
    pub(crate) fn sys_exit() -> Self {
        Self {
            flags: Flags::SYS_EXIT,
            ..Default::default()
        }
    }

    pub(crate) fn input(input: Input) -> Self {
        Self {
            flags: Flags::INPUT,
            input,
            ..Default::default()
        }
    }

    pub(crate) fn screen_chunk_change(screen_chunk_change: ChunkChange) -> Self {
        Self {
            flags: Flags::SCREEN_CHUNK_CHANGE,
            screen_chunk_change,
            ..Default::default()
        }
    }

    pub(crate) fn has_sys_exit(&self) -> bool {
        self.flags.contains(Flags::SYS_EXIT)
    }

    pub(crate) fn has_input(&self) -> bool {
        self.flags.contains(Flags::INPUT)
    }

    pub(crate) fn has_screen_chunk_change(&self) -> bool {
        self.flags.contains(Flags::SCREEN_CHUNK_CHANGE)
    }

    pub(crate) unsafe fn unwrap_input_unchecked(&self) -> Input {
        self.input
    }

    pub(crate) fn unwrap_input(&self) -> Option<Input> {
        if self.has_input() {
            Some(unsafe { self.unwrap_input_unchecked() })
        } else {
            None
        }
    }

    pub(crate) unsafe fn unwrap_screen_chunk_change_unchecked(&self) -> ChunkChange {
        self.screen_chunk_change
    }

    pub(crate) fn unwrap_screen_chunk_change(&self) -> Option<ChunkChange> {
        if self.has_screen_chunk_change() {
            Some(unsafe {self.unwrap_screen_chunk_change_unchecked()})
        } else {
            None
        }
    }
}
