
use crate::records::*;

#[cfg(feature = "rustler")]
use rustler::NifStruct;

/**
 * Container referencing all records contained in a GTFS Schedule dataset
 */
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "rustler", derive(NifStruct))]
#[cfg_attr(feature = "rustler", module = "Gtfs.Schedule.Dataset")]
#[derive(Default)]
pub struct Dataset {

/** __File Name:__ `agency.txt`

__Presence:__ Required


*/
	pub agency: Vec<Agency>,

/** __File Name:__ `stops.txt`

__Presence:__ Required


*/
	pub stops: Vec<Stops>,

/** __File Name:__ `routes.txt`

__Presence:__ Required


*/
	pub routes: Vec<Routes>,

/** __File Name:__ `trips.txt`

__Presence:__ Required


*/
	pub trips: Vec<Trips>,

/** __File Name:__ `stop_times.txt`

__Presence:__ Required


*/
	pub stop_times: Vec<StopTimes>,

/** __File Name:__ `calendar.txt`

__Presence:__ Conditionally Required


*/
	pub calendar: Option<Vec<Calendar>>,

/** __File Name:__ `calendar_dates.txt`

__Presence:__ Conditionally Required


*/
	pub calendar_dates: Option<Vec<CalendarDates>>,

/** __File Name:__ `fare_attributes.txt`

__Presence:__ Optional


*/
	pub fare_attributes: Option<Vec<FareAttributes>>,

/** __File Name:__ `fare_rules.txt`

__Presence:__ Optional


*/
	pub fare_rules: Option<Vec<FareRules>>,

/** __File Name:__ `timeframes.txt`

__Presence:__ Optional


*/
	pub timeframes: Option<Vec<Timeframes>>,

/** __File Name:__ `fare_media.txt`

__Presence:__ Optional


*/
	pub fare_media: Option<Vec<FareMedia>>,

/** __File Name:__ `fare_products.txt`

__Presence:__ Optional


*/
	pub fare_products: Option<Vec<FareProducts>>,

/** __File Name:__ `fare_leg_rules.txt`

__Presence:__ Optional


*/
	pub fare_leg_rules: Option<Vec<FareLegRules>>,

/** __File Name:__ `fare_transfer_rules.txt`

__Presence:__ Optional


*/
	pub fare_transfer_rules: Option<Vec<FareTransferRules>>,

/** __File Name:__ `areas.txt`

__Presence:__ Optional


*/
	pub areas: Option<Vec<Areas>>,

/** __File Name:__ `stop_areas.txt`

__Presence:__ Optional


*/
	pub stop_areas: Option<Vec<StopAreas>>,

/** __File Name:__ `networks.txt`

__Presence:__ Conditionally Forbidden


*/
	pub networks: Option<Vec<Networks>>,

/** __File Name:__ `route_networks.txt`

__Presence:__ Conditionally Forbidden


*/
	pub route_networks: Option<Vec<RouteNetworks>>,

/** __File Name:__ `shapes.txt`

__Presence:__ Optional


*/
	pub shapes: Option<Vec<Shapes>>,

/** __File Name:__ `frequencies.txt`

__Presence:__ Optional


*/
	pub frequencies: Option<Vec<Frequencies>>,

/** __File Name:__ `transfers.txt`

__Presence:__ Optional


*/
	pub transfers: Option<Vec<Transfers>>,

/** __File Name:__ `pathways.txt`

__Presence:__ Optional


*/
	pub pathways: Option<Vec<Pathways>>,

/** __File Name:__ `levels.txt`

__Presence:__ Conditionally Required


*/
	pub levels: Option<Vec<Levels>>,

/** __File Name:__ `location_groups.txt`

__Presence:__ Optional


*/
	pub location_groups: Option<Vec<LocationGroups>>,

/** __File Name:__ `location_group_stops.txt`

__Presence:__ Optional


*/
	pub location_group_stops: Option<Vec<LocationGroupStops>>,

/** __File Name:__ `booking_rules.txt`

__Presence:__ Optional


*/
	pub booking_rules: Option<Vec<BookingRules>>,

/** __File Name:__ `translations.txt`

__Presence:__ Optional


*/
	pub translations: Option<Vec<Translations>>,

/** __File Name:__ `feed_info.txt`

__Presence:__ RecommendedRequired


*/
	pub feed_info: Vec<FeedInfo>,

/** __File Name:__ `attributions.txt`

__Presence:__ Optional


*/
	pub attributions: Option<Vec<Attributions>>,

}


#[cfg(feature = "from-dataset")]
use std::{
	convert::AsRef,
	path::Path,
};

#[cfg(feature = "from-dataset")]
use crate::parse::parse_csv;


#[cfg(feature = "from-dataset")]
impl Dataset {
	/**
	*   Creates a [`Dataset`] from a [`Path`] by reading the expected text files
	*   from the provided directory.
	*/
	pub fn read_from_path<T: AsRef<Path>>(dataset_path: T) -> Result<Self, &'static str> {
		let dataset_path = dataset_path.as_ref();

		Ok(Self {
			agency: parse_csv(&dataset_path.join(
				"agency.txt"
			)).expect("File 'agency.txt' is required, but failed to parse"),
			stops: parse_csv(&dataset_path.join(
				"stops.txt"
			)).expect("File 'stops.txt' is required, but failed to parse"),
			routes: parse_csv(&dataset_path.join(
				"routes.txt"
			)).expect("File 'routes.txt' is required, but failed to parse"),
			trips: parse_csv(&dataset_path.join(
				"trips.txt"
			)).expect("File 'trips.txt' is required, but failed to parse"),
			stop_times: parse_csv(&dataset_path.join(
				"stop_times.txt"
			)).expect("File 'stop_times.txt' is required, but failed to parse"),
			calendar: parse_csv(&dataset_path.join(
				"calendar.txt"
			)).ok(),
			calendar_dates: parse_csv(&dataset_path.join(
				"calendar_dates.txt"
			)).ok(),
			fare_attributes: parse_csv(&dataset_path.join(
				"fare_attributes.txt"
			)).ok(),
			fare_rules: parse_csv(&dataset_path.join(
				"fare_rules.txt"
			)).ok(),
			timeframes: parse_csv(&dataset_path.join(
				"timeframes.txt"
			)).ok(),
			fare_media: parse_csv(&dataset_path.join(
				"fare_media.txt"
			)).ok(),
			fare_products: parse_csv(&dataset_path.join(
				"fare_products.txt"
			)).ok(),
			fare_leg_rules: parse_csv(&dataset_path.join(
				"fare_leg_rules.txt"
			)).ok(),
			fare_transfer_rules: parse_csv(&dataset_path.join(
				"fare_transfer_rules.txt"
			)).ok(),
			areas: parse_csv(&dataset_path.join(
				"areas.txt"
			)).ok(),
			stop_areas: parse_csv(&dataset_path.join(
				"stop_areas.txt"
			)).ok(),
			networks: parse_csv(&dataset_path.join(
				"networks.txt"
			)).ok(),
			route_networks: parse_csv(&dataset_path.join(
				"route_networks.txt"
			)).ok(),
			shapes: parse_csv(&dataset_path.join(
				"shapes.txt"
			)).ok(),
			frequencies: parse_csv(&dataset_path.join(
				"frequencies.txt"
			)).ok(),
			transfers: parse_csv(&dataset_path.join(
				"transfers.txt"
			)).ok(),
			pathways: parse_csv(&dataset_path.join(
				"pathways.txt"
			)).ok(),
			levels: parse_csv(&dataset_path.join(
				"levels.txt"
			)).ok(),
			location_groups: parse_csv(&dataset_path.join(
				"location_groups.txt"
			)).ok(),
			location_group_stops: parse_csv(&dataset_path.join(
				"location_group_stops.txt"
			)).ok(),
			booking_rules: parse_csv(&dataset_path.join(
				"booking_rules.txt"
			)).ok(),
			translations: parse_csv(&dataset_path.join(
				"translations.txt"
			)).ok(),
			feed_info: parse_csv(&dataset_path.join(
				"feed_info.txt"
			)).expect("File 'feed_info.txt' is required, but failed to parse"),
			attributions: parse_csv(&dataset_path.join(
				"attributions.txt"
			)).ok(),
		})
	}
}
