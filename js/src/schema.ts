import { integer, sqliteTable, text } from "drizzle-orm/sqlite-core";

export const games = sqliteTable("games", {
    id: integer("id").primaryKey(),
    serialId: text("serial_id"),
    developerId: integer("developer_id").references(() => developers.id),
    publisherId: integer("publisher_id").references(() => publishers.id),
    ratingId: integer("rating_id").references(() => ratings.id),
    users: integer("users"),
    franchiseId: integer("franchise_id").references(() => franchises.id),
    releaseYear: integer("release_year"),
    releaseMonth: integer("release_month"),
    regionId: integer("region_id").references(() => regions.id),
    genreId: integer("genre_id").references(() => genres.id),
    displayName: text("display_name"),
    fullName: text("full_name"),
    platformId: integer("platform_id").references(() => platforms.id),
});

export const roms = sqliteTable("roms", {
    id: integer("id").primaryKey(),
    serialId: text("serial_id"),
    name: text("name"),
    crc: integer("crc"),
});

export const developers = sqliteTable("developers", {
    id: integer("id").primaryKey(),
    name: text("name"),
});

export const publishers = sqliteTable("publishers", {
    id: integer("id").primaryKey(),
    name: text("name"),
});

export const ratings = sqliteTable("ratings", {
    id: integer("id").primaryKey(),
    name: text("name"),
});

export const franchises = sqliteTable("franchises", {
    id: integer("id").primaryKey(),
    name: text("name"),
});

export const regions = sqliteTable("regions", {
    id: integer("id").primaryKey(),
    name: text("name"),
});

export const genres = sqliteTable("genres", {
    id: integer("id").primaryKey(),
    name: text("name"),
});

export const platforms = sqliteTable("platforms", {
    id: integer("id").primaryKey(),
    name: text("name"),
    manufacturerId: integer("manufacturer_id").references(
        () => manufacturers.id
    ),
});

export const manufacturers = sqliteTable("manufacturers", {
    id: integer("id").primaryKey(),
    name: text("name"),
});
