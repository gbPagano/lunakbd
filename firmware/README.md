# Lunakb firmware

## Build firmware

You can build firmware for central and peripheral separately:

```shell
# Build central firmware
cargo build --release --bin central

# Build peripheral firmware
cargo build --release --bin peripheral

# Build peripheral2 firmware
cargo build --release --bin peripheral2
```

## Generate UF2 firmware

This project keeps the Adafruit UF2 bootloader supplied with the nice!nano.
Generate firmware for all three controllers with:

```shell
cargo make uf2
```

This produces:

```text
rmk-central.uf2      # dongle
rmk-peripheral.uf2   # right half
rmk-peripheral2.uf2  # left half
```

## Flash firmware

The flash tasks build the correct release firmware, mount the `NICENANO`
volume when needed and copy the UF2 to it:

```shell
# Left half: peripheral2
cargo make flash-left --release

# Dongle: central
cargo make flash-dongle --release

# Right half: peripheral
cargo make flash-right --release
```

Before running a task:

1. Connect the nice!nano over USB.
2. Enter the bootloader by resetting the board. On boards where double-tap is
   unreliable, hold RESET at GND briefly and release it.
3. Confirm that the `NICENANO` USB drive appears.
4. Run the corresponding `cargo make flash-* --release` command.
5. Enter the sudo password if requested.
6. Wait for the drive to disconnect and the controller to reboot.

The Adafruit bootloader remains installed, so the same procedure can be used
for recovery and all subsequent updates. `dfu-util` is not used in this mode.
