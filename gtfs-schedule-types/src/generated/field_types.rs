/* Types */

// #region Field Types

/**
Color - A color encoded as a six-digit
hexadecimal number. Refer to https://htmlcolorcodes.com to
generate a valid value (the leading "#" must not be
included).Example: FFFFFF for white, 000000 for
black or 0039A6 for the A,C,E lines in
NYMTA.
 */
pub type Color = String;

/**
Currency code - An ISO 4217 alphabetical
currency code. For the list of current currency, refer to https://en.wikipedia.org/wiki/ISO_4217#Active_codes.Example: CAD for Canadian dollars,
EUR for euros or JPY for Japanese
yen.
 */
pub type CurrencyCode = String;

/**
Currency amount - A decimal value indicating a
currency amount. The number of decimal places is specified by
ISO
4217 for the accompanying Currency code. All financial
calculations should be processed as decimal, currency, or another
equivalent type suitable for financial calculations depending on
the programming language used to consume data. Processing currency
amounts as float is discouraged due to gains or losses of money
during calculations.
 */
pub type CurrencyAmount = Text;

/**
Date - Service day in the YYYYMMDD format.
Since time within a service day may be above 24:00:00, a service
day may contain information for the subsequent day(s).Example: 20180913 for September 13th,
2018.
 */
pub type Date = Text;

/**
Email - An email address.Example: example@example.com
 */
pub type Email = Text;

/**
Enum - An option from a set of predefined
constants defined in the "Description" column.Example: The route_type field contains a
0 for tram, a 1 for subway...
 */
pub type GtfsEnum = u32;

/**
ID - An ID field value is an internal ID, not
intended to be shown to riders, and is a sequence of any UTF-8
characters. Using only printable ASCII characters is recommended.
An ID is labeled "unique ID" when it must be unique within a file.
IDs defined in one .txt file are often referenced in another .txt
file. IDs that reference an ID in another table are labeled
"foreign ID".Example: The stop_id field in stops.txt is a "unique ID". The
parent_station field in stops.txt is a "foreign ID referencing
stops.stop_id".
 */
pub type GtfsId = String;

/**
Language code - An IETF BCP 47 language code.
For an introduction to IETF BCP 47, refer to http://www.rfc-editor.org/rfc/bcp/bcp47.txt
and http://www.w3.org/International/articles/language-tags/.Example: en for English, en-US for
American English or de for German.
 */
pub type LanguageCode = Text;

/**
Latitude - WGS84 latitude in decimal degrees.
The value must be greater than or equal to -90.0 and less than or
equal to 90.0.
Example: 41.890169 for the Colosseum in
Rome.
 */
pub type Latitude = Float;

/**
Longitude - WGS84 longitude in decimal
degrees. The value must be greater than or equal to -180.0 and less
than or equal to 180.0.Example: 12.492269 for the Colosseum in
Rome.
 */
pub type Longitude = Float;

/**
Float - A floating point number.
 */
pub type Float = f64;

/**
Integer - An integer.
 */
pub type Integer = i64;

/**
Phone number - A phone number.
 */
pub type PhoneNumber = Text;

/**
Time - Time in the HH:MM:SS format (H:MM:SS is
also accepted). The time is measured from "noon minus 12h" of the
service day (effectively midnight except for days on which daylight
savings time changes occur). For times occurring after midnight on
the service day, enter the time as a value greater than 24:00:00 in
HH:MM:SS.Example: 14:30:00 for 2:30PM or
25:35:00 for 1:35AM on the next day.
 */
pub type Time = Text;

/**
Text - A string of UTF-8 characters, which is
aimed to be displayed and which must therefore be human
readable.
 */
pub type Text = String;

/**
Timezone - TZ timezone from the https://www.iana.org/time-zones.
Timezone names never contain the space character but may contain an
underscore. Refer to http://en.wikipedia.org/wiki/List_of_tz_zones
for a list of valid values.Example: Asia/Tokyo,
America/Los_Angeles or
Africa/Cairo.
 */
pub type Timezone = Text;

/**
URL - A fully qualified URL that includes
http:// or https://, and any special characters in the URL must be
correctly escaped. See the following http://www.w3.org/Addressing/URL/4_URI_Recommentations.html
for a description of how to create fully qualified URL values.
 */
pub type Url = Text;

// #endregion Field Types

////////////////////////////////////////////////////////////////////////////////

// #region Unique Id Types

/**

*/
pub type AgencyUid = GtfsId;

/**

*/
pub type StopUid = GtfsId;

/**

*/
pub type RouteUid = GtfsId;

/**

*/
pub type TripUid = GtfsId;

/**

*/
pub type ServiceUid = GtfsId;

/**

*/
pub type FareUid = GtfsId;

/**

*/
pub type FareMediaUid = GtfsId;

/**

*/
pub type AreaUid = GtfsId;

/**

*/
pub type NetworkUid = GtfsId;

/**

*/
pub type PathwayUid = GtfsId;

/**

*/
pub type LevelUid = GtfsId;

/**

*/
pub type AttributionUid = GtfsId;

// #endregion Unique Id Types

////////////////////////////////////////////////////////////////////////////////

// #region Id Types

/**

*/
pub type ZoneId = GtfsId;

/**

*/
pub type NetworkId = GtfsId;

/**

*/
pub type BlockId = GtfsId;

/**

*/
pub type TimeframeGroupId = GtfsId;

/**

*/
pub type FareProductId = GtfsId;

/**

*/
pub type LegGroupId = GtfsId;

/**

*/
pub type ShapeId = GtfsId;

// #endregion Id Types

////////////////////////////////////////////////////////////////////////////////

// #region Other Types

/**

*/
pub type NonNegativeInteger = u64;

/**

*/
pub type NonNegativeFloat = Float;

/**

*/
pub type NonZeroInteger = Integer;

/**

*/
pub type PositiveInteger = Integer;

/**

*/
pub type NonNullInteger = Integer;

/**

*/
pub type PositiveFloat = Float;

/**

*/
pub type TranslationValue = Text;

/**

*/
pub type RecordId = GtfsId;

/**

*/
pub type RecordSubId = GtfsId;

// #endregion Other Types
