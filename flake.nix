{
	description = "GTFS Types from GTFS Schedule Specification Webpage";

	nixConfig.allow-import-from-derivation = true;
	inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
	inputs.devshell.url = "github:numtide/devshell";
	inputs.flake-parts.url = "github:hercules-ci/flake-parts";

	inputs.crane = {
		url = "github:ipetkov/crane";
		inputs.nixpkgs.follows = "nixpkgs";
	};

	outputs = inputs@{ self, flake-parts, devshell, nixpkgs, crane }:
		flake-parts.lib.mkFlake { inherit inputs; } {
			imports = [
				devshell.flakeModule
			];

			systems = [
				"aarch64-darwin"
				"aarch64-linux"
				"i686-linux"
				"x86_64-darwin"
				"x86_64-linux"
			];

			perSystem = { pkgs, lib, self', system, ... }: {
				packages = {
					default = self'.packages.gtfs-schedule-types-rs-doc;
				} // (
					{
						inherit (pkgs.callPackage ./scope.nix {
							makeScope = pkgs.lib.makeScope;
							craneLib = crane.lib.${system};
						})
							mbta-gtfs
							gtfs-schedule-html
							gtfs-schedule-xhtml
							gtfs-schedule-xml
							gtfs-schedule-generated-rs-src
							gtfs-schedule-types-rs
							gtfs-schedule-types-rs-doc
							denver-rtd-gtfs
							nyc-bronx-gtfs
							nyc-manhattan-gtfs;
					}
				);

				devshells.default = {
					packages = [
						pkgs.saxon-he
						pkgs.entr
						pkgs.evcxr
						pkgs.rustc
						pkgs.cargo
						pkgs.clippy
						pkgs.rustfmt
					];

					commands = [{
						help = "develop XSLT with saxon-he";
						name = "dev-he-java";
						command = ''fd . src/ | entr -rc saxon-he -t $@'';
					}];
				};

				checks = self'.packages // {
					inherit (self'.devShells)
						default;
				};
			};
		};
}
