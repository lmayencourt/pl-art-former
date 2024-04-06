/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Louis Mayencourt
 */

/// Embedded levels map as string.
/// This allow an easy WASM deployment, as no external assets is needed to
/// store the levels.

pub const LEVEL_TRAINING: &str = "
BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB
G..............................G
G..............................G
G..............................G
G..............................G
G.....RRRR.....................G
G..............................G
G..................RRRRRRR.....G
G........................R.....G
G........................R.....G
G............RRRR........R.....G
G........................R.....G
G........R...............R.....G
G....RRRRR.....................G
G..............................G
G..............................G
DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD
";