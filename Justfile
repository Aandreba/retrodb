set windows-powershell := true

[unix]
build-db: submodules
    cd libretrodb-sqlite; make all
    cp libretrodb-sqlite/build/libretrodb.sqlite libretrodb.sqlite
    cp libretrodb.sqlite dart/libretrodb.sqlite
    cd js/example; ln -s ../../libretrodb.sqlite libretrodb.sqlite

build-dart:
    cd dart; dart run build_runner build

submodules:
    git submodule update --init --remote
