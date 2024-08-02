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
		];
	};

	mbta-gtfs = self.callPackage ./gtfs.nix {
		name = "mbta";
		src-name = "mbta-2024-01-11";
		url = "https://cdn.mbtace.com/archive/20240111.zip";
		hash = "sha256-Nvo740l+eo/vnHzZ0qOIJwsUGrp/4CPDxTpXhVlrLnI=";
	};

	gtfs-schedule-html = self.callPackage ./gtfs-schedule-xslt/gtfs-schedule-html.nix {};

	gtfs-schedule-xhtml = self.callPackage ./gtfs-schedule-xslt/gtfs-schedule-xhtml.nix {};

	gtfs-schedule-xml = self.callPackage ./gtfs-schedule-xslt/gtfs-schedule-xml.nix {
		gtfs-schedule-xhtml = ./gtfs-schedule-xslt/src/vendored/gtfs-schedule.xhtml;
		include-docs = false;
	};

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

	gtfs-realtime-proto = self.callPackage ({ fetchurl }: fetchurl {
		url = "https://gtfs.org/realtime/gtfs-realtime.proto";
		hash = "sha256-kiubGHDfAKrDisvVxuDmvlKWBzNI7toGw+MxSoIG3v8=";
	}) {};
	# gtfs-schedule-sql-schema = self.callPackage ./gtfs-schedule-sql-schema.nix {};
})
