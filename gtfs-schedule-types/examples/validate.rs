//! Example of parsing a GTFS Dataset with the Serde and CSV Crates
//!

use gtfs_schedule_types::{Dataset, Result};

fn main() -> Result<()> {
	let schedule = Dataset::from("./feed");

	println!("agency:               {:>10?}", Some(schedule.agency.len()));
	println!(
		"feed_info:            {:>10?}",
		Some(schedule.feed_info.len())
	);
	println!("routes:               {:>10?}", Some(schedule.routes.len()));
	println!(
		"stop_times:           {:>10?}",
		Some(schedule.stop_times.len())
	);
	println!("stops:                {:>10?}", Some(schedule.stops.len()));
	println!("trips:                {:>10?}", Some(schedule.trips.len()));

	println!(
		"areas:                {:>10?}",
		schedule.areas.map(|v| v.len())
	);
	println!(
		"attributions:         {:>10?}",
		schedule.attributions.map(|v| v.len())
	);
	println!(
		"calendar_dates:       {:>10?}",
		schedule.calendar_dates.map(|v| v.len())
	);
	println!(
		"calendar:             {:>10?}",
		schedule.calendar.map(|v| v.len())
	);
	println!(
		"fare_attributes:      {:>10?}",
		schedule.fare_attributes.map(|v| v.len())
	);
	println!(
		"fare_leg_rules:       {:>10?}",
		schedule.fare_leg_rules.map(|v| v.len())
	);
	println!(
		"fare_media:           {:>10?}",
		schedule.fare_media.map(|v| v.len())
	);
	println!(
		"fare_products:        {:>10?}",
		schedule.fare_products.map(|v| v.len())
	);
	println!(
		"fare_rules:           {:>10?}",
		schedule.fare_rules.map(|v| v.len())
	);
	println!(
		"fare_transfer_rules:  {:>10?}",
		schedule.fare_transfer_rules.map(|v| v.len())
	);
	println!(
		"frequencies:          {:>10?}",
		schedule.frequencies.map(|v| v.len())
	);
	println!(
		"levels:               {:>10?}",
		schedule.levels.map(|v| v.len())
	);
	println!(
		"networks:             {:>10?}",
		schedule.networks.map(|v| v.len())
	);
	println!(
		"pathways:             {:>10?}",
		schedule.pathways.map(|v| v.len())
	);
	println!(
		"route_networks:       {:>10?}",
		schedule.route_networks.map(|v| v.len())
	);
	println!(
		"shapes:               {:>10?}",
		schedule.shapes.map(|v| v.len())
	);
	println!(
		"stop_areas:           {:>10?}",
		schedule.stop_areas.map(|v| v.len())
	);
	println!(
		"timeframes:           {:>10?}",
		schedule.timeframes.map(|v| v.len())
	);
	println!(
		"transfers:            {:>10?}",
		schedule.transfers.map(|v| v.len())
	);
	println!(
		"translations:         {:>10?}",
		schedule.translations.map(|v| v.len())
	);

	Ok(())
}
