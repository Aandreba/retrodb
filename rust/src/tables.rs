#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawGame<S> {
    pub id: u32,
    pub serial_id: Option<S>,
    pub developer_id: Option<u32>,
    pub publisher_id: Option<u32>,
    pub rating_id: Option<u32>,
    pub users: Option<u32>,
    pub franchise_id: Option<u32>,
    pub release_year: Option<u32>,
    pub release_month: Option<u8>,
    pub region_id: Option<u32>,
    pub genre_id: Option<u32>,
    pub display_name: Option<S>,
    pub full_name: Option<S>,
    pub platform_id: Option<u32>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawRom<S> {
    pub id: u32,
    pub serial_id: Option<S>,
    pub name: Option<S>,
    pub crc: Option<u32>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawDeveloper<S> {
    pub id: u32,
    pub name: S,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawPublisher<S> {
    pub id: u32,
    pub name: S,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawRating<S> {
    pub id: u32,
    pub name: S,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawFranchise<S> {
    pub id: u32,
    pub name: S,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawRegion<S> {
    pub id: u32,
    pub name: S,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawGenre<S> {
    pub id: u32,
    pub name: S,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawPlatform<S> {
    pub id: u32,
    pub name: S,
    pub manufacturer_id: Option<u32>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawManufacturer<S> {
    pub id: u32,
    pub name: S,
}
