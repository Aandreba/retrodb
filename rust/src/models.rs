#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Game<S> {
    pub id: u32,
    pub serial_id: Option<S>,
    pub developer: Option<Developer<S>>,
    pub publisher: Option<Publisher<S>>,
    pub rating: Option<Rating<S>>,
    pub users: Option<u32>,
    pub franchise: Option<Franchise<S>>,
    pub release_year: Option<u32>,
    pub release_month: Option<u8>,
    pub region: Option<Region<S>>,
    pub genre: Option<Genre<S>>,
    pub display_name: Option<S>,
    pub full_name: Option<S>,
    pub platform: Option<Platform<S>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rom<S> {
    pub id: u32,
    pub serial_id: Option<S>,
    pub name: Option<S>,
    pub crc: Option<u32>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Developer<S> {
    pub id: u32,
    pub name: S,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Publisher<S> {
    pub id: u32,
    pub name: S,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rating<S> {
    pub id: u32,
    pub name: S,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Franchise<S> {
    pub id: u32,
    pub name: S,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Region<S> {
    pub id: u32,
    pub name: S,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Genre<S> {
    pub id: u32,
    pub name: S,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Platform<S> {
    pub id: u32,
    pub name: S,
    pub manufacturer: Option<Manufacturer<S>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Manufacturer<S> {
    pub id: u32,
    pub name: S,
}
