mod process_list;

use process_list::ProcessList;

use u16cstr::u16cstr;
use widestring::{U16CStr, U16CString};
use windows::{
	core::{PCSTR, PCWSTR},
	Win32::{
		Foundation::GetLastError,
		System::{
			Diagnostics::Debug::WriteProcessMemory,
			LibraryLoader::{GetModuleHandleW, GetProcAddress},
			Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE},
			Threading::{
				CreateRemoteThread, OpenProcess, PROCESS_CREATE_THREAD, PROCESS_QUERY_INFORMATION,
				PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
			},
		},
	},
};

fn main() -> anyhow::Result<()> {
	let process_entry = ProcessList::new()?
		.into_iter()
		.find(|entry| {
			let name = U16CStr::from_slice_truncate(&entry.szExeFile)
				.unwrap()
				.to_string()
				.unwrap();

			name == "h3hota HD.exe"
		})
		.expect("HotA needs to be running");

	let access_permissions = PROCESS_CREATE_THREAD
		| PROCESS_QUERY_INFORMATION
		| PROCESS_VM_OPERATION
		| PROCESS_VM_READ
		| PROCESS_VM_WRITE;

	let process = unsafe { OpenProcess(access_permissions, false, process_entry.th32ProcessID) }?;

	let target_address = unsafe {
		VirtualAllocEx(
			process,
			std::ptr::null(),
			1024,
			MEM_RESERVE | MEM_COMMIT,
			PAGE_EXECUTE_READWRITE,
		)
	};
	if target_address.is_null() {
		unsafe { GetLastError() }.ok()?;
	}

	let dll_path = std::path::PathBuf::from("target/i686-pc-windows-msvc/debug/haspid_lib.dll")
		.canonicalize()?;
	let dll_path = U16CString::from_os_str(dll_path)?;

	let mut written = 0;
	unsafe {
		WriteProcessMemory(
			process,
			target_address,
			dll_path.as_ptr() as _,
			(dll_path.len() + 1) * std::mem::size_of::<u16>(),
			&mut written,
		)
	}
	.ok()?;
	assert!(written > 0);

	let kernel32 = unsafe { GetModuleHandleW(PCWSTR(u16cstr!("kernel32.dll").as_ptr())) };

	let load_library_w = unsafe { GetProcAddress(kernel32, PCSTR("LoadLibraryW\0".as_ptr())) }
		.ok_or_else(|| anyhow::anyhow!("Couldn't find LoadLibraryW"))?;

	let mut tid = 0u32;
	let _thread = unsafe {
		CreateRemoteThread(
			process,
			std::ptr::null(),
			0,
			Some(std::mem::transmute(load_library_w)),
			target_address,
			0,
			&mut tid,
		)
	}?;

	Ok(())
}
