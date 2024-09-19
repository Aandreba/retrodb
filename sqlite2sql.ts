import { $ } from "bun";

const content = await $`sqlite3 libretrodb.sqlite -batch ".dump"`.text();
const newContent = content
    .replaceAll("PRAGMA foreign_keys=OFF;\n", "")
    .replaceAll("INTEGER", "BIGINT");
Bun.write("libretrodb.sql", newContent);
