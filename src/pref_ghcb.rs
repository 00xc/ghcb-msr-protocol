use crate::{
	parse_msr, GhcbMsrError, GhcbMsrInfo, GhcbMsrRequest, GhcbMsrResp,
};

/// A request from the guest asking the hypervisor for a preferred GPA
/// to use for the GHCB.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrefGhcbGpaReq {
	data: u64,
}

impl PrefGhcbGpaReq {
	pub const fn new() -> Self {
		Self { data: 0 }
	}
}

impl GhcbMsrRequest for PrefGhcbGpaReq {
	type Resp = PrefGhcbGpaResp;
	fn data(&self) -> u64 {
		self.data
	}
	fn info(&self) -> GhcbMsrInfo {
		GhcbMsrInfo::PREF_GHCB_GPA_REQ
	}
}

/// A response from the hypervisor indicating the preferred GFN for
/// the GHCB (GPA = GFN << 12).
pub struct PrefGhcbGpaResp {
	/// The preferred GFN.
	pub gfn: u64,
}

impl TryFrom<u64> for PrefGhcbGpaResp {
	type Error = GhcbMsrError;
	fn try_from(resp: u64) -> Result<Self, Self::Error> {
		let (info, data) = parse_msr(resp);
		let info = GhcbMsrInfo::try_from(info)?;
		if info != GhcbMsrInfo::PREF_GHCB_GPA_RESP {
			return Err(GhcbMsrError::MismatchedInfo);
		}
		Ok(Self { gfn: data })
	}
}

impl GhcbMsrResp for PrefGhcbGpaResp {}
