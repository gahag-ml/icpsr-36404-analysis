use std::collections::HashMap;

use super::*;


#[derive(Debug, Default)]
pub struct Distribution {
	pub total_records: usize,
	pub sex: HashMap<Sex, usize>,
	pub admission_type: HashMap<AdmissionType, usize>,
	pub offense_type: HashMap<OffenseType, usize>,
	pub education: HashMap<Education, usize>,
	pub admission_year: HashMap<u16, usize>,
	pub release_year: HashMap<u16, usize>,
	pub mandatory_release_year: HashMap<u16, usize>,
	pub projected_release_year: HashMap<u16, usize>,
	pub parole_eligibility_year: HashMap<u16, usize>,
	pub sentence: HashMap<Sentence, usize>,
	pub offense_detailed_type: HashMap<OffenseDetailedType, usize>,
	pub race: HashMap<Race, usize>,
	pub age_admission: HashMap<Age, usize>,
	pub age_release: HashMap<Age, usize>,
	pub time_served: HashMap<TimeServed, usize>,
	pub release_type: HashMap<ReleaseType, usize>,
	pub state: HashMap<State, usize>,
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
		insert_field!(admission_year);
		insert_field!(release_year);
		insert_field!(mandatory_release_year);
		insert_field!(projected_release_year);
		insert_field!(parole_eligibility_year);
		insert_field!(sentence);
		insert_field!(offense_detailed_type);
		insert_field!(race);
		insert_field!(age_admission);
		insert_field!(age_release);
		insert_field!(time_served);
		insert_field!(release_type);
		insert_field!(state);

		self.total_records += 1;
	}
}


impl std::fmt::Display for Distribution {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let total_records = self.total_records as f64;

		macro_rules! print_field {
			($field: ident) =>  {
				writeln!(f, "{}:", stringify!($field))?;

				for (value, count) in &self.$field {
					let percentage = (*count as f64 * 100.0) / total_records;

					writeln!(f, "	{:?}: {} ({:.1}%)", value, count, percentage,)?;
				}
			};
		}

		writeln!(f, "records: {}", self.total_records)?;

		print_field!(sex);
		print_field!(admission_type);
		print_field!(offense_type);
		print_field!(education);
		print_field!(admission_year);
		print_field!(release_year);
		print_field!(mandatory_release_year);
		print_field!(projected_release_year);
		print_field!(parole_eligibility_year);
		print_field!(sentence);
		print_field!(offense_detailed_type);
		print_field!(race);
		print_field!(age_admission);
		print_field!(age_release);
		print_field!(time_served);
		print_field!(release_type);
		print_field!(state);

		Ok(())
	}
}
