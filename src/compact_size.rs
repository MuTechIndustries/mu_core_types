use std::mem;

pub fn value_for_little_endian(data: &Vec<u8>) -> u64 {
	match data[0] {
		0...252 => data[0] as u64,
		253 => {
			let mut result_bytes: [u8;8] = [0;8];
			for i in 1..3 {
				result_bytes[i-1] = data[i];
			}
			let result: u64 = unsafe {mem::transmute(result_bytes)};
			result
		},
		254 => {
			let mut result_bytes: [u8;8] = [0;8];
			for i in 1..5 {
				result_bytes[i-1] = data[i];
			}
			let result: u64 = unsafe {mem::transmute(result_bytes)};
			result			
		},
		255 => {
			let mut result_bytes: [u8;8] = [0;8];
			for i in 1..9 {
				result_bytes[i-1] = data[i];
			}
			let result: u64 = unsafe {mem::transmute(result_bytes)};
			result			
		},
		_ => panic!("First byte should be between 0-255."),
	}
}

#[test]
fn test_create_by_little_endian_data() {
	// One byte value 17
	let data: Vec<u8> = vec![17];
	let one_byte: u64 = value_for_little_endian(&data);
	assert_eq!(one_byte, 17);

	// Two byte value 1000
	let data: Vec<u8> = vec![0b11111101, 0b11101000, 0b00000011];
	let two_byte: u64 = value_for_little_endian(&data);
	assert_eq!(two_byte, 1000);

	// Four byte value 70000
	let data: Vec<u8> = vec![0b11111110, 0b01110000, 0b00010001, 0b00000001, 0b00000000];
	let two_byte: u64 = value_for_little_endian(&data);
	assert_eq!(two_byte, 70000);

	// Eight byte value 18446634746297128183
	let data: Vec<u8> = vec![0b11111111, 0b11110111, 0b00010100, 0b01101011, 0b00111010, 0b10010001, 0b10011100, 0b11111111, 0b11111111];
	let four_byte: u64 = value_for_little_endian(&data);
	assert_eq!(four_byte, 18446634746297128183);
}

pub fn big_endian_value(data: &Vec<u8>) -> u64 {
	match data[0] {
		0...252 => data[0] as u64,
		253 => {
			let mut result_bytes: [u8;8] = [0;8];
			for i in 1..3 {
				let index = 3 - i;
				result_bytes[i-1] = data[index];
			}
			let result: u64 = unsafe {mem::transmute(result_bytes)};
			result
		},
		254 => {
			let mut result_bytes: [u8;8] = [0;8];
			for i in 1..5 {
				let index = 5 - i;
				result_bytes[i-1] = data[index];
			}
			let result: u64 = unsafe {mem::transmute(result_bytes)};
			result			
		},
		255 => {
			let mut result_bytes: [u8;8] = [0;8];
			for i in 1..9 {
				let index = 9 - i;
				result_bytes[i-1] = data[index];
			}
			let result: u64 = unsafe {mem::transmute(result_bytes)};
			result			
		},
		_ => panic!("First byte should be between 0-255."),
	}
}

#[test]
fn test_create_by_big_endian_data() {
	// One byte value 17
	let data: Vec<u8> = vec![17];
	let one_byte: u64 = big_endian_value(&data);
	assert_eq!(one_byte, 17);

	// Two byte value 1000
	let data: Vec<u8> = vec![0b11111101, 0b00000011, 0b11101000];
	let two_byte: u64 = big_endian_value(&data);
	assert_eq!(two_byte, 1000);

	// // Four byte value 70000
	let data: Vec<u8> = vec![0b11111110, 0b00000000, 0b00000001, 0b00010001, 0b01110000];
	let two_byte: u64 = big_endian_value(&data);
	assert_eq!(two_byte, 70000);

	// // Eight byte value 18446634746297128183
	let data: Vec<u8> = vec![0b11111111, 0b11111111, 0b11111111, 0b10011100, 0b10010001, 0b00111010, 0b01101011, 0b00010100, 0b11110111];
	let four_byte: u64 = big_endian_value(&data);
	assert_eq!(four_byte, 18446634746297128183);
}