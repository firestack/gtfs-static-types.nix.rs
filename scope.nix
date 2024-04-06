{ lib
	, makeScope
	, newScope
	, craneLib
	, stdenv
	, libiconv
}:
makeScope newScope (self: {
	inherit craneLib;

	mbta-gtfs = self.callPackage ./mbta-gtfs.nix {};

	denver-rtd-gtfs = self.callPackage ({ fetchzip }: fetchzip {
		url = "https://www.rtd-denver.com/files/gtfs/google_transit.zip";
		stripRoot = false;
		hash = "sha256-5VjT/dxxEAodkYF/228ENXonD/v4WnCjx1aqvt43KS0=";
	}) {};

	nyc-bronx-gtfs = self.callPackage ({ fetchzip }: fetchzip {
		url = "http://web.mta.info/developers/data/nyct/bus/google_transit_bronx.zip";
		stripRoot = false;
		hash = "sha256-x/n2wT5Vct7U6ucevCDftW4GojaFuV0rQdIAJ7gp5P4=";
	}) {};

	nyc-manhattan-gtfs = self.callPackage ({ fetchzip }: fetchzip {
		url = "http://web.mta.info/developers/data/nyct/bus/google_transit_manhattan.zip";
		stripRoot = false;
		hash = "sha256-C7In6exqbnPLYPIsB9t11x3zfmaWipYHa2P778rL+ZU=";
	}) {};

	gtfs-schedule-html = self.callPackage ./gtfs-schedule-xslt/gtfs-schedule-html.nix {};

	gtfs-schedule-xhtml = self.callPackage ./gtfs-schedule-xslt/gtfs-schedule-xhtml.nix {};

	gtfs-schedule-xml = self.callPackage ./gtfs-schedule-xslt/gtfs-schedule-xml.nix {};

	gtfs-schedule-generated-rs-src = self.callPackage ./gtfs-schedule-xslt/gtfs-schedule-generated-rs-src.nix {};

	gtfs-schedule-types-rs-common-args = {
		src = self.callPackage ./gtfs-schedule-types/combined-srcs.nix {
			inherit craneLib;
		};

		strictDeps = true;
		doCheck = false;

		buildInputs = [] ++ lib.optionals stdenv.isDarwin [
			libiconv
		];

	};
	gtfs-schedule-types-rs = self.callPackage (./gtfs-schedule-types) {
		inherit craneLib;
	};

	gtfs-schedule-types-rs-doc = self.callPackage (./gtfs-schedule-types/rustdoc.nix) {
		inherit craneLib;
	};

	gtfs-schedule-types-rs-cargo-artifacts = self.callPackage ./gtfs-schedule-types/cargo-artifacts.nix {
		inherit craneLib;
	};
})
