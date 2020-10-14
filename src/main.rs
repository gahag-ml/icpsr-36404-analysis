mod args;
mod data;
mod util;
mod itemset;

use std::{
	cmp,
	collections::HashSet,
	io::{self, Write},
	time
};

use dci::DataSet;

use onehot::OneHot;

use crate::{
	args::Command,
	data::{
		Sex,
		Race,
		Record,
		distribution::Distribution as DataDistribution
	},
	itemset::ItemSet,
};


fn read_records<R: io::BufRead>(
	reader: R,
	recidivists: bool,
	sex: Option<Sex>,
	race: Option<Race>,
) -> io::Result<(Vec<Record>, DataDistribution)> {
	let clock = time::Instant::now();

	let mut records = Vec::with_capacity(8_000_000); // number of expected records
	let mut ids: HashSet<Box<[u8]>> = HashSet::new();
	let mut data_distribution = DataDistribution::new();

	let mut line_splitter = util::RawLineSplitter::new(reader);

	line_splitter.read_line()?; // skip header

	let mut line_count: usize = 0;

	while let Some(line) = line_splitter.read_line()? {
		line_count += 1;

		let mut fields = line.split(|&c| c == b'\t');

		let id = fields.next().unwrap();

		if recidivists && !ids.contains(id) {
			ids.insert(id.into());
			continue;
		}

		match data::Record::parse(fields) {
			Ok(record) => {
				let mut valid = record.admission_type != data::AdmissionType::Missing
				           && record.offense_type   != data::OffenseType::Missing
				           // && record.education      != data::Education::Missing
				           && record.race           != data::Race::Missing
				           && record.age_admission  != data::AgeAdmission::Missing
				           && record.time_served    != data::TimeServed::Missing
				           && record.release_type   != data::ReleaseType::Missing
				           && record.release_type   != data::ReleaseType::Missing;

				if let Some(sex) = &sex {
					valid &= record.sex == *sex;
				}

				if let Some(race) = &race {
					valid &= record.race == *race;
				}

				if valid {
					records.push(record);
					data_distribution.insert(&record);
				}
			},

			Err(err) => {
				log::warn!(
					"invalid record at line {}: {}\n{:?}",
					line_count,
					err,
					String::from_utf8_lossy(line)
				);
			}
		};
	}

	log::info!("Importing dataset took {:.2?}", clock.elapsed());

	Ok((records, data_distribution))
}


fn encode_records(records: &[Record]) -> dci::Matrix<ItemSet> {
	let clock = time::Instant::now();

	let dataset: dci::Matrix<ItemSet> = onehot::vertical(&records).into();

	log::info!("Encoding dataset took {:.2?}", clock.elapsed());

	dataset
}


fn save_dataset(dataset: &dci::Matrix<ItemSet>) -> anyhow::Result<()> {
	let stdout = io::stdout();
	let stdout = stdout.lock();
	let mut stdout = io::BufWriter::with_capacity(
		8 * 1024 * 1204,
		stdout,
	);

	let dataset: &bitmatrix::BitMatrix = dataset.into();

	let clock = time::Instant::now();

	rmp_serde::encode::write(&mut stdout, &dataset)?;
	stdout.flush()?;

	log::info!("Saving dataset took {:.2?}", clock.elapsed());

	Ok(())
}


fn load_dataset<R: io::BufRead>(reader: R) -> anyhow::Result<dci::Matrix<ItemSet>> {
	let clock = time::Instant::now();

	let dataset: bitmatrix::BitMatrix = rmp_serde::decode::from_read(reader)?;

	log::info!("Restoring dataset took {:.2?}", clock.elapsed());

	log::info!("Restored {}x{} matrix.", dataset.height(), dataset.width());

	Ok(dataset.into())
}


fn run_dci(dataset: &dci::Matrix<ItemSet>, min_sup: dci::Support) -> Vec<(ItemSet, dci::Support)> {
	let clock = time::Instant::now();

	let mut result = dci::parallel::closed(dataset, min_sup);

	log::info!("Dci took {:.2?}", clock.elapsed());

	result.sort_unstable_by_key(
		|(_itemset, support)| cmp::Reverse(*support)
	);

	result
}


fn main() -> anyhow::Result<()> {
	let command = args::parse(
		std::env::args()
	)?;

	match command {
		Command::Help(message) | Command::Version(message) => {
			println!("{}", message);
			return Ok(());
		},
		_ => (),
	};

	simplelog::TermLogger
		::init(
			log::LevelFilter::Trace,
			simplelog::ConfigBuilder
				::new()
				.set_time_level(log::LevelFilter::Off)
				.build(),
			simplelog::TerminalMode::Stderr
		)
		.unwrap();

	let stdin = io::stdin();
	let stdin = stdin.lock();

	let (dataset, min_sup_ratio) = match command {
		Command::Distribution { recidivists, sex, race } => {
			let (_, data_distribution) = read_records(stdin, recidivists, sex, race)?;

			log::info!("{}", data_distribution);

			return Ok(());
		},

		Command::Load { min_sup_ratio } => (
			load_dataset(stdin)?,
			min_sup_ratio
		),

		Command::Save { recidivists, sex, race } => {
			let (records, data_distribution) = read_records(stdin, recidivists, sex, race)?;

			log::info!("{}", data_distribution);

			let dataset = encode_records(&records);

			save_dataset(&dataset)?;

			return Ok(());
		},

		Command::Run { min_sup_ratio, recidivists, sex, race } => {
			let (records, data_distribution) = read_records(stdin, recidivists, sex, race)?;

			println!("{}", data_distribution);

			(encode_records(&records), min_sup_ratio)
		},

		_ => unreachable!(),
	};

	let transactions = dataset.transactions_count();

	println!("Transactions: {}", transactions);
	println!("Items ({}): {}", data::Record::ONEHOT_LEN, ItemSet::full());

	let min_sup = (transactions as f64 * min_sup_ratio) as usize;

	println!("minsup: {} ({:.1}%)", min_sup, 100.0 * min_sup_ratio);

	let result = run_dci(&dataset, min_sup);

	for (closed_itemset, support) in result {
		println!(
			"{} ({:.1}%): {}",
			support,
			(support as f64 * 100.0) / transactions as f64,
			closed_itemset
		);
	}

	Ok(())
}
