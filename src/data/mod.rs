pub mod distribution;

use std::convert::TryFrom;

use serde::{Serialize, Deserialize};

use onehot::OneHot;


macro_rules! tryfrom {
	($name: ident, $( $pattern: pat => $value: expr ),+) => {
		impl TryFrom<&[u8]> for $name {
			type Error = String;

			fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
				match value {
					$( $pattern => Ok($value) ),+,
					_ => Err(
						format!(
							"invalid {}: {}",
							stringify!($name),
							String::from_utf8_lossy(value)
						)
					)
				}
			}
		}
	};
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(OneHot)]
pub enum Sex {
	Male,
	Female,
}

tryfrom!(
	Sex,
	b"1" => Sex::Male,
	b"2" => Sex::Female
);


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(OneHot)]
pub enum AdmissionType {
	New,
	Parole,
	Other,
	#[onehot(ignore)]
	Missing,
}

tryfrom!(
	AdmissionType,
	b"1" => AdmissionType::New,
	b"2" => AdmissionType::Parole,
	b"3" => AdmissionType::Other,
	b"9" => AdmissionType::Missing
);


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(OneHot)]
pub enum OffenseType {
	Violent,
	Property,
	Drugs,
	PublicOrder,
	Other,
	#[onehot(ignore)]
	Missing,
}

tryfrom!(
	OffenseType,
	b"1" => OffenseType::Violent,
	b"2" => OffenseType::Property,
	b"3" => OffenseType::Drugs,
	b"4" => OffenseType::PublicOrder,
	b"5" => OffenseType::Other,
	b"9" => OffenseType::Missing
);


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(OneHot)]
pub enum Education {
	NoHighSchool,
	HighSchool,
	College,
	#[onehot(ignore)]
	Missing,
}

tryfrom!(
	Education,
	b"1" => Education::NoHighSchool,
	b"2" => Education::HighSchool,
	b"3" => Education::College,
	b"9" => Education::Missing
);


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(OneHot)]
pub enum Race {
	White,
	Black,
	Hispanic,
	Other,
	#[onehot(ignore)]
	Missing,
}

tryfrom!(
	Race,
	b"1" => Race::White,
	b"2" => Race::Black,
	b"3" => Race::Hispanic,
	b"4" => Race::Other,
	b"9" => Race::Missing
);


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(OneHot)]
#[allow(non_camel_case_types)]
pub enum AgeAdmission {
	Age_18_24,
	Age_25_34,
	Age_35_44,
	Age_45_54,
	Age_55_plus,
	#[onehot(ignore)]
	Missing,
}

tryfrom!(
	AgeAdmission,
	b"1" => AgeAdmission::Age_18_24,
	b"2" => AgeAdmission::Age_25_34,
	b"3" => AgeAdmission::Age_35_44,
	b"4" => AgeAdmission::Age_45_54,
	b"5" => AgeAdmission::Age_55_plus,
	b"9" => AgeAdmission::Missing
);


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(OneHot)]
#[allow(non_camel_case_types)]
pub enum TimeServed {
	Years_0_1,
	Years_1_2,
	Years_2_5,
	Years_5_10,
	Years_10_plus,
	#[onehot(ignore)]
	Missing,
}

tryfrom!(
	TimeServed,
	b"0" => TimeServed::Years_0_1,
	b"1" => TimeServed::Years_1_2,
	b"2" => TimeServed::Years_2_5,
	b"3" => TimeServed::Years_5_10,
	b"4" => TimeServed::Years_10_plus,
	b"9" => TimeServed::Missing
);


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(OneHot)]
pub enum ReleaseType {
	Conditional,
	Unconditional,
	Other,
	#[onehot(ignore)]
	Missing
}

tryfrom!(
	ReleaseType,
	b"1" => ReleaseType::Conditional,
	b"2" => ReleaseType::Unconditional,
	b"3" => ReleaseType::Other,
	// For some reason, the nines were replaced with spaces in this field:
	b" " => ReleaseType::Missing
);


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(OneHot)]
pub struct Record {
	pub sex: Sex,
	pub admission_type: AdmissionType,
	pub offense_type: OffenseType,
	#[onehot(ignore)] // For some reason, all records have missing education.
	pub education: Education,
	pub race: Race,
	pub age_admission: AgeAdmission,
	pub time_served: TimeServed,
	pub release_type: ReleaseType,
}


impl Record {
	pub fn parse<'a>(mut fields: impl Iterator<Item = &'a[u8]>) -> Result<Self, String> {
		fn parse<'a, T>(fields: &mut impl Iterator<Item = &'a[u8]>, ix: usize) -> Result<T, String>
		where
			T: TryFrom<&'a [u8]>,
			<T as TryFrom<&'a [u8]>>::Error: Into<String>
		{
			fields
				.nth(ix)
				.ok_or_else(
					|| format!("missing field {}", stringify!($name))
				)
				.and_then(
					|field| TryFrom
						::try_from(field)
						.map_err(Into::into)
				)
		};

		let record = Record {
			sex            : parse(&mut fields, 0)?,
			admission_type : parse(&mut fields, 0)?,
			offense_type   : parse(&mut fields, 0)?,
			education      : parse(&mut fields, 0)?,
			race           : parse(&mut fields, 7)?,
			age_admission  : parse(&mut fields, 0)?,
			time_served    : parse(&mut fields, 1)?,
			release_type   : parse(&mut fields, 0)?,
		};

		Ok(record)
	}
}
