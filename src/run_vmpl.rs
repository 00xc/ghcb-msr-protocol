use crate::{
	parse_msr, GhcbMsrError, GhcbMsrInfo, GhcbMsrRequest, GhcbMsrResp,
};

/// A request to the hypervisor to run the vCPU using the VMSA
/// associated with the request VMPL level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunVmplReq {
	data: u64,
	info: GhcbMsrInfo,
}

impl RunVmplReq {
	pub const fn new(vmpl: u8) -> Self {
		Self {
			data: (vmpl as u64) << 20,
			info: GhcbMsrInfo::RUN_VMPL_REQ,
		}
	}
}

impl GhcbMsrRequest for RunVmplReq {
	type Resp = RunVmplResp;
	fn data(&self) -> u64 {
		self.data
	}
	fn info(&self) -> GhcbMsrInfo {
		self.info
	}
}

/// A response from the hypervisor after requesting running at a
/// specified VPML.
pub struct RunVmplResp {
	/// Non-zero if the hypervisor was unable to run the vCPU at the
	/// requested VPML.
	pub error_code: u32,
}

impl TryFrom<u64> for RunVmplResp {
	type Error = GhcbMsrError;
	fn try_from(resp: u64) -> Result<Self, Self::Error> {
		let (info, data) = parse_msr(resp);
		let info = GhcbMsrInfo::try_from(info)?;
		if info != GhcbMsrInfo::RUN_VMPL_RESP {
			return Err(GhcbMsrError::MismatchedInfo);
		}
		if data & 0xfffff != 0 {
			return Err(GhcbMsrError::InvalidData);
		}
		let error_code = (data >> 20) as u32;
		Ok(Self { error_code })
	}
}

impl GhcbMsrResp for RunVmplResp {}
