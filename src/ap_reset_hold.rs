use crate::{
	parse_msr, GhcbMsrError, GhcbMsrInfo, GhcbMsrRequest, GhcbMsrResp,
};

/// A request from the guest for the AP be placed in a HLT loop
/// awaiting an INIT-SIPI-SIPI request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApResetHoldReq {
	data: u64,
	info: GhcbMsrInfo,
}

impl ApResetHoldReq {
	pub const fn new() -> Self {
		Self {
			data: 0,
			info: GhcbMsrInfo::AP_RESET_HOLD_REQ,
		}
	}
}

impl GhcbMsrRequest for ApResetHoldReq {
	type Resp = ApResetHoldResp;
	fn data(&self) -> u64 {
		self.data
	}
	fn info(&self) -> GhcbMsrInfo {
		self.info
	}
}

/// A response from the hypervisor after an INIT-SIPI-SIPI sequence
/// has been received for the targeted AP to take it out of HLT.
pub struct ApResetHoldResp {}

impl TryFrom<u64> for ApResetHoldResp {
	type Error = GhcbMsrError;
	fn try_from(resp: u64) -> Result<Self, Self::Error> {
		let (info, data) = parse_msr(resp);
		let info = GhcbMsrInfo::try_from(info)?;
		if info != GhcbMsrInfo::AP_RESET_HOLD_RESP {
			return Err(GhcbMsrError::MismatchedInfo);
		}
		if data == 0 {
			return Err(GhcbMsrError::InvalidData);
		}
		Ok(Self {})
	}
}

impl GhcbMsrResp for ApResetHoldResp {}
