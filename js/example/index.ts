import { drizzle } from "drizzle-orm/bun-sqlite";
import { Database } from "bun:sqlite";
import { getCoverArt, RETRODB_URL, schema } from "retrodb";
import { like } from "drizzle-orm";

const path = await RETRODB_URL;
console.log(path);

const sqlite = new Database(path, { readonly: true });
const db = drizzle(sqlite, { schema });

const result = await db.query.games.findMany({
    with: {
        platform: {
            columns: {
                name: true,
            },
            with: {
                manufacturer: {
                    columns: {
                        name: true,
                    },
                },
            },
        },
    },
    where: (fields) => like(fields.displayName, "%Mario Kart DS%"),
});

console.log(
    result.map((entry) => ({
        ...entry,
        cover: getCoverArt(entry),
    }))
);
