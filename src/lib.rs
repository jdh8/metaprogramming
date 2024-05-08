// This file is part of the metaprogramming project.
//
// Copyright (C) 2024 Chen-Pang He <jdh8@skymizer.com>
//
// This Source Code Form is subject to the terms of the Mozilla
// Public License v. 2.0. If a copy of the MPL was not distributed
// with this file, You can obtain one at http://mozilla.org/MPL/2.0/.
#![no_std]

use core::marker::PhantomData;

pub trait Truth {}

pub struct BoolConstant<const VALUE: bool>;
impl Truth for BoolConstant<true> {}

pub struct Same<T, U>(PhantomData<T>, PhantomData<U>);

#[allow(clippy::mismatching_type_param_order)]
impl<T> Truth for Same<T, T> {}

pub trait AssociatedType {
    type Type;
}

pub struct Conditional<const COND: bool, T, F>(PhantomData<T>, PhantomData<F>);

impl<T, F> AssociatedType for Conditional<true, T, F> {
    type Type = T;
}

impl<T, F> AssociatedType for Conditional<false, T, F> {
    type Type = F;
}

pub type ConditionalType<const COND: bool, T, F> = <Conditional<COND, T, F> as AssociatedType>::Type;

pub struct EnableIf<const COND: bool, T>(PhantomData<T>);

impl<T> AssociatedType for EnableIf<true, T> {
    type Type = T;
}

pub type EnableIfType<const COND: bool, T> = <EnableIf<COND, T> as AssociatedType>::Type;