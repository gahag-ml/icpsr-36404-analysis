use std::collections::HashMap;

use super::*;


#[derive(Debug, Default)]
pub struct Distribution {
	pub total_records: usize,
	pub sex: HashMap<Sex, usize>,
	pub admission_type: HashMap<AdmissionType, usize>,
	pub offense_type: HashMap<OffenseType, usize>,
	pub education: HashMap<Education, usize>,
	pub race: HashMap<Race, usize>,
	pub age_admission: HashMap<AgeAdmission, usize>,
	pub time_served: HashMap<TimeServed, usize>,
	pub release_type: HashMap<ReleaseType, usize>,
}


impl Distribution {
	pub fn new() -> Self {
		Default::default()
	}


	pub fn insert(&mut self, record: &Record) {
		macro_rules! insert_field {
			($field: ident) =>  {
				if let Some(count) = self.$field.get_mut(&record.$field) {
					*count += 1;
				}
				else {
					self.$field.insert(record.$field, 1);
				}
			};
		}

		insert_field!(sex);
		insert_field!(admission_type);
		insert_field!(offense_type);
		insert_field!(education);
		insert_field!(race);
		insert_field!(age_admission);
		insert_field!(time_served);
		insert_field!(release_type);

		self.total_records += 1;
	}
}


impl std::fmt::Display for Distribution {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let total_records = self.total_records as f64;

		macro_rules! print_field {
			($field: ident) =>  {
				f.write_str(stringify!($field))?;
				f.write_str(":\n")?;

				for (value, count) in &self.$field {
					writeln!(
						f,
						"	{:?}: {} ({:.1}%)",
						value,
						count,
						(*count as f64 * 100.0) / total_records,
					)?;
				}
			};
		}

		writeln!(f, "records: {}", self.total_records)?;

		print_field!(sex);
		print_field!(admission_type);
		print_field!(offense_type);
		print_field!(education);
		print_field!(race);
		print_field!(age_admission);
		print_field!(time_served);
		print_field!(release_type);

		Ok(())
	}
}
