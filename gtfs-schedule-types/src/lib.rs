//! GTFS Schedule Types generated from the HTML Documentation
//!
// #![feature(test)]
// extern crate test;

mod generated;
pub use generated::{field_types, records, schedule::Dataset};

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Temporary type alias to provide generic error (Error type to be made discrete later)
pub type Result<T, E = Error> =
	core::result::Result<T, E>;

use std::{
	convert::{AsRef, TryFrom},
	path::Path,
};

#[cfg(feature = "from-dataset")]
impl TryFrom<&Path> for Dataset
{
	type Error = Error;

	fn try_from(dataset_path: &Path) -> core::result::Result<Self, Self::Error> {
		Ok(Self {
			agency: parse_csv_record(dataset_path)?,
			routes: parse_csv_record(dataset_path)?,
			stop_times: parse_csv_record(dataset_path)?,
			stops: parse_csv_record(dataset_path)?,
			trips: parse_csv_record(dataset_path)?,

			feed_info: parse_csv_record(dataset_path).ok(),
			areas: parse_csv_record(dataset_path).ok(),
			attributions: parse_csv_record(dataset_path).ok(),
			calendar_dates: parse_csv_record(dataset_path).ok(),
			calendar: parse_csv_record(dataset_path).ok(),
			fare_attributes: parse_csv_record(dataset_path).ok(),
			fare_leg_rules: parse_csv_record(dataset_path).ok(),
			fare_media: parse_csv_record(dataset_path).ok(),
			fare_products: parse_csv_record(dataset_path).ok(),
			fare_rules: parse_csv_record(dataset_path).ok(),
			fare_transfer_rules: parse_csv_record(dataset_path).ok(),
			frequencies: parse_csv_record(dataset_path).ok(),
			levels: parse_csv_record(dataset_path).ok(),
			networks: parse_csv_record(dataset_path).ok(),
			pathways: parse_csv_record(dataset_path).ok(),
			route_networks: parse_csv_record(dataset_path).ok(),
			shapes: parse_csv_record(dataset_path).ok(),
			stop_areas: parse_csv_record(dataset_path).ok(),
			timeframes: parse_csv_record(dataset_path).ok(),
			transfers: parse_csv_record(dataset_path).ok(),
			translations: parse_csv_record(dataset_path).ok(),
		})
	}
}

// #[cfg(feature = "from-dataset")]
// fn parse_csv<RecordType>(filename: &Path) -> Result<Vec<RecordType>>
// where
// 	RecordType: serde::de::DeserializeOwned,
// {
// 	csv::Reader::from_path(filename)
// 		.and_then(|mut reader| reader.deserialize().collect::<Result<Vec<RecordType>, _>>())
// 		.map_err(|e| e.into())
// }

#[cfg(feature = "from-dataset")]
fn parse_csv_record<RecordType>(directory: &Path) -> Result<Vec<RecordType>>
where
	RecordType: serde::de::DeserializeOwned + DatasetFilename,
{
	csv::Reader::from_path(&directory.join(RecordType::FILENAME))
		.and_then(|mut reader| reader.deserialize().collect::<Result<Vec<RecordType>, _>>())
		.map_err(|e| e.into())
}

trait DatasetFilename {
	const FILENAME: &'static str;
}

// impl DatasetFilename for records::Agency {
// 	const FILENAME: &'static str = "agency.txt";
// }

// impl DatasetFilename for records::FareMedia {
// 	const FILENAME: &'static str = "fare_media.txt";
// }

// #[cfg(test)]
// mod tests {
// 	use test::Bencher;

// 	#[bench]
// 	fn bench_add_two(b: &mut Bencher) {
// 		b.iter(|| crate::Dataset::from("./feed"));
// 	}
// }
