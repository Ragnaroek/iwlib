use super::util;

pub fn rlew_expand(source: &Vec<u8>, len: usize, rlew_tag: u16) -> Vec<u8> {
	let mut expanded = Vec::with_capacity(len);

	let trail = source.len() % 2 != 0;
	let loop_len = if trail { len - 1 } else { len };

	let mut reader = util::new_data_reader(source);
	loop {
		let value = reader.read_u16();
		if value != rlew_tag {
			let bytes = value.to_le_bytes();
			expanded.push(bytes[0]);
			expanded.push(bytes[1]);
		} else {
			let count = reader.read_u16();
			let value = reader.read_u16();
			let bytes = value.to_le_bytes();
			for _ in 0..count {
				expanded.push(bytes[0]);
				expanded.push(bytes[1]);
			}
		}

		if expanded.len() >= loop_len {
			break;
		}
	}
	
	if trail {
		expanded.push(source[source.len()-1]);
	}

	expanded
}

const NEARTAG : u8 = 0xa7;
const FARTAG : u8 = 0xa8;

pub fn carmack_expand(data: &[u8], len: usize) -> Vec<u8> {
	let mut expanded = Vec::with_capacity(len);

	let mut length = len / 2;
	let mut in_ptr = 0;

	while length != 0 {
		let word_count = data[in_ptr]; 
		let ch_high = data[in_ptr+1];
		in_ptr += 2;

		if ch_high == NEARTAG {
			let offset = data[in_ptr];
			in_ptr += 1;

			if word_count == 0 {
				expanded.push(offset);
				expanded.push(NEARTAG);
				length -= 1;
			} else {
				let mut copy_ptr = expanded.len() - (offset as usize * 2);
				length -= word_count as usize;
				for _ in 0..(word_count as usize * 2) {
					expanded.push(expanded[copy_ptr]);
					copy_ptr += 1;
				}
			}
		} else if ch_high == FARTAG {
			let offset_low = data[in_ptr];
			in_ptr += 1;

			if word_count == 0 {
				expanded.push(offset_low);
				expanded.push(FARTAG);
				length -= 1;
			} else {
				let offset_high = data[in_ptr];
				in_ptr += 1;

				let mut offset = offset_high as usize;
				offset <<= 8;
				offset |= offset_low as usize;

				let mut copy_ptr = (offset-1) * 2;
				length -= word_count as usize;
				for _ in 0..(word_count as usize * 2) {
					expanded.push(expanded[copy_ptr]);
					copy_ptr += 1;
				}
			}
		} else {
			// add word as is (destructured here as count and ch_high)
			expanded.push(word_count);
			expanded.push(ch_high);
			length -= 1;
		}
	}

	// handle trailing byte at the end if len is odd
	if expanded.len() != len {
		expanded.push(data[in_ptr]);
	}

	expanded
}