
macro_rules! impl_hash_serde {
	($name: ident, $len: expr) => {
		impl ::serde::Serialize for $name {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
				let mut slice = [0u8; 2 + 2 * $len];
				::serial::serialize(&mut slice, &self.0, serializer)
			}
		}

		impl<'de> ::serde::Deserialize<'de> for $name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
				let mut bytes = [0u8; $len];
				::serial::deserialize_check_len(deserializer, ::serial::ExpectedLen::Exact(&mut bytes))?;
				Ok($name(bytes))
			}
		}
	}
}

macro_rules! impl_uint_serde {
	($name: ident, $len: expr) => {
		impl ::serde::Serialize for $name {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
				let mut slice = [0u8; 2 + 2 * $len * 8];
				let mut bytes = [0u8; $len * 8];
				self.to_big_endian(&mut bytes);
				::serial::serialize_uint(&mut slice, &bytes, serializer)
			}
		}

		impl<'de> ::serde::Deserialize<'de> for $name {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
				let mut bytes = [0u8; $len * 8];
				let wrote = ::serial::deserialize_check_len(deserializer, ::serial::ExpectedLen::Between(0, &mut bytes))?;
				Ok(bytes[0..wrote].into())
			}
		}
	}
}

macro_rules! impl_uint_conversions {
	($hash: ident, $uint: ident) => {
		impl From<$uint> for $hash {
			fn from(value: $uint) -> Self {
				let mut ret = $hash::zero();
				value.to_big_endian(ret.as_bytes_mut());
				ret
			}
		}

		impl<'a> From<&'a $uint> for $hash {
			fn from(value: &'a $uint) -> Self {
				let mut ret = $hash::zero();
				value.to_big_endian(ret.as_bytes_mut());
				ret
			}
		}

		impl From<$hash> for $uint {
			fn from(value: $hash) -> Self {
				Self::from(&value)
			}
		}

		impl<'a> From<&'a $hash> for $uint {
			fn from(value: &'a $hash) -> Self {
				Self::from(value.as_ref() as &[u8])
			}
		}
	}
}
