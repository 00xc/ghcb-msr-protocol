use crate::{
	parse_msr, GhcbMsrError, GhcbMsrInfo, GhcbMsrRequest, GhcbMsrResp,
};

/// Requested register value for a [`CpuidReq`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CpuidReg {
	EAX = 0,
	EBX = 1,
	ECX = 2,
	EDX = 3,
}

impl From<u8> for CpuidReg {
	fn from(val: u8) -> Self {
		match val {
			_ if val == Self::EAX as u8 => Self::EAX,
			_ if val == Self::EBX as u8 => Self::EBX,
			_ if val == Self::ECX as u8 => Self::ECX,
			_ if val == Self::EDX as u8 => Self::EDX,
			_ => unreachable!(),
		}
	}
}

/// A request from the guest to retrieve a CPUID function register
/// value from the hypervisor. Needed by the guest before the GHCB
/// can be set up. Only one register can be obtained at a time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuidReq {
	data: u64,
}

impl CpuidReq {
	pub const fn new(cpuid_function: u32, reg: CpuidReg) -> Self {
		let func = cpuid_function as u64;
		let reg = reg as u64;
		Self {
			data: (func << 20) | (reg << 18),
		}
	}
}

impl GhcbMsrRequest for CpuidReq {
	type Resp = CpuidResp;
	fn data(&self) -> u64 {
		self.data
	}
	fn info(&self) -> GhcbMsrInfo {
		GhcbMsrInfo::CPUID_REQ
	}
}

/// A response from the hypervisor to a CPUID request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuidResp {
	/// CPUID function register value
	pub function: u32,
	/// Returned CPUID register value
	pub reg: CpuidReg,
}

impl TryFrom<u64> for CpuidResp {
	type Error = GhcbMsrError;
	fn try_from(resp: u64) -> Result<Self, Self::Error> {
		let (info, data) = parse_msr(resp);
		let info = GhcbMsrInfo::try_from(info)?;
		if info != GhcbMsrInfo::CPUID_RESP {
			return Err(GhcbMsrError::MismatchedInfo);
		}
		if data & 0xff != 0 {
			return Err(GhcbMsrError::InvalidData);
		}
		let function = ((data >> 20) & 0xffffffff) as u32;
		let reg = CpuidReg::from(((data >> 18) & 0b11) as u8);
		Ok(Self { function, reg })
	}
}

impl GhcbMsrResp for CpuidResp {}
