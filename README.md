ghcb-msr-protocol
=================

A library providing strongly typed and error-checked primitives
for the guest-side communication of the AMD
[SEV-ES Guest-Hypervisor Communication Block](https://www.amd.com/system/files/TechDocs/1-guest-hypervisor-communication-block-standardization.pdf)
(GHCB) MSR protocol (section 2.3.1).

The crate is only concerned with the creation of correct requests,
and parsing and error-checking the responses from the hypervisor.
Interacting with the MSRs and handling the responses are left to
the user.

The crate does not perform any allocations, does not depend on the
standard Rust library and has no other external dependencies. It
also uses
[`#![forbid(unsafe_code)]`](https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html#safe-and-unsafe-interact).

Usage
-----

Refer to the generated cargo documentation.

Example
-------

```rust
use ghcb_msr_protocol::{
    sev_info::SevInfoReq,
    GhcbMsrRequest
};

// Create the request and send it to the hypervisor
let req = SevInfoReq::new();
wrmsr(GHCB_MSR, req.msr());
vmgexit();

// Read the response back and parse it
let raw_resp = rdmsr(GHCB_MSR);
match req.response(raw_resp) {
    Ok(r) => println!("Min. protocol version: {}", r.min_ver),
    Err(e) => println!("Error: {:?}", e),
}
```
