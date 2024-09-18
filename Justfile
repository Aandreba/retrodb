set windows-powershell := true

[unix]
build-db: submodules
    cd libretrodb-sqlite; make all
    cp libretrodb-sqlite/build/libretrodb.sqlite libretrodb.sqlite

build-dart:
    cd dart; dart run build_runner build

build-js:
    cd js; tsc

submodules:
    git submodule update --init --remote
