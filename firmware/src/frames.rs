// SPDX-License-Identifier: GPL-3.0-or-later

// Frame data adapted from QMK Firmware's Torn Bongo Cat animation:
// https://github.com/qmk/qmk_firmware/blob/168792f220cfe61cd7f30fcdb2992492db87db2d/keyboards/torn/bongocat.c
// Copyright 2020 Richard Titmuss
// Licensed under GPL-2.0-or-later.
//
// Format: column-major page format - 4 pages x 128 columns.
// Each byte encodes 8 vertical pixels in one column of one page.
//
// To draw with embedded-graphics, iterate pages (0..4) and columns (0..128):
//   pixel(col, page*8 + bit) = (frame[page*128 + col] >> bit) & 1

pub const FRAME_SIZE: usize = 512; // 128 * 32 / 8

// 5 idle frames - cat resting, subtle breathing animation.
// The first two source frames are byte-identical.
pub const IDLE: [[u8; FRAME_SIZE]; 5] = [
    include!("frames/idle_0.hex"),
    include!("frames/idle_0.hex"),
    include!("frames/idle_2.hex"),
    include!("frames/idle_3.hex"),
    include!("frames/idle_4.hex"),
];

// 1 prep frame - cat with both paws raised.
pub const PREP: [u8; FRAME_SIZE] = include!("frames/prep_0.hex");

// 1 adapted fury frame - left half of tap_0 plus right half of tap_1.
pub const FURY: [u8; FRAME_SIZE] = include!("frames/fury_0.hex");

// 2 tap frames - cat alternating paw strikes.
pub const TAP: [[u8; FRAME_SIZE]; 2] = [include!("frames/tap_0.hex"), include!("frames/tap_1.hex")];
