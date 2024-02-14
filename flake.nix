{
	description = "GTFS Types from GTFS Spec";

	inputs.devshell.url = "github:numtide/devshell";
	inputs.flake-parts.url = "github:hercules-ci/flake-parts";

	outputs = inputs@{ self, flake-parts, devshell, nixpkgs }:
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

			perSystem = { pkgs, lib, self', ... }: let linesFrom = lib.concatStringsSep "\n"; in {
				packages.gtfs-static-html = pkgs.fetchurl {
					name = "gtfs-static.html";
					url = "https://gtfs.org/schedule/reference";
					hash = "sha256-RHcc3G2lksdgGIlJuethbhoDST+TeHXjcBNGMbiawCQ=";
				};

				packages.gtfs-static-xhtml = pkgs.runCommand "gtfs-static.xhtml" {
					buildInputs = [pkgs.html-tidy];
				} (linesFrom [
					"tidy -asxhtml --new-inline-tags c -o $out ${self'.packages.gtfs-static-html} || true"
				]);

				packages.gtfs-static-xml = pkgs.runCommand "gtfs-static.xml" {
					buildInputs = [ pkgs.saxon-he ];
				} (linesFrom [
					"saxon-he -t \\"
						"-s:${self'.packages.gtfs-static-xhtml} \\"
						"-xsl:${./src/xsl/gtfs-static.xml.xsl} \\"
						"-o:$out"
				]);

				# packages.gtfs-static-rs = pkgs.runCommand "gtfs-static-rs" {
				# 	buildInputs = [ pkgs.saxon-he ];
				# 	src = ./src/xsl;
				# } (linesFrom [
				# 	"ls -la"
				# 	"mkdir $out"
				# 	"saxon-he -t \\"
				# 		"-s:${self'.packages.gtfs-static-xml} \\"
				# 		"-xsl:${./src/xsl/gtfs-static.rs.xsl}"
				# 	"cp ./gtfs-static/* $out/"
				# 	"ln -s ${self'.packages.gtfs-static-xml} $out/gtfs-static.xml"
				# ]);


				packages.gtfs-static-rs = pkgs.stdenvNoCC.mkDerivation {
					pname = "gtfs-static-rs";
					version = "0.0.1";

					src = ./src/xsl;

					buildInputs = [ pkgs.saxon-he ];

					buildPhase = linesFrom [
						"ls -la"
						"mkdir $out"

						"saxon-he -t \\"
							"-s:${self'.packages.gtfs-static-xml} \\"
							"-xsl:gtfs-static.rs.xsl"
					];

					installPhase = linesFrom [
						"cp ./gtfs-static/* $out/"
						"ln -s ${self'.packages.gtfs-static-xml} $out/gtfs-static.xml"
					];

				};

				devshells.default = {
					packages = [
						pkgs.saxon-he
						pkgs.entr
					];

					commands = [{
						help = "develop XSLT with saxon-he";
						name = "dev-he-java";
						command = ''fd . src/ | entr -rc saxon-he -t $@'';
					}];
				};
			};
		};
}
