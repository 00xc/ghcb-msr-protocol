use crate::{
	parse_msr, GhcbMsrError, GhcbMsrInfo, GhcbMsrRequest, GhcbMsrResp,
};

/// A request from the guest to retrieve the hypervisor's feature
/// bitmap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatureSupportReq {
	data: u64,
}

impl FeatureSupportReq {
	pub const fn new() -> Self {
		Self { data: 0 }
	}
}

impl GhcbMsrRequest for FeatureSupportReq {
	type Resp = FeatureSupportResp;
	fn data(&self) -> u64 {
		self.data
	}
	fn info(&self) -> GhcbMsrInfo {
		GhcbMsrInfo::FEAT_SUPPORT_REQ
	}
}

/// A response from the hypervisor containing its feature bitmap
pub struct FeatureSupportResp {
	pub features: u64,
}

impl TryFrom<u64> for FeatureSupportResp {
	type Error = GhcbMsrError;
	fn try_from(resp: u64) -> Result<Self, Self::Error> {
		let (info, data) = parse_msr(resp);
		let info = GhcbMsrInfo::try_from(info)?;
		if info != GhcbMsrInfo::FEAT_SUPPORT_RESP {
			return Err(GhcbMsrError::MismatchedInfo);
		}
		Ok(Self { features: data })
	}
}

impl GhcbMsrResp for FeatureSupportResp {}
