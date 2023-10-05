#![no_std]
#![forbid(unsafe_code)]

//! A library providing strongly typed and error-checked primitives
//! for the guest-side communication of the AMD
//! [SEV-ES Guest-Hypervisor Communication Block](https://www.amd.com/system/files/TechDocs/56421-guest-hypervisor-communication-block-standardization.pdf)
//! (GHCB) MSR protocol (section 2.3.1).
//!
//! The crate is only concerned with the creation of correct requests,
//! and parsing and error-checking the responses from the hypervisor.
//! Interacting with the MSRs and handling the responses are left to
//! the user.
//!
//! The crate does not perform any allocations, does not depend on the
//! standard Rust library and has no other external dependencies. It
//! also uses [`#![forbid(unsafe_code)]`](https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html#how-safe-and-unsafe-interact).
//!
//! # Usage
//!
//! The usage flow for this crate is very simple. The user simply
//! creates requests using the provided types, writes them to the
//! GHCB MSR, and parses them back using the provided facilities as
//! well.
//!
//! The library is designed to be strictly type safe. Specifically,
//! this is done through the [`GhcbMsrRequest`] trait, which every
//! request implements. Every implementor of the trait has a method (
//! [`GhcbMsrRequest::response()`]) to parse the corresponding
//! response, adding extra sanity checks for that specific response
//! if necessary. The request and response types are tied through
//! the [`GhcbMsrRequest::Resp`] generic associated type.
//!
//! ## Example
//!
//! ```no_run
//! # fn wrmsr(msr: u32, val: u64) { }
//! # fn rdmsr(msr: u32) -> u64 { 0xdeadbeef }
//! # fn vmgexit() { }
//! # const GHCB_MSR: u32 = 0xC0010130;
//! use ghcb_msr_protocol::{
//!     sev_info::SevInfoReq,
//!     GhcbMsrRequest
//! };
//!
//! // Create the request and send it to the hypervisor
//! let req = SevInfoReq::new();
//! wrmsr(GHCB_MSR, req.msr());
//! vmgexit();
//!
//! // Read the response back and parse it
//! let raw_resp = rdmsr(GHCB_MSR);
//! match req.response(raw_resp) {
//!     Ok(r) => println!("Min. protocol version: {}", r.min_ver),
//!     Err(e) => println!("Error: {:?}", e),
//! }
//! ```

use core::fmt::Debug;

/// SEV capability information.
pub mod sev_info;

/// CPUID retrieval.
pub mod cpuid;

/// INIT-SIPI-SIPI sequence synchronization.
pub mod ap_reset_hold;

/// Hypervisor GHCB GPA preference.
pub mod pref_ghcb;

/// GHCB registration.
pub mod register_ghcb;

/// Page state changes.
pub mod page_state;

/// VMPL switching.
pub mod run_vmpl;

/// Hypervisor feature support.
pub mod feature_support;

/// Guest termination.
pub mod termination;

/// Potential errors encountered when parsing the hypervisor's response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GhcbMsrError {
	/// Invalid (unknown) response code.
	InvalidInfo,
	/// The response code does not match our request.
	MismatchedInfo,
	/// Format of the data section is invalid.
	InvalidData,
	/// The data in the response does not match our request. This can
	/// happen if the registered GPA for
	/// [`RegisterGhcbResp`](register_ghcb::RegisterGhcbResp)
	/// does not match the one in the corresponding
	/// [`RegisterGhcbReq`](register_ghcb::RegisterGhcbReq).
	MismatchedData(u64),
	/// The guest requested termination and the hypervisor did not
	/// comply.
	ShouldNotReturn,
}

/// Request/response codes for the MSR protocol. These are returned by
/// the [`GhcbMsrRequest::info()`] method of the request types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum GhcbMsrInfo {
	GHCB_GPA = 0x00,
	/// See [`sev_info::SevInfoReq`].
	SEV_INFO_RESP = 0x01,
	/// See [`sev_info::SevInfoResp`].
	SEV_INFO_REQ = 0x02,
	/// See [`cpuid::CpuidReq`].
	CPUID_REQ = 0x04,
	/// See [`cpuid::CpuidResp`].
	CPUID_RESP = 0x05,
	/// See [`ap_reset_hold::ApResetHoldReq`].
	AP_RESET_HOLD_REQ = 0x06,
	/// See [`ap_reset_hold::ApResetHoldResp`].
	AP_RESET_HOLD_RESP = 0x07,
	/// See [`pref_ghcb::PrefGhcbGpaReq`].
	PREF_GHCB_GPA_REQ = 0x10,
	/// See [`pref_ghcb::PrefGhcbGpaResp`].
	PREF_GHCB_GPA_RESP = 0x11,
	/// See [`register_ghcb::RegisterGhcbReq`].
	REG_GHCB_GPA_REQ = 0x12,
	/// See [`register_ghcb::RegisterGhcbResp`].
	REG_GHCB_GPA_RESP = 0x13,
	/// See [`page_state::PageStateReq`].
	STATE_CHANGE_REQ = 0x14,
	/// See [`page_state::PageStateResp`].
	STATE_CHANGE_RESP = 0x15,
	/// See [`run_vmpl::RunVmplReq`].
	RUN_VMPL_REQ = 0x16,
	/// See [`run_vmpl::RunVmplResp`].
	RUN_VMPL_RESP = 0x17,
	/// See [`feature_support::FeatureSupportReq`].
	FEAT_SUPPORT_REQ = 0x80,
	/// See [`feature_support::FeatureSupportResp`].
	FEAT_SUPPORT_RESP = 0x81,
	/// See [`termination::TerminationReq`].
	TERM_REQ = 0x100,
}

impl TryFrom<u16> for GhcbMsrInfo {
	type Error = GhcbMsrError;
	fn try_from(val: u16) -> Result<Self, Self::Error> {
		match val {
			v if v == Self::GHCB_GPA as u16 => Ok(Self::GHCB_GPA),
			v if v == Self::SEV_INFO_RESP as u16 => {
				Ok(Self::SEV_INFO_RESP)
			}
			v if v == Self::SEV_INFO_REQ as u16 => {
				Ok(Self::SEV_INFO_REQ)
			}
			v if v == Self::CPUID_REQ as u16 => Ok(Self::CPUID_REQ),
			v if v == Self::CPUID_RESP as u16 => Ok(Self::CPUID_RESP),
			v if v == Self::AP_RESET_HOLD_REQ as u16 => {
				Ok(Self::AP_RESET_HOLD_REQ)
			}
			v if v == Self::AP_RESET_HOLD_RESP as u16 => {
				Ok(Self::AP_RESET_HOLD_RESP)
			}
			v if v == Self::PREF_GHCB_GPA_REQ as u16 => {
				Ok(Self::PREF_GHCB_GPA_REQ)
			}
			v if v == Self::PREF_GHCB_GPA_RESP as u16 => {
				Ok(Self::PREF_GHCB_GPA_RESP)
			}
			v if v == Self::REG_GHCB_GPA_REQ as u16 => {
				Ok(Self::REG_GHCB_GPA_REQ)
			}
			v if v == Self::REG_GHCB_GPA_RESP as u16 => {
				Ok(Self::REG_GHCB_GPA_RESP)
			}
			v if v == Self::STATE_CHANGE_REQ as u16 => {
				Ok(Self::STATE_CHANGE_REQ)
			}
			v if v == Self::STATE_CHANGE_RESP as u16 => {
				Ok(Self::STATE_CHANGE_RESP)
			}
			v if v == Self::RUN_VMPL_REQ as u16 => {
				Ok(Self::RUN_VMPL_REQ)
			}
			v if v == Self::RUN_VMPL_RESP as u16 => {
				Ok(Self::RUN_VMPL_RESP)
			}
			v if v == Self::FEAT_SUPPORT_REQ as u16 => {
				Ok(Self::FEAT_SUPPORT_REQ)
			}
			v if v == Self::FEAT_SUPPORT_RESP as u16 => {
				Ok(Self::FEAT_SUPPORT_RESP)
			}
			v if v == Self::TERM_REQ as u16 => Ok(Self::TERM_REQ),
			_ => Err(GhcbMsrError::InvalidInfo),
		}
	}
}

/// Trait implemented by all GHCB MSR requests.
pub trait GhcbMsrRequest {
	type Resp: GhcbMsrResp;
	/// The GHCBInfo segment of the MSR.
	fn info(&self) -> GhcbMsrInfo;
	/// The GHCBData segment of the MSR.
	fn data(&self) -> u64;
	/// Parse a response from the hypervisor, with additional checks
	/// based on the request that originated it.
	fn response(
		&self,
		resp: u64,
	) -> Result<Self::Resp, GhcbMsrError> {
		Self::Resp::try_from(resp)
	}
	/// The numeric value of the request that should be written to
	/// the MSR.
	fn msr(&self) -> u64 {
		((self.data() & 0xfffffffffffff) << 12)
			| (self.info() as u64 & 0xfff)
	}
}

/// Trait implemented by all GHCB MSR responses.
pub trait GhcbMsrResp: TryFrom<u64, Error = GhcbMsrError> {}

fn parse_msr(msr: u64) -> (u16, u64) {
	((msr & 0xfff) as u16, msr >> 12)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[should_panic]
	#[test]
	fn it_works() {
		let req = sev_info::SevInfoReq::new();
		let _ = req.response(0).unwrap();
	}
}
