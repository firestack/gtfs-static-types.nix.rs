use gtfs_realtime_types::FeedMessage;

fn main() -> Result<(), ()> {
	let json = std::fs::read_to_string("./VehiclePositions.json").unwrap();
	let feed: FeedMessage = serde_json::from_str(&json).unwrap();
	dbg!(feed);
	Ok(())
}

