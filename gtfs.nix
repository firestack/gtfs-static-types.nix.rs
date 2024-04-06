{ lib
, src
, name
, runCommand
, ...
}: runCommand "${name}-gtfs" {} (let
	rootDirectory = "$out/share/gtfs/";
in lib.concatLines [
	"set -ex"
	"mkdir -p $out/share/gtfs"
	"ln -sfT ${src} ${rootDirectory}/${name}"
	"set +x"
])

