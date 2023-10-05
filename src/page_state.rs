use crate::{
	parse_msr, GhcbMsrError, GhcbMsrInfo, GhcbMsrRequest, GhcbMsrResp,
};

/// The state that the page will be set to after a successful
/// [`PageStateReq`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PageOp {
	Private = 1,
	Shared = 2,
}

/// A request from the guest to change the state of a page specified
/// with a GFN.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageStateReq {
	data: u64,
}

impl PageStateReq {
	pub const fn new(gfn: u64, op: PageOp) -> Self {
		let op = op as u64;
		Self {
			data: (op << 40) | gfn,
		}
	}
}

impl GhcbMsrRequest for PageStateReq {
	type Resp = PageStateResp;
	fn data(&self) -> u64 {
		self.data
	}
	fn info(&self) -> GhcbMsrInfo {
		GhcbMsrInfo::STATE_CHANGE_REQ
	}
}

/// A response from the hypervisor indicating whether the page had its
/// state changed.
pub struct PageStateResp {
	/// The error code. A non-zero value indicates an error occurred
	/// and the page did not change state.
	pub error_code: u32,
}

impl TryFrom<u64> for PageStateResp {
	type Error = GhcbMsrError;
	fn try_from(resp: u64) -> Result<Self, Self::Error> {
		let (info, data) = parse_msr(resp);
		let info = GhcbMsrInfo::try_from(info)?;
		if info != GhcbMsrInfo::STATE_CHANGE_RESP {
			return Err(GhcbMsrError::MismatchedInfo);
		}
		if data & 0xfffff != 0 {
			return Err(GhcbMsrError::InvalidData);
		}

		let error_code = (data >> 20) as u32;
		Ok(Self { error_code })
	}
}

impl GhcbMsrResp for PageStateResp {}
