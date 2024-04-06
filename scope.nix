{ lib
, makeScope
, newScope
, craneLib
}:
makeScope newScope (self: {
 	inherit craneLib;
 
 	gtfs = self.callPackage ./gtfs-sources.nix {

	gtfs-srcs = [
			self.mbta-gtfs
			self.denver-rtd-gtfs
			self.nyc-bronx-gtfs
			self.nyc-manhattan-gtfs
		];
	};

	mbta-gtfs = self.callPackage ./gtfs.nix {
		name = "mbta";
		src-name = "mbta-2024-01-11";
		url = "https://cdn.mbtace.com/archive/20240111.zip";
		hash = "sha256-Nvo740l+eo/vnHzZ0qOIJwsUGrp/4CPDxTpXhVlrLnI=";
	};

	denver-rtd-gtfs = self.callPackage ./gtfs.nix {
		name = "denver-rtd";
		url = "https://www.rtd-denver.com/files/gtfs/google_transit.zip";
		hash = "sha256-b9BqQYwlduf7TeUC/qtlqqfReUuqz1iZDidqsr7AgKs=";
	};

	nyc-bronx-gtfs = self.callPackage ./gtfs.nix {
		name = "nyc-bronx-gtfs";
		url = "http://web.mta.info/developers/data/nyct/bus/google_transit_bronx.zip";
		hash = "sha256-GEQ3mwj543yCN4mNh9gsi3zjHaGFfLqNdOVx/EEMGso=";
	};

	nyc-manhattan-gtfs = self.callPackage ./gtfs.nix {
		name = "nyc-manhattan-gtfs";
		url = "http://web.mta.info/developers/data/nyct/bus/google_transit_manhattan.zip";
		hash = "sha256-QHbAWWbTdMbkgdKh5FY3b4odvDpIYc//xU9LfHJNL/g=";
	};

	gtfs-schedule-html = self.callPackage ./gtfs-schedule-xslt/gtfs-schedule-html.nix {};

	gtfs-schedule-xhtml = self.callPackage ./gtfs-schedule-xslt/gtfs-schedule-xhtml.nix {};

	gtfs-schedule-xml = self.callPackage ./gtfs-schedule-xslt/gtfs-schedule-xml.nix {};

	gtfs-schedule-generated-rs-src = self.callPackage ./gtfs-schedule-xslt/gtfs-schedule-generated-rs-src.nix {};

	gtfs-schedule-types-rs-common-args = {
		src = self.callPackage ./gtfs-schedule-types/combined-srcs.nix {};

		strictDeps = true;
		doCheck = false;
	};

	gtfs-schedule-types-rs = self.callPackage (./gtfs-schedule-types) {};

	gtfs-schedule-types-rs-doc = self.callPackage (./gtfs-schedule-types/rustdoc.nix) {};

	gtfs-schedule-types-rs-cargo-artifacts = self.callPackage ./gtfs-schedule-types/cargo-artifacts.nix {
		gtfs-schedule-types-rs-common-args = {
			src = craneLib.cleanCargoSource (craneLib.path ./gtfs-schedule-types);
			strictDeps = true;
		};
	};
})
