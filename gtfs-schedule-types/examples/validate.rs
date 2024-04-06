//! Example of parsing a GTFS Dataset with the Serde and CSV Crates
//!

use gtfs_schedule_types::{Dataset, Result};
use std::io::{stdout, Write};

fn main() -> Result<()> {
	let schedule = Dataset::try_from("./feed".as_ref())?;

	let mut lock = stdout().lock();

	write!(
		lock,
		r#"
agency	{}
feed_info	{}
routes	{}
stop_times	{}
stops	{}
trips	{}
areas	{}
attributions	{}
calendar_dates	{}
calendar	{}
fare_attributes	{}
fare_leg_rules	{}
fare_media	{}
fare_products	{}
fare_rules	{}
fare_transfer_rules	{}
frequencies	{}
levels	{}
networks	{}
pathways	{}
route_networks	{}
shapes	{}
stop_areas	{}
timeframes	{}
transfers	{}
translations	{}
"#,
		schedule.agency.len(),
		display_optional_table(schedule.feed_info),
		schedule.routes.len(),
		schedule.stop_times.len(),
		schedule.stops.len(),
		schedule.trips.len(),
		display_optional_table(schedule.areas),
		display_optional_table(schedule.attributions),
		display_optional_table(schedule.calendar_dates),
		display_optional_table(schedule.calendar),
		display_optional_table(schedule.fare_attributes),
		display_optional_table(schedule.fare_leg_rules),
		display_optional_table(schedule.fare_media),
		display_optional_table(schedule.fare_products),
		display_optional_table(schedule.fare_rules),
		display_optional_table(schedule.fare_transfer_rules),
		display_optional_table(schedule.frequencies),
		display_optional_table(schedule.levels),
		display_optional_table(schedule.networks),
		display_optional_table(schedule.pathways),
		display_optional_table(schedule.route_networks),
		display_optional_table(schedule.shapes),
		display_optional_table(schedule.stop_areas),
		display_optional_table(schedule.timeframes),
		display_optional_table(schedule.transfers),
		display_optional_table(schedule.translations),
	)?;

	Ok(())
}
const NONE_STR: &'static str = "None";

use std::borrow::Cow;
fn display_optional_table<'a, T>(table: Option<Vec<T>>) -> Cow<'a, str> {
	table
		.map(|v| Cow::from(v.len().to_string()))
		.unwrap_or(Cow::from(NONE_STR))
}
