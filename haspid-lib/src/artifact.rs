#[derive(Debug, Copy, Clone)]
pub enum Sword {
	Centaur,
	Blackshard,
	Gnoll,
	Ogre,
	Hellfire,
	Gladius,
	Judgement,
	Dragon,
}

impl From<[u32; 2]> for Sword {
	fn from(data: [u32; 2]) -> Self {
		use Sword::*;

		match data[0] {
			0x07 => Centaur,
			0x08 => Blackshard,
			0x09 => Gnoll,
			0x0A => Ogre,
			0x0B => Hellfire,
			0x0C => Gladius,
			0x23 => Judgement,
			0x26 => Dragon,
			_ => panic!("Invalid sword"),
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub enum Shield {
	Dwarven,
	Yawning,
	Gnoll,
	Ogre,
	Damned,
	Sentinel,
	Lion,
	Dragon,
}

impl From<[u32; 2]> for Shield {
	fn from(data: [u32; 2]) -> Self {
		use Shield::*;

		match data[0] {
			0x0D => Dwarven,
			0x0E => Yawning,
			0x0F => Gnoll,
			0x10 => Ogre,
			0x11 => Damned,
			0x12 => Sentinel,
			0x22 => Lion,
			0x27 => Dragon,
			_ => panic!("Invalid shield"),
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub enum Helmet {
	Unicorn,
	Skull,
	Chaos,
	Magi,
	Hellstorm,
	Thunder,
	Enlightenment,
	Dragon,
}

impl From<[u32; 2]> for Helmet {
	fn from(data: [u32; 2]) -> Self {
		use Helmet::*;

		match data[0] {
			0x13 => Unicorn,
			0x14 => Skull,
			0x15 => Chaos,
			0x16 => Magi,
			0x17 => Hellstorm,
			0x18 => Thunder,
			0x24 => Enlightenment,
			0x2C => Dragon,
			_ => panic!("Invalid helmet"),
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub enum Chestplate {
	Wood,
	Rib,
	Basilisk,
	Cyclops,
	Brimstone,
	Cuirass,
	Wonder,
	Dragon,
}

impl From<[u32; 2]> for Chestplate {
	fn from(data: [u32; 2]) -> Self {
		use Chestplate::*;

		match data[0] {
			0x19 => Wood,
			0x1A => Rib,
			0x1B => Basilisk,
			0x1C => Cyclops,
			0x1D => Brimstone,
			0x1E => Cuirass,
			0x1F => Wonder,
			0x28 => Dragon,
			_ => panic!("Invalid chestplate"),
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub enum Necklace {
	Bliss,
	Dragon,
	Dispassion,
	Swiftness,
}

impl From<[u32; 2]> for Necklace {
	fn from(data: [u32; 2]) -> Self {
		use Necklace::*;

		match data[0] {
			0x21 => Bliss,
			0x2B => Dragon,
			0x64 => Dispassion,
			0x61 => Swiftness,
			_ => panic!("Invalid necklace"),
		}
	}
}
