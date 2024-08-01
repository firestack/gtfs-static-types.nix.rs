<?xml version="1.0" encoding="UTF-8" ?>
<!DOCTYPE example [
	<!ENTITY bt "<xsl:text>`</xsl:text>">
]>
<xsl:stylesheet
	xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
	xmlns:xs="http://www.w3.org/2001/XMLSchema"
	xmlns:saxon="http://saxon.sf.net/"
	xmlns:rs="https://www.rust-lang.org.kaylafire.me/"
	xmlns:x="http://www.w3.org/1999/xhtml"

	exclude-result-prefixes="#all"
	expand-text="yes"
	version="3.0"
>
<xsl:param name="out" as="xs:string" select="'.'" />
<xsl:param name="debug" as="xs:boolean" select="false()" />
<xsl:import href="./markdown.xsl" />
<xsl:include href="./functions.xsl" />
<xsl:output method="adaptive" />
<xsl:strip-space elements="*"/>
<xsl:variable name="xml-serialize-opts" select="map { 'method': 'html', 'indent': true() }"></xsl:variable>

<xsl:variable name="unique-field-id-map">
	<xsl:copy-of select="//fields/field[type/name='Unique ID' or type/name='ID']"/>
</xsl:variable>
<!-- Root Template -->
<xsl:template match="/gtfs-schedule" mode="#default" saxon:explain="yes" >

	<xsl:if test="$debug = true()">
		<xsl:result-document href="{$out}/debug/search.xml" method="xml" indent="yes">
			<xsl:for-each select="distinct-values(//field/presence)">
				<xsl:copy-of select="."/>
			</xsl:for-each>
		</xsl:result-document>
	</xsl:if>

	<xsl:result-document href="{$out}/generated/field_types.rs" method="text">
		<xsl:call-template name="types"/>
	</xsl:result-document>
	<xsl:result-document href="{$out}/generated/records.rs" method="text">
		<xsl:call-template name="records"/>
	</xsl:result-document>

	<xsl:result-document href="{$out}/generated/schedule.rs" method="text">
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
pub struct Dataset {{
<xsl:for-each select="//records/record">
/** __File Name:__ &bt;{name}&bt;

__Presence:__ {presence}

{serialize(description/summary/node(), $xml-serialize-opts)}
*/
	pub {substring-before(name, '.txt')}: {rs:wrap-type-with-presence(presence, concat("Vec&lt;", rs:struct-name-from-filename(name), "&gt;"))},
</xsl:for-each>
}}

<![CDATA[
#[cfg(feature = "from-dataset")]
use std::{{
	convert::AsRef,
	path::Path,
}};

#[cfg(feature = "from-dataset")]
use crate::parse::parse_csv;


#[cfg(feature = "from-dataset")]
impl Dataset {{
	/**
	*   Creates a [`Dataset`] from a [`Path`] by reading the expected text files
	*   from the provided directory.
	*/
	pub fn read_from_path<T: AsRef<Path>>(dataset_path: T) -> Result<Self, &'static str> {{
		let dataset_path = dataset_path.as_ref();
]]>
		Ok(Self {{<xsl:for-each select="//records/record">
			{substring-before(name, '.txt')}: parse_csv(&amp;dataset_path.join(
				"{name}"
			))<xsl:choose>
				<xsl:when test="not(rs:type_is_optional(presence))">.expect("File '{name}' is required, but failed to parse")</xsl:when>
				<xsl:otherwise>.ok()</xsl:otherwise>
			</xsl:choose>,</xsl:for-each>
		}})
	}}
}}
</xsl:result-document>
</xsl:template>

<!-- #endregion Types -->
<!-- Call by `@name` because we want access to the whole document -->
<xsl:template name="types">
/* Types */

// #region Field Types
<xsl:apply-templates mode="type-definition" select="//types/type"/>
// #endregion Field Types

////////////////////////////////////////////////////////////////////////////////

// #region Unique Id Types
<xsl:apply-templates mode="type-definition" select="
	//fields/field[
		generate-id()
			= generate-id(
				key(
					'typesDistinct',
					rs:gtfs-type(
						type/name,
						'',
						name,
						$unique-field-id-map
					)
			)[type/name='Unique ID']
		)]
		/type"/>
// #endregion Unique Id Types

////////////////////////////////////////////////////////////////////////////////

// #region Id Types
<xsl:apply-templates mode="type-definition" select="
	//fields/field[
		generate-id()
			= generate-id(
				key(
					'typesDistinct',
					rs:gtfs-type(
						type/name,
						'',
						name,
						$unique-field-id-map
					)
			)[type/name='ID']
		)]
		/type"/>
// #endregion Id Types

////////////////////////////////////////////////////////////////////////////////

// #region Other Types
<xsl:apply-templates mode="type-definition" select="
	//fields/field[
		not(type/name='ID') and
		not(type/name='Unique ID') and
		not(rs:is-foreign-id(type/name)) and

		generate-id()
			= generate-id(
				key(
					'typesDistinct',
					rs:gtfs-type(
						type/name,
						'',
						name,
						$unique-field-id-map
					)
			)[1]
		)]
		/type"/>
// #endregion Other Types

</xsl:template>

<xsl:template mode="type-definition" match=".">
<xsl:variable name="typeName"><xsl:apply-templates select="." mode="type" /></xsl:variable>
/**
<xsl:copy-of select="description/x:body"/>
 */
pub type <xsl:value-of select="$typeName"/> = <xsl:value-of select="rs:map-gtfs-type-to-rust($typeName)"/>;
</xsl:template>

<!-- #region Type Formatting -->
<!-- Type `XML -> Rust` Formatting -->
<xsl:key
	name="typesDistinct"
	match="//types/type | //fields/field"
>
	<xsl:apply-templates mode="type" select="."/>
</xsl:key>

<!-- <xsl:key name="" match=""></xsl:key> -->

<xsl:template mode="type" match="field/type">
	<xsl:copy-of select="rs:gtfs-type(name, '', ../name, $unique-field-id-map)" />
</xsl:template>

<xsl:template mode="type" match="types/type">
	<xsl:copy-of select='rs:gtfs-type(name, "unknown", "GtfsId", $unique-field-id-map)' />
</xsl:template>
<!-- #endregion Type Formatting -->

<!-- #endregion Types -->

<!-- #region Structs -->
<!-- Call by `@name` because we want access to the whole document -->
<xsl:template name="records">
use crate::field_types::*;

#[cfg(feature = "sql")]
use sea_orm::entity::prelude::*;

#[cfg(feature = "rustler")]
use rustler::NifStruct;

/* Structs */
<xsl:apply-templates mode="struct" select="//records/record" />
</xsl:template>

<xsl:template mode="struct" match="record">
/** &bt;{name}&bt;

{serialize(description/summary/node(), $xml-serialize-opts)}

## Description
{serialize(description/x:body/node(), $xml-serialize-opts)}
<!-- <xsl:copy-of select="serialize(description/x:body/*, <xsl:output method='html' indent='yes' />)"/> -->
 */
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "sql", derive(DeriveEntityModel))]
#[cfg_attr(feature = "sql", sea_orm(table_name="{name}"))]
#[cfg_attr(feature = "rustler", derive(NifStruct))]
#[cfg_attr(feature = "rustler", module = "Gtfs.Schedule.{rs:struct-name-from-filename(name)}")]
pub struct {rs:struct-name-from-filename(name)} {{<xsl:apply-templates
	mode="struct"
	select="fields/field"
>
	<xsl:with-param name="primary-key" select="primary-key"/>
</xsl:apply-templates>}}
</xsl:template>

<xsl:template mode="struct" match="field" ><xsl:param name="primary-key"/>
	/** Record: __{presence}__
{serialize(description/x:body/node(), $xml-serialize-opts)}
	*/
	<xsl:if test="$primary-key = name">
	#[cfg_attr(feature = "sql", sea_orm(primary_key))]</xsl:if>
	pub {name}: {rs:gtfs-type(type, presence, name, $unique-field-id-map)},
</xsl:template>
<!-- #endregion Structs -->

</xsl:stylesheet>
