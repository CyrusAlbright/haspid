use windows::core::Result;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::ToolHelp::{
	CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};

pub struct ProcessList {
	snapshot: HANDLE,
	first: bool,
}

impl ProcessList {
	pub fn new() -> Result<Self> {
		let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)? };

		Ok(Self {
			snapshot,
			first: true,
		})
	}
}

impl Iterator for ProcessList {
	type Item = PROCESSENTRY32W;

	fn next(&mut self) -> Option<Self::Item> {
		let mut entry = PROCESSENTRY32W {
			dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
			..Default::default()
		};

		if self.first {
			self.first = false;
			unsafe { Process32FirstW(self.snapshot, &mut entry) }
		} else {
			unsafe { Process32NextW(self.snapshot, &mut entry) }
		}
		.ok()
		.ok()
		.map(|_| entry)
	}
}
