// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::Range;

trait Itble<'r, T, I: Iterator<Item=T>> { fn iter(&'r self) -> I; }

impl<'r> Itble<'r, usize, Range<usize>> for (usize, usize) {
    fn iter(&'r self) -> Range<usize> {
        let &(min, max) = self;
        min..max
    }
}

fn check<'r, I: Iterator<Item=usize>, T: Itble<'r, usize, I>>(cont: &T) -> bool
//~^ HELP as shown: fn check<'r, I: Iterator<Item = usize>, T: Itble<'r, usize, I>>(cont: &'r T)
{
    let cont_iter = cont.iter();
//~^ ERROR cannot infer an appropriate lifetime for autoref due to conflicting requirements
    let result = cont_iter.fold(Some(0), |state, val| {
        state.map_or(None, |mask| {
            let bit = 1 << val;
            if mask & bit == 0 {Some(mask|bit)} else {None}
        })
    });
    result.is_some()
}

fn main() {
    check((3, 5));
//~^ ERROR mismatched types
//~| expected `&_`
//~| found `(_, _)`
//~| expected &-ptr
//~| found tuple
//~| HELP run `rustc --explain E0308` to see a detailed explanation
}
