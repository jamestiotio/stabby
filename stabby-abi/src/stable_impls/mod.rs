//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   Pierre Avital, <pierre.avital@me.com>
//

use crate::{istable::Or, str::Str, *};

macro_rules! same_as {
    ($t: ty, $($name: tt)*) => {
        type Align = <$t as IStable>::Align;
        type Size = <$t as IStable>::Size;
        type UnusedBits = <$t as IStable>::UnusedBits;
        type ForbiddenValues = <$t as IStable>::ForbiddenValues;
        type HasExactlyOneNiche = <$t as IStable>::HasExactlyOneNiche;
        primitive_report!($($name)*);
    };
    ($t: ty) => {
        type Align = <$t as IStable>::Align;
        type Size = <$t as IStable>::Size;
        type UnusedBits = <$t as IStable>::UnusedBits;
        type ForbiddenValues = <$t as IStable>::ForbiddenValues;
        type HasExactlyOneNiche = <$t as IStable>::HasExactlyOneNiche;
    };
}

#[allow(dead_code)]
const ARCH: &[u8] = _ARCH;
#[cfg(target_arch = "x86")]
const _ARCH: &[u8] = b"x86";
#[cfg(target_arch = "x86_64")]
const _ARCH: &[u8] = b"x86_64";
#[cfg(target_arch = "arm")]
const _ARCH: &[u8] = b"arm";
#[cfg(target_arch = "aarch64")]
const _ARCH: &[u8] = b"aarch64";
#[cfg(target_arch = "loongarch64")]
const _ARCH: &[u8] = b"loongarch64";
#[cfg(target_arch = "m68k")]
const _ARCH: &[u8] = b"m68k";
#[cfg(target_arch = "mips")]
const _ARCH: &[u8] = b"mips";
#[cfg(target_arch = "mips64")]
const _ARCH: &[u8] = b"mips64";
#[cfg(target_arch = "powerpc")]
const _ARCH: &[u8] = b"powerpc";
#[cfg(target_arch = "powerpc64")]
const _ARCH: &[u8] = b"powerpc64";
#[cfg(target_arch = "riscv64")]
const _ARCH: &[u8] = b"riscv64";
#[cfg(target_arch = "s390x")]
const _ARCH: &[u8] = b"s390x";
#[cfg(target_arch = "sparc64")]
const _ARCH: &[u8] = b"sparc64";

macro_rules! check {
    ($t: ty) => {
        #[cfg(not(docsrs))]
        const _: () = {
            let mut buffer = [0; 1024];
            let mut len = 0;
            macro_rules! write {
                ($e: expr) => {{
                    let mut i = 0;
                    let e = $e;
                    while i < e.len() {
                        buffer[len + i] = e[i];
                        i += 1;
                    }
                    len += e.len();
                }};
            }

            let stabby = <<$t as IStable>::Align as crate::typenum2::Unsigned>::USIZE as u8;
            let rust = core::mem::align_of::<$t>() as u8;
            if stabby != rust {
                write!(stringify!($t).as_bytes());
                write!(b"'s alignment was mis-evaluated by stabby, this is definitely a bug and may cause UB. Please create an issue using this link: https://github.com/ZettaScaleLabs/stabby/issues/new?title=");
                write!(stringify!($t).as_bytes());
                write!(b"%20misaligned%20on%20");
                write!(crate::stable_impls::ARCH);
                write!(b"&body=Stabby%20says%20");
                const fn fmt2digit(n: u8) -> [u8; 2] {
                    [b'0' + (n / 10), b'0' + (n % 10)]
                }
                write!(fmt2digit(stabby));
                write!(b"%2C%20Rust%20says%20");
                write!(fmt2digit(rust));
                write!(b"\r\n");
            }
            let stabby = <<$t as IStable>::Size as crate::typenum2::Unsigned>::USIZE ;
            let rust = core::mem::size_of::<$t>() ;
            if stabby != rust {
                write!(stringify!($t).as_bytes());
                write!(b"'s size was mis-evaluated by stabby, this is definitely a bug and may cause UB. Please create an issue using this link: https://github.com/ZettaScaleLabs/stabby/issues/new?title=");
                write!(stringify!($t).as_bytes());
                write!(b"%20missized%20on%20");
                write!(crate::stable_impls::ARCH);
                write!(b"&body=Stabby%20says%20");
                const fn fmt6digit(n: usize) -> [u8; 6] {
                    [
                        b'0' + ((n / 100000) % 10) as u8,
                        b'0' + ((n / 10000) % 10) as u8,
                        b'0' + ((n / 1000) % 10) as u8,
                        b'0' + ((n / 100) % 10) as u8,
                        b'0' + ((n / 10) % 10) as u8,
                        b'0' + (n % 10) as u8
                    ]
                }
                write!(fmt6digit(stabby));
                write!(b"%2C%20Rust%20says%20");
                write!(fmt6digit(rust));
                write!(b"\r\n");
            }
            match <<$t as IStable>::HasExactlyOneNiche as crate::istable::ISaturatingAdd>::VALUE {
                crate::istable::SaturatingAddValue::B0 => {
                    if rust == core::mem::size_of::<Option<$t>>() {
                        write!(stringify!($t).as_bytes());
                        write!(b"'s niches were mis-evaluated by stabby, this is definitely a bug and may cause UB. Please create an issue using this link: https://github.com/ZettaScaleLabs/stabby/issues/new?title=");
                        write!(stringify!($t).as_bytes());
                        write!(b"%20has%20niches%20but%20stabby%20does%20not%20find%20any");
                        write!(b"\r\n");
                    }
                }
                crate::istable::SaturatingAddValue::B1 => {
                    if rust != core::mem::size_of::<Option<$t>>() {
                        write!(stringify!($t).as_bytes());
                        write!(b"'s niches were mis-evaluated by stabby, this is definitely a bug and may cause UB. Please create an issue using this link: https://github.com/ZettaScaleLabs/stabby/issues/new?title=");
                        write!(stringify!($t).as_bytes());
                        write!(b"%20has%20no%20niches%20but%20stabby%20claims%20one%20found%20exactly");
                        write!(b"\r\n");
                    }
                    if rust == core::mem::size_of::<Option<Option<$t>>>() {
                        write!(stringify!($t).as_bytes());
                        write!(b"'s niches were mis-evaluated by stabby, this is definitely a bug and may cause UB. Please create an issue using this link: https://github.com/ZettaScaleLabs/stabby/issues/new?title=");
                        write!(stringify!($t).as_bytes());
                        write!(b"%20has%20multiple%20niches%20but%20stabby%20claims%20one%20found%20exactly");
                        write!(b"\r\n");
                    }
                }
                _ => {}
            }
            if len != 0 {
                panic!("{}", unsafe {
                    core::str::from_utf8_unchecked(core::slice::from_raw_parts(buffer.as_ptr(), len))
                })
            }
        };
    };
}

macro_rules! nz_holes {
    ($t: ty) => {
        Array<$t, U0, End>
    };
    ($t: ty, $($tt: tt)*) => {
        Array<$t, U0, nz_holes!($($tt)*)>
    };
}
unsafe impl IStable for () {
    type Size = U0;
    type Align = U1;
    type ForbiddenValues = End;
    type UnusedBits = End;
    type HasExactlyOneNiche = B0;
    type ContainsIndirections = B0;
    primitive_report!("()");
}
unsafe impl<T> IStable for core::marker::PhantomData<T> {
    type Size = U0;
    type Align = U1;
    type ForbiddenValues = End;
    type UnusedBits = End;
    type HasExactlyOneNiche = B0;
    type ContainsIndirections = B0;
    primitive_report!("core::marker::PhantomData");
}
unsafe impl IStable for core::marker::PhantomPinned {
    type Size = U0;
    type Align = U1;
    type ForbiddenValues = End;
    type UnusedBits = End;
    type HasExactlyOneNiche = B0;
    type ContainsIndirections = B0;
    primitive_report!("core::marker::PhantomPinned");
}
macro_rules! illegal_values {
    ([$($l: tt)*], [$($r: tt)*]) => {
        Or<illegal_values!($($l)*), illegal_values!($($r)*)>
    };
    ($t: ty, $($tt: tt)*) => {
        Or<Array<U0, $t, End>, illegal_values!($($tt)*)>
    };
    ($t: ty) => {
        Array<U0, $t, End>
    };
}
unsafe impl IStable for bool {
    type Align = U1;
    type Size = U1;
    type ForbiddenValues = illegal_values!(
        [
            [
                [
                    [
                        [[[U2], [U3, U4]], [[U5, U6], [U7, U8]]],
                        [[[U9, U10], [U11, U12]], [[U13, U14], [U15, U16]]]
                    ],
                    [
                        [[[U17, U18], [U19, U20]], [[U21, U22], [U23, U24]]],
                        [[[U25, U26], [U27, U28]], [[U29, U30], [U31, U32]]]
                    ]
                ],
                [
                    [
                        [[[U33, U34], [U35, U36]], [[U37, U38], [U39, U40]]],
                        [[[U41, U42], [U43, U44]], [[U45, U46], [U47, U48]]]
                    ],
                    [
                        [[[U49, U50], [U51, U52]], [[U53, U54], [U55, U56]]],
                        [[[U57, U58], [U59, U60]], [[U61, U62], [U63, U64]]]
                    ]
                ]
            ],
            [
                [
                    [
                        [[[U65, U66], [U67, U68]], [[U69, U70], [U71, U72]]],
                        [[[U73, U74], [U75, U76]], [[U77, U78], [U79, U80]]]
                    ],
                    [
                        [[[U81, U82], [U83, U84]], [[U85, U86], [U87, U88]]],
                        [[[U89, U90], [U91, U92]], [[U93, U94], [U95, U96]]]
                    ]
                ],
                [
                    [
                        [[[U97, U98], [U99, U100]], [[U101, U102], [U103, U104]]],
                        [[[U105, U106], [U107, U108]], [[U109, U110], [U111, U112]]]
                    ],
                    [
                        [[[U113, U114], [U115, U116]], [[U117, U118], [U119, U120]]],
                        [[[U121, U122], [U123, U124]], [[U125, U126], [U127, U128]]]
                    ]
                ]
            ]
        ],
        [
            [
                [
                    [
                        [[[U129], [U130, U131]], [[U132, U133], [U134, U135]]],
                        [[[U136, U137], [U138, U139]], [[U140, U141], [U142, U143]]]
                    ],
                    [
                        [[[U144, U145], [U146, U147]], [[U148, U149], [U150, U151]]],
                        [[[U152, U153], [U154, U155]], [[U156, U157], [U158, U159]]]
                    ]
                ],
                [
                    [
                        [[[U160, U161], [U162, U163]], [[U164, U165], [U166, U167]]],
                        [[[U168, U169], [U170, U171]], [[U172, U173], [U174, U175]]]
                    ],
                    [
                        [[[U176, U177], [U178, U179]], [[U180, U181], [U182, U183]]],
                        [[[U184, U185], [U186, U187]], [[U188, U189], [U190, U191]]]
                    ]
                ]
            ],
            [
                [
                    [
                        [[[U192, U193], [U194, U195]], [[U196, U197], [U198, U199]]],
                        [[[U200, U201], [U202, U203]], [[U204, U205], [U206, U207]]]
                    ],
                    [
                        [[[U208, U209], [U210, U211]], [[U212, U213], [U214, U215]]],
                        [[[U216, U217], [U218, U219]], [[U220, U221], [U222, U223]]]
                    ]
                ],
                [
                    [
                        [[[U224, U225], [U226, U227]], [[U228, U229], [U230, U231]]],
                        [[[U232, U233], [U234, U235]], [[U236, U237], [U238, U239]]]
                    ],
                    [
                        [[[U240, U241], [U242, U243]], [[U244, U245], [U246, U247]]],
                        [[[U248, U249], [U250, U251]], [[U252, U253], [U254, U255]]]
                    ]
                ]
            ]
        ]
    );
    type UnusedBits = End;
    type HasExactlyOneNiche = B0;
    type ContainsIndirections = B0;
    primitive_report!("bool");
}

unsafe impl IStable for u8 {
    type UnusedBits = End;
    type ForbiddenValues = End;
    type Align = U1;
    type Size = U1;
    type HasExactlyOneNiche = B0;
    type ContainsIndirections = B0;
    primitive_report!("u8");
}
check!(u8);
unsafe impl IStable for core::num::NonZeroU8 {
    type Align = U1;
    type Size = U1;
    type UnusedBits = End;
    type ForbiddenValues = nz_holes!(U0);
    type HasExactlyOneNiche = B1;
    type ContainsIndirections = B0;
    primitive_report!("core::num::NonZeroU8");
}
unsafe impl IStable for u16 {
    type UnusedBits = End;
    type ForbiddenValues = End;
    type Align = U2;
    type Size = U2;
    type HasExactlyOneNiche = B0;
    type ContainsIndirections = B0;
    primitive_report!("u16");
}
check!(u16);
unsafe impl IStable for core::num::NonZeroU16 {
    type ForbiddenValues = nz_holes!(U0, U1);
    type UnusedBits = End;
    type Align = U2;
    type Size = U2;
    type HasExactlyOneNiche = B1;
    type ContainsIndirections = B0;
    primitive_report!("core::num::NonZeroU16");
}
unsafe impl IStable for u32 {
    type UnusedBits = End;
    type ForbiddenValues = End;
    type Align = U4;
    type Size = U4;
    type HasExactlyOneNiche = B0;
    type ContainsIndirections = B0;
    primitive_report!("u32");
}
check!(u32);
unsafe impl IStable for core::num::NonZeroU32 {
    type ForbiddenValues = nz_holes!(U0, U1, U2, U3);
    type UnusedBits = End;
    type Align = U4;
    type Size = U4;
    type HasExactlyOneNiche = B1;
    type ContainsIndirections = B0;
    primitive_report!("core::num::NonZeroU32");
}
unsafe impl IStable for u64 {
    type UnusedBits = End;
    type ForbiddenValues = End;
    type Align = U8;
    type Size = U8;
    type HasExactlyOneNiche = B0;
    type ContainsIndirections = B0;
    primitive_report!("u64");
}
check!(u64);
unsafe impl IStable for core::num::NonZeroU64 {
    type UnusedBits = End;
    type ForbiddenValues = nz_holes!(U0, U1, U2, U3, U4, U5, U6, U7);
    type Align = U8;
    type Size = U8;
    type HasExactlyOneNiche = B1;
    type ContainsIndirections = B0;
    primitive_report!("core::num::NonZeroU64");
}

unsafe impl IStable for u128 {
    type UnusedBits = End;
    type ForbiddenValues = End;
    type Size = U16;
    type HasExactlyOneNiche = B0;
    #[cfg(not(target_arch = "aarch64"))]
    type Align = U8;
    #[cfg(target_arch = "aarch64")]
    type Align = U16;
    type ContainsIndirections = B0;
    primitive_report!("u128");
}

check!(u128);

unsafe impl IStable for core::num::NonZeroU128 {
    type UnusedBits = End;
    type ForbiddenValues =
        nz_holes!(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15);
    type Size = U16;
    type HasExactlyOneNiche = B1;
    type Align = <u128 as IStable>::Align;
    type ContainsIndirections = B0;
    primitive_report!("core::num::NonZeroU128");
}

unsafe impl IStable for usize {
    #[cfg(target_pointer_width = "64")]
    same_as!(u64, "usize");
    #[cfg(target_pointer_width = "32")]
    same_as!(u32, "usize");
    #[cfg(target_pointer_width = "16")]
    same_as!(u16, "usize");
    #[cfg(target_pointer_width = "8")]
    same_as!(u8, "usize");
    type ContainsIndirections = B0;
}

check!(usize);
unsafe impl IStable for core::num::NonZeroUsize {
    #[cfg(target_pointer_width = "64")]
    same_as!(core::num::NonZeroU64, "core::num::NonZeroUsize");
    #[cfg(target_pointer_width = "32")]
    same_as!(core::num::NonZeroU32, "core::num::NonZeroUsize");
    #[cfg(target_pointer_width = "16")]
    same_as!(core::num::NonZeroU16, "core::num::NonZeroUsize");
    #[cfg(target_pointer_width = "8")]
    same_as!(core::num::NonZeroU8, "core::num::NonZeroUsize");
    type ContainsIndirections = B0;
}

unsafe impl IStable for i8 {
    same_as!(u8, "i8");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::num::NonZeroI8 {
    same_as!(core::num::NonZeroU8, "core::num::NonZeroI8");
    type ContainsIndirections = B0;
}
unsafe impl IStable for i16 {
    same_as!(u16, "i16");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::num::NonZeroI16 {
    same_as!(core::num::NonZeroU16, "core::num::NonZeroI16");
    type ContainsIndirections = B0;
}
unsafe impl IStable for i32 {
    same_as!(u32, "i32");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::num::NonZeroI32 {
    same_as!(core::num::NonZeroU32, "core::num::NonZeroI32");
    type ContainsIndirections = B0;
}
unsafe impl IStable for i64 {
    same_as!(u64, "i64");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::num::NonZeroI64 {
    same_as!(core::num::NonZeroU64, "core::num::NonZeroI64");
    type ContainsIndirections = B0;
}
unsafe impl IStable for i128 {
    same_as!(u128, "i128");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::num::NonZeroI128 {
    same_as!(core::num::NonZeroU128, "core::num::NonZeroI128");
    type ContainsIndirections = B0;
}
unsafe impl IStable for isize {
    same_as!(usize, "isize");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::num::NonZeroIsize {
    same_as!(core::num::NonZeroUsize, "core::num::NonZeroIsize");
    type ContainsIndirections = B0;
}

unsafe impl<T: IStable> IStable for core::mem::ManuallyDrop<T> {
    same_as!(T, <T as IStable>::REPORT.name.as_str());
    type ContainsIndirections = T::ContainsIndirections;
}
unsafe impl<T: IStable> IStable for core::mem::MaybeUninit<T> {
    same_as!(T, <T as IStable>::REPORT.name.as_str());
    type ContainsIndirections = T::ContainsIndirections;
}
unsafe impl<T: IStable> IStable for core::cell::UnsafeCell<T> {
    same_as!(T, <T as IStable>::REPORT.name.as_str());
    type ContainsIndirections = T::ContainsIndirections;
}

unsafe impl<T: IStable> IStable for *const T {
    same_as!(usize, "*const", T);
    type ContainsIndirections = B1;
}
unsafe impl<T: IStable> IStable for *mut T {
    same_as!(usize, "*mut", T);
    type ContainsIndirections = B1;
}
unsafe impl<T: IStable> IStable for core::ptr::NonNull<T> {
    same_as!(core::num::NonZeroUsize, "core::ptr::NonNull", T);
    type ContainsIndirections = B1;
}
unsafe impl<T: IStable> IStable for core::sync::atomic::AtomicPtr<T> {
    same_as!(*mut T, "core::sync::atomic::AtomicPtr", T);
    type ContainsIndirections = B1;
}
unsafe impl IStable for core::sync::atomic::AtomicBool {
    same_as!(bool, "core::sync::atomic::AtomicBool");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::sync::atomic::AtomicI8 {
    same_as!(i8, "core::sync::atomic::AtomicI8");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::sync::atomic::AtomicI16 {
    same_as!(i16, "core::sync::atomic::AtomicI16");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::sync::atomic::AtomicI32 {
    same_as!(i32, "core::sync::atomic::AtomicI32");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::sync::atomic::AtomicI64 {
    same_as!(i64, "core::sync::atomic::AtomicI64");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::sync::atomic::AtomicIsize {
    same_as!(isize, "core::sync::atomic::AtomicIsize");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::sync::atomic::AtomicU8 {
    same_as!(u8, "core::sync::atomic::AtomicU8");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::sync::atomic::AtomicU16 {
    same_as!(u16, "core::sync::atomic::AtomicU16");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::sync::atomic::AtomicU32 {
    same_as!(u32, "core::sync::atomic::AtomicU32");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::sync::atomic::AtomicU64 {
    same_as!(u64, "core::sync::atomic::AtomicU64");
    type ContainsIndirections = B0;
}
unsafe impl IStable for core::sync::atomic::AtomicUsize {
    same_as!(usize, "core::sync::atomic::AtomicUsize");
    type ContainsIndirections = B0;
}
unsafe impl<T: IStable> IStable for &T {
    same_as!(core::num::NonZeroUsize, "&", T);
    type ContainsIndirections = B1;
}
unsafe impl<T: IStable> IStable for &mut T {
    same_as!(core::num::NonZeroUsize, "&mut", T);
    type ContainsIndirections = B1;
}
unsafe impl<T: IStable> IStable for core::pin::Pin<T> {
    same_as!(T, "core::pin::Pin", T);
    type ContainsIndirections = T::ContainsIndirections;
}
unsafe impl IStable for f32 {
    same_as!(u32, "f32");
    type ContainsIndirections = B0;
}
unsafe impl IStable for f64 {
    same_as!(u64, "f64");
    type ContainsIndirections = B0;
}

pub struct HasExactlyOneNiche<A, B>(core::marker::PhantomData<(A, B)>);
unsafe impl<T: IStable> IStable for core::option::Option<T>
where
    HasExactlyOneNiche<core::option::Option<T>, T::HasExactlyOneNiche>: IStable,
{
    same_as!(HasExactlyOneNiche<core::option::Option<T>, T::HasExactlyOneNiche>);
    type ContainsIndirections = T::ContainsIndirections;
    const REPORT: &'static report::TypeReport =
        <HasExactlyOneNiche<core::option::Option<T>, T::HasExactlyOneNiche> as IStable>::REPORT;
    const ID: u64 = crate::report::gen_id(Self::REPORT);
}
unsafe impl<T: IStable> IStable for HasExactlyOneNiche<core::option::Option<T>, B1> {
    type Size = T::Size;
    type Align = T::Align;
    type ForbiddenValues = End;
    type UnusedBits = End;
    type HasExactlyOneNiche = B0;
    type ContainsIndirections = T::ContainsIndirections;
    const REPORT: &'static report::TypeReport = &report::TypeReport {
        name: Str::new("Option"),
        module: Str::new("core::option"),
        fields: StableLike::new(Some(&report::FieldReport {
            name: Str::new("Some"),
            ty: T::REPORT,
            next_field: StableLike::new(None),
        })),
        version: 0,
        tyty: report::TyTy::Enum(Str::new("rust")),
    };
    const ID: u64 = crate::report::gen_id(Self::REPORT);
}
unsafe impl<Ok: IStable, Err: IStable> IStable for core::result::Result<Ok, Err>
where
    HasExactlyOneNiche<Self, (Ok::HasExactlyOneNiche, Err::Size)>: IStable,
{
    type ContainsIndirections = <Ok::ContainsIndirections as Bit>::Or<Err::ContainsIndirections>;
    same_as!(HasExactlyOneNiche<Self, (Ok::HasExactlyOneNiche, Err::Size)>);
    const REPORT: &'static report::TypeReport =
        <HasExactlyOneNiche<Self, (Ok::HasExactlyOneNiche, Err::Size)> as IStable>::REPORT;
    const ID: u64 = crate::report::gen_id(Self::REPORT);
}
unsafe impl<Ok: IStable, Err: IStable> IStable
    for HasExactlyOneNiche<core::result::Result<Ok, Err>, (B1, U0)>
{
    type Size = Ok::Size;
    type Align = Ok::Align;
    type ForbiddenValues = End;
    type UnusedBits = End;
    type HasExactlyOneNiche = B0;
    type ContainsIndirections = <Ok::ContainsIndirections as Bit>::Or<Err::ContainsIndirections>;
    const REPORT: &'static report::TypeReport = &report::TypeReport {
        name: Str::new("Result"),
        module: Str::new("core::result"),
        fields: StableLike::new(Some(&report::FieldReport {
            name: Str::new("Ok"),
            ty: Ok::REPORT,
            next_field: StableLike::new(None),
        })),
        version: 0,
        tyty: report::TyTy::Enum(Str::new("rust")),
    };
    const ID: u64 = crate::report::gen_id(Self::REPORT);
}
unsafe impl<Ok: IStable, Err: IStable, BO, B: Bit, I: Unsigned> IStable
    for HasExactlyOneNiche<core::result::Result<Ok, Err>, (BO, UInt<I, B>)>
where
    HasExactlyOneNiche<Self, (Err::HasExactlyOneNiche, Ok::Size)>: IStable,
{
    same_as!(HasExactlyOneNiche<Self, (Err::HasExactlyOneNiche, Ok::Size)>);
    type ContainsIndirections = <Ok::ContainsIndirections as Bit>::Or<Err::ContainsIndirections>;
    const REPORT: &'static report::TypeReport =
        <HasExactlyOneNiche<Self, (Err::HasExactlyOneNiche, Ok::Size)> as IStable>::REPORT;
    const ID: u64 = crate::report::gen_id(Self::REPORT);
}
unsafe impl<Ok: IStable, Err: IStable, T> IStable
    for HasExactlyOneNiche<HasExactlyOneNiche<core::result::Result<Ok, Err>, T>, (B1, U0)>
{
    type Size = Err::Size;
    type Align = Err::Align;
    type ForbiddenValues = End;
    type UnusedBits = End;
    type HasExactlyOneNiche = B0;
    type ContainsIndirections = <Ok::ContainsIndirections as Bit>::Or<Err::ContainsIndirections>;
    const REPORT: &'static report::TypeReport = &report::TypeReport {
        name: Str::new("Result"),
        module: Str::new("core::result"),
        fields: StableLike::new(Some(&report::FieldReport {
            name: Str::new("Err"),
            ty: Err::REPORT,
            next_field: StableLike::new(None),
        })),
        version: 0,
        tyty: report::TyTy::Enum(Str::new("rust")),
    };
    const ID: u64 = crate::report::gen_id(Self::REPORT);
}

struct NameAggregator<L: IStable, R: IStable>(core::marker::PhantomData<(L, R)>);
unsafe impl<L: IStable, R: IStable> IStable for NameAggregator<L, R> {
    type Size = U0;
    type Align = U1;
    type ForbiddenValues = End;
    type UnusedBits = End;
    type HasExactlyOneNiche = B0;
    type ContainsIndirections = B0;
    const REPORT: &'static report::TypeReport = &report::TypeReport {
        name: Str::new("signature"),
        module: Str::new("stabby"),
        fields: StableLike::new(Some(&report::FieldReport {
            name: Str::new("_"),
            ty: L::REPORT,
            next_field: StableLike::new(Some(&report::FieldReport {
                name: Str::new("_"),
                ty: R::REPORT,
                next_field: StableLike::new(None),
            })),
        })),
        version: 0,
        tyty: report::TyTy::Struct,
    };
    const ID: u64 = crate::report::gen_id(Self::REPORT);
}
macro_rules! union {
    ($head: ident,) => {
        $head
    };
    ($head: ident, $($tail: ident,)*) => {
        NameAggregator<$head, union!($($tail,)*)>
    };
}

macro_rules! fnstable {
    (-> $o: ident) => {
        unsafe impl<$o: IStable > IStable for extern "C" fn() -> $o {
            same_as!(core::num::NonZeroUsize, "extern \"C\" fn", $o);
            type ContainsIndirections = B1;
        }
        unsafe impl<$o: IStable > IStable for unsafe extern "C" fn() -> $o {
            same_as!(core::num::NonZeroUsize, "unsafe extern \"C\" fn", $o);
            type ContainsIndirections = B1;
        }
        #[rustversion::since(1.71)]
        unsafe impl<$o: IStable > IStable for extern "C-unwind" fn() -> $o {
            same_as!(core::num::NonZeroUsize, "extern \"C-unwind\" fn", $o);
            type ContainsIndirections = B1;
        }
        #[rustversion::since(1.71)]
        unsafe impl<$o: IStable > IStable for unsafe extern "C-unwind" fn() -> $o {
            same_as!(core::num::NonZeroUsize, "unsafe extern \"C-unwind\" fn", $o);
            type ContainsIndirections = B1;
        }
    };
    ($t: ident, $($tt: ident, )* -> $o: ident) => {
        unsafe impl< $o , $t, $($tt,)* > IStable for extern "C" fn($t, $($tt,)*) -> $o
        where $o : IStable, $t: IStable, $($tt: IStable,)* {
            same_as!(core::num::NonZeroUsize, "extern \"C\" fn", union!($o, $t, $($tt,)*));
            type ContainsIndirections = B1;
        }
        unsafe impl< $o : IStable, $t: IStable, $($tt: IStable,)* > IStable for unsafe extern "C" fn($t, $($tt,)*) -> $o {
            same_as!(core::num::NonZeroUsize, "unsafe extern \"C\" fn", union!($o, $t, $($tt,)*));
            type ContainsIndirections = B1;
        }
        #[rustversion::since(1.71)]
        unsafe impl< $o , $t, $($tt,)* > IStable for extern "C-unwind" fn($t, $($tt,)*) -> $o
        where $o : IStable, $t: IStable, $($tt: IStable,)* {
            same_as!(core::num::NonZeroUsize, "extern \"C-unwind\" fn", union!($o, $t, $($tt,)*));
            type ContainsIndirections = B1;
        }
        #[rustversion::since(1.71)]
        unsafe impl< $o : IStable, $t: IStable, $($tt: IStable,)* > IStable for unsafe extern "C-unwind" fn($t, $($tt,)*) -> $o {
            same_as!(core::num::NonZeroUsize, "unsafe extern \"C-unwind\" fn", union!($o, $t, $($tt,)*));
            type ContainsIndirections = B1;
        }
        fnstable!($($tt,)* -> $o);
    };
}
fnstable!(I15, I14, I13, I12, I11, I10, I9, I8, I7, I6, I5, I4, I3, I2, I1, -> Output);

#[cfg(feature = "abi_stable")]
mod abi_stable;

#[cfg(all(feature = "std", target_family = "unix"))]
unsafe impl IStable for std::os::fd::OwnedFd {
    same_as!(core::ffi::c_int);
    type ContainsIndirections = B1;
    primitive_report!("std::os::fd::OwnedFd");
}
#[cfg(all(feature = "std", target_family = "unix"))]
unsafe impl<'a> IStable for std::os::fd::BorrowedFd<'a> {
    same_as!(core::ffi::c_int);
    type ContainsIndirections = B1;
    primitive_report!("std::os::fd::BorrowedFd");
}

const ARRAY_NAME: [&str; 129] = [
    "[T;0]", "[T;1]", "[T;2]", "[T;3]", "[T;4]", "[T;5]", "[T;6]", "[T;7]", "[T;8]", "[T;9]",
    "[T;10]", "[T;11]", "[T;12]", "[T;13]", "[T;14]", "[T;15]", "[T;16]", "[T;17]", "[T;18]",
    "[T;19]", "[T;20]", "[T;21]", "[T;22]", "[T;23]", "[T;24]", "[T;25]", "[T;26]", "[T;27]",
    "[T;28]", "[T;29]", "[T;30]", "[T;31]", "[T;32]", "[T;33]", "[T;34]", "[T;35]", "[T;36]",
    "[T;37]", "[T;38]", "[T;39]", "[T;40]", "[T;41]", "[T;42]", "[T;43]", "[T;44]", "[T;45]",
    "[T;46]", "[T;47]", "[T;48]", "[T;49]", "[T;50]", "[T;51]", "[T;52]", "[T;53]", "[T;54]",
    "[T;55]", "[T;56]", "[T;57]", "[T;58]", "[T;59]", "[T;60]", "[T;61]", "[T;62]", "[T;63]",
    "[T;64]", "[T;65]", "[T;66]", "[T;67]", "[T;68]", "[T;69]", "[T;70]", "[T;71]", "[T;72]",
    "[T;73]", "[T;74]", "[T;75]", "[T;76]", "[T;77]", "[T;78]", "[T;79]", "[T;80]", "[T;81]",
    "[T;82]", "[T;83]", "[T;84]", "[T;85]", "[T;86]", "[T;87]", "[T;88]", "[T;89]", "[T;90]",
    "[T;91]", "[T;92]", "[T;93]", "[T;94]", "[T;95]", "[T;96]", "[T;97]", "[T;98]", "[T;99]",
    "[T;100]", "[T;101]", "[T;102]", "[T;103]", "[T;104]", "[T;105]", "[T;106]", "[T;107]",
    "[T;108]", "[T;109]", "[T;110]", "[T;111]", "[T;112]", "[T;113]", "[T;114]", "[T;115]",
    "[T;116]", "[T;117]", "[T;118]", "[T;119]", "[T;120]", "[T;121]", "[T;122]", "[T;123]",
    "[T;124]", "[T;125]", "[T;126]", "[T;127]", "[T;128]",
];
macro_rules! sliceimpl {
    ($size: ty) => {
        unsafe impl<T: IStable> IStable for [T; <$size as Unsigned>::USIZE] {
            type Size = <T::Size as Unsigned>::Mul<$size>;
            type Align = T::Align;
            type ForbiddenValues = <<$size as Unsigned>::Equal<U1> as Bit>::FvTernary<T::ForbiddenValues, End>;
            type UnusedBits = <<$size as Unsigned>::Equal<U1> as Bit>::UbTernary<T::UnusedBits, End>;
            type HasExactlyOneNiche = <<$size as Unsigned>::Equal<U0> as Bit>::SaddTernary<
                B0,
                <<$size as Unsigned>::Equal<U1> as Bit>::SaddTernary<T::HasExactlyOneNiche, Saturator>,
            >;
            type ContainsIndirections = T::ContainsIndirections;
            primitive_report!(ARRAY_NAME[<$size as Unsigned>::USIZE], T);
        }
    };
    ($size: ty, $($t: tt)*) => {
        sliceimpl!($size);
        sliceimpl!($($t)*);
    };
}

sliceimpl!(
    U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20,
    U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31, U32, U33, U34, U35, U36, U37, U38, U39,
    U40, U41, U42, U43, U44, U45, U46, U47, U48, U49, U50, U51, U52, U53, U54, U55, U56, U57, U58,
    U59, U60, U61, U62, U63
);
sliceimpl!(
    U64, U65, U66, U67, U68, U69, U70, U71, U72, U73, U74, U75, U76, U77, U78, U79, U80, U81, U82,
    U83, U84, U85, U86, U87, U88, U89, U90, U91, U92, U93, U94, U95, U96, U97, U98, U99, U100,
    U101, U102, U103, U104, U105, U106, U107, U108, U109, U110, U111, U112, U113, U114, U115, U116,
    U117, U118, U119, U120, U121, U122, U123, U124, U125, U126, U127, U128
);
