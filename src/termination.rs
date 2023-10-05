use crate::{GhcbMsrError, GhcbMsrInfo, GhcbMsrRequest, GhcbMsrResp};

/// The reason for a [`TerminationReq`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TerminationReason {
	/// General termination request.
	GeneralTermination = 1,
	/// SEV-ES/GHCB Protocol range is not supported.
	GhcbProtRangeNotSupported = 2,
	/// SEV-SNP features not supported
	SevSnpNotSupported = 3,
}

/// A request from the guest to be terminated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminationReq {
	data: u64,
}

impl TerminationReq {
	pub const fn new(
		code_set: u8,
		reason: TerminationReason,
	) -> Self {
		let code_set = code_set as u64;
		let reason = reason as u64;
		Self {
			data: (reason << 4) | code_set,
		}
	}
}

impl GhcbMsrRequest for TerminationReq {
	type Resp = TerminationResp;
	fn data(&self) -> u64 {
		self.data
	}
	fn info(&self) -> GhcbMsrInfo {
		GhcbMsrInfo::TERM_REQ
	}
}

#[doc(hidden)]
pub struct TerminationResp {}

impl TryFrom<u64> for TerminationResp {
	type Error = GhcbMsrError;
	fn try_from(_resp: u64) -> Result<Self, Self::Error> {
		Err(GhcbMsrError::ShouldNotReturn)
	}
}

impl GhcbMsrResp for TerminationResp {}
