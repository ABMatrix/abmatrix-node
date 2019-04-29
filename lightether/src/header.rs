use primitives::{H256, U256, Address, Bloom};
use rlp::{Rlp, RlpStream, Encodable, DecoderError, Decodable};
use keccak::keccak;
#[cfg(feature = "std")] use std::vec;
#[cfg(not(feature = "std"))] use alloc::vec::Vec;
#[cfg(feature = "std")] use std::iter;
#[cfg(not(feature = "std"))] use core::iter;

/// Type for block number.
pub type BlockNumber = u64;

/// Vector of bytes.
pub type Bytes = Vec<u8>;

enum Seal {
	/// The seal/signature is included.
	With,
	/// The seal/signature is not included.
	Without,
}

#[derive(Debug)]
pub struct Header {
	/// Parent hash.
	pub parent_hash: H256,
	/// Block timestamp.
	pub timestamp: u64,
	/// Block number.
	pub number: BlockNumber,
	/// Block author.
	pub author: Address,

	/// Transactions root.
	pub transactions_root: H256,
	/// Block uncles hash.
	pub uncles_hash: H256,
	/// Block extra data.
	pub extra_data: Bytes,

	/// State root.
	pub state_root: H256,
	/// Block receipts root.
	pub receipts_root: H256,
	/// Block bloom.
	pub log_bloom: Bloom,
	/// Gas used for contracts execution.
	pub gas_used: U256,
	/// Block gas limit.
	pub gas_limit: U256,

	/// Block difficulty.
	pub difficulty: U256,
	/// Vector of post-RLP-encoded fields.
	pub seal: Vec<Bytes>,

	/// Memoized hash of that header and the seal.
	pub hash: Option<H256>,
}

impl Header {
    	/// Get the seal field with RLP-decoded values as bytes.
	pub fn decode_seal<'a, T: iter::FromIterator<&'a [u8]>>(&'a self) -> Result<T, DecoderError> {
		self.seal.iter().map(|rlp| {
			Rlp::new(rlp).data()
		}).collect()
	}
}

impl Decodable for Header {
	fn decode(r: &Rlp) -> Result<Self, DecoderError> {
		let mut blockheader = Header {
			parent_hash: r.val_at(0)?,
			uncles_hash: r.val_at(1)?,
			author: r.val_at(2)?,
			state_root: r.val_at(3)?,
			transactions_root: r.val_at(4)?,
			receipts_root: r.val_at(5)?,
			log_bloom: r.val_at(6)?,
			difficulty: r.val_at(7)?,
			number: r.val_at(8)?,
			gas_limit: r.val_at(9)?,
			gas_used: r.val_at(10)?,
			timestamp: r.val_at(11)?,
			extra_data: r.val_at(12)?,
			seal: Vec::new(),
			hash: keccak(r.as_raw()).into(),
		};

		for i in 13..r.item_count()? {
			blockheader.seal.push(r.at(i)?.as_raw().to_vec())
		}

		Ok(blockheader)
	}
}

impl Encodable for Header {
	fn rlp_append(&self, s: &mut RlpStream) {
        let with_seal = Seal::With;

		if let Seal::With = with_seal {
			s.begin_list(13 + self.seal.len());
		} else {
			s.begin_list(13);
		}

		s.append(&self.parent_hash);
		s.append(&self.uncles_hash);
		s.append(&self.author);
		s.append(&self.state_root);
		s.append(&self.transactions_root);
		s.append(&self.receipts_root);
		s.append(&self.log_bloom);
		s.append(&self.difficulty);
		s.append(&self.number);
		s.append(&self.gas_limit);
		s.append(&self.gas_used);
		s.append(&self.timestamp);
		s.append(&self.extra_data);

		if let Seal::With = with_seal {
			for b in &self.seal {
				s.append_raw(b, 1);
			}
		}
	}
}

#[cfg(test)]
mod tests {
    use rustc_hex::FromHex;
	use rlp;
	use super::Header;
    use alloc::vec::Vec;
    
	#[test]
	fn test_header_seal_fields() {
		// that's rlp of block header created with ethash engine.
		let header_rlp = "f901f9a0d405da4e66f1445d455195229624e133f5baafe72b5cf7b3c36c12c8146e98b7a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347948888f1f195afa192cfee860698584c030f4c9db1a05fb2b4bfdef7b314451cb138a534d225c922fc0e5fbe25e451142732c3e25c25a088d2ec6b9860aae1a2c3b299f72b6a5d70d7f7ba4722c78f2c49ba96273c2158a007c6fdfa8eea7e86b81f5b0fc0f78f90cc19f4aa60d323151e0cac660199e9a1b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008302008003832fefba82524d84568e932a80a0a0349d8c3df71f1a48a9df7d03fd5f14aeee7d91332c009ecaff0a71ead405bd88ab4e252a7e8c2a23".from_hex().unwrap();
		let mix_hash = "a0a0349d8c3df71f1a48a9df7d03fd5f14aeee7d91332c009ecaff0a71ead405bd".from_hex().unwrap();
		let mix_hash_decoded = "a0349d8c3df71f1a48a9df7d03fd5f14aeee7d91332c009ecaff0a71ead405bd".from_hex().unwrap();
		let nonce = "88ab4e252a7e8c2a23".from_hex().unwrap();
		let nonce_decoded = "ab4e252a7e8c2a23".from_hex().unwrap();

		let header: Header = rlp::decode(&header_rlp).expect("error decoding header");
		let seal_fields = header.seal.clone();
		assert_eq!(seal_fields.len(), 2);
		assert_eq!(seal_fields[0], mix_hash);
		assert_eq!(seal_fields[1], nonce);

		let decoded_seal = header.decode_seal::<Vec<_>>().unwrap();
		assert_eq!(decoded_seal.len(), 2);
		assert_eq!(decoded_seal[0], &*mix_hash_decoded);
		assert_eq!(decoded_seal[1], &*nonce_decoded);
	}

	#[test]
	fn decode_and_encode_header() {
		// that's rlp of block header created with ethash engine.
		let header_rlp = "f901f9a0d405da4e66f1445d455195229624e133f5baafe72b5cf7b3c36c12c8146e98b7a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347948888f1f195afa192cfee860698584c030f4c9db1a05fb2b4bfdef7b314451cb138a534d225c922fc0e5fbe25e451142732c3e25c25a088d2ec6b9860aae1a2c3b299f72b6a5d70d7f7ba4722c78f2c49ba96273c2158a007c6fdfa8eea7e86b81f5b0fc0f78f90cc19f4aa60d323151e0cac660199e9a1b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008302008003832fefba82524d84568e932a80a0a0349d8c3df71f1a48a9df7d03fd5f14aeee7d91332c009ecaff0a71ead405bd88ab4e252a7e8c2a23".from_hex().unwrap();

		let header: Header = rlp::decode(&header_rlp).expect("error decoding header");
		let encoded_header = rlp::encode(&header);

		assert_eq!(header_rlp, encoded_header);
	}

	#[test]
	fn reject_header_with_large_timestamp() {
		// that's rlp of block header created with ethash engine.
		// The encoding contains a large timestamp (295147905179352825856)
		let header_rlp = "f901f9a0d405da4e66f1445d455195229624e133f5baafe72b5cf7b3c36c12c8146e98b7a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347948888f1f195afa192cfee860698584c030f4c9db1a05fb2b4bfdef7b314451cb138a534d225c922fc0e5fbe25e451142732c3e25c25a088d2ec6b9860aae1a2c3b299f72b6a5d70d7f7ba4722c78f2c49ba96273c2158a007c6fdfa8eea7e86b81f5b0fc0f78f90cc19f4aa60d323151e0cac660199e9a1b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008302008003832fefba82524d891000000000000000000080a0a0349d8c3df71f1a48a9df7d03fd5f14aeee7d91332c009ecaff0a71ead405bd88ab4e252a7e8c2a23".from_hex().unwrap();

		// This should fail decoding timestamp
		let header: Result<Header, _> = rlp::decode(&header_rlp);
		assert_eq!(header.unwrap_err(), rlp::DecoderError::RlpIsTooBig);
	}
}