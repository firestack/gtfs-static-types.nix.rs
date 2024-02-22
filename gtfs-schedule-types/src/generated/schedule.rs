use crate::records::*;

/**
 * Container referencing all records contained in a GTFS Schedule dataset
 */
#[derive(Debug, Default)]
pub struct Dataset {
	/** __File Name:__ `agency.txt`

	__Presence:__ Required

	Transit agencies with service represented in this dataset.
	*/
	pub agency: Vec<Agency>,

	/** __File Name:__ `stops.txt`

	__Presence:__ Required

	Stops where vehicles pick up or drop off riders. Also defines
	stations and station entrances.
	*/
	pub stops: Vec<Stops>,

	/** __File Name:__ `routes.txt`

	__Presence:__ Required

	Transit routes. A route is a group of trips that are displayed
	to riders as a single service.
	*/
	pub routes: Vec<Routes>,

	/** __File Name:__ `trips.txt`

	__Presence:__ Required

	Trips for each route. A trip is a sequence of two or more stops
	that occur during a specific time period.
	*/
	pub trips: Vec<Trips>,

	/** __File Name:__ `stop_times.txt`

	__Presence:__ Required

	Times that a vehicle arrives at and departs from stops for each
	trip.
	*/
	pub stop_times: Vec<StopTimes>,

	/** __File Name:__ `calendar.txt`

	__Presence:__ Conditionally Required

	Service dates specified using a weekly schedule with start and
	end dates.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> unless all dates of service are defined
	in <a xmlns="http://www.w3.org/1999/xhtml" href="#calendar_datestxt">calendar_dates.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml">
	- Optional otherwise.
	*/
	pub calendar: Option<Vec<Calendar>>,

	/** __File Name:__ `calendar_dates.txt`

	__Presence:__ Conditionally Required

	Exceptions for the services defined in the <a xmlns="http://www.w3.org/1999/xhtml" href="#calendartxt">calendar.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> if <a xmlns="http://www.w3.org/1999/xhtml" href="#calendartxt">calendar.txt</a> is omitted. In which case <a xmlns="http://www.w3.org/1999/xhtml" href="#calendar_datestxt">calendar_dates.txt</a> must contain all dates
	of service.<br xmlns="http://www.w3.org/1999/xhtml">
	- Optional otherwise.
	*/
	pub calendar_dates: Option<Vec<CalendarDates>>,

	/** __File Name:__ `fare_attributes.txt`

	__Presence:__ Optional

	Fare information for a transit agency's routes.
	*/
	pub fare_attributes: Option<Vec<FareAttributes>>,

	/** __File Name:__ `fare_rules.txt`

	__Presence:__ Optional

	Rules to apply fares for itineraries.
	*/
	pub fare_rules: Option<Vec<FareRules>>,

	/** __File Name:__ `timeframes.txt`

	__Presence:__ Optional

	Date and time periods to use in fare rules for fares that
	depend on date and time factors.
	*/
	pub timeframes: Option<Vec<Timeframes>>,

	/** __File Name:__ `fare_media.txt`

	__Presence:__ Optional

	To describe the fare media that can be employed to use fare
	products.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	File <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_mediatxt">fare_media.txt</a> describes concepts
	that are not represented in <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_attributestxt">fare_attributes.txt</a> and <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rulestxt">fare_rules.txt</a>. As such, the use of <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_mediatxt">fare_media.txt</a> is entirely separate from files
	<a xmlns="http://www.w3.org/1999/xhtml" href="#fare_attributestxt">fare_attributes.txt</a> and <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rulestxt">fare_rules.txt</a>.
	*/
	pub fare_media: Option<Vec<FareMedia>>,

	/** __File Name:__ `fare_products.txt`

	__Presence:__ Optional

	To describe the different types of tickets or fares that can be
	purchased by riders.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	File <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_productstxt">fare_products.txt</a> describes
	fare products that are not represented in <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_attributestxt">fare_attributes.txt</a> and <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rulestxt">fare_rules.txt</a>. As such, the use of <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_productstxt">fare_products.txt</a> is entirely separate from
	files <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_attributestxt">fare_attributes.txt</a> and
	<a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rulestxt">fare_rules.txt</a>.
	*/
	pub fare_products: Option<Vec<FareProducts>>,

	/** __File Name:__ `fare_leg_rules.txt`

	__Presence:__ Optional

	Fare rules for individual legs of travel.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	File <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_leg_rulestxt">fare_leg_rules.txt</a> provides a
	more detailed method for modeling fare structures. As such, the use
	of <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_leg_rulestxt">fare_leg_rules.txt</a> is entirely
	separate from files <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_attributestxt">fare_attributes.txt</a> and <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rulestxt">fare_rules.txt</a>.
	*/
	pub fare_leg_rules: Option<Vec<FareLegRules>>,

	/** __File Name:__ `fare_transfer_rules.txt`

	__Presence:__ Optional

	Fare rules for transfers between legs of travel.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Along with <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_leg_rulestxt">fare_leg_rules.txt</a>,
	file <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_transfer_rulestxt">fare_transfer_rules.txt</a>
	provides a more detailed method for modeling fare structures. As
	such, the use of <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_transfer_rulestxt">fare_transfer_rules.txt</a> is entirely
	separate from files <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_attributestxt">fare_attributes.txt</a> and <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rulestxt">fare_rules.txt</a>.
	*/
	pub fare_transfer_rules: Option<Vec<FareTransferRules>>,

	/** __File Name:__ `areas.txt`

	__Presence:__ Optional

	Area grouping of locations.
	*/
	pub areas: Option<Vec<Areas>>,

	/** __File Name:__ `stop_areas.txt`

	__Presence:__ Optional

	Rules to assign stops to areas.
	*/
	pub stop_areas: Option<Vec<StopAreas>>,

	/** __File Name:__ `networks.txt`

	__Presence:__ Conditionally Forbidden

	Network grouping of routes.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Forbidden:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Forbidden</strong> if <code xmlns="http://www.w3.org/1999/xhtml">network_id</code> exists in
	<a xmlns="http://www.w3.org/1999/xhtml" href="#routestxt">routes.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml">
	- Optional otherwise.
	*/
	pub networks: Option<Vec<Networks>>,

	/** __File Name:__ `route_networks.txt`

	__Presence:__ Conditionally Forbidden

	Rules to assign routes to networks.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Forbidden:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Forbidden</strong> if <code xmlns="http://www.w3.org/1999/xhtml">network_id</code> exists in
	<a xmlns="http://www.w3.org/1999/xhtml" href="#routestxt">routes.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml">
	- Optional otherwise.
	*/
	pub route_networks: Option<Vec<RouteNetworks>>,

	/** __File Name:__ `shapes.txt`

	__Presence:__ Optional

	Rules for mapping vehicle travel paths, sometimes referred to
	as route alignments.
	*/
	pub shapes: Option<Vec<Shapes>>,

	/** __File Name:__ `frequencies.txt`

	__Presence:__ Optional

	Headway (time between trips) for headway-based service or a
	compressed representation of fixed-schedule service.
	*/
	pub frequencies: Option<Vec<Frequencies>>,

	/** __File Name:__ `transfers.txt`

	__Presence:__ Optional

	Rules for making connections at transfer points between
	routes.
	*/
	pub transfers: Option<Vec<Transfers>>,

	/** __File Name:__ `pathways.txt`

	__Presence:__ Optional

	Pathways linking together locations within stations.
	*/
	pub pathways: Option<Vec<Pathways>>,

	/** __File Name:__ `levels.txt`

	__Presence:__ Conditionally Required

	Levels within stations.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> when describing pathways with elevators
	(<code xmlns="http://www.w3.org/1999/xhtml">pathway_mode=5</code>).<br xmlns="http://www.w3.org/1999/xhtml">
	- Optional otherwise.
	*/
	pub levels: Option<Vec<Levels>>,

	/** __File Name:__ `translations.txt`

	__Presence:__ Optional

	Translations of customer-facing dataset values.
	*/
	pub translations: Option<Vec<Translations>>,

	/** __File Name:__ `feed_info.txt`

	__Presence:__ RecommendedRequired

	Dataset metadata, including publisher, version, and expiration
	information.
	*/
	pub feed_info: Vec<FeedInfo>,

	/** __File Name:__ `attributions.txt`

	__Presence:__ Optional

	Dataset attributions.
	*/
	pub attributions: Option<Vec<Attributions>>,
}
