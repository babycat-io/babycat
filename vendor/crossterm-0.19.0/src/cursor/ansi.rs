//! This module provides cursor related ANSI escape codes.

use crate::csi;
use std::fmt;

pub(crate) fn move_to_csi_sequence(f: &mut impl fmt::Write, x: u16, y: u16) -> fmt::Result {
    write!(f, csi!("{};{}H"), y + 1, x + 1)
}

pub(crate) fn move_up_csi_sequence(f: &mut impl fmt::Write, count: u16) -> fmt::Result {
    if count != 0 {
        write!(f, csi!("{}A"), count)
    } else {
        Ok(())
    }
}

pub(crate) fn move_right_csi_sequence(f: &mut impl fmt::Write, count: u16) -> fmt::Result {
    if count != 0 {
        write!(f, csi!("{}C"), count)
    } else {
        Ok(())
    }
}

pub(crate) fn move_down_csi_sequence(f: &mut impl fmt::Write, count: u16) -> fmt::Result {
    if count != 0 {
        write!(f, csi!("{}B"), count)
    } else {
        Ok(())
    }
}

pub(crate) fn move_left_csi_sequence(f: &mut impl fmt::Write, count: u16) -> fmt::Result {
    if count != 0 {
        write!(f, csi!("{}D"), count)
    } else {
        Ok(())
    }
}

pub(crate) fn move_to_column_csi_sequence(f: &mut impl fmt::Write, count: u16) -> fmt::Result {
    write!(f, csi!("{}G"), count)
}

pub(crate) fn move_to_previous_line_csi_sequence(
    f: &mut impl fmt::Write,
    count: u16,
) -> fmt::Result {
    write!(f, csi!("{}F"), count)
}

pub(crate) fn move_to_next_line_csi_sequence(f: &mut impl fmt::Write, count: u16) -> fmt::Result {
    write!(f, csi!("{}E"), count)
}

pub(crate) const SAVE_POSITION_CSI_SEQUENCE: &str = "\x1B7";
pub(crate) const RESTORE_POSITION_CSI_SEQUENCE: &str = "\x1B8";
pub(crate) const HIDE_CSI_SEQUENCE: &str = csi!("?25l");
pub(crate) const SHOW_CSI_SEQUENCE: &str = csi!("?25h");
pub(crate) const ENABLE_BLINKING_CSI_SEQUENCE: &str = csi!("?12h");
pub(crate) const DISABLE_BLINKING_CSI_SEQUENCE: &str = csi!("?12l");
