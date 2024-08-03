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
				legacyPackages.scope = pkgs.callPackage ./scope.nix {
					makeScope = pkgs.lib.makeScope;
					craneLib = crane.lib.${system};
				};
				legacyPackages.craneLib = crane.lib.${system};

				packages = {
					default = self'.packages.gtfs-schedule-types-rs-doc;
				} // (builtins.removeAttrs self'.legacyPackages.scope [
					# Scope outputs
					"callPackage"
					"newScope"
					"override"
					"overrideDerivation"
					"overrideScope"
					"overrideScope'"
					"packages"

					# Ignored inputs and transient attributes
					"craneLib"
					"gtfs-schedule-types-rs-common-args"
				]);

				devshells.default = {
					packages = [
						pkgs.saxon-he
						pkgs.entr
						pkgs.evcxr
						pkgs.rustc
						pkgs.cargo
						pkgs.clippy
						pkgs.rustfmt
						pkgs.html-tidy
						(pkgs.sqlite.override {interactive = true;})
						pkgs.protobuf
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

					# unchanged-gtfs-xhtml = (pkgs.runCommand "unchanged-gtfs-xhtml" {} (lib.concatLines [
					# 	"diff ${self'.packages.gtfs-schedule-xhtml} ${./gtfs-schedule-xslt/src/vendored/gtfs-schedule.xhtml}"
					# 	"mkdir $out"
					# ]));

					unchanged-gtfs-xml = (pkgs.runCommand "unchanged-gtfs-xml" {} (lib.concatLines [
						"diff ${self'.packages.gtfs-schedule-xml} ${./gtfs-schedule-xslt/src/vendored/gtfs-schedule.xml}"
						"mkdir $out"
					]));

					unchanged-gtfs-schedule-code = (pkgs.runCommand "unchanged-gtfs-schedule-code" {} (lib.concatLines [
						"diff ${self'.packages.gtfs-schedule-generated-rs-src}/generated ${./gtfs-schedule-types/src/generated}"
						"mkdir $out"
					]));

					unchanged-protobuf-code = (pkgs.runCommand "unchanged-protobuf-code" {} (lib.concatLines [
						"diff ${self'.packages.gtfs-realtime-proto} ${./gtfs-realtime-types/src/gtfs-realtime.proto}"
						"mkdir $out"
					]));
				};
			};
		};
}
