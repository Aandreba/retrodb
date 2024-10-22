pub use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
pub struct Query<'a, C> {
    pub cols: C,
    pub r#where: &'a str,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

impl<'a, C: IntoIterator<Item = Column>> Query<'a, C> {
    pub const fn new(cols: C) -> Self {
        Self {
            cols,
            r#where: "TRUE",
            limit: None,
            offset: None,
        }
    }

    #[inline]
    pub fn r#where(mut self, r#where: &'a str) -> Self {
        self.r#where = r#where;
        self
    }

    #[inline]
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    #[inline]
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Column {
    SerialId,
    Developer,
    Publisher,
    Rating,
    Users,
    Franchise,
    ReleaseYear,
    ReleaseMonth,
    Region,
    Genre,
    DisplayName,
    FullName,
    Platform,
    Roms,
}

impl Column {
    pub(super) fn transform(self, fields: &mut Vec<&'static str>, joins: &mut Vec<&'static str>) {
        fields.push(match self {
            Column::SerialId => "games.serial_id",
            Column::Developer => {
                joins.push("LEFT JOIN developers ON games.developer_id = developers.id");
                "developers.name AS developer"
            }
            Column::Publisher => {
                joins.push("LEFT JOIN publishers ON games.publisher_id = publishers.id");
                "publishers.name AS publisher"
            }
            Column::Rating => {
                joins.push("LEFT JOIN ratings ON games.rating_id = ratings.id");
                "ratings.name AS rating"
            }
            Column::Users => "games.users",
            Column::Franchise => {
                joins.push("LEFT JOIN franchises ON games.franchise_id = franchises.id");
                "franchises.name AS franchise"
            }
            Column::ReleaseYear => "games.release_year",
            Column::ReleaseMonth => "games.release_month",
            Column::Region => {
                joins.push("LEFT JOIN regions ON games.region_id = regions.id");
                "regions.name AS region"
            }
            Column::Genre => {
                joins.push("LEFT JOIN genres ON games.genre_id = genres.id");
                "genres.name AS genre"
            }
            Column::DisplayName => "games.display_name",
            Column::FullName => "games.full_name",
            Column::Platform => {
                joins.extend([
                    "LEFT JOIN platforms ON games.platform_id = platforms.id",
                    "LEFT JOIN manufacturers ON platforms.manufacturer_id = manufacturers.id",
                ]);
                "platforms.name AS platform, manufacturers.name AS manufacturer"
            }
            Column::Roms => {
                joins.push("INNER JOIN roms ON games.serial_id = roms.serial_id");
                return fields.extend([
                    "roms.id as rom_id",
                    "roms.name as rom_name",
                    "roms.crc as rom_crc",
                ]);
            }
        });
    }
}
