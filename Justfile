set windows-powershell := true

build: update
    cd libretrodb-sqlite; make all
    cp libretrodb-sqlite/build/libretrodb.sqlite libretrodb.sqlite
    dart run build_runner build

update:
    git submodule update --init --force
