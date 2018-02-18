// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Reference: RISC-V ELF psABI specification
// https://github.com/riscv/riscv-elf-psabi-doc

use abi::{ArgType, FnType};

// Procedure Calling Convention
//
// Scalars wider than 2✕XLEN are passed by reference and are replaced in the
// argument list with the address.
//
// Aggregates larger than 2✕XLEN bits are passed by reference and are replaced
// in the argument list with the address, as are C++ aggregates with nontrivial
// copy constructors, destructors, or vtables.
//

// 32-bit Architecture
const XLEN: u64 = 32;

fn classify_ret_ty(ret: &mut ArgType) {
    if ret.layout.size.bits() > 2 * XLEN {
        ret.make_indirect();
    }
}

fn classify_arg_ty(arg: &mut ArgType) {
    if arg.layout.size.bits() > 2 * XLEN {
        arg.make_indirect();
    }
}

pub fn compute_abi_info(fty: &mut FnType) {
    if !fty.ret.is_ignore() {
        classify_ret_ty(&mut fty.ret);
    }

    for arg in &mut fty.args {
        if arg.is_ignore() {
            continue;
        }
        classify_arg_ty(arg);
    }
}
