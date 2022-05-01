pub mod artifact;

use axum::{routing::get, Router};

// NOTE: This magic number changes between versions. If this crashes or does nothing, this is probably the cause
const HEROES_LIST_ADDRESS: u32 = 0x55f413b8;
const HEROES_AMOUNT: usize = 170;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct HeroDataRaw {
	_padding_1: [u8; 34],
	player: u8,
	name: [u8; 13],
	_padding_2: [u8; 97],
	unit_types: [u8; 28],
	unit_amounts: [u8; 28],
	_padding_3: [u8; 100],
	equipped_artifacts: [u8; 152],
	_padding_4: [u8; 15],
	backpack_artifacts: [u8; 256],
	_padding_5: [u8; 418],
	primary_skills: [u8; 4],
	_padding_6: [u8; 24],
}

struct HeroData {
	player: Option<u8>,
	name: String,
	primary_skills: [u8; 4],
	equipped_artifacts: [u8; 152],
	backpack_artifacts: [u8; 256],
	unit_types: [u8; 28],
	unit_amounts: [u8; 28],
}

impl From<HeroDataRaw> for HeroData {
	fn from(raw: HeroDataRaw) -> Self {
		let HeroDataRaw {
			_padding_1,
			player,
			name,
			_padding_2,
			unit_types,
			unit_amounts,
			_padding_3,
			equipped_artifacts,
			_padding_4,
			backpack_artifacts,
			_padding_5,
			primary_skills,
			_padding_6,
		} = raw;

		let player = if player == 0xFF { None } else { Some(player) };
		let name = unsafe { std::ffi::CStr::from_ptr(name.as_ptr() as _) }
			.to_str()
			.unwrap()
			.to_string();

		Self {
			player,
			name,
			primary_skills,
			equipped_artifacts,
			backpack_artifacts,
			unit_types,
			unit_amounts,
		}
	}
}

fn get_all_heroes() -> [HeroData; HEROES_AMOUNT] {
	let heroes_list_base =
		unsafe { (HEROES_LIST_ADDRESS as *const *const HeroDataRaw).read_volatile() };

	let heroes_list_raw: &'static [HeroDataRaw; HEROES_AMOUNT] =
		unsafe { std::mem::transmute(heroes_list_base) };

	heroes_list_raw.map(|raw: HeroDataRaw| HeroData::from(raw))
}

fn get_active_heroes() -> Vec<HeroData> {
	let heroes_list_base =
		unsafe { (HEROES_LIST_ADDRESS as *const *const HeroDataRaw).read_volatile() };

	let heroes_list_raw: &'static [HeroDataRaw; HEROES_AMOUNT] =
		unsafe { std::mem::transmute(heroes_list_base) };

	let mut list = Vec::new();

	for raw in heroes_list_raw {
		if raw.player != 0xFF {
			list.push(HeroData::from(*raw));
		}
	}

	list
}

#[ctor::ctor]
fn ctor() {
	std::thread::spawn(move || {
		tokio::runtime::Builder::new_current_thread()
			.enable_all()
			.build()
			.unwrap()
			.block_on(async move {
				let app = Router::new().route(
					"/",
					get(move || async move {
						let mut response = String::new();

						for hero in get_active_heroes() {
							response.push_str(&format!(
								"{}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}\n",
								hero.name,
								hero.player,
								hero.primary_skills,
								hero.equipped_artifacts,
								hero.backpack_artifacts,
								hero.unit_types,
								hero.unit_amounts
							));
						}

						format!("{response}")
					}),
				);

				axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
					.serve(app.into_make_service())
					.await
					.unwrap();
			});
	});
}
