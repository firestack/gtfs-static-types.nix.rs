use crate::field_types::*;

/* Structs */

/** `agency.txt`

Transit agencies with service represented in this dataset.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Required</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>agency_id</code>)</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Agency {
	/** Record: __Conditionally Required__

	Identifies a transit brand which is often synonymous with a
	transit agency. Note that in some cases, such as when a single
	agency operates multiple separate services, agencies and brands are
	distinct. This document uses the term "agency" in place of "brand".
	A dataset may contain data from multiple agencies.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> when the dataset contains data for
	multiple transit agencies.<br xmlns="http://www.w3.org/1999/xhtml">
	- Recommended otherwise.
		 */
	pub agency_id: Option<AgencyUid>,

	/** Record: __Required__

	Full name of the transit agency.
		 */
	pub agency_name: Text,

	/** Record: __Required__

	URL of the transit agency.
		 */
	pub agency_url: Url,

	/** Record: __Required__

	Timezone where the transit agency is located. If multiple
	agencies are specified in the dataset, each must have the same
	<code xmlns="http://www.w3.org/1999/xhtml">agency_timezone</code>.
		 */
	pub agency_timezone: Timezone,

	/** Record: __Optional__

	Primary language used by this transit agency. Should be
	provided to help GTFS consumers choose capitalization rules and
	other language-specific settings for the dataset.
		 */
	pub agency_lang: Option<LanguageCode>,

	/** Record: __Optional__

	A voice telephone number for the specified agency. This field
	is a string value that presents the telephone number as typical for
	the agency's service area. It may contain punctuation marks to
	group the digits of the number. Dialable text (for example,
	TriMet's "503-238-RIDE") is permitted, but the field must not
	contain any other descriptive text.
		 */
	pub agency_phone: Option<PhoneNumber>,

	/** Record: __Optional__

	URL of a web page that allows a rider to purchase tickets or
	other fare instruments for that agency online.
		 */
	pub agency_fare_url: Option<Url>,

	/** Record: __Optional__

	Email address actively monitored by the agency’s customer
	service department. This email address should be a direct contact
	point where transit riders can reach a customer service
	representative at the agency.
		 */
	pub agency_email: Option<Email>,
}

/** `stops.txt`

Stops where vehicles pick up or drop off riders. Also defines
stations and station entrances.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Required</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>stop_id</code>)</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stops {
	/** Record: __Required__

	Identifies a location: stop/platform, station, entrance/exit,
	generic node or boarding area (see
	<code xmlns="http://www.w3.org/1999/xhtml">location_type</code>).<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Multiple routes may use the same <code xmlns="http://www.w3.org/1999/xhtml">stop_id</code>.
		 */
	pub stop_id: StopUid,

	/** Record: __Optional__

	Short text or a number that identifies the location for riders.
	These codes are often used in phone-based transit information
	systems or printed on signage to make it easier for riders to get
	information for a particular location. The <code xmlns="http://www.w3.org/1999/xhtml">stop_code</code>
	may be the same as <code xmlns="http://www.w3.org/1999/xhtml">stop_id</code> if it is public facing.
	This field should be left empty for locations without a code
	presented to riders.
		 */
	pub stop_code: Option<Text>,

	/** Record: __Conditionally Required__

	Name of the location. The <code xmlns="http://www.w3.org/1999/xhtml">stop_name</code> should match
	the agency's rider-facing name for the location as printed on a
	timetable, published online, or represented on signage. For
	translations into other languages, use <a xmlns="http://www.w3.org/1999/xhtml" href="#translationstxt">translations.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	When the location is a boarding area
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=4</code>), the <code xmlns="http://www.w3.org/1999/xhtml">stop_name</code> should
	contains the name of the boarding area as displayed by the agency.
	It could be just one letter (like on some European intercity
	railway stations), or text like “Wheelchair boarding area” (NYC’s
	Subway) or “Head of short trains” (Paris’ RER).<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> for locations which are stops
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=0</code>), stations
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=1</code>) or entrances/exits
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=2</code>).<br xmlns="http://www.w3.org/1999/xhtml">
	- Optional for locations which are generic nodes
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=3</code>) or boarding areas
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=4</code>).
		 */
	pub stop_name: Option<Text>,

	/** Record: __Optional__

	Readable version of the <code xmlns="http://www.w3.org/1999/xhtml">stop_name</code>. See
	"Text-to-speech field" in the <a xmlns="http://www.w3.org/1999/xhtml" href="#term-definitions">Term
	   Definitions</a> for more.
		 */
	pub tts_stop_name: Option<Text>,

	/** Record: __Optional__

	Description of the location that provides useful, quality
	information. Should not be a duplicate of
	<code xmlns="http://www.w3.org/1999/xhtml">stop_name</code>.
		 */
	pub stop_desc: Option<Text>,

	/** Record: __Conditionally Required__

	Latitude of the location.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	For stops/platforms (<code xmlns="http://www.w3.org/1999/xhtml">location_type=0</code>) and boarding
	area (<code xmlns="http://www.w3.org/1999/xhtml">location_type=4</code>), the coordinates must be the
	ones of the bus pole — if exists — and otherwise of where the
	travelers are boarding the vehicle (on the sidewalk or the
	platform, and not on the roadway or the track where the vehicle
	stops).<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> for locations which are stops
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=0</code>), stations
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=1</code>) or entrances/exits
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=2</code>).<br xmlns="http://www.w3.org/1999/xhtml">
	- Optional for locations which are generic nodes
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=3</code>) or boarding areas
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=4</code>).
		 */
	pub stop_lat: Option<Latitude>,

	/** Record: __Conditionally Required__

	Longitude of the location.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	For stops/platforms (<code xmlns="http://www.w3.org/1999/xhtml">location_type=0</code>) and boarding
	area (<code xmlns="http://www.w3.org/1999/xhtml">location_type=4</code>), the coordinates must be the
	ones of the bus pole — if exists — and otherwise of where the
	travelers are boarding the vehicle (on the sidewalk or the
	platform, and not on the roadway or the track where the vehicle
	stops).<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> for locations which are stops
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=0</code>), stations
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=1</code>) or entrances/exits
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=2</code>).<br xmlns="http://www.w3.org/1999/xhtml">
	- Optional for locations which are generic nodes
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=3</code>) or boarding areas
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=4</code>).
		 */
	pub stop_lon: Option<Longitude>,

	/** Record: __Conditionally Required__

	Identifies the fare zone for a stop. If this record represents
	a station or station entrance, the <code xmlns="http://www.w3.org/1999/xhtml">zone_id</code> is
	ignored.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> if providing fare information using
	<a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rulestxt">fare_rules.txt</a><br xmlns="http://www.w3.org/1999/xhtml">
	- Optional otherwise.
		 */
	pub zone_id: Option<ZoneId>,

	/** Record: __Optional__

	URL of a web page about the location. This should be different
	from the <code xmlns="http://www.w3.org/1999/xhtml">agency.agency_url</code> and the
	<code xmlns="http://www.w3.org/1999/xhtml">routes.route_url</code> field values.
		 */
	pub stop_url: Option<Url>,

	/** Record: __Optional__

	Location type. Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> (or blank) - <strong xmlns="http://www.w3.org/1999/xhtml">Stop</strong> (or
	<strong xmlns="http://www.w3.org/1999/xhtml">Platform</strong>). A location where passengers board or
	disembark from a transit vehicle. Is called a platform when defined
	within a <code xmlns="http://www.w3.org/1999/xhtml">parent_station</code>.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - <strong xmlns="http://www.w3.org/1999/xhtml">Station</strong>. A physical structure or
	area that contains one or more platform.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - <strong xmlns="http://www.w3.org/1999/xhtml">Entrance/Exit</strong>. A location where
	passengers can enter or exit a station from the street. If an
	entrance/exit belongs to multiple stations, it may be linked by
	pathways to both, but the data provider must pick one of them as
	parent.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">3</code> - <strong xmlns="http://www.w3.org/1999/xhtml">Generic Node</strong>. A location within a
	station, not matching any other <code xmlns="http://www.w3.org/1999/xhtml">location_type</code>, that
	may be used to link together pathways define in <a xmlns="http://www.w3.org/1999/xhtml" href="#pathwaystxt">pathways.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">4</code> - <strong xmlns="http://www.w3.org/1999/xhtml">Boarding Area</strong>. A specific
	location on a platform, where passengers can board and/or alight
	vehicles.
		 */
	pub location_type: Option<GtfsEnum>,

	/** Record: __Conditionally Required__

	Defines hierarchy between the different locations defined in
	<a xmlns="http://www.w3.org/1999/xhtml" href="#stopstxt">stops.txt</a>. It contains the ID of the parent
	location, as followed:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Stop/platform</strong> (<code xmlns="http://www.w3.org/1999/xhtml">location_type=0</code>):
	the <code xmlns="http://www.w3.org/1999/xhtml">parent_station</code> field contains the ID of a
	station.<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Station</strong> (<code xmlns="http://www.w3.org/1999/xhtml">location_type=1</code>): this
	field must be empty.<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Entrance/exit</strong> (<code xmlns="http://www.w3.org/1999/xhtml">location_type=2</code>) or
	<strong xmlns="http://www.w3.org/1999/xhtml">generic node</strong> (<code xmlns="http://www.w3.org/1999/xhtml">location_type=3</code>): the
	<code xmlns="http://www.w3.org/1999/xhtml">parent_station</code> field contains the ID of a station
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=1</code>)<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Boarding Area</strong> (<code xmlns="http://www.w3.org/1999/xhtml">location_type=4</code>):
	the <code xmlns="http://www.w3.org/1999/xhtml">parent_station</code> field contains ID of a
	platform.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> for locations which are entrances
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=2</code>), generic nodes
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=3</code>) or boarding areas
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=4</code>).<br xmlns="http://www.w3.org/1999/xhtml">
	- Optional for stops/platforms
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=0</code>).<br xmlns="http://www.w3.org/1999/xhtml">
	- Forbidden for stations (<code xmlns="http://www.w3.org/1999/xhtml">location_type=1</code>).
		 */
	pub parent_station: Option<StopUid>,

	/** Record: __Optional__

	Timezone of the location. If the location has a parent station,
	it inherits the parent station’s timezone instead of applying its
	own. Stations and parentless stops with empty
	<code xmlns="http://www.w3.org/1999/xhtml">stop_timezone</code> inherit the timezone specified by
	<code xmlns="http://www.w3.org/1999/xhtml">agency.agency_timezone</code>. The times provided in <a xmlns="http://www.w3.org/1999/xhtml" href="#stop_timestxt">stop_times.txt</a> are in the timezone specified
	by <code xmlns="http://www.w3.org/1999/xhtml">agency.agency_timezone</code>, not
	<code xmlns="http://www.w3.org/1999/xhtml">stop_timezone</code>. This ensures that the time values in a
	trip always increase over the course of a trip, regardless of which
	timezones the trip crosses.
		 */
	pub stop_timezone: Option<Timezone>,

	/** Record: __Optional__

	Indicates whether wheelchair boardings are possible from the
	location. Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	For parentless stops:<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> or empty - No accessibility information for the
	stop.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Some vehicles at this stop can be boarded by a
	rider in a wheelchair.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - Wheelchair boarding is not possible at this
	stop.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	For child stops:<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> or empty - Stop will inherit its
	<code xmlns="http://www.w3.org/1999/xhtml">wheelchair_boarding</code> behavior from the parent station,
	if specified in the parent.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - There exists some accessible path from outside the
	station to the specific stop/platform.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - There exists no accessible path from outside the
	station to the specific stop/platform.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	For station entrances/exits:<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> or empty - Station entrance will inherit its
	<code xmlns="http://www.w3.org/1999/xhtml">wheelchair_boarding</code> behavior from the parent station,
	if specified for the parent.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Station entrance is wheelchair accessible.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - No accessible path from station entrance to
	stops/platforms.
		 */
	pub wheelchair_boarding: Option<GtfsEnum>,

	/** Record: __Optional__

	Level of the location. The same level may be used by multiple
	unlinked stations.
		 */
	pub level_id: Option<LevelUid>,

	/** Record: __Optional__

	Platform identifier for a platform stop (a stop belonging to a
	station). This should be just the platform identifier (eg. "G" or
	"3"). Words like “platform” or "track" (or the feed’s
	language-specific equivalent) should not be included. This allows
	feed consumers to more easily internationalize and localize the
	platform identifier into other languages.
		 */
	pub platform_code: Option<Text>,
}

/** `routes.txt`

Transit routes. A route is a group of trips that are displayed
to riders as a single service.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Required</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>route_id</code>)</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Routes {
	/** Record: __Required__

	Identifies a route.
		 */
	pub route_id: RouteUid,

	/** Record: __Conditionally Required__

	Agency for the specified route.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> if multiple agencies are defined in
	<a xmlns="http://www.w3.org/1999/xhtml" href="#agency">agency.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml">
	- Recommended otherwise.
		 */
	pub agency_id: Option<AgencyUid>,

	/** Record: __Conditionally Required__

	Short name of a route. Often a short, abstract identifier
	(e.g., "32", "100X", "Green") that riders use to identify a route.
	Both <code xmlns="http://www.w3.org/1999/xhtml">route_short_name</code> and <code xmlns="http://www.w3.org/1999/xhtml">route_long_name</code>
	may be defined.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> if <code xmlns="http://www.w3.org/1999/xhtml">routes.route_long_name</code>
	is empty.<br xmlns="http://www.w3.org/1999/xhtml">
	- Recommended if there is a brief service designation. This should
	be the commonly-known passenger name of the service, and should be
	no longer than 12 characters.
		 */
	pub route_short_name: Option<Text>,

	/** Record: __Conditionally Required__

	Full name of a route. This name is generally more descriptive
	than the <code xmlns="http://www.w3.org/1999/xhtml">route_short_name</code> and often includes the
	route's destination or stop. Both <code xmlns="http://www.w3.org/1999/xhtml">route_short_name</code> and
	<code xmlns="http://www.w3.org/1999/xhtml">route_long_name</code> may be defined.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> if <code xmlns="http://www.w3.org/1999/xhtml">routes.route_short_name</code>
	is empty.<br xmlns="http://www.w3.org/1999/xhtml">
	- Optional otherwise.
		 */
	pub route_long_name: Option<Text>,

	/** Record: __Optional__

	Description of a route that provides useful, quality
	information. Should not be a duplicate of
	<code xmlns="http://www.w3.org/1999/xhtml">route_short_name</code> or <code xmlns="http://www.w3.org/1999/xhtml">route_long_name</code>.
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: "A" trains operate between Inwood-207 St, Manhattan
	   and Far Rockaway-Mott Avenue, Queens at all times. Also from about
	   6AM until about midnight, additional "A" trains operate between
	   Inwood-207 St and Lefferts Boulevard (trains typically alternate
	   between Lefferts Blvd and Far Rockaway).</em>
		 */
	pub route_desc: Option<Text>,

	/** Record: __Required__

	Indicates the type of transportation used on a route. Valid
	options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> - Tram, Streetcar, Light rail. Any light rail or
	street level system within a metropolitan area.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Subway, Metro. Any underground rail system within
	a metropolitan area.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - Rail. Used for intercity or long-distance
	travel.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">3</code> - Bus. Used for short- and long-distance bus
	routes.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">4</code> - Ferry. Used for short- and long-distance boat
	service.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">5</code> - Cable tram. Used for street-level rail cars where
	the cable runs beneath the vehicle (e.g., cable car in San
	Francisco).<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">6</code> - Aerial lift, suspended cable car (e.g., gondola
	lift, aerial tramway). Cable transport where cabins, cars, gondolas
	or open chairs are suspended by means of one or more cables.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">7</code> - Funicular. Any rail system designed for steep
	inclines.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">11</code> - Trolleybus. Electric buses that draw power from
	overhead wires using poles.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">12</code> - Monorail. Railway in which the track consists of
	a single rail or a beam.
		 */
	pub route_type: GtfsEnum,

	/** Record: __Optional__

	URL of a web page about the particular route. Should be
	different from the <code xmlns="http://www.w3.org/1999/xhtml">agency.agency_url</code> value.
		 */
	pub route_url: Option<Url>,

	/** Record: __Optional__

	Route color designation that matches public facing material.
	Defaults to white (<code xmlns="http://www.w3.org/1999/xhtml">FFFFFF</code>) when omitted or left empty.
	The color difference between <code xmlns="http://www.w3.org/1999/xhtml">route_color</code> and
	<code xmlns="http://www.w3.org/1999/xhtml">route_text_color</code> should provide sufficient contrast
	when viewed on a black and white screen.
		 */
	pub route_color: Option<Color>,

	/** Record: __Optional__

	Legible color to use for text drawn against a background of
	<code xmlns="http://www.w3.org/1999/xhtml">route_color</code>. Defaults to black (<code xmlns="http://www.w3.org/1999/xhtml">000000</code>)
	when omitted or left empty. The color difference between
	<code xmlns="http://www.w3.org/1999/xhtml">route_color</code> and <code xmlns="http://www.w3.org/1999/xhtml">route_text_color</code> should
	provide sufficient contrast when viewed on a black and white
	screen.
		 */
	pub route_text_color: Option<Color>,

	/** Record: __Optional__

	Orders the routes in a way which is ideal for presentation to
	customers. Routes with smaller <code xmlns="http://www.w3.org/1999/xhtml">route_sort_order</code> values
	should be displayed first.
		 */
	pub route_sort_order: Option<NonNegativeInteger>,

	/** Record: __Optional__

	Indicates that the rider can board the transit vehicle at any
	point along the vehicle’s travel path as described by <a xmlns="http://www.w3.org/1999/xhtml" href="#shapestxt">shapes.txt</a>, on every trip of the route. Valid
	options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> - Continuous stopping pickup.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> or empty - No continuous stopping pickup.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - Must phone agency to arrange continuous stopping
	pickup.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">3</code> - Must coordinate with driver to arrange continuous
	stopping pickup.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Values for <code xmlns="http://www.w3.org/1999/xhtml">routes.continuous_pickup</code> may be overridden
	by defining values in <code xmlns="http://www.w3.org/1999/xhtml">stop_times.continuous_pickup</code> for
	specific <code xmlns="http://www.w3.org/1999/xhtml">stop_time</code>s along the route.
		 */
	pub continuous_pickup: Option<GtfsEnum>,

	/** Record: __Optional__

	Indicates that the rider can alight from the transit vehicle at
	any point along the vehicle’s travel path as described by <a xmlns="http://www.w3.org/1999/xhtml" href="#shapestxt">shapes.txt</a>, on every trip of the route. Valid
	options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> - Continuous stopping drop off.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> or empty - No continuous stopping drop off.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - Must phone agency to arrange continuous stopping
	drop off.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">3</code> - Must coordinate with driver to arrange continuous
	stopping drop off.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Values for <code xmlns="http://www.w3.org/1999/xhtml">routes.continuous_drop_off</code> may be
	overridden by defining values in
	<code xmlns="http://www.w3.org/1999/xhtml">stop_times.continuous_drop_off</code> for specific
	<code xmlns="http://www.w3.org/1999/xhtml">stop_time</code>s along the route.
		 */
	pub continuous_drop_off: Option<GtfsEnum>,

	/** Record: __Conditionally Forbidden__

	Identifies a group of routes. Multiple rows in <a xmlns="http://www.w3.org/1999/xhtml" href="#routestxt">routes.txt</a> may have the same
	<code xmlns="http://www.w3.org/1999/xhtml">network_id</code>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Forbidden:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Forbidden</strong> if the <a xmlns="http://www.w3.org/1999/xhtml" href="#route_networkstxt">route_networks.txt</a> file exists.<br xmlns="http://www.w3.org/1999/xhtml">
	- Optional otherwise.
		 */
	pub network_id: Option<NetworkId>,
}

/** `trips.txt`

Trips for each route. A trip is a sequence of two or more stops
that occur during a specific time period.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Required</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>trip_id</code>)</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Trips {
	/** Record: __Required__

	Identifies a route.
		 */
	pub route_id: RouteUid,

	/** Record: __Required__

	Identifies a set of dates when service is available for one or
	more routes.
		 */
	pub service_id: ServiceUid,

	/** Record: __Required__

	Identifies a trip.
		 */
	pub trip_id: TripUid,

	/** Record: __Optional__

	Text that appears on signage identifying the trip's destination
	to riders. Should be used to distinguish between different patterns
	of service on the same route.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If the headsign changes during a trip, values for
	<code xmlns="http://www.w3.org/1999/xhtml">trip_headsign</code> may be overridden by defining values in
	<code xmlns="http://www.w3.org/1999/xhtml">stop_times.stop_headsign</code> for specific
	<code xmlns="http://www.w3.org/1999/xhtml">stop_time</code>s along the trip.
		 */
	pub trip_headsign: Option<Text>,

	/** Record: __Optional__

	Public facing text used to identify the trip to riders, for
	instance, to identify train numbers for commuter rail trips. If
	riders do not commonly rely on trip names,
	<code xmlns="http://www.w3.org/1999/xhtml">trip_short_name</code> should be empty. A
	<code xmlns="http://www.w3.org/1999/xhtml">trip_short_name</code> value, if provided, should uniquely
	identify a trip within a service day; it should not be used for
	destination names or limited/express designations.
		 */
	pub trip_short_name: Option<Text>,

	/** Record: __Optional__

	Indicates the direction of travel for a trip. This field should
	not be used in routing; it provides a way to separate trips by
	direction when publishing time tables. Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> - Travel in one direction (e.g. outbound
	travel).<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Travel in the opposite direction (e.g. inbound
	travel).
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: The <code>trip_headsign</code> and
	   <code>direction_id</code> fields may be used together to assign a
	   name to travel in each direction for a set of trips. A <a href="#tripstxt">trips.txt</a> file could contain these records for use
	   in time tables:</em><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">trip_id,...,trip_headsign,direction_id</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1234,...,Airport,0</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1505,...,Downtown,1</code>
		 */
	pub direction_id: Option<GtfsEnum>,

	/** Record: __Optional__

	Identifies the block to which the trip belongs. A block
	consists of a single trip or many sequential trips made using the
	same vehicle, defined by shared service days and
	<code xmlns="http://www.w3.org/1999/xhtml">block_id</code>. A <code xmlns="http://www.w3.org/1999/xhtml">block_id</code> may have trips with
	different service days, making distinct blocks. See the <a xmlns="http://www.w3.org/1999/xhtml" href="#example-blocks-and-service-day">example below</a>. To provide
	in-seat transfers information, <a xmlns="http://www.w3.org/1999/xhtml" href="#transferstxt">transfers</a> of <code xmlns="http://www.w3.org/1999/xhtml">transfer_type</code><code xmlns="http://www.w3.org/1999/xhtml">4</code> should be provided instead.
		 */
	pub block_id: Option<BlockId>,

	/** Record: __Conditionally Required__

	Identifies a geospatial shape describing the vehicle travel
	path for a trip.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> if the trip has a continuous pickup or
	drop-off behavior defined either in <a xmlns="http://www.w3.org/1999/xhtml" href="#routestxt">routes.txt</a> or in <a xmlns="http://www.w3.org/1999/xhtml" href="#stop_timestxt">stop_times.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml">
	- Optional otherwise.
		 */
	pub shape_id: Option<ShapeId>,

	/** Record: __Optional__

	Indicates wheelchair accessibility. Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> or empty - No accessibility information for the
	trip.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Vehicle being used on this particular trip can
	accommodate at least one rider in a wheelchair.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - No riders in wheelchairs can be accommodated on
	this trip.
		 */
	pub wheelchair_accessible: Option<GtfsEnum>,

	/** Record: __Optional__

	Indicates whether bikes are allowed. Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> or empty - No bike information for the trip.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Vehicle being used on this particular trip can
	accommodate at least one bicycle.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - No bicycles are allowed on this trip.
		 */
	pub bikes_allowed: Option<GtfsEnum>,
}

/** `stop_times.txt`

Times that a vehicle arrives at and departs from stops for each
trip.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Required</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>trip_id</code>,
   <code>stop_sequence</code>)</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StopTimes {
	/** Record: __Required__

	Identifies a trip.
		 */
	pub trip_id: TripUid,

	/** Record: __Conditionally Required__

	Arrival time at the stop (defined by
	<code xmlns="http://www.w3.org/1999/xhtml">stop_times.stop_id</code>) for a specific trip (defined by
	<code xmlns="http://www.w3.org/1999/xhtml">stop_times.trip_id</code>) in the time zone specified by
	<code xmlns="http://www.w3.org/1999/xhtml">agency.agency_timezone</code>, not
	<code xmlns="http://www.w3.org/1999/xhtml">stops.stop_timezone</code>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If there are not separate times for arrival and departure at a
	stop, <code xmlns="http://www.w3.org/1999/xhtml">arrival_time</code> and <code xmlns="http://www.w3.org/1999/xhtml">departure_time</code>
	should be the same.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	For times occurring after midnight on the service day, enter the
	time as a value greater than 24:00:00 in HH:MM:SS.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If exact arrival and departure times (<code xmlns="http://www.w3.org/1999/xhtml">timepoint=1</code> or
	empty) are not available, estimated or interpolated arrival and
	departure times (<code xmlns="http://www.w3.org/1999/xhtml">timepoint=0</code>) should be
	provided.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> for the first and last stop in a trip
	(defined by <code xmlns="http://www.w3.org/1999/xhtml">stop_times.stop_sequence</code>).<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> for <code xmlns="http://www.w3.org/1999/xhtml">timepoint=1</code>.<br xmlns="http://www.w3.org/1999/xhtml">
	- Optional otherwise.
		 */
	pub arrival_time: Option<Time>,

	/** Record: __Conditionally Required__

	Departure time from the stop (defined by
	<code xmlns="http://www.w3.org/1999/xhtml">stop_times.stop_id</code>) for a specific trip (defined by
	<code xmlns="http://www.w3.org/1999/xhtml">stop_times.trip_id</code>) in the time zone specified by
	<code xmlns="http://www.w3.org/1999/xhtml">agency.agency_timezone</code>, not
	<code xmlns="http://www.w3.org/1999/xhtml">stops.stop_timezone</code>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If there are not separate times for arrival and departure at a
	stop, <code xmlns="http://www.w3.org/1999/xhtml">arrival_time</code> and <code xmlns="http://www.w3.org/1999/xhtml">departure_time</code>
	should be the same.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	For times occurring after midnight on the service day, enter the
	time as a value greater than 24:00:00 in HH:MM:SS.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If exact arrival and departure times (<code xmlns="http://www.w3.org/1999/xhtml">timepoint=1</code> or
	empty) are not available, estimated or interpolated arrival and
	departure times (<code xmlns="http://www.w3.org/1999/xhtml">timepoint=0</code>) should be
	provided.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> for <code xmlns="http://www.w3.org/1999/xhtml">timepoint=1</code>.<br xmlns="http://www.w3.org/1999/xhtml">
	- Optional otherwise.
		 */
	pub departure_time: Option<Time>,

	/** Record: __Required__

	Identifies the serviced stop. All stops serviced during a trip
	must have a record in <a xmlns="http://www.w3.org/1999/xhtml" href="#stop_timestxt">stop_times.txt</a>.
	Referenced locations must be stops/platforms, i.e. their
	<code xmlns="http://www.w3.org/1999/xhtml">stops.location_type</code> value must be <code xmlns="http://www.w3.org/1999/xhtml">0</code> or
	empty. A stop may be serviced multiple times in the same trip, and
	multiple trips and routes may service the same stop.
		 */
	pub stop_id: StopUid,

	/** Record: __Required__

	Order of stops for a particular trip. The values must increase
	along the trip but do not need to be consecutive.
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: The first location on the trip could have a
	   <code>stop_sequence</code>=<code>1</code>, the second location on
	   the trip could have a <code>stop_sequence</code>=<code>23</code>,
	   the third location could have a
	   <code>stop_sequence</code>=<code>40</code>, and so on.</em>
		 */
	pub stop_sequence: NonNegativeInteger,

	/** Record: __Optional__

	Text that appears on signage identifying the trip's destination
	to riders. This field overrides the default
	<code xmlns="http://www.w3.org/1999/xhtml">trips.trip_headsign</code> when the headsign changes between
	stops. If the headsign is displayed for an entire trip,
	<code xmlns="http://www.w3.org/1999/xhtml">trips.trip_headsign</code> should be used instead.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	A <code xmlns="http://www.w3.org/1999/xhtml">stop_headsign</code> value specified for one
	<code xmlns="http://www.w3.org/1999/xhtml">stop_time</code> does not apply to subsequent
	<code xmlns="http://www.w3.org/1999/xhtml">stop_time</code>s in the same trip. If you want to override
	the <code xmlns="http://www.w3.org/1999/xhtml">trip_headsign</code> for multiple <code xmlns="http://www.w3.org/1999/xhtml">stop_time</code>s
	in the same trip, the <code xmlns="http://www.w3.org/1999/xhtml">stop_headsign</code> value must be
	repeated in each <code xmlns="http://www.w3.org/1999/xhtml">stop_time</code> row.
		 */
	pub stop_headsign: Option<Text>,

	/** Record: __Optional__

	Indicates pickup method. Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> or empty - Regularly scheduled pickup.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - No pickup available.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - Must phone agency to arrange pickup.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">3</code> - Must coordinate with driver to arrange
	pickup.
		 */
	pub pickup_type: Option<GtfsEnum>,

	/** Record: __Optional__

	Indicates drop off method. Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> or empty - Regularly scheduled drop off.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - No drop off available.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - Must phone agency to arrange drop off.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">3</code> - Must coordinate with driver to arrange drop
	off.
		 */
	pub drop_off_type: Option<GtfsEnum>,

	/** Record: __Optional__

	Indicates that the rider can board the transit vehicle at any
	point along the vehicle’s travel path as described by <a xmlns="http://www.w3.org/1999/xhtml" href="#shapestxt">shapes.txt</a>, from this <code xmlns="http://www.w3.org/1999/xhtml">stop_time</code> to
	the next <code xmlns="http://www.w3.org/1999/xhtml">stop_time</code> in the trip’s
	<code xmlns="http://www.w3.org/1999/xhtml">stop_sequence</code>. Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> - Continuous stopping pickup.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> or empty - No continuous stopping pickup.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - Must phone agency to arrange continuous stopping
	pickup.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">3</code> - Must coordinate with driver to arrange continuous
	stopping pickup.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If this field is populated, it overrides any continuous pickup
	behavior defined in <a xmlns="http://www.w3.org/1999/xhtml" href="#routestxt">routes.txt</a>. If this
	field is empty, the <code xmlns="http://www.w3.org/1999/xhtml">stop_time</code> inherits any continuous
	pickup behavior defined in <a xmlns="http://www.w3.org/1999/xhtml" href="#routestxt">routes.txt</a>.
		 */
	pub continuous_pickup: Option<GtfsEnum>,

	/** Record: __Optional__

	Indicates that the rider can alight from the transit vehicle at
	any point along the vehicle’s travel path as described by <a xmlns="http://www.w3.org/1999/xhtml" href="#shapestxt">shapes.txt</a>, from this <code xmlns="http://www.w3.org/1999/xhtml">stop_time</code> to
	the next <code xmlns="http://www.w3.org/1999/xhtml">stop_time</code> in the trip’s
	<code xmlns="http://www.w3.org/1999/xhtml">stop_sequence</code>. Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> - Continuous stopping drop off.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> or empty - No continuous stopping drop off.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - Must phone agency to arrange continuous stopping
	drop off.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">3</code> - Must coordinate with driver to arrange continuous
	stopping drop off.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If this field is populated, it overrides any continuous drop-off
	behavior defined in <a xmlns="http://www.w3.org/1999/xhtml" href="#routestxt">routes.txt</a>. If this
	field is empty, the <code xmlns="http://www.w3.org/1999/xhtml">stop_time</code> inherits any continuous
	drop-off behavior defined in <a xmlns="http://www.w3.org/1999/xhtml" href="#routestxt">routes.txt</a>.
		 */
	pub continuous_drop_off: Option<GtfsEnum>,

	/** Record: __Optional__

	Actual distance traveled along the associated shape, from the
	first stop to the stop specified in this record. This field
	specifies how much of the shape to draw between any two stops
	during a trip. Must be in the same units used in <a xmlns="http://www.w3.org/1999/xhtml" href="#shapestxt">shapes.txt</a>. Values used for
	<code xmlns="http://www.w3.org/1999/xhtml">shape_dist_traveled</code> must increase along with
	<code xmlns="http://www.w3.org/1999/xhtml">stop_sequence</code>; they must not be used to show reverse
	travel along a route.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Recommended for routes that have looping or inlining (the vehicle
	crosses or travels over the same portion of alignment in one trip).
	See <a xmlns="http://www.w3.org/1999/xhtml" href="#shapestxt"><code>shapes.shape_dist_traveled</code></a>.
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: If a bus travels a distance of 5.25 kilometers from
	   the start of the shape to the
	   stop,<code>shape_dist_traveled</code>=<code>5.25</code>.</em>
		 */
	pub shape_dist_traveled: Option<NonNegativeFloat>,

	/** Record: __Recommended__

	Indicates if arrival and departure times for a stop are
	strictly adhered to by the vehicle or if they are instead
	approximate and/or interpolated times. This field allows a GTFS
	producer to provide interpolated stop-times, while indicating that
	the times are approximate. Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> - Times are considered approximate.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> or empty - Times are considered exact.
		 */
	pub timepoint: Option<GtfsEnum>,
}

/** `calendar.txt`

Service dates specified using a weekly schedule with start and
end dates.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> unless all dates of service are defined
in <a xmlns="http://www.w3.org/1999/xhtml" href="#calendar_datestxt">calendar_dates.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml">
- Optional otherwise.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Conditionally Required</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>service_id</code>)</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Calendar {
	/** Record: __Required__

	Identifies a set of dates when service is available for one or
	more routes.
		 */
	pub service_id: ServiceUid,

	/** Record: __Required__

	Indicates whether the service operates on all Mondays in the
	date range specified by the <code xmlns="http://www.w3.org/1999/xhtml">start_date</code> and
	<code xmlns="http://www.w3.org/1999/xhtml">end_date</code> fields. Note that exceptions for particular
	dates may be listed in <a xmlns="http://www.w3.org/1999/xhtml" href="#calendar_datestxt">calendar_dates.txt</a>. Valid options
	are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Service is available for all Mondays in the date
	range.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> - Service is not available for Mondays in the date
	range.
		 */
	pub monday: GtfsEnum,

	/** Record: __Required__

	Functions in the same way as <code xmlns="http://www.w3.org/1999/xhtml">monday</code> except applies
	to Tuesdays
		 */
	pub tuesday: GtfsEnum,

	/** Record: __Required__

	Functions in the same way as <code xmlns="http://www.w3.org/1999/xhtml">monday</code> except applies
	to Wednesdays
		 */
	pub wednesday: GtfsEnum,

	/** Record: __Required__

	Functions in the same way as <code xmlns="http://www.w3.org/1999/xhtml">monday</code> except applies
	to Thursdays
		 */
	pub thursday: GtfsEnum,

	/** Record: __Required__

	Functions in the same way as <code xmlns="http://www.w3.org/1999/xhtml">monday</code> except applies
	to Fridays
		 */
	pub friday: GtfsEnum,

	/** Record: __Required__

	Functions in the same way as <code xmlns="http://www.w3.org/1999/xhtml">monday</code> except applies
	to Saturdays.
		 */
	pub saturday: GtfsEnum,

	/** Record: __Required__

	Functions in the same way as <code xmlns="http://www.w3.org/1999/xhtml">monday</code> except applies
	to Sundays.
		 */
	pub sunday: GtfsEnum,

	/** Record: __Required__

	Start service day for the service interval.
		 */
	pub start_date: Date,

	/** Record: __Required__

	End service day for the service interval. This service day is
	included in the interval.
		 */
	pub end_date: Date,
}

/** `calendar_dates.txt`

Exceptions for the services defined in the <a xmlns="http://www.w3.org/1999/xhtml" href="#calendartxt">calendar.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> if <a xmlns="http://www.w3.org/1999/xhtml" href="#calendartxt">calendar.txt</a> is omitted. In which case <a xmlns="http://www.w3.org/1999/xhtml" href="#calendar_datestxt">calendar_dates.txt</a> must contain all dates
of service.<br xmlns="http://www.w3.org/1999/xhtml">
- Optional otherwise.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Conditionally Required</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>service_id</code>, <code>date</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">The <a href="#calendar_datestxt">calendar_dates.txt</a> table
   explicitly activates or disables service by date. It may be used in
   two ways.</p><ul xmlns="http://www.w3.org/1999/xhtml">
   <li>Recommended: Use <a href="#calendar_datestxt">calendar_dates.txt</a> in conjunction with
	  <a href="#calendartxt">calendar.txt</a> to define exceptions to the
	  default service patterns defined in <a href="#calendartxt">calendar.txt</a>. If service is generally regular,
	  with a few changes on explicit dates (for instance, to accommodate
	  special event services, or a school schedule), this is a good
	  approach. In this case <code>calendar_dates.service_id</code> is a
	  foreign ID referencing <code>calendar.service_id</code>.</li>
   <li>Alternate: Omit <a href="#calendartxt">calendar.txt</a>, and
	  specify each date of service in <a href="#calendardatestxt">calendar_dates.txt</a>. This allows for
	  considerable service variation and accommodates service without
	  normal weekly schedules. In this case <code>service_id</code> is an
	  ID.</li>
</ul>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CalendarDates {
	/** Record: __Required__

	Identifies a set of dates when a service exception occurs for
	one or more routes. Each (<code xmlns="http://www.w3.org/1999/xhtml">service_id</code>,
	<code xmlns="http://www.w3.org/1999/xhtml">date</code>) pair may only appear once in <a xmlns="http://www.w3.org/1999/xhtml" href="#calendar_datestxt">calendar_dates.txt</a> if using <a xmlns="http://www.w3.org/1999/xhtml" href="#calendartxt">calendar.txt</a> and <a xmlns="http://www.w3.org/1999/xhtml" href="#calendar_datestxt">calendar_dates.txt</a> in conjunction. If a
	<code xmlns="http://www.w3.org/1999/xhtml">service_id</code> value appears in both <a xmlns="http://www.w3.org/1999/xhtml" href="#calendartxt">calendar.txt</a> and <a xmlns="http://www.w3.org/1999/xhtml" href="#calendar_datestxt">calendar_dates.txt</a>, the information in
	<a xmlns="http://www.w3.org/1999/xhtml" href="#calendardatestxt">calendar_dates.txt</a> modifies the
	service information specified in <a xmlns="http://www.w3.org/1999/xhtml" href="#calendartxt">calendar.txt</a>.
		 */
	pub service_id: ServiceUid,

	/** Record: __Required__

	Date when service exception occurs.
		 */
	pub date: Date,

	/** Record: __Required__

	Indicates whether service is available on the date specified in
	the date field. Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Service has been added for the specified
	date.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - Service has been removed for the specified date.
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: Suppose a route has one set of trips available on
	   holidays and another set of trips available on all other days. One
	   <code>service_id</code> could correspond to the regular service
	   schedule and another <code>service_id</code> could correspond to
	   the holiday schedule. For a particular holiday, the <a href="#calendar_datestxt">calendar_dates.txt</a> file could be used to
	   add the holiday to the holiday <code>service_id</code> and to
	   remove the holiday from the regular <code>service_id</code>
	   schedule.</em>
		 */
	pub exception_type: GtfsEnum,
}

/** `fare_attributes.txt`

Fare information for a transit agency's routes.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Optional</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>fare_id</code>)</p><p xmlns="http://www.w3.org/1999/xhtml"><strong>Versions</strong><br>
   There are two modelling options for describing fares. GTFS-Fares V1
   is the legacy option for describing minimal fare information.
   GTFS-Fares V2 is an updated method that allows for a more detailed
   account of an agency's fare structure. Both are allowed to be
   present in a dataset, but only one method should be used by a data
   consumer for a given dataset. It is recommended that GTFS-Fares V2
   takes precedence over GTFS-Fares V1.<br><br>
   The files associated with GTFS-Fares V1 are:<br>
   - <a href="#fare_attributestxt">fare_attributes.txt</a><br>
   - <a href="#fare_rulestxt">fare_rules.txt</a><br><br>
   The files associated with GTFS-Fares V2 are:<br>
   - <a href="#fare_mediatxt">fare_media.txt</a><br>
   - <a href="#fare_productstxt">fare_products.txt</a><br>
   - <a href="#fare_leg_rulestxt">fare_leg_rules.txt</a><br>
   - <a href="#fare_transfer_rulestxt">fare_transfer_rules.txt</a></p><p xmlns="http://www.w3.org/1999/xhtml"><br></p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FareAttributes {
	/** Record: __Required__

	Identifies a fare class.
		 */
	pub fare_id: FareUid,

	/** Record: __Required__

	Fare price, in the unit specified by
	<code xmlns="http://www.w3.org/1999/xhtml">currency_type</code>.
		 */
	pub price: NonNegativeFloat,

	/** Record: __Required__

	Currency used to pay the fare.
		 */
	pub currency_type: CurrencyCode,

	/** Record: __Required__

	Indicates when the fare must be paid. Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> - Fare is paid on board.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Fare must be paid before boarding.
		 */
	pub payment_method: GtfsEnum,

	/** Record: __Required__

	Indicates the number of transfers permitted on this fare. Valid
	options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> - No transfers permitted on this fare.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Riders may transfer once.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - Riders may transfer twice.<br xmlns="http://www.w3.org/1999/xhtml">
	empty - Unlimited transfers are permitted.
		 */
	pub transfers: GtfsEnum,

	/** Record: __Conditionally Required__

	Identifies the relevant agency for a fare.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> if multiple agencies are defined in
	<a xmlns="http://www.w3.org/1999/xhtml" href="#agencytxt">agency.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml">
	- Recommended otherwise.
		 */
	pub agency_id: Option<AgencyUid>,

	/** Record: __Optional__

	Length of time in seconds before a transfer expires. When
	<code xmlns="http://www.w3.org/1999/xhtml">transfers</code>=<code xmlns="http://www.w3.org/1999/xhtml">0</code> this field may be used to
	indicate how long a ticket is valid for or it may be left
	empty.
		 */
	pub transfer_duration: Option<NonNegativeInteger>,
}

/** `fare_rules.txt`

Rules to apply fares for itineraries.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Optional</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>*</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">The <a href="#farerulestxt">fare_rules.txt</a> table specifies
   how fares in <a href="#fare_attributestxt">fare_attributes.txt</a>
   apply to an itinerary. Most fare structures use some combination of
   the following rules:</p><ul xmlns="http://www.w3.org/1999/xhtml">
   <li>Fare depends on origin or destination stations.</li>
   <li>Fare depends on which zones the itinerary passes through.</li>
   <li>Fare depends on which route the itinerary uses.</li>
</ul><p xmlns="http://www.w3.org/1999/xhtml">For examples that demonstrate how to specify a fare structure
   with <a href="#farerulestxt">fare_rules.txt</a> and <a href="#fareattributestxt">fare_attributes.txt</a>, see <a href="https://web.archive.org/web/20111207224351/https://code.google.com/p/googletransitdatafeed/wiki/FareExamples">
	  FareExamples</a> in the GoogleTransitDataFeed open source project
   wiki.</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FareRules {
	/** Record: __Required__

	Identifies a fare class.
		 */
	pub fare_id: FareUid,

	/** Record: __Optional__

	Identifies a route associated with the fare class. If several
	routes with the same fare attributes exist, create a record in
	<a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rules.txt">fare_rules.txt</a> for each route.
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: If fare class "b" is valid on route "TSW" and "TSE",
	   the <a href="#fare_rules.txt">fare_rules.txt</a> file would contain
	   these records for the fare class:</em><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">fare_id,route_id</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">b,TSW</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">b,TSE</code>
		 */
	pub route_id: Option<RouteUid>,

	/** Record: __Optional__

	Identifies an origin zone. If a fare class has multiple origin
	zones, create a record in <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rules.txt">fare_rules.txt</a> for each
	<code xmlns="http://www.w3.org/1999/xhtml">origin_id</code>.
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: If fare class "b" is valid for all travel originating
	   from either zone "2" or zone "8", the <a href="#fare_rules.txt">fare_rules.txt</a> file would contain these
	   records for the fare class:</em><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">fare_id,...,origin_id</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">b,...,2</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">b,...,8</code>
		 */
	pub origin_id: Option<ZoneId>,

	/** Record: __Optional__

	Identifies a destination zone. If a fare class has multiple
	destination zones, create a record in <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rules.txt">fare_rules.txt</a> for each
	<code xmlns="http://www.w3.org/1999/xhtml">destination_id</code>.
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: The <code>origin_id</code> and
	   <code>destination_id</code> fields could be used together to
	   specify that fare class "b" is valid for travel between zones 3 and
	   4, and for travel between zones 3 and 5, the <a href="#fare_rules.txt">fare_rules.txt</a> file would contain these
	   records for the fare class:</em><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">fare_id,...,origin_id,destination_id</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">b,...,3,4</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">b,...,3,5</code>
		 */
	pub destination_id: Option<ZoneId>,

	/** Record: __Optional__

	Identifies the zones that a rider will enter while using a
	given fare class. Used in some systems to calculate correct fare
	class.
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: If fare class "c" is associated with all travel on the
	   GRT route that passes through zones 5, 6, and 7 the <a href="#fare_rules.txt">fare_rules.txt</a> would contain these
	   records:</em><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">fare_id,route_id,...,contains_id</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">c,GRT,...,5</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">c,GRT,...,6</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">c,GRT,...,7</code><br xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Because all <code>contains_id</code> zones must be matched for
	   the fare to apply, an itinerary that passes through zones 5 and 6
	   but not zone 7 would not have fare class "c". For more detail, see
	   <a href="https://code.google.com/p/googletransitdatafeed/wiki/FareExamples">
		  https://code.google.com/p/googletransitdatafeed/wiki/FareExamples</a>
	   in the GoogleTransitDataFeed project wiki.</em>
		 */
	pub contains_id: Option<ZoneId>,
}

/** `timeframes.txt`

Date and time periods to use in fare rules for fares that
depend on date and time factors.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Optional</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (*)</p><p xmlns="http://www.w3.org/1999/xhtml">Used to describe fares that can vary based on the time of day,
   the day of the week, or a particular day in the year. Timeframes
   can be associated with fare products in <a href="#fare_leg_rulestxt">fare_leg_rules.txt</a>.<br>
   There must not be overlapping time intervals for the same
   <code>timeframe_group_id</code> and <code>service_id</code>
   values.</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Timeframes {
	/** Record: __Required__

	Identifies a timeframe or set of timeframes.
		 */
	pub timeframe_group_id: TimeframeGroupId,

	/** Record: __Conditionally Required__

	Defines the beginning of a timeframe. The interval includes the
	start time.<br xmlns="http://www.w3.org/1999/xhtml">
	Values greater than <code xmlns="http://www.w3.org/1999/xhtml">24:00:00</code> are forbidden. An empty
	value in <code xmlns="http://www.w3.org/1999/xhtml">start_time</code> is considered
	<code xmlns="http://www.w3.org/1999/xhtml">00:00:00</code>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> if <code xmlns="http://www.w3.org/1999/xhtml">timeframes.end_time</code> is
	defined.<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Forbidden</strong> otherwise
		 */
	pub start_time: Option<Time>,

	/** Record: __Conditionally Required__

	Defines the end of a timeframe. The interval does not include
	the end time.<br xmlns="http://www.w3.org/1999/xhtml">
	Values greater than <code xmlns="http://www.w3.org/1999/xhtml">24:00:00</code> are forbidden. An empty
	value in <code xmlns="http://www.w3.org/1999/xhtml">end_time</code> is considered
	<code xmlns="http://www.w3.org/1999/xhtml">24:00:00</code>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> if <code xmlns="http://www.w3.org/1999/xhtml">timeframes.start_time</code>
	is defined.<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Forbidden</strong> otherwise
		 */
	pub end_time: Option<Time>,

	/** Record: __Required__

	Identifies a set of dates that a timeframe is in effect.
		 */
	pub service_id: ServiceUid,
}

/** `fare_media.txt`

To describe the fare media that can be employed to use fare
products.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
File <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_mediatxt">fare_media.txt</a> describes concepts
that are not represented in <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_attributestxt">fare_attributes.txt</a> and <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rulestxt">fare_rules.txt</a>. As such, the use of <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_mediatxt">fare_media.txt</a> is entirely separate from files
<a xmlns="http://www.w3.org/1999/xhtml" href="#fare_attributestxt">fare_attributes.txt</a> and <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rulestxt">fare_rules.txt</a>.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Optional</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary Key (<code>fare_media_id</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">To describe the different fare media that can be employed to use
   fare products. Fare media are physical or virtual holders used for
   the representation and/or validation of a fare product.</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FareMedia {
	/** Record: __Required__

	Identifies a fare media.
		 */
	pub fare_media_id: FareMediaUid,

	/** Record: __Optional__

	Name of the fare media.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	For fare media which are transit cards (<code xmlns="http://www.w3.org/1999/xhtml">fare_media_type
	   =2</code>) or mobile apps (<code xmlns="http://www.w3.org/1999/xhtml">fare_media_type =4</code>), the
	<code xmlns="http://www.w3.org/1999/xhtml">fare_media_name</code> should be included and should match
	the rider-facing name used by the organizations delivering
	them.
		 */
	pub fare_media_name: Option<Text>,

	/** Record: __Required__

	The type of fare media. Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> - None. Used when there is no fare media involved in
	purchasing or validating a fare product, such as paying cash to a
	driver or conductor with no physical ticket provided.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Physical paper ticket that allows a passenger to
	take either a certain number of pre-purchased trips or unlimited
	trips within a fixed period of time.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - Physical transit card that has stored tickets,
	passes or monetary value.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">3</code> - cEMV (contactless Europay, Mastercard and Visa) as
	an open-loop token container for account-based ticketing.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">4</code> - Mobile app that have stored virtual transit cards,
	tickets, passes, or monetary value.
		 */
	pub fare_media_type: GtfsEnum,
}

/** `fare_products.txt`

To describe the different types of tickets or fares that can be
purchased by riders.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
File <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_productstxt">fare_products.txt</a> describes
fare products that are not represented in <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_attributestxt">fare_attributes.txt</a> and <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rulestxt">fare_rules.txt</a>. As such, the use of <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_productstxt">fare_products.txt</a> is entirely separate from
files <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_attributestxt">fare_attributes.txt</a> and
<a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rulestxt">fare_rules.txt</a>.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Optional</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary Key (<code>fare_product_id</code>,
   <code>fare_media_id</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">To describe the different types of tickets or fares that can be
   purchased by riders.</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FareProducts {
	/** Record: __Required__

	Identifies a fare product.
		 */
	pub fare_product_id: FareProductId,

	/** Record: __Optional__

	The name of the fare product as displayed to riders.
		 */
	pub fare_product_name: Option<Text>,

	/** Record: __Optional__

	Identifies a fare media that can be employed to use the fare
	product during the trip. When <code xmlns="http://www.w3.org/1999/xhtml">fare_media_id</code> is empty,
	it is considered that the fare media is unknown.
		 */
	pub fare_media_id: Option<FareMediaUid>,

	/** Record: __Required__

	The cost of the fare product. May be negative to represent
	transfer discounts. May be zero to represent a fare product that is
	free.
		 */
	pub amount: CurrencyAmount,

	/** Record: __Required__

	The currency of the cost of the fare product.
		 */
	pub currency: CurrencyCode,
}

/** `fare_leg_rules.txt`

Fare rules for individual legs of travel.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
File <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_leg_rulestxt">fare_leg_rules.txt</a> provides a
more detailed method for modeling fare structures. As such, the use
of <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_leg_rulestxt">fare_leg_rules.txt</a> is entirely
separate from files <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_attributestxt">fare_attributes.txt</a> and <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rulestxt">fare_rules.txt</a>.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Optional</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary Key (<code>network_id, from_area_id, to_area_id,
	  from_timeframe_group_id, to_timeframe_group_id,
	  fare_product_id</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">Fare rules for individual legs of travel.</p><p xmlns="http://www.w3.org/1999/xhtml">Fares in <a href="#fare_leg_rulestxt"><code>fare_leg_rules.txt</code></a> must be
   queried by filtering all the records in the file to find rules that
   match the leg to be traveled by the rider.</p><p xmlns="http://www.w3.org/1999/xhtml">To process the cost of a leg:</p><ol xmlns="http://www.w3.org/1999/xhtml">
   <li>
	  <p>The file <a href="#fare_leg_rulestxt">fare_leg_rules.txt</a>
		 must be filtered by the fields that define the characteristics of
		 travel, these fields are:</p>
	  <ul>
		 <li><code>fare_leg_rules.network_id</code></li>
		 <li><code>fare_leg_rules.from_area_id</code></li>
		 <li><code>fare_leg_rules.to_area_id</code></li>
		 <li><code>fare_leg_rules.from_timeframe_group_id</code></li>
		 <li><code>fare_leg_rules.to_timeframe_group_id</code><br><br><br></li>
	  </ul>
   </li>
   <li>
	  <p>If the leg exactly matches a record in <a href="#fare_leg_rulestxt">fare_leg_rules.txt</a> based on the
		 characteristics of travel, that record must be processed to
		 determine the cost of the leg.<br></p>
   </li>
   <li>
	  <p>If no exact matches are found, then empty entries in
		 <code>fare_leg_rules.network_id</code>,
		 <code>fare_leg_rules.from_area_id</code>, and
		 <code>fare_leg_rules.to_area_id</code> must be checked to process
		 the cost of the leg:</p>
	  <ul>
		 <li>An empty entry in <code>fare_leg_rules.network_id</code>
			corresponds to all networks defined in <a href="#routestxt">routes.txt</a> or <a href="#networkstxt">networks.txt</a> excluding the ones listed under
			<code>fare_leg_rules.network_id</code></li>
		 <li>An empty entry in <code>fare_leg_rules.from_area_id</code>
			corresponds to all areas defined in <code>areas.area_id</code>
			excluding the ones listed under
			<code>fare_leg_rules.from_area_id</code></li>
		 <li>An empty entry in <code>fare_leg_rules.to_area_id</code>
			corresponds to all areas defined in <code>areas.area_id</code>
			excluding the ones listed under
			<code>fare_leg_rules.to_area_id</code><br></li>
	  </ul>
   </li>
   <li>
	  <p>If the leg does not match any of the rules described above, then
		 the fare is unknown.</p>
   </li>
</ol><p xmlns="http://www.w3.org/1999/xhtml"><br></p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FareLegRules {
	/** Record: __Optional__

	Identifies a group of entries in <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_leg_rulestxt">fare_leg_rules.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Used to describe fare transfer rules between
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.from_leg_group_id</code> and
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.to_leg_group_id</code>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Multiple entries in <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_leg_rulestxt">fare_leg_rules.txt</a> may belong to the same
	<code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.leg_group_id</code>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	The same entry in <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_leg_rulestxt">fare_leg_rules.txt</a> (not including
	<code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.leg_group_id</code>) must not belong to
	multiple <code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.leg_group_id</code>.
		 */
	pub leg_group_id: Option<LegGroupId>,

	/** Record: __Optional__

	Identifies a route network that applies for the fare leg
	rule.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If there are no matching <code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.network_id</code>
	values to the <code xmlns="http://www.w3.org/1999/xhtml">network_id</code> being filtered, empty
	<code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.network_id</code> will be matched by
	default.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	An empty entry in <code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.network_id</code>
	corresponds to all networks defined in <a xmlns="http://www.w3.org/1999/xhtml" href="#routestxt">routes.txt</a> or <a xmlns="http://www.w3.org/1999/xhtml" href="#networkstxt">networks.txt</a> excluding the ones listed under
	<code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.network_id</code>
		 */
	pub network_id: Option<NetworkUid>,

	/** Record: __Optional__

	Identifies a departure area.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If there are no matching <code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.from_area_id</code>
	values to the <code xmlns="http://www.w3.org/1999/xhtml">area_id</code> being filtered, empty
	<code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.from_area_id</code> will be matched by
	default.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	An empty entry in <code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.from_area_id</code>
	corresponds to all areas defined in <code xmlns="http://www.w3.org/1999/xhtml">areas.area_id</code>
	excluding the ones listed under
	<code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.from_area_id</code>
		 */
	pub from_area_id: Option<AreaUid>,

	/** Record: __Optional__

	Identifies an arrival area.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If there are no matching <code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.to_area_id</code>
	values to the <code xmlns="http://www.w3.org/1999/xhtml">area_id</code> being filtered, empty
	<code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.to_area_id</code> will be matched by
	default.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	An empty entry in <code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.to_area_id</code>
	corresponds to all areas defined in <code xmlns="http://www.w3.org/1999/xhtml">areas.area_id</code>
	excluding the ones listed under
	<code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.to_area_id</code>
		 */
	pub to_area_id: Option<AreaUid>,

	/** Record: __Optional__

	Defines the timeframe for the fare validation event at the
	start of the fare leg.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	The “start time” of the fare leg is the time at which the event is
	scheduled to occur. For example, the time could be the scheduled
	departure time of a bus at the start of a fare leg where the rider
	boards and validates their fare. For the rule matching semantics
	below, the start time is computed in local time, as determined by
	<a xmlns="http://www.w3.org/1999/xhtml" href="#localtimesemantics">Local Time Semantics</a> of <a xmlns="http://www.w3.org/1999/xhtml" href="#timeframestxt">timeframes.txt</a>. The stop or station of the
	fare leg’s departure event should be used for timezone resolution,
	where appropriate.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	For a fare leg rule that specifies a
	<code xmlns="http://www.w3.org/1999/xhtml">from_timeframe_group_id</code>, that rule will match a
	particular leg if there exists at least one record in <a xmlns="http://www.w3.org/1999/xhtml" href="#timeframestxt">timeframes.txt</a> where all of the following
	conditions are true<br xmlns="http://www.w3.org/1999/xhtml">
	- The value of <code xmlns="http://www.w3.org/1999/xhtml">timeframe_group_id</code> is equal to the
	<code xmlns="http://www.w3.org/1999/xhtml">from_timeframe_group_id</code> value.<br xmlns="http://www.w3.org/1999/xhtml">
	- The set of days identified by the record’s
	<code xmlns="http://www.w3.org/1999/xhtml">service_id</code> contains the “current day” of the fare
	leg’s start time.<br xmlns="http://www.w3.org/1999/xhtml">
	- The “time-of-day” of the fare leg's start time is greater than or
	equal to the record’s <code xmlns="http://www.w3.org/1999/xhtml">timeframes.start_time</code> value and
	less than the <code xmlns="http://www.w3.org/1999/xhtml">timeframes.end_time</code> value.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	An empty <code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.from_timeframe_group_id</code>
	indicates that the start time of the leg does not affect the
	matching of this rule.
		 */
	pub from_timeframe_group_id: Option<TimeframeGroupId>,

	/** Record: __Optional__

	Defines the timeframe for the fare validation event at the end
	of the fare leg.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	The “end time” of the fare leg is the time at which the event is
	scheduled to occur. For example, the time could be the scheduled
	arrival time of a bus at the end of a fare leg where the rider gets
	off and validates their fare. For the rule matching semantics
	below, the end time is computed in local time, as determined by
	<a xmlns="http://www.w3.org/1999/xhtml" href="#localtimesemantics">Local Time Semantics</a> of <a xmlns="http://www.w3.org/1999/xhtml" href="#timeframestxt">timeframes.txt</a>. The stop or station of the
	fare leg’s arrival event should be used for timezone resolution,
	where appropriate.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	For a fare leg rule that specifies a
	<code xmlns="http://www.w3.org/1999/xhtml">to_timeframe_group_id</code>, that rule will match a
	particular leg if there exists at least one record in <a xmlns="http://www.w3.org/1999/xhtml" href="#timeframestxt">timeframes.txt</a> where all of the following
	conditions are true<br xmlns="http://www.w3.org/1999/xhtml">
	- The value of <code xmlns="http://www.w3.org/1999/xhtml">timeframe_group_id</code> is equal to the
	<code xmlns="http://www.w3.org/1999/xhtml">to_timeframe_group_id</code> value.<br xmlns="http://www.w3.org/1999/xhtml">
	- The set of days identified by the record’s
	<code xmlns="http://www.w3.org/1999/xhtml">service_id</code> contains the “current day” of the fare
	leg’s end time.<br xmlns="http://www.w3.org/1999/xhtml">
	- The “time-of-day” of the fare leg's end time is greater than or
	equal to the record’s <code xmlns="http://www.w3.org/1999/xhtml">timeframes.start_time</code> value and
	less than the <code xmlns="http://www.w3.org/1999/xhtml">timeframes.end_time</code> value.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	An empty <code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.to_timeframe_group_id</code>
	indicates that the end time of the leg does not affect the matching
	of this rule.
		 */
	pub to_timeframe_group_id: Option<TimeframeGroupId>,

	/** Record: __Required__

	The fare product required to travel the leg.
		 */
	pub fare_product_id: FareProductId,
}

/** `fare_transfer_rules.txt`

Fare rules for transfers between legs of travel.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
Along with <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_leg_rulestxt">fare_leg_rules.txt</a>,
file <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_transfer_rulestxt">fare_transfer_rules.txt</a>
provides a more detailed method for modeling fare structures. As
such, the use of <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_transfer_rulestxt">fare_transfer_rules.txt</a> is entirely
separate from files <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_attributestxt">fare_attributes.txt</a> and <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rulestxt">fare_rules.txt</a>.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Optional</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary Key (<code>from_leg_group_id, to_leg_group_id,
	  fare_product_id, transfer_count, duration_limit</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">Fare rules for transfers between legs of travel defined in
   <a href="#fare_leg_rulestxt"><code>fare_leg_rules.txt</code></a>.</p><p xmlns="http://www.w3.org/1999/xhtml">To process the cost of a multi-leg journey:</p><ol xmlns="http://www.w3.org/1999/xhtml">
   <li>The applicable fare leg groups defined in
	  <code>fare_leg_rules.txt</code> should be determined for all
	  individual legs of travel based on the rider’s journey.</li>
   <li>
	  <p>The file <a href="#fare_transfer_rulestxt">fare_transfer_rules.txt</a> must be
		 filtered by the fields that define the characteristics of the
		 transfer, these fields are:</p>
	  <ul>
		 <li><code>fare_transfer_rules.from_leg_group_id</code></li>
		 <li><code>fare_transfer_rules.to_leg_group_id</code><br><br></li>
	  </ul>
   </li>
   <li>
	  <p>If the transfer exactly matches a record in <a href="#fare_transfer_rulestxt">fare_transfer_rules.txt</a> based on the
		 characteristics of the transfer, then that record must be processed
		 to determine the transfer cost.</p>
   </li>
   <li>If no exact matches are found, then empty entries in
	  <code>from_leg_group_id</code> or in <code>to_leg_group_id</code>
	  must be checked to process the transfer cost:

	  <ul>
		 <li>An empty entry in
			<code>fare_transfer_rules.from_leg_group_id</code> corresponds to
			all leg groups defined under
			<code>fare_leg_rules.leg_group_id</code> excluding the ones listed
			under <code>fare_transfer_rules.from_leg_group_id</code></li>
		 <li>An empty entry in
			<code>fare_transfer_rules.to_leg_group_id</code> corresponds to all
			leg groups defined under <code>fare_leg_rules.leg_group_id</code>
			excluding the ones listed under
			<code>fare_transfer_rules.to_leg_group_id</code><br><br></li>
	  </ul>
   </li>
   <li>If the transfer does not match any of the rules described
	  above, then there is no transfer arrangement and the legs are
	  considered separate.</li>
</ol><p xmlns="http://www.w3.org/1999/xhtml"><br></p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FareTransferRules {
	/** Record: __Optional__

	Identifies a group of pre-transfer fare leg rules.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If there are no matching
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.from_leg_group_id</code> values to the
	<code xmlns="http://www.w3.org/1999/xhtml">leg_group_id</code> being filtered, empty
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.from_leg_group_id</code> will be matched
	by default.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	An empty entry in
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.from_leg_group_id</code> corresponds to
	all leg groups defined under
	<code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.leg_group_id</code> excluding the ones listed
	under <code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.from_leg_group_id</code>
		 */
	pub from_leg_group_id: Option<LegGroupId>,

	/** Record: __Optional__

	Identifies a group of post-transfer fare leg rules.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If there are no matching
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.to_leg_group_id</code> values to the
	<code xmlns="http://www.w3.org/1999/xhtml">leg_group_id</code> being filtered, empty
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.to_leg_group_id</code> will be matched by
	default.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	An empty entry in <code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.to_leg_group_id</code>
	corresponds to all leg groups defined under
	<code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.leg_group_id</code> excluding the ones listed
	under <code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.to_leg_group_id</code>
		 */
	pub to_leg_group_id: Option<LegGroupId>,

	/** Record: __Conditionally Forbidden__

	Defines how many consecutive transfers the transfer rule may be
	applied to.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">-1</code> - No limit.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> or more - Defines how many transfers the transfer
	rule may span.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If a sub-journey matches multiple records with different
	<code xmlns="http://www.w3.org/1999/xhtml">transfer_count</code>s, then the rule with the minimum
	<code xmlns="http://www.w3.org/1999/xhtml">transfer_count</code> that is greater than or equal to the
	current transfer count of the sub-journey is to be selected.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Forbidden:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Forbidden</strong> if
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.from_leg_group_id</code> does not equal
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.to_leg_group_id</code>.<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> if
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.from_leg_group_id</code> equals
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.to_leg_group_id</code>.
		 */
	pub transfer_count: Option<NonZeroInteger>,

	/** Record: __Optional__

	Defines the duration limit of the transfer.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Must be expressed in integer increments of seconds.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If there is no duration limit,
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.duration_limit</code> must be empty.
		 */
	pub duration_limit: Option<PositiveInteger>,

	/** Record: __Conditionally Required__

	Defines the relative start and end of
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.duration_limit</code>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> - Between the departure fare validation of the
	current leg and the arrival fare validation of the next leg.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Between the departure fare validation of the
	current leg and the departure fare validation of the next
	leg.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - Between the arrival fare validation of the current
	leg and the departure fare validation of the next leg.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">3</code> - Between the arrival fare validation of the current
	leg and the arrival fare validation of the next leg.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> if
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.duration_limit</code> is defined.<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Forbidden</strong> if
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.duration_limit</code> is empty.
		 */
	pub duration_limit_type: Option<GtfsEnum>,

	/** Record: __Required__

	Indicates the cost processing method of transferring between
	legs in a journey:<br xmlns="http://www.w3.org/1999/xhtml"><img xmlns="http://www.w3.org/1999/xhtml" alt="" src="../../assets/2-leg.svg"><br xmlns="http://www.w3.org/1999/xhtml">
	Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> - From-leg
	<code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.fare_product_id</code> plus
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.fare_product_id</code>; A + AB.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - From-leg
	<code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.fare_product_id</code> plus
	<code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.fare_product_id</code> plus to-leg
	<code xmlns="http://www.w3.org/1999/xhtml">fare_leg_rules.fare_product_id</code>; A + AB + B.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - <code xmlns="http://www.w3.org/1999/xhtml">fare_transfer_rules.fare_product_id</code>;
	AB.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Cost processing interactions between multiple transfers in a
	journey:<br xmlns="http://www.w3.org/1999/xhtml"><img xmlns="http://www.w3.org/1999/xhtml" alt="" src="../../assets/3-leg.svg"><br xmlns="http://www.w3.org/1999/xhtml"><table xmlns="http://www.w3.org/1999/xhtml">
	   <thead>
		  <tr>
			 <th><code>fare_transfer_type</code></th>
			 <th>Processing A &gt; B</th>
			 <th>Processing B &gt; C</th>
		  </tr>
	   </thead>
	   <tbody>
		  <tr>
			 <td><code>0</code></td>
			 <td>A + AB</td>
			 <td>S + BC</td>
		  </tr>
		  <tr>
			 <td><code>1</code></td>
			 <td>A + AB +B</td>
			 <td>S + BC + C</td>
		  </tr>
		  <tr>
			 <td><code>2</code></td>
			 <td>AB</td>
			 <td>S + BC</td>
		  </tr>
	   </tbody>
	</table>
	Where S indicates the total processed cost of the preceding leg(s)
	and transfer(s).
		 */
	pub fare_transfer_type: GtfsEnum,

	/** Record: __Optional__

	The fare product required to transfer between two fare legs. If
	empty, the cost of the transfer rule is 0.
		 */
	pub fare_product_id: Option<FareProductId>,
}

/** `areas.txt`

Area grouping of locations.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Optional</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>area_id</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">Defines area identifiers.</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Areas {
	/** Record: __Required__

	Identifies an area. Must be unique in <a xmlns="http://www.w3.org/1999/xhtml" href="#areastxt">areas.txt</a>.
		 */
	pub area_id: AreaUid,

	/** Record: __Optional__

	The name of the area as displayed to the rider.
		 */
	pub area_name: Option<Text>,
}

/** `stop_areas.txt`

Rules to assign stops to areas.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Optional</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>*</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">Assigns stops from <a href="#stopstxt">stops.txt</a> to
   areas.</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StopAreas {
	/** Record: __Required__

	Identifies an area to which one or multiple
	<code xmlns="http://www.w3.org/1999/xhtml">stop_id</code>s belong. The same <code xmlns="http://www.w3.org/1999/xhtml">stop_id</code> may be
	defined in many <code xmlns="http://www.w3.org/1999/xhtml">area_id</code>s.
		 */
	pub area_id: AreaUid,

	/** Record: __Required__

	Identifies a stop. If a station (i.e. a stop with
	<code xmlns="http://www.w3.org/1999/xhtml">stops.location_type=1</code>) is defined in this field, it is
	assumed that all of its platforms (i.e. all stops with
	<code xmlns="http://www.w3.org/1999/xhtml">stops.location_type=0</code> that have this station defined
	as <code xmlns="http://www.w3.org/1999/xhtml">stops.parent_station</code>) are part of the same area.
	This behavior can be overridden by assigning platforms to other
	areas.
		 */
	pub stop_id: StopUid,
}

/** `networks.txt`

Network grouping of routes.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
Conditionally Forbidden:<br xmlns="http://www.w3.org/1999/xhtml">
- <strong xmlns="http://www.w3.org/1999/xhtml">Forbidden</strong> if <code xmlns="http://www.w3.org/1999/xhtml">network_id</code> exists in
<a xmlns="http://www.w3.org/1999/xhtml" href="#routestxt">routes.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml">
- Optional otherwise.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Conditionally Forbidden</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>network_id</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">Defines network identifiers that apply for fare leg rules.</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Networks {
	/** Record: __Required__

	Identifies a network. Must be unique in <a xmlns="http://www.w3.org/1999/xhtml" href="#networkstxt">networks.txt</a>.
		 */
	pub network_id: NetworkUid,

	/** Record: __Optional__

	The name of the network that apply for fare leg rules, as used
	by the local agency and its riders.
		 */
	pub network_name: Option<Text>,
}

/** `route_networks.txt`

Rules to assign routes to networks.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
Conditionally Forbidden:<br xmlns="http://www.w3.org/1999/xhtml">
- <strong xmlns="http://www.w3.org/1999/xhtml">Forbidden</strong> if <code xmlns="http://www.w3.org/1999/xhtml">network_id</code> exists in
<a xmlns="http://www.w3.org/1999/xhtml" href="#routestxt">routes.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml">
- Optional otherwise.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Conditionally Forbidden</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>route_id</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">Assigns routes from <a href="#routestxt">routes.txt</a> to
   networks.</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RouteNetworks {
	/** Record: __Required__

	Identifies a network to which one or multiple
	<code xmlns="http://www.w3.org/1999/xhtml">route_id</code>s belong. A <code xmlns="http://www.w3.org/1999/xhtml">route_id</code> can only be
	defined in one <code xmlns="http://www.w3.org/1999/xhtml">network_id</code>.
		 */
	pub network_id: NetworkUid,

	/** Record: __Required__

	Identifies a route.
		 */
	pub route_id: RouteUid,
}

/** `shapes.txt`

Rules for mapping vehicle travel paths, sometimes referred to
as route alignments.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Optional</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>shape_id</code>,
   <code>shape_pt_sequence</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">Shapes describe the path that a vehicle travels along a route
   alignment, and are defined in the file shapes.txt. Shapes are
   associated with Trips, and consist of a sequence of points through
   which the vehicle passes in order. Shapes do not need to intercept
   the location of Stops exactly, but all Stops on a trip should lie
   within a small distance of the shape for that trip, i.e. close to
   straight line segments connecting the shape points.</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Shapes {
	/** Record: __Required__

	Identifies a shape.
		 */
	pub shape_id: ShapeId,

	/** Record: __Required__

	Latitude of a shape point. Each record in <a xmlns="http://www.w3.org/1999/xhtml" href="#shapestxt">shapes.txt</a> represents a shape point used to define
	the shape.
		 */
	pub shape_pt_lat: Latitude,

	/** Record: __Required__

	Longitude of a shape point.
		 */
	pub shape_pt_lon: Longitude,

	/** Record: __Required__

	Sequence in which the shape points connect to form the shape.
	Values must increase along the trip but do not need to be
	consecutive.
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: If the shape "A_shp" has three points in its
	   definition, the <a href="#shapestxt">shapes.txt</a> file might
	   contain these records to define the shape:</em><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">shape_id,shape_pt_lat,shape_pt_lon,shape_pt_sequence</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">A_shp,37.61956,-122.48161,0</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">A_shp,37.64430,-122.41070,6</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">A_shp,37.65863,-122.30839,11</code>
		 */
	pub shape_pt_sequence: NonNegativeInteger,

	/** Record: __Optional__

	Actual distance traveled along the shape from the first shape
	point to the point specified in this record. Used by trip planners
	to show the correct portion of the shape on a map. Values must
	increase along with <code xmlns="http://www.w3.org/1999/xhtml">shape_pt_sequence</code>; they must not
	be used to show reverse travel along a route. Distance units must
	be consistent with those used in <a xmlns="http://www.w3.org/1999/xhtml" href="#stop_timestxt">stop_times.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Recommended for routes that have looping or inlining (the vehicle
	crosses or travels over the same portion of alignment in one
	trip).<br xmlns="http://www.w3.org/1999/xhtml"><img xmlns="http://www.w3.org/1999/xhtml" src="inlining.svg" width="200px" style="display: block; margin-left: auto; margin-right: auto;"><br xmlns="http://www.w3.org/1999/xhtml">
	If a vehicle retraces or crosses the route alignment at points in
	the course of a trip, <code xmlns="http://www.w3.org/1999/xhtml">shape_dist_traveled</code> is important
	to clarify how portions of the points in <a xmlns="http://www.w3.org/1999/xhtml" href="#shapestxt">shapes.txt</a> line up correspond with records in
	<a xmlns="http://www.w3.org/1999/xhtml" href="#stop_timestxt">stop_times.txt</a>.
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: If a bus travels along the three points defined above
	   for A_shp, the additional <code>shape_dist_traveled</code> values
	   (shown here in kilometers) would look like this:</em><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">shape_id,shape_pt_lat,shape_pt_lon,shape_pt_sequence,shape_dist_traveled</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">A_shp,37.61956,-122.48161,0,0</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">A_shp,37.64430,-122.41070,6,6.8310</code><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">A_shp,37.65863,-122.30839,11,15.8765</code>
		 */
	pub shape_dist_traveled: Option<NonNegativeFloat>,
}

/** `frequencies.txt`

Headway (time between trips) for headway-based service or a
compressed representation of fixed-schedule service.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Optional</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>trip_id</code>, <code>start_time</code>)</p><p xmlns="http://www.w3.org/1999/xhtml"><a href="#frequenciestxt">Frequencies.txt</a> represents trips
   that operate on regular headways (time between trips). This file
   may be used to represent two different types of service.</p><ul xmlns="http://www.w3.org/1999/xhtml">
   <li>Frequency-based service
	  (<code>exact_times</code>=<code>0</code>) in which service does not
	  follow a fixed schedule throughout the day. Instead, operators
	  attempt to strictly maintain predetermined headways for trips.</li>
   <li>A compressed representation of schedule-based service
	  (<code>exact_times</code>=<code>1</code>) that has the exact same
	  headway for trips over specified time period(s). In schedule-based
	  service operators try to strictly adhere to a schedule.</li>
</ul>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Frequencies {
	/** Record: __Required__

	Identifies a trip to which the specified headway of service
	applies.
		 */
	pub trip_id: TripUid,

	/** Record: __Required__

	Time at which the first vehicle departs from the first stop of
	the trip with the specified headway.
		 */
	pub start_time: Time,

	/** Record: __Required__

	Time at which service changes to a different headway (or
	ceases) at the first stop in the trip.
		 */
	pub end_time: Time,

	/** Record: __Required__

	Time, in seconds, between departures from the same stop
	(headway) for the trip, during the time interval specified by
	<code xmlns="http://www.w3.org/1999/xhtml">start_time</code> and <code xmlns="http://www.w3.org/1999/xhtml">end_time</code>. Multiple
	headways may be defined for the same trip, but must not overlap.
	New headways may start at the exact time the previous headway
	ends.
		 */
	pub headway_secs: PositiveInteger,

	/** Record: __Optional__

	Indicates the type of service for a trip. See the file
	description for more information. Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> or empty - Frequency-based trips.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Schedule-based trips with the exact same headway
	throughout the day. In this case the <code xmlns="http://www.w3.org/1999/xhtml">end_time</code> value
	must be greater than the last desired trip <code xmlns="http://www.w3.org/1999/xhtml">start_time</code>
	but less than the last desired trip start_time +
	<code xmlns="http://www.w3.org/1999/xhtml">headway_secs</code>.
		 */
	pub exact_times: Option<GtfsEnum>,
}

/** `transfers.txt`

Rules for making connections at transfer points between
routes.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Optional</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>from_stop_id</code>, <code>to_stop_id</code>,
   <code>from_trip_id</code>, <code>to_trip_id</code>,
   <code>from_route_id</code>, <code>to_route_id</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">When calculating an itinerary, GTFS-consuming applications
   interpolate transfers based on allowable time and stop proximity.
   <a href="#transferstxt">Transfers.txt</a> specifies additional
   rules and overrides for selected transfers.</p><p xmlns="http://www.w3.org/1999/xhtml">Fields <code>from_trip_id</code>, <code>to_trip_id</code>,
   <code>from_route_id</code> and <code>to_route_id</code> allow
   higher orders of specificity for transfer rules. Along with
   <code>from_stop_id</code> and <code>to_stop_id</code>, the ranking
   of specificity is as follows:</p><ol xmlns="http://www.w3.org/1999/xhtml">
   <li>Both <code>trip_id</code>s defined: <code>from_trip_id</code>
	  and <code>to_trip_id</code>.</li>
   <li>One <code>trip_id</code> and <code>route_id</code> set defined:
	  (<code>from_trip_id</code> and <code>to_route_id</code>) or
	  (<code>from_route_id</code> and <code>to_trip_id</code>).</li>
   <li>One <code>trip_id</code> defined: <code>from_trip_id</code> or
	  <code>to_trip_id</code>.</li>
   <li>Both <code>route_id</code>s defined: <code>from_route_id</code>
	  and <code>to_route_id</code>.</li>
   <li>One <code>route_id</code> defined: <code>from_route_id</code>
	  or <code>to_route_id</code>.</li>
   <li>Only <code>from_stop_id</code> and <code>to_stop_id</code>
	  defined: no route or trip related fields set.</li>
</ol><p xmlns="http://www.w3.org/1999/xhtml">For a given ordered pair of arriving trip and departing trip,
   the transfer with the greatest specificity that applies between
   these two trips is chosen. For any pair of trips, there should not
   be two transfers with equally maximal specificity that could
   apply.</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Transfers {
	/** Record: __Conditionally Required__

	Identifies a stop or station where a connection between routes
	begins. If this field refers to a station, the transfer rule
	applies to all its child stops. Refering to a station is forbiden
	for <code xmlns="http://www.w3.org/1999/xhtml">transfer_types</code> 4 and 5.
		 */
	pub from_stop_id: Option<StopUid>,

	/** Record: __Conditionally Required__

	Identifies a stop or station where a connection between routes
	ends. If this field refers to a station, the transfer rule applies
	to all child stops. Refering to a station is forbiden for
	<code xmlns="http://www.w3.org/1999/xhtml">transfer_types</code> 4 and 5.
		 */
	pub to_stop_id: Option<StopUid>,

	/** Record: __Optional__

	Identifies a route where a connection begins.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If <code xmlns="http://www.w3.org/1999/xhtml">from_route_id</code> is defined, the transfer will apply
	to the arriving trip on the route for the given
	<code xmlns="http://www.w3.org/1999/xhtml">from_stop_id</code>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If both <code xmlns="http://www.w3.org/1999/xhtml">from_trip_id</code> and <code xmlns="http://www.w3.org/1999/xhtml">from_route_id</code>
	are defined, the <code xmlns="http://www.w3.org/1999/xhtml">trip_id</code> must belong to the
	<code xmlns="http://www.w3.org/1999/xhtml">route_id</code>, and <code xmlns="http://www.w3.org/1999/xhtml">from_trip_id</code> will take
	precedence.
		 */
	pub from_route_id: Option<RouteUid>,

	/** Record: __Optional__

	Identifies a route where a connection ends.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If <code xmlns="http://www.w3.org/1999/xhtml">to_route_id</code> is defined, the transfer will apply to
	the departing trip on the route for the given
	<code xmlns="http://www.w3.org/1999/xhtml">to_stop_id</code>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If both <code xmlns="http://www.w3.org/1999/xhtml">to_trip_id</code> and <code xmlns="http://www.w3.org/1999/xhtml">to_route_id</code> are
	defined, the <code xmlns="http://www.w3.org/1999/xhtml">trip_id</code> must belong to the
	<code xmlns="http://www.w3.org/1999/xhtml">route_id</code>, and <code xmlns="http://www.w3.org/1999/xhtml">to_trip_id</code> will take
	precedence.
		 */
	pub to_route_id: Option<RouteUid>,

	/** Record: __Conditionally Required__

	Identifies a trip where a connection between routes
	begins.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If <code xmlns="http://www.w3.org/1999/xhtml">from_trip_id</code> is defined, the transfer will apply to
	the arriving trip for the given <code xmlns="http://www.w3.org/1999/xhtml">from_stop_id</code>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If both <code xmlns="http://www.w3.org/1999/xhtml">from_trip_id</code> and <code xmlns="http://www.w3.org/1999/xhtml">from_route_id</code>
	are defined, the <code xmlns="http://www.w3.org/1999/xhtml">trip_id</code> must belong to the
	<code xmlns="http://www.w3.org/1999/xhtml">route_id</code>, and <code xmlns="http://www.w3.org/1999/xhtml">from_trip_id</code> will take
	precedence. REQUIRED if <code xmlns="http://www.w3.org/1999/xhtml">transfer_type</code> is
	<code xmlns="http://www.w3.org/1999/xhtml">4</code> or <code xmlns="http://www.w3.org/1999/xhtml">5</code>.
		 */
	pub from_trip_id: Option<TripUid>,

	/** Record: __Conditionally Required__

	Identifies a trip where a connection between routes ends.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If <code xmlns="http://www.w3.org/1999/xhtml">to_trip_id</code> is defined, the transfer will apply to
	the departing trip for the given <code xmlns="http://www.w3.org/1999/xhtml">to_stop_id</code>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If both <code xmlns="http://www.w3.org/1999/xhtml">to_trip_id</code> and <code xmlns="http://www.w3.org/1999/xhtml">to_route_id</code> are
	defined, the <code xmlns="http://www.w3.org/1999/xhtml">trip_id</code> must belong to the
	<code xmlns="http://www.w3.org/1999/xhtml">route_id</code>, and <code xmlns="http://www.w3.org/1999/xhtml">to_trip_id</code> will take
	precedence. REQUIRED if <code xmlns="http://www.w3.org/1999/xhtml">transfer_type</code> is
	<code xmlns="http://www.w3.org/1999/xhtml">4</code> or <code xmlns="http://www.w3.org/1999/xhtml">5</code>.
		 */
	pub to_trip_id: Option<TripUid>,

	/** Record: __Required__

	Indicates the type of connection for the specified
	(<code xmlns="http://www.w3.org/1999/xhtml">from_stop_id</code>, <code xmlns="http://www.w3.org/1999/xhtml">to_stop_id</code>) pair. Valid
	options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> or empty - Recommended transfer point between
	routes.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Timed transfer point between two routes. The
	departing vehicle is expected to wait for the arriving one and
	leave sufficient time for a rider to transfer between routes.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - Transfer requires a minimum amount of time between
	arrival and departure to ensure a connection. The time required to
	transfer is specified by <code xmlns="http://www.w3.org/1999/xhtml">min_transfer_time</code>.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">3</code> - Transfers are not possible between routes at the
	location.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">4</code> - Passengers can transfer from one trip to another
	by staying onboard the same vehicle (an "in-seat transfer"). More
	details about this type of transfer <a xmlns="http://www.w3.org/1999/xhtml" href="#linked-trips">below</a>.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">5</code> - In-seat transfers are not allowed between
	sequential trips. The passenger must alight from the vehicle and
	re-board. More details about this type of transfer <a xmlns="http://www.w3.org/1999/xhtml" href="#linked-trips">below</a>.
		 */
	pub transfer_type: GtfsEnum,

	/** Record: __Optional__

	Amount of time, in seconds, that must be available to permit a
	transfer between routes at the specified stops. The
	<code xmlns="http://www.w3.org/1999/xhtml">min_transfer_time</code> should be sufficient to permit a
	typical rider to move between the two stops, including buffer time
	to allow for schedule variance on each route.
		 */
	pub min_transfer_time: Option<NonNegativeInteger>,
}

/** `pathways.txt`

Pathways linking together locations within stations.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Optional</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>pathway_id</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">Files <a href="#pathwaystxt">pathways.txt</a> and <a href="levelstxt">levels.txt</a> use a graph representation to describe
   subway or train stations, with nodes representing locations and
   edges representing pathways.</p><p xmlns="http://www.w3.org/1999/xhtml">To navigate from the station entrance/exit (a node represented
   as a location with <code>location_type=2</code>) to a platform (a
   node represented as a location with <code>location_type=0</code> or
   empty), the rider will move through walkways, fare gates, stairs,
   and other edges represented as pathways. Generic nodes (nodes
   represented with <code>location_type=3</code>) can be used to
   connect pathways throughout a station.</p><p xmlns="http://www.w3.org/1999/xhtml">Pathways must be defined exhaustively in a station. If any
   pathways are defined, it is assumed that all pathways throughout
   the station are represented. Therefore, the following guidelines
   apply:</p><ul xmlns="http://www.w3.org/1999/xhtml">
   <li>No dangling locations: If any location within a station has a
	  pathway, then all locations within that station should have
	  pathways, except for platforms that have boarding areas
	  (<code>location_type=4</code>, see guideline below).</li>
   <li>No pathways for a platform with boarding areas: A platform
	  (<code>location_type=0</code> or empty) that has boarding areas
	  (<code>location_type=4</code>) is treated as a parent object, not a
	  point. In such cases, the platform must not have pathways assigned.
	  All pathways should be assigned for each of the platform's boarding
	  areas.</li>
   <li>No locked platforms: Each platform
	  (<code>location_type=0</code> or empty) or boarding area
	  (<code>location_type=4</code>) must be connected to at least one
	  entrance/exit (<code>location_type=2</code>) via some chain of
	  pathways. Stations not allowing a pathway to the outside of the
	  station from a given platform are rare.</li>
</ul>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pathways {
	/** Record: __Required__

	Identifies a pathway. Used by systems as an internal identifier
	for the record. Must be unique in the dataset.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Different pathways may have the same values for
	<code xmlns="http://www.w3.org/1999/xhtml">from_stop_id</code> and <code xmlns="http://www.w3.org/1999/xhtml">to_stop_id</code>.
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: When two escalators are side-by-side in opposite
	   directions, or when a stair set and elevator go from the same place
	   to the same place, different <code>pathway_id</code> may have the
	   same <code>from_stop_id</code> and <code>to_stop_id</code>
	   values.</em>
		 */
	pub pathway_id: PathwayUid,

	/** Record: __Required__

	Location at which the pathway begins.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Must contain a <code xmlns="http://www.w3.org/1999/xhtml">stop_id</code> that identifies a platform
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=0</code> or empty), entrance/exit
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=2</code>), generic node
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=3</code>) or boarding area
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=4</code>).<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Values for <code xmlns="http://www.w3.org/1999/xhtml">stop_id</code> that identify stations
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=1</code>) are forbidden.
		 */
	pub from_stop_id: StopUid,

	/** Record: __Required__

	Location at which the pathway ends.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Must contain a <code xmlns="http://www.w3.org/1999/xhtml">stop_id</code> that identifies a platform
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=0</code> or empty), entrance/exit
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=2</code>), generic node
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=3</code>) or boarding area
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=4</code>).<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Values for <code xmlns="http://www.w3.org/1999/xhtml">stop_id</code> that identify stations
	(<code xmlns="http://www.w3.org/1999/xhtml">location_type=1</code>) are forbidden.
		 */
	pub to_stop_id: StopUid,

	/** Record: __Required__

	Type of pathway between the specified
	(<code xmlns="http://www.w3.org/1999/xhtml">from_stop_id</code>, <code xmlns="http://www.w3.org/1999/xhtml">to_stop_id</code>) pair. Valid
	options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Walkway.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">2</code> - Stairs.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">3</code> - Moving sidewalk/travelator.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">4</code> - Escalator.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">5</code> - Elevator.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">6</code> - Fare gate (or payment gate): A pathway that
	crosses into an area of the station where proof of payment is
	required to cross. Fare gates may separate paid areas of the
	station from unpaid ones, or separate different payment areas
	within the same station from each other. This information can be
	used to avoid routing passengers through stations using shortcuts
	that would require passengers to make unnecessary payments, like
	directing a passenger to walk through a subway platform to reach a
	busway.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">7</code>- Exit gate: A pathway exiting a paid area into an
	unpaid area where proof of payment is not required to cross.
		 */
	pub pathway_mode: GtfsEnum,

	/** Record: __Required__

	Indicates the direction that the pathway can be taken:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> - Unidirectional pathway that can only be used from
	<code xmlns="http://www.w3.org/1999/xhtml">from_stop_id</code> to <code xmlns="http://www.w3.org/1999/xhtml">to_stop_id</code>.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Bidirectional pathway that can be used in both
	directions.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Exit gates (<code xmlns="http://www.w3.org/1999/xhtml">pathway_mode=7</code>) must not be
	bidirectional.
		 */
	pub is_bidirectional: GtfsEnum,

	/** Record: __Optional__

	Horizontal length in meters of the pathway from the origin
	location (defined in <code xmlns="http://www.w3.org/1999/xhtml">from_stop_id</code>) to the destination
	location (defined in <code xmlns="http://www.w3.org/1999/xhtml">to_stop_id</code>).<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	This field is recommended for walkways
	(<code xmlns="http://www.w3.org/1999/xhtml">pathway_mode=1</code>), fare gates
	(<code xmlns="http://www.w3.org/1999/xhtml">pathway_mode=6</code>) and exit gates
	(<code xmlns="http://www.w3.org/1999/xhtml">pathway_mode=7</code>).
		 */
	pub length: Option<NonNegativeFloat>,

	/** Record: __Optional__

	Average time in seconds needed to walk through the pathway from
	the origin location (defined in <code xmlns="http://www.w3.org/1999/xhtml">from_stop_id</code>) to the
	destination location (defined in <code xmlns="http://www.w3.org/1999/xhtml">to_stop_id</code>).<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	This field is recommended for moving sidewalks
	(<code xmlns="http://www.w3.org/1999/xhtml">pathway_mode=3</code>), escalators
	(<code xmlns="http://www.w3.org/1999/xhtml">pathway_mode=4</code>) and elevator
	(<code xmlns="http://www.w3.org/1999/xhtml">pathway_mode=5</code>).
		 */
	pub traversal_time: Option<PositiveInteger>,

	/** Record: __Optional__

	Number of stairs of the pathway.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	A positive <code xmlns="http://www.w3.org/1999/xhtml">stair_count</code> implies that the rider walk up
	from <code xmlns="http://www.w3.org/1999/xhtml">from_stop_id</code> to <code xmlns="http://www.w3.org/1999/xhtml">to_stop_id</code>. And a
	negative <code xmlns="http://www.w3.org/1999/xhtml">stair_count</code> implies that the rider walk down
	from <code xmlns="http://www.w3.org/1999/xhtml">from_stop_id</code> to <code xmlns="http://www.w3.org/1999/xhtml">to_stop_id</code>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	This field is recommended for stairs
	(<code xmlns="http://www.w3.org/1999/xhtml">pathway_mode=2</code>).<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If only an estimated stair count can be provided, it is recommended
	to approximate 15 stairs for 1 floor.
		 */
	pub stair_count: Option<NonNullInteger>,

	/** Record: __Optional__

	Maximum slope ratio of the pathway. Valid options are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> or empty - No slope.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">Float</code> - Slope ratio of the pathway, positive for
	upwards, negative for downwards.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	This field should only be used with walkways
	(<code xmlns="http://www.w3.org/1999/xhtml">pathway_mode=1</code>) and moving sidewalks
	(<code xmlns="http://www.w3.org/1999/xhtml">pathway_mode=3</code>).
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: In the US, 0.083 (also written 8.3%) is the maximum
	   slope ratio for hand-propelled wheelchair, which mean an increase
	   of 0.083m (so 8.3cm) for each 1m.</em>
		 */
	pub max_slope: Option<Float>,

	/** Record: __Optional__

	Minimum width of the pathway in meters.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	This field is recommended if the minimum width is less than 1
	meter.
		 */
	pub min_width: Option<PositiveFloat>,

	/** Record: __Optional__

	Public facing text from physical signage that is visible to
	riders.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	May be used to provide text directions to riders, such as 'follow
	signs to '. The text in <code xmlns="http://www.w3.org/1999/xhtml">singposted_as</code> should appear
	exactly how it is printed on the signs.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	When the physical signage is multilingual, this field may be
	populated and translated following the example of
	<code xmlns="http://www.w3.org/1999/xhtml">stops.stop_name</code> in the field definition of
	<code xmlns="http://www.w3.org/1999/xhtml">feed_info.feed_lang</code>.
		 */
	pub signposted_as: Option<Text>,

	/** Record: __Optional__

	Same as <code xmlns="http://www.w3.org/1999/xhtml">signposted_as</code>, but when the pathway is
	used from the <code xmlns="http://www.w3.org/1999/xhtml">to_stop_id</code> to the
	<code xmlns="http://www.w3.org/1999/xhtml">from_stop_id</code>.
		 */
	pub reversed_signposted_as: Option<Text>,
}

/** `levels.txt`

Levels within stations.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> when describing pathways with elevators
(<code xmlns="http://www.w3.org/1999/xhtml">pathway_mode=5</code>).<br xmlns="http://www.w3.org/1999/xhtml">
- Optional otherwise.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Conditionally Required</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>level_id</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">Describes levels in a station. Useful in conjunction with
   <a href="#pathwaystxt">pathways.txt</a>.</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Levels {
	/** Record: __Required__

	Identifies a level in a station.
		 */
	pub level_id: LevelUid,

	/** Record: __Required__

	Numeric index of the level that indicates its relative
	position.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Ground level should have index <code xmlns="http://www.w3.org/1999/xhtml">0</code>, with levels above
	ground indicated by positive indices and levels below ground by
	negative indices.
		 */
	pub level_index: Float,

	/** Record: __Optional__

	Name of the level as seen by the rider inside the building or
	station.
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: Take the elevator to "Mezzanine" or "Platform" or
	   "-1".</em>
		 */
	pub level_name: Option<Text>,
}

/** `translations.txt`

Translations of customer-facing dataset values.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Optional</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>table_name</code>, <code>field_name</code>,
   <code>language</code>, <code>record_id</code>,
   <code>record_sub_id</code>, <code>field_value</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">In regions that have multiple official languages, transit
   agencies/operators typically have language-specific names and web
   pages. In order to best serve riders in those regions, it is useful
   for the dataset to include these language-dependent values.</p><p xmlns="http://www.w3.org/1999/xhtml">If both referencing methods (<code>record_id</code>,
   <code>record_sub_id</code>) and <code>field_value</code> are used
   to translate the same value in 2 different rows, the translation
   provided with (<code>record_id</code>, <code>record_sub_id</code>)
   takes precedence.</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Translations {
	/** Record: __Required__

	Defines the table that contains the field to be translated.
	Allowed values are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">agency</code><br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">stops</code><br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">routes</code><br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">trips</code><br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">stop_times</code><br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">pathways</code><br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">levels</code><br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">feed_info</code><br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">attributions</code><br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Any file added to GTFS will have a <code xmlns="http://www.w3.org/1999/xhtml">table_name</code> value
	equivalent to the file name, as listed above (i.e., not including
	the <code xmlns="http://www.w3.org/1999/xhtml">.txt</code> file extension).
		 */
	pub table_name: GtfsEnum,

	/** Record: __Required__

	Name of the field to be translated. Fields with type
	<code xmlns="http://www.w3.org/1999/xhtml">Text</code> may be translated, fields with type
	<code xmlns="http://www.w3.org/1999/xhtml">URL</code>, <code xmlns="http://www.w3.org/1999/xhtml">Email</code> and <code xmlns="http://www.w3.org/1999/xhtml">Phone number</code>
	may also be “translated” to provide resources in the correct
	language. Fields with other types should not be translated.
		 */
	pub field_name: Text,

	/** Record: __Required__

	Language of translation.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If the language is the same as in <code xmlns="http://www.w3.org/1999/xhtml">feed_info.feed_lang</code>,
	the original value of the field will be assumed to be the default
	value to use in languages without specific translations (if
	<code xmlns="http://www.w3.org/1999/xhtml">default_lang</code> doesn't specify otherwise).
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: In Switzerland, a city in an officially bilingual
	   canton is officially called “Biel/Bienne”, but would simply be
	   called “Bienne” in French and “Biel” in German.</em>
		 */
	pub language: LanguageCode,

	/** Record: __Required__

	Translated value.
		 */
	pub translation: TranslationValue,

	/** Record: __Conditionally Required__

	Defines the record that corresponds to the field to be
	translated. The value in <code xmlns="http://www.w3.org/1999/xhtml">record_id</code> must be the first
	or only field of a table's primary key, as defined in the primary
	key attribute for each table and below:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">agency_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#agencytxt">agency.txt</a><br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">stop_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#stopstxt">stops.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">route_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#routestxt">routes.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">trip_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#tripstxt">trips.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">trip_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#stop_timestxt">stop_times.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">pathway_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#pathwaystxt">pathways.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">level_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#levelstxt">levels.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">attribution_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#attributionstxt">attributions.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Fields in tables not defined above should not be translated.
	However producers sometimes add extra fields that are outside the
	official specification and these unofficial fields may be
	translated. Below is the recommended way to use
	<code xmlns="http://www.w3.org/1999/xhtml">record_id</code> for those tables:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">service_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#calendartxt">calendar.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">service_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#calendar_datestxt">calendar_dates.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">fare_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_attributestxt">fare_attributes.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">fare_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rulestxt">fare_rules.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">shape_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#shapestxt">shapes.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">trip_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#frequenciestxt">frequencies.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">from_stop_id</code> for <code xmlns="http://www.w3.org/1999/xhtml">transfers.txt</code>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Forbidden</strong> if <code xmlns="http://www.w3.org/1999/xhtml">table_name</code> is
	<code xmlns="http://www.w3.org/1999/xhtml">feed_info</code>.<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Forbidden</strong> if <code xmlns="http://www.w3.org/1999/xhtml">field_value</code> is
	defined.<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> if <code xmlns="http://www.w3.org/1999/xhtml">field_value</code> is
	empty.
		 */
	pub record_id: Option<RecordId>,

	/** Record: __Conditionally Required__

	Helps the record that contains the field to be translated when
	the table doesn’t have a unique ID. Therefore, the value in
	<code xmlns="http://www.w3.org/1999/xhtml">record_sub_id</code> is the secondary ID of the table, as
	defined by the table below:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	- None for <a xmlns="http://www.w3.org/1999/xhtml" href="#agencytxt">agency.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- None for <a xmlns="http://www.w3.org/1999/xhtml" href="#stopstxt">stops.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- None for <a xmlns="http://www.w3.org/1999/xhtml" href="#routestxt">routes.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- None for <a xmlns="http://www.w3.org/1999/xhtml" href="#tripstxt">trips.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">stop_sequence</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#stop_timestxt">stop_times.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- None for <a xmlns="http://www.w3.org/1999/xhtml" href="#pathwaystxt">pathways.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- None for <a xmlns="http://www.w3.org/1999/xhtml" href="#levelstxt">levels.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- None for <a xmlns="http://www.w3.org/1999/xhtml" href="#attributionstxt">attributions.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Fields in tables not defined above should not be translated.
	However producers sometimes add extra fields that are outside the
	official specification and these unofficial fields may be
	translated. Below is the recommended way to use
	<code xmlns="http://www.w3.org/1999/xhtml">record_sub_id</code> for those tables:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	- None for <a xmlns="http://www.w3.org/1999/xhtml" href="#calendartxt">calendar.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">date</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#calendar_datestxt">calendar_dates.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- None for <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_attributestxt">fare_attributes.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">route_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#fare_rulestxt">fare_rules.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- None for <a xmlns="http://www.w3.org/1999/xhtml" href="#shapestxt">shapes.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">start_time</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#frequenciestxt">frequencies.txt</a>;<br xmlns="http://www.w3.org/1999/xhtml">
	- <code xmlns="http://www.w3.org/1999/xhtml">to_stop_id</code> for <a xmlns="http://www.w3.org/1999/xhtml" href="#transferstxt">transfers.txt</a>.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Forbidden</strong> if <code xmlns="http://www.w3.org/1999/xhtml">table_name</code> is
	<code xmlns="http://www.w3.org/1999/xhtml">feed_info</code>.<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Forbidden</strong> if <code xmlns="http://www.w3.org/1999/xhtml">field_value</code> is
	defined.<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> if <code xmlns="http://www.w3.org/1999/xhtml">table_name=stop_times</code>
	and <code xmlns="http://www.w3.org/1999/xhtml">record_id</code> is defined.
		 */
	pub record_sub_id: Option<RecordSubId>,

	/** Record: __Conditionally Required__

	Instead of defining which record should be translated by using
	<code xmlns="http://www.w3.org/1999/xhtml">record_id</code> and <code xmlns="http://www.w3.org/1999/xhtml">record_sub_id</code>, this field
	can be used to define the value which should be translated. When
	used, the translation will be applied when the fields identified by
	<code xmlns="http://www.w3.org/1999/xhtml">table_name</code> and <code xmlns="http://www.w3.org/1999/xhtml">field_name</code> contains the
	exact same value defined in field_value.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	The field must have <strong xmlns="http://www.w3.org/1999/xhtml">exactly</strong> the value defined in
	<code xmlns="http://www.w3.org/1999/xhtml">field_value</code>. If only a subset of the value matches
	<code xmlns="http://www.w3.org/1999/xhtml">field_value</code>, the translation won’t be applied.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If two translation rules match the same record (one with
	<code xmlns="http://www.w3.org/1999/xhtml">field_value</code>, and the other one with
	<code xmlns="http://www.w3.org/1999/xhtml">record_id</code>), the rule with <code xmlns="http://www.w3.org/1999/xhtml">record_id</code> takes
	precedence.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	Conditionally Required:<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Forbidden</strong> if <code xmlns="http://www.w3.org/1999/xhtml">table_name</code> is
	<code xmlns="http://www.w3.org/1999/xhtml">feed_info</code>.<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Forbidden</strong> if <code xmlns="http://www.w3.org/1999/xhtml">record_id</code> is
	defined.<br xmlns="http://www.w3.org/1999/xhtml">
	- <strong xmlns="http://www.w3.org/1999/xhtml">Required</strong> if <code xmlns="http://www.w3.org/1999/xhtml">record_id</code> is
	empty.
		 */
	pub field_value: Option<TranslationValue>,
}

/** `feed_info.txt`

Dataset metadata, including publisher, version, and expiration
information.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Recommended</strong> (<strong>Required</strong> if
   <a href="#translations">translations.txt</a> is provided)</p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (none)</p><p xmlns="http://www.w3.org/1999/xhtml">The file contains information about the dataset itself, rather
   than the services that the dataset describes. In some cases, the
   publisher of the dataset is a different entity than any of the
   agencies.</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeedInfo {
	/** Record: __Required__

	Full name of the organization that publishes the dataset. This
	may be the same as one of the <code xmlns="http://www.w3.org/1999/xhtml">agency.agency_name</code>
	values.
		 */
	pub feed_publisher_name: Text,

	/** Record: __Required__

	URL of the dataset publishing organization's website. This may
	be the same as one of the <code xmlns="http://www.w3.org/1999/xhtml">agency.agency_url</code>
	values.
		 */
	pub feed_publisher_url: Url,

	/** Record: __Required__

	Default language used for the text in this dataset. This
	setting helps GTFS consumers choose capitalization rules and other
	language-specific settings for the dataset. The file
	<code xmlns="http://www.w3.org/1999/xhtml">translations.txt</code> can be used if the text needs to be
	translated into languages other than the default one.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	The default language may be multilingual for datasets with the
	original text in multiple languages. In such cases, the
	<code xmlns="http://www.w3.org/1999/xhtml">feed_lang</code> field should contain the language code
	<code xmlns="http://www.w3.org/1999/xhtml">mul</code> defined by the norm ISO 639-2, and a translation
	for each language used in the dataset should be provided in
	<code xmlns="http://www.w3.org/1999/xhtml">translations.txt</code>. If all the original text in the
	dataset is in the same language, then <code xmlns="http://www.w3.org/1999/xhtml">mul</code> should not
	be used.
	<hr xmlns="http://www.w3.org/1999/xhtml"><em xmlns="http://www.w3.org/1999/xhtml">Example: Consider a dataset from a multilingual country like
	   Switzerland, with the original <code>stops.stop_name</code> field
	   populated with stop names in different languages. Each stop name is
	   written according to the dominant language in that stop’s
	   geographic location, e.g. <code>Genève</code> for the
	   French-speaking city of Geneva, <code>Zürich</code> for the
	   German-speaking city of Zurich, and <code>Biel/Bienne</code> for
	   the bilingual city of Biel/Bienne. The dataset
	   <code>feed_lang</code> should be <code>mul</code> and translations
	   would be provided in <code>translations.txt</code>, in German:
	   <code>Genf</code>, <code>Zürich</code> and <code>Biel</code>; in
	   French: <code>Genève</code>, <code>Zurich</code> and
	   <code>Bienne</code>; in Italian: <code>Ginevra</code>,
	   <code>Zurigo</code> and <code>Bienna</code>; and in English:
	   <code>Geneva</code>, <code>Zurich</code> and
	   <code>Biel/Bienne</code>.</em>
		 */
	pub feed_lang: LanguageCode,

	/** Record: __Optional__

	Defines the language that should be used when the data consumer
	doesn’t know the language of the rider. It will often be
	<code xmlns="http://www.w3.org/1999/xhtml">en</code> (English).
		 */
	pub default_lang: Option<LanguageCode>,

	/** Record: __Recommended__

	The dataset provides complete and reliable schedule information
	for service in the period from the beginning of the
	<code xmlns="http://www.w3.org/1999/xhtml">feed_start_date</code> day to the end of the
	<code xmlns="http://www.w3.org/1999/xhtml">feed_end_date</code> day. Both days may be left empty if
	unavailable. The <code xmlns="http://www.w3.org/1999/xhtml">feed_end_date</code> date must not precede
	the <code xmlns="http://www.w3.org/1999/xhtml">feed_start_date</code> date if both are given. It is
	recommended that dataset providers give schedule data outside this
	period to advise of likely future service, but dataset consumers
	should treat it mindful of its non-authoritative status. If
	<code xmlns="http://www.w3.org/1999/xhtml">feed_start_date</code> or <code xmlns="http://www.w3.org/1999/xhtml">feed_end_date</code> extend
	beyond the active calendar dates defined in <a xmlns="http://www.w3.org/1999/xhtml" href="#calendartxt">calendar.txt</a> and <a xmlns="http://www.w3.org/1999/xhtml" href="#calendar_datestxt">calendar_dates.txt</a>, the dataset is making
	an explicit assertion that there is no service for dates within the
	<code xmlns="http://www.w3.org/1999/xhtml">feed_start_date</code> or <code xmlns="http://www.w3.org/1999/xhtml">feed_end_date</code> range
	but not included in the active calendar dates.
		 */
	pub feed_start_date: Option<Date>,

	/** Record: __Recommended__

	(see above)
		 */
	pub feed_end_date: Option<Date>,

	/** Record: __Recommended__

	String that indicates the current version of their GTFS
	dataset. GTFS-consuming applications can display this value to help
	dataset publishers determine whether the latest dataset has been
	incorporated.
		 */
	pub feed_version: Option<Text>,

	/** Record: __Optional__

	Email address for communication regarding the GTFS dataset and
	data publishing practices. <code xmlns="http://www.w3.org/1999/xhtml">feed_contact_email</code> is a
	technical contact for GTFS-consuming applications. Provide customer
	service contact information through <a xmlns="http://www.w3.org/1999/xhtml" href="#agencytxt">agency.txt</a>. It's recommended that at least one of
	<code xmlns="http://www.w3.org/1999/xhtml">feed_contact_email</code> or <code xmlns="http://www.w3.org/1999/xhtml">feed_contact_url</code>
	are provided.
		 */
	pub feed_contact_email: Option<Email>,

	/** Record: __Optional__

	URL for contact information, a web-form, support desk, or other
	tools for communication regarding the GTFS dataset and data
	publishing practices. <code xmlns="http://www.w3.org/1999/xhtml">feed_contact_url</code> is a technical
	contact for GTFS-consuming applications. Provide customer service
	contact information through <a xmlns="http://www.w3.org/1999/xhtml" href="#agencytxt">agency.txt</a>.
	It's recommended that at least one of <code xmlns="http://www.w3.org/1999/xhtml">feed_contact_url</code>
	or <code xmlns="http://www.w3.org/1999/xhtml">feed_contact_email</code> are provided.
		 */
	pub feed_contact_url: Option<Url>,
}

/** `attributions.txt`

Dataset attributions.

## Description
<p xmlns="http://www.w3.org/1999/xhtml">File: <strong>Optional</strong></p><p xmlns="http://www.w3.org/1999/xhtml">Primary key (<code>attribution_id</code>)</p><p xmlns="http://www.w3.org/1999/xhtml">The file defines the attributions applied to the dataset.</p>

 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Attributions {
	/** Record: __Optional__

	Identifies an attribution for the dataset or a subset of it.
	This is mostly useful for translations.
		 */
	pub attribution_id: Option<AttributionUid>,

	/** Record: __Optional__

	Agency to which the attribution applies.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	If one <code xmlns="http://www.w3.org/1999/xhtml">agency_id</code>, <code xmlns="http://www.w3.org/1999/xhtml">route_id</code>, or
	<code xmlns="http://www.w3.org/1999/xhtml">trip_id</code> attribution is defined, the other ones must be
	empty. If none of them is specified, the attribution will apply to
	the whole dataset.
		 */
	pub agency_id: Option<AgencyUid>,

	/** Record: __Optional__

	Functions in the same way as <code xmlns="http://www.w3.org/1999/xhtml">agency_id</code> except the
	attribution applies to a route. Multiple attributions may apply to
	the same route.
		 */
	pub route_id: Option<RouteUid>,

	/** Record: __Optional__

	Functions in the same way as <code xmlns="http://www.w3.org/1999/xhtml">agency_id</code> except the
	attribution applies to a trip. Multiple attributions may apply to
	the same trip.
		 */
	pub trip_id: Option<TripUid>,

	/** Record: __Required__

	Name of the organization that the dataset is attributed
	to.
		 */
	pub organization_name: Text,

	/** Record: __Optional__

	The role of the organization is producer. Valid options
	are:<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">0</code> or empty - Organization doesn’t have this
	role.<br xmlns="http://www.w3.org/1999/xhtml"><code xmlns="http://www.w3.org/1999/xhtml">1</code> - Organization does have this role.<br xmlns="http://www.w3.org/1999/xhtml"><br xmlns="http://www.w3.org/1999/xhtml">
	At least one of the fields <code xmlns="http://www.w3.org/1999/xhtml">is_producer</code>,
	<code xmlns="http://www.w3.org/1999/xhtml">is_operator</code>, or <code xmlns="http://www.w3.org/1999/xhtml">is_authority</code> should be
	set at <code xmlns="http://www.w3.org/1999/xhtml">1</code>.
		 */
	pub is_producer: Option<GtfsEnum>,

	/** Record: __Optional__

	Functions in the same way as <code xmlns="http://www.w3.org/1999/xhtml">is_producer</code> except
	the role of the organization is operator.
		 */
	pub is_operator: Option<GtfsEnum>,

	/** Record: __Optional__

	Functions in the same way as <code xmlns="http://www.w3.org/1999/xhtml">is_producer</code> except
	the role of the organization is authority.
		 */
	pub is_authority: Option<GtfsEnum>,

	/** Record: __Optional__

	URL of the organization.
		 */
	pub attribution_url: Option<Url>,

	/** Record: __Optional__

	Email of the organization.
		 */
	pub attribution_email: Option<Email>,

	/** Record: __Optional__

	Phone number of the organization.
		 */
	pub attribution_phone: Option<PhoneNumber>,
}
