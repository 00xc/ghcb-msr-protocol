use crate::{
	parse_msr, GhcbMsrError, GhcbMsrInfo, GhcbMsrRequest, GhcbMsrResp,
};

/// A request for the hypervisor to provide SEV information needed to
/// perform protocol negotiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SevInfoReq {
	data: u64,
}

impl SevInfoReq {
	pub const fn new() -> Self {
		Self { data: 0 }
	}
}

impl GhcbMsrRequest for SevInfoReq {
	type Resp = SevInfoResp;
	fn data(&self) -> u64 {
		self.data
	}
	fn info(&self) -> GhcbMsrInfo {
		GhcbMsrInfo::SEV_INFO_REQ
	}
}

/// A response from the hypervisor indicating its capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SevInfoResp {
	/// Maximum GHCB protocol version supported
	pub max_ver: u16,
	/// Minimum GHCB protocol version supported
	pub min_ver: u16,
	/// Required by the guest when entering long mode to mark the GHCB
	/// page as decrypted.
	pub enc_bit_no: u8,
}

impl TryFrom<u64> for SevInfoResp {
	type Error = GhcbMsrError;
	fn try_from(resp: u64) -> Result<Self, Self::Error> {
		let (info, data) = parse_msr(resp);
		let info = GhcbMsrInfo::try_from(info)?;
		if info != GhcbMsrInfo::SEV_INFO_RESP {
			return Err(GhcbMsrError::MismatchedInfo);
		}

		let enc_bit_no = ((data >> 12) & 0xff) as u8;
		let min_ver = ((data >> 20) & 0xffff) as u16;
		let max_ver = ((data >> 36) & 0xffff) as u16;
		Ok(Self {
			max_ver,
			min_ver,
			enc_bit_no,
		})
	}
}

impl GhcbMsrResp for SevInfoResp {}
