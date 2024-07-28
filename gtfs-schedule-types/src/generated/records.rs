
use crate::field_types::*;

/* Structs */

/** `agency.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Agency {
	/** Record: __Conditionally Required__

	*/
	
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]
	pub agency_id: Option<AgencyUid>,

	/** Record: __Required__

	*/
	
	pub agency_name: Text,

	/** Record: __Required__

	*/
	
	pub agency_url: Url,

	/** Record: __Required__

	*/
	
	pub agency_timezone: Timezone,

	/** Record: __Optional__

	*/
	
	pub agency_lang: Option<LanguageCode>,

	/** Record: __Optional__

	*/
	
	pub agency_phone: Option<PhoneNumber>,

	/** Record: __Optional__

	*/
	
	pub agency_fare_url: Option<Url>,

	/** Record: __Optional__

	*/
	
	pub agency_email: Option<Email>,
}

/** `stops.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stops {
	/** Record: __Required__

	*/
	
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]
	pub stop_id: StopUid,

	/** Record: __Optional__

	*/
	
	pub stop_code: Option<Text>,

	/** Record: __Conditionally Required__

	*/
	
	pub stop_name: Option<Text>,

	/** Record: __Optional__

	*/
	
	pub tts_stop_name: Option<Text>,

	/** Record: __Optional__

	*/
	
	pub stop_desc: Option<Text>,

	/** Record: __Conditionally Required__

	*/
	
	pub stop_lat: Option<Latitude>,

	/** Record: __Conditionally Required__

	*/
	
	pub stop_lon: Option<Longitude>,

	/** Record: __Optional__

	*/
	
	pub zone_id: Option<ZoneId>,

	/** Record: __Optional__

	*/
	
	pub stop_url: Option<Url>,

	/** Record: __Optional__

	*/
	
	pub location_type: Option<GtfsEnum>,

	/** Record: __Conditionally Required__

	*/
	
	pub parent_station: Option<StopUid>,

	/** Record: __Optional__

	*/
	
	pub stop_timezone: Option<Timezone>,

	/** Record: __Optional__

	*/
	
	pub wheelchair_boarding: Option<GtfsEnum>,

	/** Record: __Optional__

	*/
	
	pub level_id: Option<LevelUid>,

	/** Record: __Optional__

	*/
	
	pub platform_code: Option<Text>,
}

/** `routes.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Routes {
	/** Record: __Required__

	*/
	
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]
	pub route_id: RouteUid,

	/** Record: __Conditionally Required__

	*/
	
	pub agency_id: Option<AgencyUid>,

	/** Record: __Conditionally Required__

	*/
	
	pub route_short_name: Option<Text>,

	/** Record: __Conditionally Required__

	*/
	
	pub route_long_name: Option<Text>,

	/** Record: __Optional__

	*/
	
	pub route_desc: Option<Text>,

	/** Record: __Required__

	*/
	
	pub route_type: GtfsEnum,

	/** Record: __Optional__

	*/
	
	pub route_url: Option<Url>,

	/** Record: __Optional__

	*/
	
	pub route_color: Option<Color>,

	/** Record: __Optional__

	*/
	
	pub route_text_color: Option<Color>,

	/** Record: __Optional__

	*/
	
	pub route_sort_order: Option<NonNegativeInteger>,

	/** Record: __Conditionally Forbidden__

	*/
	
	pub continuous_pickup: Option<GtfsEnum>,

	/** Record: __Conditionally Forbidden__

	*/
	
	pub continuous_drop_off: Option<GtfsEnum>,

	/** Record: __Conditionally Forbidden__

	*/
	
	pub network_id: Option<NetworkId>,
}

/** `trips.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Trips {
	/** Record: __Required__

	*/
	
	pub route_id: RouteUid,

	/** Record: __Required__

	*/
	
	pub service_id: ServiceUid,

	/** Record: __Required__

	*/
	
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]
	pub trip_id: TripUid,

	/** Record: __Optional__

	*/
	
	pub trip_headsign: Option<Text>,

	/** Record: __Optional__

	*/
	
	pub trip_short_name: Option<Text>,

	/** Record: __Optional__

	*/
	
	pub direction_id: Option<GtfsEnum>,

	/** Record: __Optional__

	*/
	
	pub block_id: Option<BlockId>,

	/** Record: __Conditionally Required__

	*/
	
	pub shape_id: Option<ShapeId>,

	/** Record: __Optional__

	*/
	
	pub wheelchair_accessible: Option<GtfsEnum>,

	/** Record: __Optional__

	*/
	
	pub bikes_allowed: Option<GtfsEnum>,
}

/** `stop_times.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StopTimes {
	/** Record: __Required__

	*/
	
	pub trip_id: TripUid,

	/** Record: __Conditionally Required__

	*/
	
	pub arrival_time: Option<Time>,

	/** Record: __Conditionally Required__

	*/
	
	pub departure_time: Option<Time>,

	/** Record: __Conditionally Required__

	*/
	
	pub stop_id: Option<StopUid>,

	/** Record: __Conditionally Forbidden__

	*/
	
	pub location_group_id: Option<LocationGroupUid>,

	/** Record: __Required__

	*/
	
	pub stop_sequence: NonNegativeInteger,

	/** Record: __Optional__

	*/
	
	pub stop_headsign: Option<Text>,

	/** Record: __Conditionally Required__

	*/
	
	pub start_pickup_drop_off_window: Option<Time>,

	/** Record: __Conditionally Required__

	*/
	
	pub end_pickup_drop_off_window: Option<Time>,

	/** Record: __Conditionally Forbidden__

	*/
	
	pub pickup_type: Option<GtfsEnum>,

	/** Record: __Conditionally Forbidden__

	*/
	
	pub drop_off_type: Option<GtfsEnum>,

	/** Record: __Conditionally Forbidden__

	*/
	
	pub continuous_pickup: Option<GtfsEnum>,

	/** Record: __Conditionally Forbidden__

	*/
	
	pub continuous_drop_off: Option<GtfsEnum>,

	/** Record: __Optional__

	*/
	
	pub shape_dist_traveled: Option<NonNegativeFloat>,

	/** Record: __Recommended__

	*/
	
	pub timepoint: Option<GtfsEnum>,

	/** Record: __Optional__

	*/
	
	pub pickup_booking_rule_id: Option<BookingRuleUid>,

	/** Record: __Optional__

	*/
	
	pub drop_off_booking_rule_id: Option<BookingRuleUid>,
}

/** `calendar.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Calendar {
	/** Record: __Required__

	*/
	
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]
	pub service_id: ServiceUid,

	/** Record: __Required__

	*/
	
	pub monday: GtfsEnum,

	/** Record: __Required__

	*/
	
	pub tuesday: GtfsEnum,

	/** Record: __Required__

	*/
	
	pub wednesday: GtfsEnum,

	/** Record: __Required__

	*/
	
	pub thursday: GtfsEnum,

	/** Record: __Required__

	*/
	
	pub friday: GtfsEnum,

	/** Record: __Required__

	*/
	
	pub saturday: GtfsEnum,

	/** Record: __Required__

	*/
	
	pub sunday: GtfsEnum,

	/** Record: __Required__

	*/
	
	pub start_date: Date,

	/** Record: __Required__

	*/
	
	pub end_date: Date,
}

/** `calendar_dates.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CalendarDates {
	/** Record: __Required__

	*/
	
	pub service_id: ServiceUid,

	/** Record: __Required__

	*/
	
	pub date: Date,

	/** Record: __Required__

	*/
	
	pub exception_type: GtfsEnum,
}

/** `fare_attributes.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FareAttributes {
	/** Record: __Required__

	*/
	
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]
	pub fare_id: FareUid,

	/** Record: __Required__

	*/
	
	pub price: NonNegativeFloat,

	/** Record: __Required__

	*/
	
	pub currency_type: CurrencyCode,

	/** Record: __Required__

	*/
	
	pub payment_method: GtfsEnum,

	/** Record: __Required__

	*/
	
	pub transfers: GtfsEnum,

	/** Record: __Conditionally Required__

	*/
	
	pub agency_id: Option<AgencyUid>,

	/** Record: __Optional__

	*/
	
	pub transfer_duration: Option<NonNegativeInteger>,
}

/** `fare_rules.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FareRules {
	/** Record: __Required__

	*/
	
	pub fare_id: FareUid,

	/** Record: __Optional__

	*/
	
	pub route_id: Option<RouteUid>,

	/** Record: __Optional__

	*/
	
	pub origin_id: Option<ZoneId>,

	/** Record: __Optional__

	*/
	
	pub destination_id: Option<ZoneId>,

	/** Record: __Optional__

	*/
	
	pub contains_id: Option<ZoneId>,
}

/** `timeframes.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Timeframes {
	/** Record: __Required__

	*/
	
	pub timeframe_group_id: TimeframeGroupId,

	/** Record: __Conditionally Required__

	*/
	
	pub start_time: Option<Time>,

	/** Record: __Conditionally Required__

	*/
	
	pub end_time: Option<Time>,

	/** Record: __Required__

	*/
	
	pub service_id: ServiceUid,
}

/** `fare_media.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FareMedia {
	/** Record: __Required__

	*/
	
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]
	pub fare_media_id: FareMediaUid,

	/** Record: __Optional__

	*/
	
	pub fare_media_name: Option<Text>,

	/** Record: __Required__

	*/
	
	pub fare_media_type: GtfsEnum,
}

/** `fare_products.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FareProducts {
	/** Record: __Required__

	*/
	
	pub fare_product_id: FareProductId,

	/** Record: __Optional__

	*/
	
	pub fare_product_name: Option<Text>,

	/** Record: __Optional__

	*/
	
	pub fare_media_id: Option<FareMediaUid>,

	/** Record: __Required__

	*/
	
	pub amount: CurrencyAmount,

	/** Record: __Required__

	*/
	
	pub currency: CurrencyCode,
}

/** `fare_leg_rules.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FareLegRules {
	/** Record: __Optional__

	*/
	
	pub leg_group_id: Option<LegGroupId>,

	/** Record: __Optional__

	*/
	
	pub network_id: Option<NetworkUid>,

	/** Record: __Optional__

	*/
	
	pub from_area_id: Option<AreaUid>,

	/** Record: __Optional__

	*/
	
	pub to_area_id: Option<AreaUid>,

	/** Record: __Optional__

	*/
	
	pub from_timeframe_group_id: Option<TimeframeGroupId>,

	/** Record: __Optional__

	*/
	
	pub to_timeframe_group_id: Option<TimeframeGroupId>,

	/** Record: __Required__

	*/
	
	pub fare_product_id: FareProductId,

	/** Record: __Optional__

	*/
	
	pub rule_priority: Option<NonNegativeInteger>,
}

/** `fare_transfer_rules.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FareTransferRules {
	/** Record: __Optional__

	*/
	
	pub from_leg_group_id: Option<LegGroupId>,

	/** Record: __Optional__

	*/
	
	pub to_leg_group_id: Option<LegGroupId>,

	/** Record: __Conditionally Forbidden__

	*/
	
	pub transfer_count: Option<NonZeroInteger>,

	/** Record: __Optional__

	*/
	
	pub duration_limit: Option<PositiveInteger>,

	/** Record: __Conditionally Required__

	*/
	
	pub duration_limit_type: Option<GtfsEnum>,

	/** Record: __Required__

	*/
	
	pub fare_transfer_type: GtfsEnum,

	/** Record: __Optional__

	*/
	
	pub fare_product_id: Option<FareProductId>,
}

/** `areas.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Areas {
	/** Record: __Required__

	*/
	
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]
	pub area_id: AreaUid,

	/** Record: __Optional__

	*/
	
	pub area_name: Option<Text>,
}

/** `stop_areas.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StopAreas {
	/** Record: __Required__

	*/
	
	pub area_id: AreaUid,

	/** Record: __Required__

	*/
	
	pub stop_id: StopUid,
}

/** `networks.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Networks {
	/** Record: __Required__

	*/
	
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]
	pub network_id: NetworkUid,

	/** Record: __Optional__

	*/
	
	pub network_name: Option<Text>,
}

/** `route_networks.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RouteNetworks {
	/** Record: __Required__

	*/
	
	pub network_id: NetworkUid,

	/** Record: __Required__

	*/
	
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]
	pub route_id: RouteUid,
}

/** `shapes.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Shapes {
	/** Record: __Required__

	*/
	
	pub shape_id: ShapeId,

	/** Record: __Required__

	*/
	
	pub shape_pt_lat: Latitude,

	/** Record: __Required__

	*/
	
	pub shape_pt_lon: Longitude,

	/** Record: __Required__

	*/
	
	pub shape_pt_sequence: NonNegativeInteger,

	/** Record: __Optional__

	*/
	
	pub shape_dist_traveled: Option<NonNegativeFloat>,
}

/** `frequencies.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Frequencies {
	/** Record: __Required__

	*/
	
	pub trip_id: TripUid,

	/** Record: __Required__

	*/
	
	pub start_time: Time,

	/** Record: __Required__

	*/
	
	pub end_time: Time,

	/** Record: __Required__

	*/
	
	pub headway_secs: PositiveInteger,

	/** Record: __Optional__

	*/
	
	pub exact_times: Option<GtfsEnum>,
}

/** `transfers.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Transfers {
	/** Record: __Conditionally Required__

	*/
	
	pub from_stop_id: Option<StopUid>,

	/** Record: __Conditionally Required__

	*/
	
	pub to_stop_id: Option<StopUid>,

	/** Record: __Optional__

	*/
	
	pub from_route_id: Option<RouteUid>,

	/** Record: __Optional__

	*/
	
	pub to_route_id: Option<RouteUid>,

	/** Record: __Conditionally Required__

	*/
	
	pub from_trip_id: Option<TripUid>,

	/** Record: __Conditionally Required__

	*/
	
	pub to_trip_id: Option<TripUid>,

	/** Record: __Required__

	*/
	
	pub transfer_type: GtfsEnum,

	/** Record: __Optional__

	*/
	
	pub min_transfer_time: Option<NonNegativeInteger>,
}

/** `pathways.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pathways {
	/** Record: __Required__

	*/
	
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]
	pub pathway_id: PathwayUid,

	/** Record: __Required__

	*/
	
	pub from_stop_id: StopUid,

	/** Record: __Required__

	*/
	
	pub to_stop_id: StopUid,

	/** Record: __Required__

	*/
	
	pub pathway_mode: GtfsEnum,

	/** Record: __Required__

	*/
	
	pub is_bidirectional: GtfsEnum,

	/** Record: __Optional__

	*/
	
	pub length: Option<NonNegativeFloat>,

	/** Record: __Optional__

	*/
	
	pub traversal_time: Option<PositiveInteger>,

	/** Record: __Optional__

	*/
	
	pub stair_count: Option<NonNullInteger>,

	/** Record: __Optional__

	*/
	
	pub max_slope: Option<Float>,

	/** Record: __Optional__

	*/
	
	pub min_width: Option<PositiveFloat>,

	/** Record: __Optional__

	*/
	
	pub signposted_as: Option<Text>,

	/** Record: __Optional__

	*/
	
	pub reversed_signposted_as: Option<Text>,
}

/** `levels.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Levels {
	/** Record: __Required__

	*/
	
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]
	pub level_id: LevelUid,

	/** Record: __Required__

	*/
	
	pub level_index: Float,

	/** Record: __Optional__

	*/
	
	pub level_name: Option<Text>,
}

/** `location_groups.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LocationGroups {
	/** Record: __Required__

	*/
	
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]
	pub location_group_id: LocationGroupUid,

	/** Record: __Optional__

	*/
	
	pub location_group_name: Option<Text>,
}

/** `location_group_stops.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LocationGroupStops {
	/** Record: __Required__

	*/
	
	pub location_group_id: LocationGroupUid,

	/** Record: __Required__

	*/
	
	pub stop_id: StopUid,
}

/** `booking_rules.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BookingRules {
	/** Record: __Required__

	*/
	
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]
	pub booking_rule_id: BookingRuleUid,

	/** Record: __Required__

	*/
	
	pub booking_type: GtfsEnum,

	/** Record: __Conditionally Required__

	*/
	
	pub prior_notice_duration_min: Option<Integer>,

	/** Record: __Conditionally Forbidden__

	*/
	
	pub prior_notice_duration_max: Option<Integer>,

	/** Record: __Conditionally Required__

	*/
	
	pub prior_notice_last_day: Option<Integer>,

	/** Record: __Conditionally Required__

	*/
	
	pub prior_notice_last_time: Option<Time>,

	/** Record: __Conditionally Forbidden__

	*/
	
	pub prior_notice_start_day: Option<Integer>,

	/** Record: __Conditionally Required__

	*/
	
	pub prior_notice_start_time: Option<Time>,

	/** Record: __Conditionally Forbidden__

	*/
	
	pub prior_notice_service_id: Option<ServiceUid>,

	/** Record: __Optional__

	*/
	
	pub message: Option<Text>,

	/** Record: __Optional__

	*/
	
	pub pickup_message: Option<Text>,

	/** Record: __Optional__

	*/
	
	pub drop_off_message: Option<Text>,

	/** Record: __Optional__

	*/
	
	pub phone_number: Option<PhoneNumber>,

	/** Record: __Optional__

	*/
	
	pub info_url: Option<Url>,

	/** Record: __Optional__

	*/
	
	pub booking_url: Option<Url>,
}

/** `translations.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Translations {
	/** Record: __Required__

	*/
	
	pub table_name: GtfsEnum,

	/** Record: __Required__

	*/
	
	pub field_name: Text,

	/** Record: __Required__

	*/
	
	pub language: LanguageCode,

	/** Record: __Required__

	*/
	
	pub translation: TranslationValue,

	/** Record: __Conditionally Required__

	*/
	
	pub record_id: Option<RecordId>,

	/** Record: __Conditionally Required__

	*/
	
	pub record_sub_id: Option<RecordSubId>,

	/** Record: __Conditionally Required__

	*/
	
	pub field_value: Option<TranslationValue>,
}

/** `feed_info.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeedInfo {
	/** Record: __Required__

	*/
	
	pub feed_publisher_name: Text,

	/** Record: __Required__

	*/
	
	pub feed_publisher_url: Url,

	/** Record: __Required__

	*/
	
	pub feed_lang: LanguageCode,

	/** Record: __Optional__

	*/
	
	pub default_lang: Option<LanguageCode>,

	/** Record: __Recommended__

	*/
	
	pub feed_start_date: Option<Date>,

	/** Record: __Recommended__

	*/
	
	pub feed_end_date: Option<Date>,

	/** Record: __Recommended__

	*/
	
	pub feed_version: Option<Text>,

	/** Record: __Optional__

	*/
	
	pub feed_contact_email: Option<Email>,

	/** Record: __Optional__

	*/
	
	pub feed_contact_url: Option<Url>,
}

/** `attributions.txt`



## Description


 */
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Attributions {
	/** Record: __Optional__

	*/
	
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]
	pub attribution_id: Option<AttributionUid>,

	/** Record: __Optional__

	*/
	
	pub agency_id: Option<AgencyUid>,

	/** Record: __Optional__

	*/
	
	pub route_id: Option<RouteUid>,

	/** Record: __Optional__

	*/
	
	pub trip_id: Option<TripUid>,

	/** Record: __Required__

	*/
	
	pub organization_name: Text,

	/** Record: __Optional__

	*/
	
	pub is_producer: Option<GtfsEnum>,

	/** Record: __Optional__

	*/
	
	pub is_operator: Option<GtfsEnum>,

	/** Record: __Optional__

	*/
	
	pub is_authority: Option<GtfsEnum>,

	/** Record: __Optional__

	*/
	
	pub attribution_url: Option<Url>,

	/** Record: __Optional__

	*/
	
	pub attribution_email: Option<Email>,

	/** Record: __Optional__

	*/
	
	pub attribution_phone: Option<PhoneNumber>,
}
