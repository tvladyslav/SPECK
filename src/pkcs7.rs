use padding::*;
use std::iter::{Repeat, Take};

pub struct PKCS7;

impl PaddingGenerator for PKCS7 {
	type PaddingIterator = Take<Repeat<u8>>;

	fn set_padding<I: ExactSizeIterator<Item=u8>> (plaintext: I, block_len: usize) -> MyChain<I, Self::PaddingIterator> {
		assert!(block_len != 0 && block_len < 256, "Sorry, wrong block length!");

		let appendix: usize = plaintext.len() % block_len;
		let padding_size: usize = (block_len - appendix);

		MyChain::new(plaintext, ::std::iter::repeat(padding_size as u8).take(padding_size), padding_size)
	}

	fn remove_padding<J> (mut ciphertext: J, block_len: usize) -> Result<J, PaddingError>
		where J: ExactSizeIterator<Item=u8> + DoubleEndedIterator<Item=u8> {
		if (ciphertext.len() == 0 || ciphertext.len() % block_len != 0) {
			return Err(PaddingError::WrongCiphertextLength);
		}

		let padding_byte: u8 = ciphertext.next_back().unwrap();
		let padding_size: usize = padding_byte as usize - 1;

		let result: bool = ciphertext.by_ref().rev().take(padding_size).all(|x| x == padding_byte);
		if (result) {
			Ok(ciphertext)
		} else {
			Err(PaddingError::WrongPadding)
		}
	}
}

#[test]
fn pkcs7_block_8() {
	const B: usize = 8;

	let tuple1: PaddingTuple = (&[], B, &[08; 08]);
	let tuple2: PaddingTuple = (&[0xAA, 0xCC, 0xEE, 0xBB, 0x13], B, &[0xAA, 0xCC, 0xEE, 0xBB, 0x13, 03, 03, 03]);
	let tuple3: PaddingTuple = (&[0xAA, 0xCC, 0xEE, 0x48, 0x13, 0xFF, 0x11, 0xDD], B, &[0xAA, 0xCC, 0xEE, 0x48, 0x13, 0xFF, 0x11, 0xDD, 08, 08, 08, 08, 08, 08, 08, 08]);
	let tuple4: PaddingTuple = (&[0x10, 0xCC, 0x73, 0xBB, 0x13, 0xFF, 0x11, 0xDD, 0x50, 0x24], B, &[0x10, 0xCC, 0x73, 0xBB, 0x13, 0xFF, 0x11, 0xDD, 0x50, 0x24, 06, 06, 06, 06, 06, 06]);

	_check_padding::<PKCS7>(tuple1);
	_check_padding::<PKCS7>(tuple2);
	_check_padding::<PKCS7>(tuple3);
	_check_padding::<PKCS7>(tuple4);
}

#[test]
fn pkcs7_block_16() {
	const B: usize = 16;

	let tuple1: PaddingTuple = (&[], B, &[16; 16]);
	let tuple2: PaddingTuple = (&[0xAA, 0xCC, 0xEE, 0xBB, 0x13], B, &[0xAA, 0xCC, 0xEE, 0xBB, 0x13, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11]);
	let tuple3: PaddingTuple = (&[0xAA, 0xCC, 0xEE, 0x48, 0x13, 0xFF, 0x11, 0xDD], B, &[0xAA, 0xCC, 0xEE, 0x48, 0x13, 0xFF, 0x11, 0xDD, 08, 08, 08, 08, 08, 08, 08, 08]);
	let tuple4: PaddingTuple = (&[0x10, 0xCC, 0x73, 0xBB, 0x13, 0xFF, 0x11, 0xDD, 0x50, 0x24], B, &[0x10, 0xCC, 0x73, 0xBB, 0x13, 0xFF, 0x11, 0xDD, 0x50, 0x24, 06, 06, 06, 06, 06, 06]);
	let tuple5: PaddingTuple = (&[0x10, 0xCC, 0x73, 0xBB, 0x13, 0xFF, 0x11, 0xDD, 0x50, 0x24, 0x37, 0x22, 0xF5, 0xD3, 00, 0x1C], 
								B, 
								&[0x10, 0xCC, 0x73, 0xBB, 0x13, 0xFF, 0x11, 0xDD, 0x50, 0x24, 0x37, 0x22, 0xF5, 0xD3, 00, 0x1C, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16]);
	let tuple6: PaddingTuple = (&[0x10, 0xCC, 0x73, 0xBB, 0x13, 0xFF, 0x11, 0xDD, 0x50, 0x24, 0x37, 0x22, 0xF5, 0xD3, 00, 0x1C, 0xCA, 0x6D, 0x34, 0x66, 0xB1, 0xB1, 0x25],
								B,
								&[0x10, 0xCC, 0x73, 0xBB, 0x13, 0xFF, 0x11, 0xDD, 0x50, 0x24, 0x37, 0x22, 0xF5, 0xD3, 00, 0x1C, 0xCA, 0x6D, 0x34, 0x66, 0xB1, 0xB1, 0x25, 09, 09, 09, 09, 09, 09, 09, 09, 09]);

	_check_padding::<PKCS7>(tuple1);
	_check_padding::<PKCS7>(tuple2);
	_check_padding::<PKCS7>(tuple3);
	_check_padding::<PKCS7>(tuple4);
	_check_padding::<PKCS7>(tuple5);
	_check_padding::<PKCS7>(tuple6);

}

#[test]
fn pkcs7_block_misc() {
	let tuple1: PaddingTuple = (&[], 3, &[3, 3, 3]);
	let tuple2: PaddingTuple = (&[0xAA, 0xCC, 0xEE, 0xBB, 0x13], 7, &[0xAA, 0xCC, 0xEE, 0xBB, 0x13, 02, 02]);
	let tuple3: PaddingTuple = (&[0xAA, 0xCC, 0xEE, 0x48, 0x13, 0xFF, 0x11, 0xDD], 11, &[0xAA, 0xCC, 0xEE, 0x48, 0x13, 0xFF, 0x11, 0xDD, 03, 03, 03]);
	let tuple4: PaddingTuple = (&[0x10, 0xCC, 0x73, 0xBB, 0x13, 0xFF, 0x11, 0xDD, 0x50, 0x24], 6, &[0x10, 0xCC, 0x73, 0xBB, 0x13, 0xFF, 0x11, 0xDD, 0x50, 0x24, 02, 02]);
	let tuple5: PaddingTuple = (&[0x10, 0xCC, 0x73, 0xBB, 0x13, 0xFF, 0x11, 0xDD, 0x50, 0x24, 0x37, 0x22, 0xF5, 0xD3, 00, 0x1C],
								13,
								&[0x10, 0xCC, 0x73, 0xBB, 0x13, 0xFF, 0x11, 0xDD, 0x50, 0x24, 0x37, 0x22, 0xF5, 0xD3, 00, 0x1C, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]);

	_check_padding::<PKCS7>(tuple1);
	_check_padding::<PKCS7>(tuple2);
	_check_padding::<PKCS7>(tuple3);
	_check_padding::<PKCS7>(tuple4);
	_check_padding::<PKCS7>(tuple5);
}

#[test]
#[should_panic]
fn pkcs7_should_fail_1() {
	let tuple1: PaddingTuple = (&[0xAA, 0xCC, 0xEE, 0xBB, 0x13], 7, &[0xAA, 0xCC, 0xEE, 0xBB, 0x13, 01, 02]);
	_check_remove_padding::<PKCS7>(tuple1);
}

#[test]
#[should_panic]
fn pkcs7_should_fail_2() {
	let tuple1: PaddingTuple = (&[0xAA, 0xCC, 0xEE, 0xBB, 0x13], 7, &[0xAA, 0xCC, 0xEE, 0xBB, 0x13, 02, 02, 02]);
	_check_remove_padding::<PKCS7>(tuple1);
}

#[test]
#[should_panic]
fn pkcs7_should_fail_3() {
	let tuple1: PaddingTuple = (&[0xAA, 0xCC, 0xEE, 0xBB, 0x13], 7, &[0xAA, 0xCC, 0xEE, 0xBB, 0x13, 02]);
	_check_remove_padding::<PKCS7>(tuple1);
}

#[test]
#[should_panic]
fn pkcs7_should_fail_4() {
	let tuple1: PaddingTuple = (&[0xAA, 0xCC, 0xEE, 0xBB, 0x13], 7, &[0xAA, 0xCC, 0xEE, 0xBB, 03, 03, 03]);
	_check_remove_padding::<PKCS7>(tuple1);
}
