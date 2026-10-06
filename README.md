# Lunakbd

[![License: GPL-3.0-or-later](https://img.shields.io/badge/license-GPL--3.0--or--later-blue)](LICENSE)

A split wireless keyboard inspired by the [Corne](https://github.com/foostan/crkbd) and the [Sofle](https://github.com/josefadamcik/SofleKeyboard): schematic and PCB in KiCad, case in FreeCAD, and firmware built on [RMK](https://github.com/HaoboGu/rmk), running on nRF52840 promicro controllers.

The two halves connect over BLE to a central USB dongle currently under development.

![two-halves-angled.png](photos/two-halves-angled.png)
![up-down.png](photos/up-down.png)
![up-down-4.png](photos/up-down-4.png)
![up-down-3.png](photos/up-down-3.png)
![up-down-2.png](photos/up-down-2.png)
![single-half-front.png](photos/single-half-front.png)
![single-half-rear.png](photos/single-half-rear.png)
![single-half-side.png](photos/single-half-side.png)
![two-halves-upright.png](photos/two-halves-upright.png)

## Repository layout

| Path                                       | Contents                                                           |
| ------------------------------------------ | ------------------------------------------------------------------ |
| [`pcb/`](pcb)                              | KiCad project, schematic and PCB                                   |
| [`libraries/`](libraries)                  | Custom KiCad symbol/footprint library (`lunalib`)                  |
| [`layout/`](layout)                        | Keyboard layout definitions (KLE JSON)                             |
| [`case/`](case)                            | FreeCAD sources, STEP exports and printable 3MFs                   |
| [`models/`](models)                        | 3D models used in the assembly                                     |
| [`firmware/`](firmware)                    | Rust/RMK firmware - see [`firmware/README.md`](firmware/README.md) |
| [`photos/`](photos), [`renders/`](renders) | Build photos and renders                                           |

## License

Copyright (C) 2025-2026 gbPagano

This project is licensed under the GNU General Public License as published
by the Free Software Foundation, either version 3 of the License, or (at
your option) any later version - see the [`LICENSE`](LICENSE) file for the
full license text.

It is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE.
