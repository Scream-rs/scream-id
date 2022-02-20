const ACCOUNT_ID_MASK: u64 = 0xFFFFFFFF;
const ACCOUNT_INSTANCE_MASK: u64 = 0x000FFFFF;

#[derive(PartialEq, Debug)]
pub enum Universe {
    Invalid = 0,
    Public = 1,
    Beta = 2,
    Internal = 3,
    Dev = 4,
}

#[derive(PartialEq, Debug)]
pub enum Type {
    Invalid = 0,
    Individual = 1,
    Multiseat = 2,
    GameServer = 3,
    AnonGameServer = 4,
    Pending = 5,
    ContentServer = 6,
    Clan = 7,
    Chat = 8,
    P2PSuperSeeder = 9,
    AnonUser = 10,
}

#[derive(PartialEq, Debug)]
pub enum Instance {
    All = 0,
    Desktop = 1,
    Console = 2,
    Web = 4,
}

#[derive(PartialEq, Debug)]
pub struct SteamID {
    universe: Universe,
    type_: Type,
    instance: Instance,
    account_id: u32,
}

impl Universe {
    fn from_u32(value: u32) -> Option<Universe> {
        match value {
            0 => Some(Universe::Invalid),
            1 => Some(Universe::Public),
            2 => Some(Universe::Beta),
            3 => Some(Universe::Internal),
            4 => Some(Universe::Dev),
            _ => None,
        }
    }
}

impl Type {
    fn from_u32(value: u32) -> Option<Type> {
        match value {
            0 => Some(Type::Invalid),
            1 => Some(Type::Individual),
            2 => Some(Type::Multiseat),
            3 => Some(Type::GameServer),
            4 => Some(Type::AnonGameServer),
            5 => Some(Type::Pending),
            6 => Some(Type::ContentServer),
            7 => Some(Type::Clan),
            8 => Some(Type::Chat),
            9 => Some(Type::P2PSuperSeeder),
            10 => Some(Type::AnonUser),
            _ => None,
        }
    }
}

impl Instance {
    fn from_u32(value: u32) -> Option<Instance> {
        match value {
            0 => Some(Instance::All),
            1 => Some(Instance::Desktop),
            2 => Some(Instance::Console),
            4 => Some(Instance::Web),
            _ => None,
        }
    }
}

impl SteamID {
    /// Attempt to parse a SteamID from a string.
    /// Returns None if the input is not a valid SteamID.
    /// You can pass it any kind of SteamID. (EXCEPT Steam3 IDS TODO!)
    ///
    /// # Examples:
    ///
    /// ```
    /// let steamid = scream_id::SteamID::new("23");
    /// assert_eq!(steamid, None);
    /// ```
    pub fn new(input: &str) -> Option<Self> {
        let mut id = Self {
            universe: Universe::Invalid,
            type_: Type::Invalid,
            instance: Instance::All,
            account_id: 0,
        };

        if let Some(id64) = SteamID::validate_steam64(input) {
            id.universe = Universe::from_u32((id64 >> 56) as u32).unwrap_or(Universe::Invalid);
            id.type_ = Type::from_u32(((id64 >> 52) & 0xF) as u32).unwrap_or(Type::Invalid);
            id.instance = Instance::from_u32(((id64 >> 32) & ACCOUNT_INSTANCE_MASK) as u32)
                .unwrap_or(Instance::All);
            id.account_id = (id64 & ACCOUNT_ID_MASK) as u32;
        } else if let Some(id2) = SteamID::validate_steam2(input) {
            let mut parts = id2.split(':');

            parts.next();
            let universe = parts.next().unwrap();
            let account_id = parts.next().unwrap();

            id.type_ = Type::Individual;
            id.instance = Instance::Desktop;
            id.account_id = account_id.parse::<u32>().unwrap();
            id.universe = match universe.parse::<u32>().unwrap() {
                0 => Universe::Public, // If 0 it should be public?
                _ => {
                    Universe::from_u32(universe.parse::<u32>().unwrap()).unwrap_or(Universe::Public)
                }
            }
        } else {
            return None;
        }

        Some(id)
    }

    /// Tries to render the SteamID as a string.
    ///
    /// # Examples:
    /// ```
    /// let steamid = scream_id::SteamID::new("76561198403256399");
    ///
    /// assert_eq!(steamid.unwrap().render_as_steam2(), Some(String::from("STEAM_0:1:221495335")));
    /// ```
    pub fn render_as_steam2<'a>(self) -> Option<String> {
        if self.type_ != Type::Individual {
            return None;
        }

        let mut universe = self.universe as u32;

        if universe == 1 {
            universe = 0;
        }

        Some(format!(
            "STEAM_{}:{}:{}",
            universe,
            self.instance as u32,
            ((self.account_id / 2) as f64).floor()
        ))
    }

    /*
    pub fn validate_3(input: &str) -> Option<&str> {
        todo!()
    }
     */

    /// Validates a Steam2 id and returns it if it is valid.
    ///
    /// # Examples:
    ///
    /// ```
    /// let id = scream_id::SteamID::validate_steam2("STEAM_0:1:221495335").unwrap();
    ///
    /// assert_eq!(id,"STEAM_0:1:221495335");
    /// ```
    pub fn validate_steam2(input: &str) -> Option<&str> {
        // EG: STEAM_0:0:23071901

        let mut parts = input.split(':');

        if parts.clone().count() != 3 {
            return None;
        }

        let universe = parts.next().unwrap();

        if universe != "STEAM_0" && universe != "STEAM_1" {
            return None;
        }

        Some(input)
    }

    /// Validates a SteamID64 and returns it if it is valid.
    ///
    /// # Examples:
    ///
    /// ```
    /// let id = scream_id::SteamID::validate_steam64("76561198403256399").unwrap();
    ///
    /// assert_eq!(id,76561198403256399);
    /// ```
    pub fn validate_steam64(input: &str) -> Option<u64> {
        if input.len() == 17 {
            if let Ok(id) = u64::from_str_radix(input, 10) {
                if (id & ACCOUNT_ID_MASK) != 0 {
                    return Some(id);
                }
            }
        }

        None
    }
}
