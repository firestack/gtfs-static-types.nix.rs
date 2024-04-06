{ lib,
, src
, name
, runCommand
, ...
}: runCommand name {} (let
	rootDirectory = "$out/share/gtfs/${name}/"
in lib.concatLines [
	"set -ex"
	"mkdir -p $out/share/gtfs/${name}/"
	"ln -s ${src} ${rootDirectory}"
	"set +x"
])

