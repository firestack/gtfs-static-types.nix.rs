use std::io::Result;

fn main() -> Result<()> {
	prost_build::Config::new()
		.message_attribute(
			".",
			"#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]"
		)
		.compile_protos(&["src/gtfs-realtime.proto"], &["src/"])?;
	//prost_build::compile_protos(&["src/gtfs-realtime.proto"], &["src/"])?;
	Ok(())
}
