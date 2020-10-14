use clap::{
	clap_app,
	crate_authors,
	crate_version,
	crate_description,
	value_t
};

use crate::data;


#[derive(Debug, Clone, PartialEq)]
pub enum Command {
	Help(Box<str>),
	Version(Box<str>),
	Run {
		min_sup_ratio: f64,
		recidivists: bool,
		sex: Option<data::Sex>,
		race: Option<data::Race>,
	},
	Save {
		recidivists: bool,
		sex: Option<data::Sex>,
		race: Option<data::Race>,
	},
	Load {
		min_sup_ratio: f64,
	},
	Distribution {
		recidivists: bool,
		sex: Option<data::Sex>,
		race: Option<data::Race>,
	},
}


pub fn parse(args: impl Iterator<Item = String>) -> anyhow::Result<Command> {
	let mut app = clap_app!(
		analyzer =>
			(version: crate_version!())
			(author: crate_authors!())
			(about: crate_description!())

			(@subcommand distribution =>
				(about: "load the original dataset from stdin and display the data distribution")
				(@arg recidivists: --recidivists "whether to include only recidivists")
				(@arg sex: --sex +takes_value possible_value[male female] "include only the given sex")
				(@arg race: --race +takes_value possible_value[black white hispanic other] "include only the given race"))

			(@subcommand run =>
				(about: "runs the entire pipeline")
				(@arg min_sup: +required "the minimum support ratio ([0, 1.0])")
				(@arg recidivists: --recidivists "whether to include only recidivists")
				(@arg sex: --sex +takes_value possible_value[male female] "include only the given sex")
				(@arg race: --race +takes_value possible_value[black white hispanic other] "include only the given race"))

			(@subcommand save =>
				(about: "load the original dataset from stdin and output the serialized matrix to stdout")
				(@arg recidivists: --recidivists "whether to include only recidivists")
				(@arg sex: --sex +takes_value possible_value[male female] "include only the given sex")
				(@arg race: --race +takes_value possible_value[black white hispanic other] "include only the given race"))

			(@subcommand load =>
				(about: "load the serialized matrix from stdin and run the algorithm")
				(@arg min_sup: +required "the minimum support ratio ([0, 1.0])"))
	);

	match app.get_matches_from_safe_borrow(args) {
		Ok(matches) => Ok(
			match matches.subcommand() {
				("distribution", Some(matches)) => Command::Distribution {
					recidivists: matches.is_present("recidivists"),
					sex: parse_sex(matches.value_of("sex")),
					race: parse_race(matches.value_of("race")),
				},
				("save", Some(matches)) => Command::Save {
					recidivists: matches.is_present("recidivists"),
					sex: parse_sex(matches.value_of("sex")),
					race: parse_race(matches.value_of("race")),
				},
				("load", Some(matches)) => Command::Load {
					min_sup_ratio: validate_min_sup(
						value_t!(matches, "min_sup", f64)?
					)?,
				},
				("run", Some(matches)) => Command::Run {
					min_sup_ratio: validate_min_sup(
						value_t!(matches, "min_sup", f64)?
					)?,
					recidivists: matches.is_present("recidivists"),
					sex: parse_sex(matches.value_of("sex")),
					race: parse_race(matches.value_of("race")),
				},
				_ => {
					let mut out = Vec::new();

					app.write_help(&mut out)?;

					let help = String::from_utf8(out)?;

					Command::Help(help.into())
				},
			}
		),

		Err(error) => match error.kind {
			clap::ErrorKind::HelpDisplayed => Ok(
				Command::Help(error.message.into_boxed_str())
			),
			clap::ErrorKind::VersionDisplayed => Ok(
				Command::Version(error.message.into_boxed_str())
			),
			_ => Err(error.into())
		}
	}
}


fn validate_min_sup(min_sup: f64) -> anyhow::Result<f64> {
	if (0.0 ..= 1.0).contains(&min_sup) {
		Ok(min_sup)
	}
	else {
		Err(
			anyhow::anyhow!("invalid minimum support: {}", min_sup)
		)
	}
}


fn parse_sex(arg: Option<&str>) -> Option<data::Sex> {
	arg.map(
		|arg| match arg {
			"male"   => data::Sex::Male,
			"female" => data::Sex::Female,
			_ => panic!("invalid sex arg"),
		}
	)
}


fn parse_race(arg: Option<&str>) -> Option<data::Race> {
	arg.map(
		|arg| match arg {
			"black"    => data::Race::Black,
			"hispanic" => data::Race::Hispanic,
			"white"    => data::Race::White,
			"other"    => data::Race::Other,
			_ => panic!("invalid race arg"),
		}
	)
}
