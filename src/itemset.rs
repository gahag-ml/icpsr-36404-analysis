use std::fmt;

use lazy_static::lazy_static;

use bitvec::{
	array::BitArray,
	order::Lsb0,
};

use onehot::OneHot;

use super::data;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemSet(
	BitArray<
		Lsb0,
		[usize; bitvec::mem::elts::<usize>(data::Record::ONEHOT_LEN)]
	>
);


impl ItemSet {
	pub fn full() -> Self {
		let mut itemset = <Self as dci::ItemSet>::empty();

		let bits = itemset.0
			.iter_mut()
			.take(data::Record::ONEHOT_LEN);

		for bit in bits {
			bit.set(true);
		}

		itemset
	}
}


impl<'a> IntoIterator for &'a ItemSet {
	type Item = usize;

	type IntoIter = std::iter::FilterMap<
		std::iter::Enumerate<
			bitvec::slice::Iter<'a, Lsb0, usize>
		>,
		fn((usize, &bool)) -> Option<usize>
	>;

	fn into_iter(self) -> Self::IntoIter {
		self.0
			.iter()
			.enumerate()
			.filter_map(
				|(ix, item)| if *item { Some(ix) } else { None }
			)
	}
}


impl dci::ItemSet for ItemSet {
	fn empty() -> Self {
		Self(
			BitArray::zeroed()
		)
	}

	fn add(&mut self, item: usize) {
		self.0.set(item, true);
	}
}


impl fmt::Display for ItemSet {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		lazy_static! {
			static ref LABEL: Box<[&'static str]> = data::Record::labels().collect();
		}

		f.write_str("{")?;

		let mut iter = self.into_iter();

		if let Some(item) = iter.next() {
			write!(f, "{}", LABEL[item])?;
		}

		for item in iter {
			write!(f, ", {}", LABEL[item])?;
		}

		f.write_str("}")?;

		Ok(())
	}
}
