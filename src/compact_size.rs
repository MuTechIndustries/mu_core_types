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
fn test_value_for_little_endian() {
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

pub fn value_for_big_endian(data: &Vec<u8>) -> u64 {
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
fn test_value_for_big_endian() {
	// One byte value 17
	let data: Vec<u8> = vec![17];
	let one_byte: u64 = value_for_big_endian(&data);
	assert_eq!(one_byte, 17);

	// Two byte value 1000
	let data: Vec<u8> = vec![0b11111101, 0b00000011, 0b11101000];
	let two_byte: u64 = value_for_big_endian(&data);
	assert_eq!(two_byte, 1000);

	// // Four byte value 70000
	let data: Vec<u8> = vec![0b11111110, 0b00000000, 0b00000001, 0b00010001, 0b01110000];
	let two_byte: u64 = value_for_big_endian(&data);
	assert_eq!(two_byte, 70000);

	// // Eight byte value 18446634746297128183
	let data: Vec<u8> = vec![0b11111111, 0b11111111, 0b11111111, 0b10011100, 0b10010001, 0b00111010, 0b01101011, 0b00010100, 0b11110111];
	let four_byte: u64 = value_for_big_endian(&data);
	assert_eq!(four_byte, 18446634746297128183);
}

pub fn little_endian_bytes_for_value( value: u64 ) -> Box<Vec<u8>> {
	let resulting_bytes: [u8;8] = unsafe { mem::transmute(value) };
	let mut result: Box<Vec<u8>> = Box::new(Vec::with_capacity(9));
	match value {
		0...252 => {
			(*result).push(resulting_bytes[0]);
		},
		252...4095 => {
			(*result).push(0b11111101);
			(*result).extend_from_slice(&resulting_bytes[0..2]);
		},
		4096...4294967295 => {
			(*result).push(0b11111110);
			(*result).extend_from_slice(&resulting_bytes[0..4]);
		},
		_ => {
			(*result).push(0b11111111);
			(*result).extend_from_slice(&resulting_bytes[0..8]);
		},
	}
	result
}

#[test]
fn test_little_endian_bytes_for_value() {
	let expected_bytes: Vec<u8> = vec![0];
	assert_eq!(*little_endian_bytes_for_value(0), expected_bytes);

	let expected_bytes: Vec<u8> = vec![112];
	assert_eq!(*little_endian_bytes_for_value(112), expected_bytes);

	let expected_bytes: Vec<u8> = vec![0b11111101, 0b11101000, 0b00000011];
	assert_eq!(*little_endian_bytes_for_value(1000), expected_bytes);

	let expected_bytes: Vec<u8> = vec![0b11111110, 0b01110000, 0b00010001, 0b00000001, 0b00000000];
	assert_eq!(*little_endian_bytes_for_value(70000), expected_bytes);

	let expected_bytes: Vec<u8> = vec![0b11111111, 0b11110111, 0b00010100, 0b01101011, 0b00111010, 0b10010001, 0b10011100, 0b11111111, 0b11111111];
	assert_eq!(*little_endian_bytes_for_value(18446634746297128183), expected_bytes);
}

pub fn big_endian_bytes_for_value(value: u64) -> Box<Vec<u8>> {
	let mut little_endian_result: Vec<u8> = *little_endian_bytes_for_value(value);
	let mut result: Box<Vec<u8>> = Box::new(Vec::with_capacity(9));
	// first: u8, rest: Vec<u8>
	let (first, rest) = (*little_endian_result).split_first_mut().unwrap();
	(*result).push(*first);
	rest.reverse();
	(*result).extend_from_slice(rest);
	result
}

#[test]
fn test_big_endian_bytes_for_value() {
	let expected_bytes: Vec<u8> = vec![0];
	assert_eq!(*big_endian_bytes_for_value(0), expected_bytes);

	let expected_bytes: Vec<u8> = vec![112];
	assert_eq!(*big_endian_bytes_for_value(112), expected_bytes);

	let expected_bytes: Vec<u8> = vec![0b11111101, 0b00000011, 0b11101000];
	assert_eq!(*big_endian_bytes_for_value(1000), expected_bytes);

	let expected_bytes: Vec<u8> = vec![0b11111110, 0b00000000, 0b00000001, 0b00010001, 0b01110000];
	assert_eq!(*big_endian_bytes_for_value(70000), expected_bytes);

	let expected_bytes: Vec<u8> = vec![0b11111111, 0b11111111, 0b11111111, 0b10011100, 0b10010001, 0b00111010, 0b01101011, 0b00010100, 0b11110111];
	assert_eq!(*big_endian_bytes_for_value(18446634746297128183), expected_bytes);
}

// Didn't need this after all but the algorythm might come in handy in the future
// #[inline]
// // Returns the index where all it's value and all consecutive values are zero
// // Returns eight if there is no such index meaning all data in bytes for value are in use
// fn first_index_of_consecutive_zeros_for_array_of_bytes( byte_array: &[u8;8] ) -> usize {
// 	// Set to 9 if no zeros are found it will still be nine to provide a check of no zero bytes
// 	let mut first_zero_index: usize = 8;
// 	for i in 0..8 {
// 		if byte_array[i] == 0 {
// 			if first_zero_index == 8 {
// 				first_zero_index = i;
// 			}
// 		} else {
// 			if first_zero_index != 8 {
// 				first_zero_index = 8;
// 			}
// 		}
// 	}
// 	first_zero_index
// }

// #[test]
// fn test_first_index_of_consecutive_zeros_for_array_of_bytes() {
// 	let test_array: [u8;8] = [0,0,0,0,0,0,0,0];
// 	assert_eq!(first_index_of_consecutive_zeros_for_array_of_bytes(&test_array), 0);

// 	let test_array: [u8;8] = [112,0,0,0,0,0,0,0];
// 	assert_eq!(first_index_of_consecutive_zeros_for_array_of_bytes(&test_array), 1);

// 	let test_array: [u8;8] = [112,123,0,0,0,0,0,0];
// 	assert_eq!(first_index_of_consecutive_zeros_for_array_of_bytes(&test_array), 2);

// 	let test_array: [u8;8] = [112,0,1,0,5,0,0,0];
// 	assert_eq!(first_index_of_consecutive_zeros_for_array_of_bytes(&test_array), 5);

// 	let test_array: [u8;8] = [123,22,21,1,0,123,1,0];
// 	assert_eq!(first_index_of_consecutive_zeros_for_array_of_bytes(&test_array), 7);

// 	let test_array: [u8;8] = [123,22,21,1,0,123,1,12];
// 	assert_eq!(first_index_of_consecutive_zeros_for_array_of_bytes(&test_array), 8);
// }