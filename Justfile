set windows-powershell := true

build-db: submodules
    cd libretrodb-sqlite; make all
    cp libretrodb-sqlite/build/libretrodb.sqlite libretrodb.sqlite

build-dart:
    cd dart; dart run build_runner build

submodules:
    git submodule update --init --remote
