const IN_NODE =
    typeof process === "object" &&
    typeof process.versions === "object" &&
    typeof process.versions.node === "string";

export const RETRODB_URL = locateFile();

async function locateFile() {
    const url = new URL("../libretrodb.sqlite", import.meta.url);
    if (url.protocol === "file:") {
        return IN_NODE
            ? (await import("url")).fileURLToPath(url)
            : url.pathname;
    }
    return url.toString() ?? "";
}
