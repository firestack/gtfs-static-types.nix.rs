{ lib

# # The caller can provide a source if prefered
# , gtfs-src ? null
# # but otherwise expect that a `url` and `hash` will be provided
# , url ? null
# , hash ? null
, fetchzip ? null

# GTFS Distribution Name,
# 	e.g., `MBTA`, `nyc-bronx`, `nyc-manhattan`
, name
, runCommand

, ...
}@args:

assert args.src or null == null ->
	fetchzip != null
	&& args.url != null
	&& args.hash != null;
let
	src =
		# Prioritze `src` over `url` & `hash`
		if args.src or null == null
		then fetchzip {
			name = "GTFS Schedule ${name}";

			inherit (args) url hash;

			stripRoot = false;
		}
		else args.src;

	rootDirectory = "$out/share/gtfs/";
in

runCommand "${name}-gtfs" {} (lib.concatLines [
	"mkdir -p ${rootDirectory}"
	"ln -sfT ${src} ${rootDirectory}/${name}"
])

