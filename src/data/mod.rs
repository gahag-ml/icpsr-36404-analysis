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
pub enum OffenseDetailedType {
	Murder,
	NegligentManslaughter,
	Rape,
	Robbery,
	Assault,
	OtherViolent,
	Burglary,
	Larceny,
	VehicleTheft,
	Fraud,
	OtherProperty,
	Drugs,
	PublicOrder,
	Other,
	#[onehot(ignore)]
	Missing,
}

tryfrom!(
	OffenseDetailedType,
	b"1" => OffenseDetailedType::Murder,
	b"2" => OffenseDetailedType::NegligentManslaughter,
	b"3" => OffenseDetailedType::Rape,
	b"4" => OffenseDetailedType::Robbery,
	b"5" => OffenseDetailedType::Assault,
	b"6" => OffenseDetailedType::OtherViolent,
	b"7" => OffenseDetailedType::Burglary,
	b"8" => OffenseDetailedType::Larceny,
	b"9" => OffenseDetailedType::VehicleTheft,
	b"10" => OffenseDetailedType::Fraud,
	b"11" => OffenseDetailedType::OtherProperty,
	b"12" => OffenseDetailedType::Drugs,
	b"13" => OffenseDetailedType::PublicOrder,
	b"14" => OffenseDetailedType::Other,
	b"99" => OffenseDetailedType::Missing
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
pub enum Age {
	Age_18_24,
	Age_25_34,
	Age_35_44,
	Age_45_54,
	Age_55_plus,
	#[onehot(ignore)]
	Missing,
}

tryfrom!(
	Age,
	b"1" => Age::Age_18_24,
	b"2" => Age::Age_25_34,
	b"3" => Age::Age_35_44,
	b"4" => Age::Age_45_54,
	b"5" => Age::Age_55_plus,
	b"9" => Age::Missing,
	// For some reason, the nines were replaced with spaces in the age_release field:
	b" " => Age::Missing
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
#[allow(non_camel_case_types)]
pub enum Sentence {
	Years_0_1,
	Years_1_2,
	Years_2_5,
	Years_5_10,
	Years_10_25,
	Years_25_plus,
	Years_life,
	#[onehot(ignore)]
	Missing,
}

tryfrom!(
	Sentence,
	b"0" => Sentence::Years_0_1,
	b"1" => Sentence::Years_1_2,
	b"2" => Sentence::Years_2_5,
	b"3" => Sentence::Years_5_10,
	b"4" => Sentence::Years_10_25,
	b"5" => Sentence::Years_25_plus,
	b"6" => Sentence::Years_life,
	b"9" => Sentence::Missing,
	// For some reason, some nines were replaced with spaces in this field:
	b" " => Sentence::Missing
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
#[allow(non_camel_case_types)]
pub enum State {
	Alabama,
	Alaska,
	Arizona,
	Arkansas,
	California,
	Colorado,
	Connecticut,
	Delaware,
	DistrictOfColumbia,
	Florida,
	Georgia,
	Hawaii,
	Idaho,
	Illinois,
	Indiana,
	Iowa,
	Kansas,
	Kentucky,
	Louisiana,
	Maine,
	Maryland,
	Massachusetts,
	Michigan,
	Minnesota,
	Mississippi,
	Missouri,
	Montana,
	Nebraska,
	Nevada,
	NewHampshire,
	NewJersey,
	NewMexico,
	NewYork,
	NorthCarolina,
	NorthDakota,
	Ohio,
	Oklahoma,
	Oregon,
	Pennsylvania,
	RhodeIsland,
	SouthCarolina,
	SouthDakota,
	Tennessee,
	Texas,
	Utah,
	Vermont,
	Virginia,
	Washington,
	WestVirginia,
	Wisconsin,
	Wyoming,
}

tryfrom!(
	State,
	b"1" => State::Alabama,
	b"2" => State::Alaska,
	b"4" => State::Arizona,
	b"5" => State::Arkansas,
	b"6" => State::California,
	b"8" => State::Colorado,
	b"9" => State::Connecticut,
	b"10" => State::Delaware,
	b"11" => State::DistrictOfColumbia,
	b"12" => State::Florida,
	b"13" => State::Georgia,
	b"15" => State::Hawaii,
	b"16" => State::Idaho,
	b"17" => State::Illinois,
	b"18" => State::Indiana,
	b"19" => State::Iowa,
	b"20" => State::Kansas,
	b"21" => State::Kentucky,
	b"22" => State::Louisiana,
	b"23" => State::Maine,
	b"24" => State::Maryland,
	b"25" => State::Massachusetts,
	b"26" => State::Michigan,
	b"27" => State::Minnesota,
	b"28" => State::Mississippi,
	b"29" => State::Missouri,
	b"30" => State::Montana,
	b"31" => State::Nebraska,
	b"32" => State::Nevada,
	b"33" => State::NewHampshire,
	b"34" => State::NewJersey,
	b"35" => State::NewMexico,
	b"36" => State::NewYork,
	b"37" => State::NorthCarolina,
	b"38" => State::NorthDakota,
	b"39" => State::Ohio,
	b"40" => State::Oklahoma,
	b"41" => State::Oregon,
	b"42" => State::Pennsylvania,
	b"44" => State::RhodeIsland,
	b"45" => State::SouthCarolina,
	b"46" => State::SouthDakota,
	b"47" => State::Tennessee,
	b"48" => State::Texas,
	b"49" => State::Utah,
	b"50" => State::Vermont,
	b"51" => State::Virginia,
	b"53" => State::Washington,
	b"54" => State::WestVirginia,
	b"55" => State::Wisconsin,
	b"56" => State::Wyoming
);


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[derive(OneHot)]
pub struct Record {
	pub sex: Sex,
	pub admission_type: AdmissionType,
	pub offense_type: OffenseType,
	pub sentence: Sentence,
	pub race: Race,
	pub age_admission: Age,
	pub time_served: TimeServed,
	pub release_type: ReleaseType,
	#[onehot(ignore)] // For some reason, all records have missing education.
	pub education: Education,
	// As the fields of type year have broad and unbalanced values, let us ignore them.
	#[onehot(ignore)]
	pub admission_year: u16,
	#[onehot(ignore)]
	pub release_year: u16,
	#[onehot(ignore)]
	pub mandatory_release_year: u16,
	#[onehot(ignore)]
	pub projected_release_year: u16,
	#[onehot(ignore)]
	pub parole_eligibility_year: u16,
	#[onehot(ignore)] // This is worse than the generic version because there are many
	pub offense_detailed_type: OffenseDetailedType, // infrequent variants.
	#[onehot(ignore)] // Age at release is redundant with admission age and time served.
	pub age_release: Age,
	#[onehot(ignore)] // State is very unbalanced, only California and Texas have more
	pub state: State, // than 10% cover.
}


impl Record {
	pub fn parse<'a>(mut fields: impl Iterator<Item = &'a[u8]>) -> Result<Self, String> {
		fn parse<'a, T>(fields: &mut impl Iterator<Item = &'a[u8]>) -> Result<T, String>
		where
			T: TryFrom<&'a [u8]>,
			<T as TryFrom<&'a [u8]>>::Error: Into<String>
		{
			fields
				.next()
				.ok_or_else(
					|| format!("missing field {}", stringify!($name))
				)
				.and_then(
					|field| TryFrom
						::try_from(field)
						.map_err(Into::into)
				)
		};

		use std::{
			error::Error,
			str::FromStr,
		};

		fn parse_str<'a, T>(fields: &mut impl Iterator<Item = &'a[u8]>) -> Result<T, String>
		where
			T: FromStr + Default,
			<T as FromStr>::Err: Error,
		{
			let field = fields
				.next()
				.ok_or_else(
					|| format!("missing field {}", stringify!($name))
				)?;

			if field == b" " {
				return Ok(
					T::default()
				)
			}

			let field_str = std::str
				::from_utf8(field)
				.map_err(
					|_| "invalid string"
				)?;

			T
				::from_str(field_str)
				.map_err(
					|error| error.to_string()
				)
		};

		let record = Record {
			sex                     : parse(&mut fields)?,
			admission_type          : parse(&mut fields)?,
			offense_type            : parse(&mut fields)?,
			education               : parse(&mut fields)?,
			admission_year          : parse_str(&mut fields)?,
			release_year            : parse_str(&mut fields)?,
			mandatory_release_year  : parse_str(&mut fields)?,
			projected_release_year  : parse_str(&mut fields)?,
			parole_eligibility_year : parse_str(&mut fields)?,
			sentence                : parse(&mut fields)?,
			offense_detailed_type   : parse(&mut fields)?,
			race                    : parse(&mut fields)?,
			age_admission           : parse(&mut fields)?,
			age_release             : parse(&mut fields)?,
			time_served             : parse(&mut fields)?,
			release_type            : parse(&mut fields)?,
			state                   : parse(&mut fields)?,
		};

		Ok(record)
	}
}
