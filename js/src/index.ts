export { RETRODB_URL } from "./locate";
export * as schema from "./schema";
export * from "./schema";

type Platform = {
    name: string;
    manufacturer?: string | { name: string } | null;
};

export function getCoverArt(args: {
    fullName: string | undefined | null;
    platform: Platform | undefined | null;
}) {
    return getThumbnailUrl({
        ...args,
        type: "Named_Boxarts",
    } as const);
}

export function getSnapshot(args: {
    fullName: string | undefined | null;
    platform: Platform | undefined | null;
}) {
    return getThumbnailUrl({
        ...args,
        type: "Named_Snaps",
    } as const);
}

export function getTitleScreens(args: {
    fullName: string | undefined | null;
    platform: Platform | undefined | null;
}) {
    return getThumbnailUrl({
        ...args,
        type: "Named_Titles",
    } as const);
}

export function getThumbnailUrl({
    platform,
    fullName,
    type,
}: {
    type: string;
    fullName: string | undefined | null;
    platform: Platform | undefined | null;
}) {
    const platformName = platform?.name;
    const manufacturer =
        typeof platform?.manufacturer === "string"
            ? platform?.manufacturer
            : platform?.manufacturer?.name;

    const manufacturerPath =
        manufacturer !== undefined
            ? `${manufacturer} - ${platformName}`
            : platformName;

    return manufacturerPath !== undefined
        ? `https://thumbnails.libretro.com/${encodeURIComponent(
              manufacturerPath
          )}/${encodeURIComponent(type)}/${encodeURIComponent(
              `${fullName}.png`
          )}`
        : undefined;
}
