import { relations } from "drizzle-orm";
import { integer, sqliteTable, text } from "drizzle-orm/sqlite-core";

export const games = sqliteTable("games", {
    id: integer("id").primaryKey(),
    serialId: text("serial_id"),
    developerId: integer("developer_id"),
    publisherId: integer("publisher_id"),
    ratingId: integer("rating_id"),
    users: integer("users"),
    franchiseId: integer("franchise_id"),
    releaseYear: integer("release_year"),
    releaseMonth: integer("release_month"),
    regionId: integer("region_id"),
    genreId: integer("genre_id"),
    displayName: text("display_name"),
    fullName: text("full_name"),
    platformId: integer("platform_id"),
});

export const roms = sqliteTable("roms", {
    id: integer("id").primaryKey(),
    serialId: text("serial_id"),
    name: text("name"),
    crc: integer("crc"),
});

export const developers = sqliteTable("developers", {
    id: integer("id").primaryKey(),
    name: text("name").notNull(),
});

export const publishers = sqliteTable("publishers", {
    id: integer("id").primaryKey(),
    name: text("name").notNull(),
});

export const ratings = sqliteTable("ratings", {
    id: integer("id").primaryKey(),
    name: text("name").notNull(),
});

export const franchises = sqliteTable("franchises", {
    id: integer("id").primaryKey(),
    name: text("name").notNull(),
});

export const regions = sqliteTable("regions", {
    id: integer("id").primaryKey(),
    name: text("name").notNull(),
});

export const genres = sqliteTable("genres", {
    id: integer("id").primaryKey(),
    name: text("name").notNull(),
});

export const platforms = sqliteTable("platforms", {
    id: integer("id").primaryKey(),
    name: text("name").notNull(),
    manufacturerId: integer("manufacturer_id"),
});

export const manufacturers = sqliteTable("manufacturers", {
    id: integer("id").primaryKey(),
    name: text("name").notNull(),
});

/* RELATIONS */
export const gamesRelations = relations(games, ({ one, many }) => ({
    roms: many(roms),
    developer: one(developers, {
        fields: [games.developerId],
        references: [developers.id],
    }),
    publisher: one(publishers, {
        fields: [games.publisherId],
        references: [publishers.id],
    }),
    rating: one(ratings, {
        fields: [games.ratingId],
        references: [ratings.id],
    }),
    franchise: one(franchises, {
        fields: [games.franchiseId],
        references: [franchises.id],
    }),
    region: one(regions, {
        fields: [games.regionId],
        references: [regions.id],
    }),
    genre: one(genres, {
        fields: [games.genreId],
        references: [genres.id],
    }),
    platform: one(platforms, {
        fields: [games.platformId],
        references: [platforms.id],
    }),
}));

export const romsRelations = relations(roms, ({ one }) => ({
    game: one(games, {
        fields: [roms.serialId],
        references: [games.serialId],
    }),
}));

export const platformsRelations = relations(platforms, ({ one }) => ({
    manufacturer: one(manufacturers, {
        fields: [platforms.manufacturerId],
        references: [manufacturers.id],
    }),
}));
