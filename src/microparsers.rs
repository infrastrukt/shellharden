/*
 * Copyright 2016 - 2018 Andreas Nordal
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

pub fn predlen(pred: &Fn(u8) -> bool, horizon: &[u8]) -> usize {
	let mut i: usize = 0;
	while i < horizon.len() && pred(horizon[i]) {
		i += 1;
	}
	i
}

pub fn is_identifierhead(c: u8) -> bool {
	(c >= b'a' && c <= b'z')
	|| (c >= b'A' && c <= b'Z')
	|| (c == b'_')
}

pub fn is_identifiertail(c: u8) -> bool {
	(c >= b'a' && c <= b'z')
	|| (c >= b'A' && c <= b'Z')
	|| (c >= b'0' && c <= b'9')
	|| (c == b'_')
}

pub fn identifierlen(horizon: &[u8]) -> usize {
	return if horizon.len() > 0 && is_identifierhead(horizon[0]) {
		1 + predlen(&is_identifiertail, &horizon[1 ..])
	} else {
		0
	}
}

pub fn is_whitespace(c: u8) -> bool {
	c <= b' '
}

pub fn is_word(byte: u8) -> bool {
	match byte {
		0 ... b' ' => false,
		b'&' => false,
		b'(' => false,
		b')' => false,
		b';' => false,
		b'<' => false,
		b'>' => false,
		b'`' => false,
		b'|' => false,
		_ => true
	}
}
