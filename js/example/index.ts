import { drizzle } from "drizzle-orm/bun-sqlite";
import { Database } from "bun:sqlite";
import { getCoverArt, schema } from "retrodb";
import { like } from "drizzle-orm";

const sqlite = new Database("libretrodb.sqlite", { readonly: true });
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
