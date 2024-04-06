{ symlinkJoin

, gtfs-srcs

}: symlinkJoin {
	name = "GTFS Schedules";
	paths = gtfs-srcs;
}
