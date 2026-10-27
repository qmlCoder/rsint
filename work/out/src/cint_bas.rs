pub const BAS_SLOTS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn CINTlen_cart(l: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (l + 1 as ::core::ffi::c_int) * (l + 2 as ::core::ffi::c_int)
        / 2 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINTlen_spinor(
    bas_id: ::core::ffi::c_int,
    mut bas: *const ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if 0 as ::core::ffi::c_int
        == *bas.offset((BAS_SLOTS * bas_id + 4 as ::core::ffi::c_int) as isize)
    {
        return 4 as ::core::ffi::c_int
            * *bas.offset((BAS_SLOTS * bas_id + 1 as ::core::ffi::c_int) as isize)
            + 2 as ::core::ffi::c_int
    } else if *bas.offset((BAS_SLOTS * bas_id + 4 as ::core::ffi::c_int) as isize)
        < 0 as ::core::ffi::c_int
    {
        return 2 as ::core::ffi::c_int
            * *bas.offset((BAS_SLOTS * bas_id + 1 as ::core::ffi::c_int) as isize)
            + 2 as ::core::ffi::c_int
    } else {
        return 2 as ::core::ffi::c_int
            * *bas.offset((BAS_SLOTS * bas_id + 1 as ::core::ffi::c_int) as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTcgtos_cart(
    bas_id: ::core::ffi::c_int,
    mut bas: *const ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut l: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * bas_id + 1 as ::core::ffi::c_int) as isize);
    return (l + 1 as ::core::ffi::c_int) * (l + 2 as ::core::ffi::c_int)
        / 2 as ::core::ffi::c_int
        * *bas.offset((BAS_SLOTS * bas_id + 3 as ::core::ffi::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTcgto_cart(
    bas_id: ::core::ffi::c_int,
    mut bas: *const ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut l: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * bas_id + 1 as ::core::ffi::c_int) as isize);
    return (l + 1 as ::core::ffi::c_int) * (l + 2 as ::core::ffi::c_int)
        / 2 as ::core::ffi::c_int
        * *bas.offset((BAS_SLOTS * bas_id + 3 as ::core::ffi::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTcgtos_spheric(
    bas_id: ::core::ffi::c_int,
    mut bas: *const ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (*bas.offset((BAS_SLOTS * bas_id + 1 as ::core::ffi::c_int) as isize)
        * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        * *bas.offset((BAS_SLOTS * bas_id + 3 as ::core::ffi::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTcgto_spheric(
    bas_id: ::core::ffi::c_int,
    mut bas: *const ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (*bas.offset((BAS_SLOTS * bas_id + 1 as ::core::ffi::c_int) as isize)
        * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        * *bas.offset((BAS_SLOTS * bas_id + 3 as ::core::ffi::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTcgtos_spinor(
    bas_id: ::core::ffi::c_int,
    mut bas: *const ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return CINTlen_spinor(bas_id, bas)
        * *bas.offset((BAS_SLOTS * bas_id + 3 as ::core::ffi::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTcgto_spinor(
    bas_id: ::core::ffi::c_int,
    mut bas: *const ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return CINTlen_spinor(bas_id, bas)
        * *bas.offset((BAS_SLOTS * bas_id + 3 as ::core::ffi::c_int) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CINTtot_pgto_spheric(
    mut bas: *const ::core::ffi::c_int,
    nbas: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nbas {
        s
            += (*bas.offset((BAS_SLOTS * i + 1 as ::core::ffi::c_int) as isize)
                * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                * *bas.offset((BAS_SLOTS * i + 2 as ::core::ffi::c_int) as isize);
        i += 1;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn CINTtot_pgto_spinor(
    mut bas: *const ::core::ffi::c_int,
    nbas: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nbas {
        s
            += CINTlen_spinor(i, bas)
                * *bas.offset((BAS_SLOTS * i + 2 as ::core::ffi::c_int) as isize);
        i += 1;
    }
    return s;
}
unsafe extern "C" fn tot_cgto_accum(
    mut f: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    mut bas: *const ::core::ffi::c_int,
    nbas: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut s: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nbas {
        s
            += ::core::mem::transmute::<
                _,
                fn(_, _) -> ::core::ffi::c_int,
            >(
                Some(f.expect("non-null function pointer"))
                    .expect("non-null function pointer"),
            )(i, bas);
        i += 1;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn CINTtot_cgto_spheric(
    mut bas: *const ::core::ffi::c_int,
    nbas: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return tot_cgto_accum(
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                CINTcgto_spheric
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        ),
        bas,
        nbas,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTtot_cgto_spinor(
    mut bas: *const ::core::ffi::c_int,
    nbas: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return tot_cgto_accum(
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                CINTcgto_spinor
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        ),
        bas,
        nbas,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTtot_cgto_cart(
    mut bas: *const ::core::ffi::c_int,
    nbas: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return tot_cgto_accum(
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                CINTcgto_cart
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        ),
        bas,
        nbas,
    );
}
unsafe extern "C" fn shells_cgto_offset(
    mut f: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    mut ao_loc: *mut ::core::ffi::c_int,
    mut bas: *const ::core::ffi::c_int,
    nbas: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    *ao_loc.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < nbas {
        *ao_loc.offset(i as isize) = *ao_loc
            .offset((i - 1 as ::core::ffi::c_int) as isize)
            + ::core::mem::transmute::<
                _,
                fn(_, _) -> ::core::ffi::c_int,
            >(
                Some(f.expect("non-null function pointer"))
                    .expect("non-null function pointer"),
            )(i - 1 as ::core::ffi::c_int, bas);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTshells_cart_offset(
    mut ao_loc: *mut ::core::ffi::c_int,
    mut bas: *const ::core::ffi::c_int,
    nbas: ::core::ffi::c_int,
) {
    shells_cgto_offset(
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                CINTcgto_cart
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        ),
        ao_loc,
        bas,
        nbas,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTshells_spheric_offset(
    mut ao_loc: *mut ::core::ffi::c_int,
    mut bas: *const ::core::ffi::c_int,
    nbas: ::core::ffi::c_int,
) {
    shells_cgto_offset(
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                CINTcgto_spheric
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        ),
        ao_loc,
        bas,
        nbas,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTshells_spinor_offset(
    mut ao_loc: *mut ::core::ffi::c_int,
    mut bas: *const ::core::ffi::c_int,
    nbas: ::core::ffi::c_int,
) {
    shells_cgto_offset(
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
            Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
        >(
            Some(
                CINTcgto_spinor
                    as unsafe extern "C" fn(
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
        ),
        ao_loc,
        bas,
        nbas,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTcart_comp(
    mut nx: *mut ::core::ffi::c_int,
    mut ny: *mut ::core::ffi::c_int,
    mut nz: *mut ::core::ffi::c_int,
    lmax: ::core::ffi::c_int,
) {
    let mut inc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut lx: ::core::ffi::c_int = 0;
    let mut ly: ::core::ffi::c_int = 0;
    let mut lz: ::core::ffi::c_int = 0;
    lx = lmax;
    while lx >= 0 as ::core::ffi::c_int {
        ly = lmax - lx;
        while ly >= 0 as ::core::ffi::c_int {
            lz = lmax - lx - ly;
            *nx.offset(inc as isize) = lx;
            *ny.offset(inc as isize) = ly;
            *nz.offset(inc as isize) = lz;
            inc += 1;
            ly -= 1;
        }
        lx -= 1;
    }
}
