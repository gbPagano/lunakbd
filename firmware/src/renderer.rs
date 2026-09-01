// SPDX-License-Identifier: GPL-3.0-or-later

use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::{FONT_6X10, FONT_7X13};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;
use rmk::display::{DisplayRenderer, RenderContext};
use rmk::event::BatteryStatusEvent;
use rmk::types::battery::{BatteryStatus, ChargeState};

use crate::frames;

/// Draw a frame stored in SSD1306 page format onto an embedded-graphics display.
///
/// `cols` is the number of columns in the source frame. The data must contain
/// `cols * pages` bytes, packed as `data[page * cols + col]`.
///
/// Each byte encodes 8 vertical pixels in one column of one page,
/// with bit 0 at the top of the 8-pixel strip.
fn draw_page_format_frame<D: DrawTarget<Color = BinaryColor>>(
    display: &mut D,
    data: &[u8],
    cols: usize,
    offset_x: i32,
    offset_y: i32,
) {
    let pages = data.len() / cols;

    for page in 0..pages {
        for col in 0..cols {
            let byte = data[page * cols + col];
            if byte == 0 {
                continue;
            }
            for bit in 0..8u32 {
                if byte & (1 << bit) != 0 {
                    Pixel(
                        Point::new(
                            col as i32 + offset_x,
                            page as i32 * 8 + bit as i32 + offset_y,
                        ),
                        BinaryColor::On,
                    )
                    .draw(display)
                    .ok();
                }
            }
        }
    }
}

fn fmt_battery_pct(buf: &mut heapless::String<5>, battery: BatteryStatusEvent) {
    use core::fmt::Write as _;

    match *battery {
        BatteryStatus::Unavailable => write!(buf, "?").ok(),
        BatteryStatus::Available {
            charge_state: ChargeState::Charging,
            ..
        } => write!(buf, "CHG").ok(),
        BatteryStatus::Available { level: Some(v), .. } => write!(buf, "{v}%").ok(),
        BatteryStatus::Available { level: None, .. } => write!(buf, "FUL").ok(),
    };
}

const CHECK: [u8; 8] = [0x01, 0x02, 0x04, 0x08, 0x90, 0x60, 0x60, 0x00];
const CROSS: [u8; 8] = [0x82, 0x44, 0x28, 0x10, 0x28, 0x44, 0x82, 0x00];
const ICON_SIZE: u32 = 8;
const BT_ICON: [u8; 28] = [
    0x3E, 0x00, 0x67, 0x00, 0xE3, 0x80, 0xE9, 0x80, 0x8C, 0x80, 0xC9, 0x80, 0xE3, 0x80, 0xE3, 0x80,
    0xC9, 0x80, 0x8C, 0x80, 0xE9, 0x80, 0xE3, 0x80, 0x67, 0x00, 0x3E, 0x00,
];
const BT_ICON_W: u32 = 9;

const SLEEP_ICON_W: usize = 48;
const SLEEP_ICON: [u8; 288] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 128, 128, 128, 192, 192,
    192, 192, 192, 192, 192, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 31, 31, 31, 31, 31, 31, 143, 239, 255, 255, 255, 255, 255, 127,
    31, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 128, 192, 192, 192, 224,
    224, 240, 240, 240, 248, 248, 248, 248, 240, 0, 128, 224, 248, 254, 255, 255, 255, 255, 223,
    199, 225, 224, 224, 224, 224, 224, 192, 30, 30, 30, 158, 254, 254, 254, 255, 127, 63, 14, 0, 0,
    0, 7, 15, 15, 15, 15, 15, 7, 135, 243, 255, 255, 255, 255, 255, 15, 1, 0, 7, 15, 15, 15, 15,
    15, 15, 15, 7, 7, 7, 7, 7, 7, 3, 3, 3, 16, 60, 127, 127, 63, 63, 63, 61, 60, 60, 60, 24, 0, 0,
    0, 0, 0, 0, 0, 192, 248, 255, 255, 255, 255, 255, 255, 248, 252, 252, 126, 126, 126, 62, 62,
    28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 3, 3, 3, 3, 3, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const STROKE: PrimitiveStyle<BinaryColor> = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
const FILL: PrimitiveStyle<BinaryColor> = PrimitiveStyle::with_fill(BinaryColor::On);

fn draw_battery_icon<D: DrawTarget<Color = BinaryColor>>(
    battery: BatteryStatusEvent,
    display: &mut D,
    x: i32,
    y: i32,
) {
    const NUM_BARS: i32 = 9;
    const BODY_W: i32 = 7;
    const BODY_H: i32 = NUM_BARS + 2;
    const NUB_W: i32 = 3;

    let body_y = y + 1;
    let nub_x = x + (BODY_W - NUB_W) / 2;

    Rectangle::new(Point::new(nub_x, y), Size::new(NUB_W as u32, 1))
        .into_styled(STROKE)
        .draw(display)
        .ok();
    Rectangle::new(
        Point::new(x, body_y),
        Size::new(BODY_W as u32, BODY_H as u32),
    )
    .into_styled(STROKE)
    .draw(display)
    .ok();

    let bars = match *battery {
        BatteryStatus::Available {
            level: Some(pct), ..
        } => ((pct as i32 * NUM_BARS) + 99) / 100,
        BatteryStatus::Available { level: None, .. } => NUM_BARS,
        BatteryStatus::Unavailable => 0,
    };

    for i in 0..bars {
        let bar_y = body_y + BODY_H - 2 - i;
        Rectangle::new(Point::new(x + 1, bar_y), Size::new((BODY_W - 2) as u32, 1))
            .into_styled(FILL)
            .draw(display)
            .ok();
    }
}

const DEFAULT_FURY_WPM: u16 = 80;
const DEFAULT_IDLE_TICKS_PER_FRAME: u8 = 3;
const DEFAULT_TAP_HOLD_TICKS: u8 = 3;
const DEFAULT_TAP_IDLE_TICKS: u8 = 5;
const BONGO_Y_OFFSET: i32 = 33;

/// Animation state.
#[derive(Clone, Copy, PartialEq, Eq)]
enum BongoState {
    /// Cat typing - alternates paws on each key press.
    Normal,
    /// Cat going wild - rapidly alternates both paws.
    Fury,
}

/// Split dongle status and Bongo Cat OLED renderer.
///
/// Draws peripheral connection and battery status above the Bongo Cat animation
/// on a 128x64 OLED.
/// - **Normal**: each key press alternates left/right paw.
/// - **Fury** (high WPM): both paws smashing, one toggle per render.
///
/// Animation speed is controlled by `render_interval` in `keyboard.toml`.
pub struct DongleRenderer {
    // config
    /// WPM threshold above which the cat enters fury mode.
    fury_wpm: u16,
    /// Number of renders before advancing to the next idle frame.
    idle_ticks_per_frame: u8,
    /// Number of renders the paw stays down after a tap.
    tap_hold_ticks: u8,
    /// Number of renders in PREP pose before falling back to idle animation.
    tap_idle_ticks: u8,
    // state
    bongo_state: BongoState,
    tap_paw: bool,
    idle_frame: u8,
    idle_tick: u8,
    tap_hold: u8,
    tap_inactivity: u8,
}

impl Default for DongleRenderer {
    fn default() -> Self {
        Self {
            fury_wpm: DEFAULT_FURY_WPM,
            idle_ticks_per_frame: DEFAULT_IDLE_TICKS_PER_FRAME,
            tap_hold_ticks: DEFAULT_TAP_HOLD_TICKS,
            tap_idle_ticks: DEFAULT_TAP_IDLE_TICKS,
            bongo_state: BongoState::Normal,
            tap_paw: false,
            idle_frame: 0,
            idle_tick: 0,
            tap_hold: 0,
            tap_inactivity: 0,
        }
    }
}

impl DongleRenderer {
    pub fn with_fury_wpm(mut self, wpm: u16) -> Self {
        self.fury_wpm = wpm;
        self
    }

    /// Set how many renders must pass before the idle animation advances one frame.
    pub fn with_idle_ticks_per_frame(mut self, ticks: u8) -> Self {
        self.idle_ticks_per_frame = ticks;
        self
    }

    /// Set how many renders the paw stays in the TAP pose after a key press.
    pub fn with_tap_hold_ticks(mut self, ticks: u8) -> Self {
        self.tap_hold_ticks = ticks;
        self
    }

    /// Set how many renders the cat stays in PREP pose before returning to idle animation.
    pub fn with_tap_idle_ticks(mut self, ticks: u8) -> Self {
        self.tap_idle_ticks = ticks;
        self
    }

    fn next_idle_frame(&mut self) -> &'static [u8; frames::FRAME_SIZE] {
        self.idle_tick += 1;
        if self.idle_tick >= self.idle_ticks_per_frame {
            self.idle_tick = 0;
            self.idle_frame = (self.idle_frame + 1) % frames::IDLE.len() as u8;
        }
        &frames::IDLE[self.idle_frame as usize]
    }
}

impl DisplayRenderer<BinaryColor> for DongleRenderer {
    fn render<D: DrawTarget<Color = BinaryColor>>(&mut self, ctx: &RenderContext, display: &mut D) {
        display.clear(BinaryColor::Off).ok();

        if ctx.sleeping {
            draw_page_format_frame(display, &SLEEP_ICON, SLEEP_ICON_W, 40, 8);
            return;
        }

        let connected = ctx.peripherals_connected;
        let batteries = ctx.peripheral_batteries;
        let text_style = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);

        fn draw_bt_icon<D: DrawTarget<Color = BinaryColor>>(display: &mut D, x: i32, y: i32) {
            let raw: ImageRaw<BinaryColor> = ImageRaw::new(&BT_ICON, BT_ICON_W);
            Image::new(&raw, Point::new(x, y)).draw(display).ok();
        }

        if connected[0] {
            if matches!(*batteries[0], BatteryStatus::Unavailable) {
                draw_bt_icon(display, 0, 0);
                let raw: ImageRaw<BinaryColor> = ImageRaw::new(&CHECK, ICON_SIZE);
                Image::new(&raw, Point::new(BT_ICON_W as i32 + 3, 3))
                    .draw(display)
                    .ok();
            } else {
                draw_battery_icon(batteries[0], display, 0, 0);
                let mut buf: heapless::String<5> = heapless::String::new();
                fmt_battery_pct(&mut buf, batteries[0]);
                Text::new(&buf, Point::new(10, 11), text_style)
                    .draw(display)
                    .ok();
            }
        } else {
            draw_bt_icon(display, 0, 0);
            let raw: ImageRaw<BinaryColor> = ImageRaw::new(&CROSS, ICON_SIZE);
            Image::new(&raw, Point::new(BT_ICON_W as i32 + 3, 4))
                .draw(display)
                .ok();
        }

        let right_x = 128 - BT_ICON_W as i32;
        if connected[1] {
            if matches!(*batteries[1], BatteryStatus::Unavailable) {
                draw_bt_icon(display, right_x, 0);
                let raw: ImageRaw<BinaryColor> = ImageRaw::new(&CHECK, ICON_SIZE);
                Image::new(&raw, Point::new(right_x - 11, 3))
                    .draw(display)
                    .ok();
            } else {
                draw_battery_icon(batteries[1], display, right_x, 0);
                let mut buf: heapless::String<5> = heapless::String::new();
                fmt_battery_pct(&mut buf, batteries[1]);
                let text_w = buf.len() as i32 * 7;
                Text::new(&buf, Point::new(right_x - text_w - 6, 11), text_style)
                    .draw(display)
                    .ok();
            }
        } else {
            draw_bt_icon(display, right_x, 0);
            let raw: ImageRaw<BinaryColor> = ImageRaw::new(&CROSS, ICON_SIZE);
            Image::new(&raw, Point::new(right_x - 11, 4))
                .draw(display)
                .ok();
        }

        // WPM drives state transitions.
        let new_state = if ctx.wpm >= self.fury_wpm {
            BongoState::Fury
        } else {
            BongoState::Normal
        };
        if new_state != self.bongo_state {
            self.bongo_state = new_state;
            self.idle_frame = 0;
            self.idle_tick = 0;
        }
        // Alternate paw on each key press (only in Normal; Fury manages tap_paw itself).
        if ctx.key_press_latch && self.bongo_state != BongoState::Fury {
            self.tap_paw = !self.tap_paw;
            self.tap_hold = self.tap_hold_ticks;
            self.tap_inactivity = 0;
        }

        let data: &[u8; frames::FRAME_SIZE] = match self.bongo_state {
            BongoState::Normal => {
                if self.tap_hold > 0 {
                    self.tap_hold -= 1;
                    self.tap_inactivity = 0;
                    &frames::TAP[self.tap_paw as usize]
                } else if self.tap_inactivity < self.tap_idle_ticks {
                    self.tap_inactivity += 1;
                    &frames::PREP
                } else {
                    self.next_idle_frame()
                }
            }
            BongoState::Fury => {
                self.tap_paw = !self.tap_paw;
                if self.tap_paw {
                    &frames::FURY
                } else {
                    &frames::PREP
                }
            }
        };

        draw_page_format_frame(display, data, 128, 9, BONGO_Y_OFFSET);

        // WPM overlay - bottom-right corner.
        let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let mut buf: heapless::String<8> = heapless::String::new();
        core::fmt::write(&mut buf, format_args!("{}", ctx.wpm)).ok();
        let x = 128 - buf.len() as i32 * 6;
        Text::new(&buf, Point::new(x, 63), style).draw(display).ok();
    }
}
