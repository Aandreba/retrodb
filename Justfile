set windows-powershell := true

build: submodules
    cd libretrodb-sqlite; make all
    cp libretrodb-sqlite/build/libretrodb.sqlite libretrodb.sqlite
    dart run build_runner build

submodules:
    git submodule update --init --remote
