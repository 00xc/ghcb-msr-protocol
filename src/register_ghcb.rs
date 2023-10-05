use crate::{
	parse_msr, GhcbMsrError, GhcbMsrInfo, GhcbMsrRequest, GhcbMsrResp,
};

/// A request from the guest to indicate to the hypervisor the GFN
/// (`GFN << 12 = GPA`) of the GHCB page for the vCPU invoking the
/// `VMGEXIT`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegisterGhcbReq {
	data: u64,
}

impl RegisterGhcbReq {
	pub const fn new(gfn: u64) -> Self {
		Self { data: gfn }
	}
}

impl GhcbMsrRequest for RegisterGhcbReq {
	type Resp = RegisterGhcbResp;
	fn data(&self) -> u64 {
		self.data
	}
	fn info(&self) -> GhcbMsrInfo {
		GhcbMsrInfo::REG_GHCB_GPA_REQ
	}
	fn response(
		&self,
		resp: u64,
	) -> Result<Self::Resp, GhcbMsrError> {
		let resp = Self::Resp::try_from(resp)?;
		if resp.gfn != self.data {
			return Err(GhcbMsrError::MismatchedData(resp.gfn));
		}
		Ok(resp)
	}
}

/// A response from the hypervisor after a request to register a GHCB
/// GFN.
pub struct RegisterGhcbResp {
	/// The registered GHCB GFN.
	pub gfn: u64,
}

impl TryFrom<u64> for RegisterGhcbResp {
	type Error = GhcbMsrError;
	fn try_from(resp: u64) -> Result<Self, Self::Error> {
		let (info, data) = parse_msr(resp);
		let info = GhcbMsrInfo::try_from(info)?;
		if info != GhcbMsrInfo::REG_GHCB_GPA_RESP {
			return Err(GhcbMsrError::MismatchedInfo);
		}
		Ok(Self { gfn: data })
	}
}

impl GhcbMsrResp for RegisterGhcbResp {}
