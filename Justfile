set windows-powershell := true

[unix]
build-db: submodules
    cd libretrodb-sqlite; make all
    cp libretrodb-sqlite/build/libretrodb.sqlite libretrodb.sqlite
    sqlite3 libretrodb.sqlite -batch ".dump" > libretrodb.sql
    cd js/example/public; ln -s ../../../libretrodb.sql libretrodb.sql

build-dart:
    cd dart; dart run build_runner build

submodules:
    git submodule update --init --remote
