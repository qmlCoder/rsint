extern "C" {
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn CINTcart_comp(
        nx: *mut ::core::ffi::c_int,
        ny: *mut ::core::ffi::c_int,
        nz: *mut ::core::ffi::c_int,
        lmax: ::core::ffi::c_int,
    );
    fn CINTrys_roots(
        nroots: ::core::ffi::c_int,
        x: ::core::ffi::c_double,
        u: *mut ::core::ffi::c_double,
        w: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PairData {
    pub rij: [::core::ffi::c_double; 3],
    pub eij: ::core::ffi::c_double,
    pub cceij: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CINTOpt {
    pub index_xyz_array: *mut *mut ::core::ffi::c_int,
    pub non0ctr: *mut *mut ::core::ffi::c_int,
    pub sortedidx: *mut *mut ::core::ffi::c_int,
    pub nbas: ::core::ffi::c_int,
    pub log_max_coeff: *mut *mut ::core::ffi::c_double,
    pub pairdata: *mut *mut PairData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CINTEnvVars {
    pub atm: *mut ::core::ffi::c_int,
    pub bas: *mut ::core::ffi::c_int,
    pub env: *mut ::core::ffi::c_double,
    pub shls: *mut ::core::ffi::c_int,
    pub natm: ::core::ffi::c_int,
    pub nbas: ::core::ffi::c_int,
    pub i_l: ::core::ffi::c_int,
    pub j_l: ::core::ffi::c_int,
    pub k_l: ::core::ffi::c_int,
    pub l_l: ::core::ffi::c_int,
    pub nfi: ::core::ffi::c_int,
    pub nfj: ::core::ffi::c_int,
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed_0,
    pub nf: ::core::ffi::c_int,
    pub rys_order: ::core::ffi::c_int,
    pub x_ctr: [::core::ffi::c_int; 4],
    pub gbits: ::core::ffi::c_int,
    pub ncomp_e1: ::core::ffi::c_int,
    pub ncomp_e2: ::core::ffi::c_int,
    pub ncomp_tensor: ::core::ffi::c_int,
    pub li_ceil: ::core::ffi::c_int,
    pub lj_ceil: ::core::ffi::c_int,
    pub lk_ceil: ::core::ffi::c_int,
    pub ll_ceil: ::core::ffi::c_int,
    pub g_stride_i: ::core::ffi::c_int,
    pub g_stride_k: ::core::ffi::c_int,
    pub g_stride_l: ::core::ffi::c_int,
    pub g_stride_j: ::core::ffi::c_int,
    pub nrys_roots: ::core::ffi::c_int,
    pub g_size: ::core::ffi::c_int,
    pub g2d_ijmax: ::core::ffi::c_int,
    pub g2d_klmax: ::core::ffi::c_int,
    pub common_factor: ::core::ffi::c_double,
    pub expcutoff: ::core::ffi::c_double,
    pub rirj: [::core::ffi::c_double; 3],
    pub rkrl: [::core::ffi::c_double; 3],
    pub rx_in_rijrx: *mut ::core::ffi::c_double,
    pub rx_in_rklrx: *mut ::core::ffi::c_double,
    pub ri: *mut ::core::ffi::c_double,
    pub rj: *mut ::core::ffi::c_double,
    pub rk: *mut ::core::ffi::c_double,
    pub c2rust_unnamed_1: C2RustUnnamed,
    pub f_g0_2e: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub f_g0_2d4d: Option<unsafe extern "C" fn() -> ()>,
    pub f_gout: Option<unsafe extern "C" fn() -> ()>,
    pub opt: *mut CINTOpt,
    pub idx: *mut ::core::ffi::c_int,
    pub ai: [::core::ffi::c_double; 1],
    pub aj: [::core::ffi::c_double; 1],
    pub ak: [::core::ffi::c_double; 1],
    pub al: [::core::ffi::c_double; 1],
    pub fac: [::core::ffi::c_double; 1],
    pub rij: [::core::ffi::c_double; 3],
    pub rkl: [::core::ffi::c_double; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub rl: *mut ::core::ffi::c_double,
    pub grids: *mut ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub nfl: ::core::ffi::c_int,
    pub ngrids: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub nfk: ::core::ffi::c_int,
    pub grids_offset: ::core::ffi::c_int,
}
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 91] = unsafe {
    ::core::mem::transmute::<
        [u8; 91],
        [::core::ffi::c_char; 91],
    >(
        *b"void CINTinit_int1e_EnvVars(CINTEnvVars *, int *, int *, int *, int, int *, int, double *)\0",
    )
};
pub const M_PI: ::core::ffi::c_double = 3.14159265358979323846f64;
pub const PTR_EXPCUTOFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTR_RINV_ORIG: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PTR_RINV_ZETA: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const CHARGE_OF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTR_FRAC_CHARGE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ATM_SLOTS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const BAS_SLOTS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const GAUSSIAN_NUC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FRAC_CHARGE_NUC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn CINTinit_int1e_EnvVars(
    mut envs: *mut CINTEnvVars,
    mut ng: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    (*envs).natm = natm;
    (*envs).nbas = nbas;
    (*envs).atm = atm;
    (*envs).bas = bas;
    (*envs).env = env;
    (*envs).shls = shls;
    let i_sh: ::core::ffi::c_int = *shls.offset(0 as ::core::ffi::c_int as isize);
    let j_sh: ::core::ffi::c_int = *shls.offset(1 as ::core::ffi::c_int as isize);
    (*envs).i_l = *bas.offset((BAS_SLOTS * i_sh + 1 as ::core::ffi::c_int) as isize);
    (*envs).j_l = *bas.offset((BAS_SLOTS * j_sh + 1 as ::core::ffi::c_int) as isize);
    (*envs).x_ctr[0 as ::core::ffi::c_int as usize] = *bas
        .offset((BAS_SLOTS * i_sh + 3 as ::core::ffi::c_int) as isize);
    (*envs).x_ctr[1 as ::core::ffi::c_int as usize] = *bas
        .offset((BAS_SLOTS * j_sh + 3 as ::core::ffi::c_int) as isize);
    (*envs).nfi = ((*envs).i_l + 1 as ::core::ffi::c_int)
        * ((*envs).i_l + 2 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
    (*envs).nfj = ((*envs).j_l + 1 as ::core::ffi::c_int)
        * ((*envs).j_l + 2 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
    (*envs).nf = (*envs).nfi * (*envs).nfj;
    (*envs).common_factor = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
    if *env.offset(PTR_EXPCUTOFF as isize)
        == 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        (*envs).expcutoff = EXPCUTOFF as ::core::ffi::c_double;
    } else {
        (*envs).expcutoff = if 40 as ::core::ffi::c_int as ::core::ffi::c_double
            > *env.offset(0 as ::core::ffi::c_int as isize)
        {
            40 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            *env.offset(0 as ::core::ffi::c_int as isize)
        };
    }
    (*envs).li_ceil = (*envs).i_l + *ng.offset(IINC as isize);
    (*envs).lj_ceil = (*envs).j_l + *ng.offset(JINC as isize);
    (*envs).ri = env
        .offset(
            *atm
                .offset(
                    (ATM_SLOTS
                        * *bas
                            .offset(
                                (8 as ::core::ffi::c_int * i_sh + 0 as ::core::ffi::c_int)
                                    as isize,
                            ) + 1 as ::core::ffi::c_int) as isize,
                ) as isize,
        );
    (*envs).rj = env
        .offset(
            *atm
                .offset(
                    (ATM_SLOTS
                        * *bas
                            .offset(
                                (8 as ::core::ffi::c_int * j_sh + 0 as ::core::ffi::c_int)
                                    as isize,
                            ) + 1 as ::core::ffi::c_int) as isize,
                ) as isize,
        );
    (*envs).gbits = *ng.offset(GSHIFT as isize);
    (*envs).ncomp_e1 = *ng.offset(POS_E1 as isize);
    (*envs).ncomp_tensor = *ng.offset(TENSOR as isize);
    if *ng.offset(SLOT_RYS_ROOTS as isize) > 0 as ::core::ffi::c_int {
        (*envs).nrys_roots = *ng.offset(SLOT_RYS_ROOTS as isize);
    } else {
        (*envs).nrys_roots = ((*envs).li_ceil + (*envs).lj_ceil)
            / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    }
    let mut dli: ::core::ffi::c_int = 0;
    let mut dlj: ::core::ffi::c_int = 0;
    let mut ibase: ::core::ffi::c_int = ((*envs).li_ceil > (*envs).lj_ceil)
        as ::core::ffi::c_int;
    if ibase != 0 {
        dli = (*envs).li_ceil + (*envs).lj_ceil + 1 as ::core::ffi::c_int;
        dlj = (*envs).lj_ceil + 1 as ::core::ffi::c_int;
        (*envs).rirj[0 as ::core::ffi::c_int as usize] = *(*envs)
            .ri
            .offset(0 as ::core::ffi::c_int as isize)
            - *(*envs).rj.offset(0 as ::core::ffi::c_int as isize);
        (*envs).rirj[1 as ::core::ffi::c_int as usize] = *(*envs)
            .ri
            .offset(1 as ::core::ffi::c_int as isize)
            - *(*envs).rj.offset(1 as ::core::ffi::c_int as isize);
        (*envs).rirj[2 as ::core::ffi::c_int as usize] = *(*envs)
            .ri
            .offset(2 as ::core::ffi::c_int as isize)
            - *(*envs).rj.offset(2 as ::core::ffi::c_int as isize);
    } else {
        dli = (*envs).li_ceil + 1 as ::core::ffi::c_int;
        dlj = (*envs).li_ceil + (*envs).lj_ceil + 1 as ::core::ffi::c_int;
        (*envs).rirj[0 as ::core::ffi::c_int as usize] = *(*envs)
            .rj
            .offset(0 as ::core::ffi::c_int as isize)
            - *(*envs).ri.offset(0 as ::core::ffi::c_int as isize);
        (*envs).rirj[1 as ::core::ffi::c_int as usize] = *(*envs)
            .rj
            .offset(1 as ::core::ffi::c_int as isize)
            - *(*envs).ri.offset(1 as ::core::ffi::c_int as isize);
        (*envs).rirj[2 as ::core::ffi::c_int as usize] = *(*envs)
            .rj
            .offset(2 as ::core::ffi::c_int as isize)
            - *(*envs).ri.offset(2 as ::core::ffi::c_int as isize);
    }
    (*envs).g_stride_i = (*envs).nrys_roots;
    (*envs).g_stride_j = (*envs).nrys_roots * dli;
    (*envs).g_size = (*envs).nrys_roots * dli * dlj;
    (*envs).g_stride_k = (*envs).g_size;
    (*envs).g_stride_l = (*envs).g_size;
    '_c2rust_label: {
        if i_sh < 1048576 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"i_sh < SHLS_MAX\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g1e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                78 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_0: {
        if j_sh < 1048576 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"j_sh < SHLS_MAX\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g1e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                79 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_1: {
        if (*envs).i_l < 15 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"envs->i_l < ANG_MAX\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g1e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                80 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_2: {
        if (*envs).j_l < 15 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"envs->j_l < ANG_MAX\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g1e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                81 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_3: {
        if *bas
            .offset((8 as ::core::ffi::c_int * i_sh + 0 as ::core::ffi::c_int) as isize)
            >= 0 as ::core::ffi::c_int
        {} else {
            __assert_fail(
                b"bas(ATOM_OF,i_sh) >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g1e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                82 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_4: {
        if *bas
            .offset((8 as ::core::ffi::c_int * j_sh + 0 as ::core::ffi::c_int) as isize)
            >= 0 as ::core::ffi::c_int
        {} else {
            __assert_fail(
                b"bas(ATOM_OF,j_sh) >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g1e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                83 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_5: {
        if *bas
            .offset((8 as ::core::ffi::c_int * i_sh + 0 as ::core::ffi::c_int) as isize)
            < natm
        {} else {
            __assert_fail(
                b"bas(ATOM_OF,i_sh) < natm\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g1e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_6: {
        if *bas
            .offset((8 as ::core::ffi::c_int * j_sh + 0 as ::core::ffi::c_int) as isize)
            < natm
        {} else {
            __assert_fail(
                b"bas(ATOM_OF,j_sh) < natm\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g1e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_7: {
        if (*envs).nrys_roots < 32 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"envs->nrys_roots < MXRYSROOTS\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g1e.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                86 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTg1e_index_xyz(
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let i_l: ::core::ffi::c_int = (*envs).i_l;
    let j_l: ::core::ffi::c_int = (*envs).j_l;
    let nfi: ::core::ffi::c_int = (*envs).nfi;
    let nfj: ::core::ffi::c_int = (*envs).nfj;
    let di: ::core::ffi::c_int = (*envs).g_stride_i;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ofx: ::core::ffi::c_int = 0;
    let mut ofjx: ::core::ffi::c_int = 0;
    let mut ofy: ::core::ffi::c_int = 0;
    let mut ofjy: ::core::ffi::c_int = 0;
    let mut ofz: ::core::ffi::c_int = 0;
    let mut ofjz: ::core::ffi::c_int = 0;
    let mut i_nx: [::core::ffi::c_int; 136] = [0; 136];
    let mut i_ny: [::core::ffi::c_int; 136] = [0; 136];
    let mut i_nz: [::core::ffi::c_int; 136] = [0; 136];
    let mut j_nx: [::core::ffi::c_int; 136] = [0; 136];
    let mut j_ny: [::core::ffi::c_int; 136] = [0; 136];
    let mut j_nz: [::core::ffi::c_int; 136] = [0; 136];
    CINTcart_comp(
        &raw mut i_nx as *mut ::core::ffi::c_int,
        &raw mut i_ny as *mut ::core::ffi::c_int,
        &raw mut i_nz as *mut ::core::ffi::c_int,
        i_l,
    );
    CINTcart_comp(
        &raw mut j_nx as *mut ::core::ffi::c_int,
        &raw mut j_ny as *mut ::core::ffi::c_int,
        &raw mut j_nz as *mut ::core::ffi::c_int,
        j_l,
    );
    ofx = 0 as ::core::ffi::c_int;
    ofy = (*envs).g_size;
    ofz = (*envs).g_size * 2 as ::core::ffi::c_int;
    n = 0 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    while j < nfj {
        ofjx = ofx + dj * j_nx[j as usize];
        ofjy = ofy + dj * j_ny[j as usize];
        ofjz = ofz + dj * j_nz[j as usize];
        i = 0 as ::core::ffi::c_int;
        while i < nfi {
            *idx.offset((n + 0 as ::core::ffi::c_int) as isize) = ofjx
                + di * i_nx[i as usize];
            *idx.offset((n + 1 as ::core::ffi::c_int) as isize) = ofjy
                + di * i_ny[i as usize];
            *idx.offset((n + 2 as ::core::ffi::c_int) as isize) = ofjz
                + di * i_nz[i as usize];
            n += 3 as ::core::ffi::c_int;
            i += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTg1e_ovlp(
    mut g: *mut ::core::ffi::c_double,
    mut envs: *mut CINTEnvVars,
) -> ::core::ffi::c_int {
    let mut gx: *mut ::core::ffi::c_double = g;
    let mut gy: *mut ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut aij: ::core::ffi::c_double = (*envs).ai[0 as ::core::ffi::c_int as usize]
        + (*envs).aj[0 as ::core::ffi::c_int as usize];
    *gx.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *gy.offset(0 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    *gz.offset(0 as ::core::ffi::c_int as isize) = (*envs)
        .fac[0 as ::core::ffi::c_int as usize] * SQRTPI * M_PI / (aij * sqrt(aij));
    let mut nmax: ::core::ffi::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    if nmax == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    let mut rij: *mut ::core::ffi::c_double = &raw mut (*envs).rij
        as *mut ::core::ffi::c_double;
    let mut rirj: *mut ::core::ffi::c_double = &raw mut (*envs).rirj
        as *mut ::core::ffi::c_double;
    let mut lj: ::core::ffi::c_int = 0;
    let mut di: ::core::ffi::c_int = 0;
    let mut dj: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let mut rx: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    if (*envs).li_ceil > (*envs).lj_ceil {
        lj = (*envs).lj_ceil;
        di = (*envs).g_stride_i;
        dj = (*envs).g_stride_j;
        rx = (*envs).ri;
    } else {
        lj = (*envs).li_ceil;
        di = (*envs).g_stride_j;
        dj = (*envs).g_stride_i;
        rx = (*envs).rj;
    }
    let mut rijrx: [::core::ffi::c_double; 3] = [0.; 3];
    rijrx[0 as ::core::ffi::c_int as usize] = *rij
        .offset(0 as ::core::ffi::c_int as isize)
        - *rx.offset(0 as ::core::ffi::c_int as isize);
    rijrx[1 as ::core::ffi::c_int as usize] = *rij
        .offset(1 as ::core::ffi::c_int as isize)
        - *rx.offset(1 as ::core::ffi::c_int as isize);
    rijrx[2 as ::core::ffi::c_int as usize] = *rij
        .offset(2 as ::core::ffi::c_int as isize)
        - *rx.offset(2 as ::core::ffi::c_int as isize);
    *gx.offset(di as isize) = rijrx[0 as ::core::ffi::c_int as usize]
        * *gx.offset(0 as ::core::ffi::c_int as isize);
    *gy.offset(di as isize) = rijrx[1 as ::core::ffi::c_int as usize]
        * *gy.offset(0 as ::core::ffi::c_int as isize);
    *gz.offset(di as isize) = rijrx[2 as ::core::ffi::c_int as usize]
        * *gz.offset(0 as ::core::ffi::c_int as isize);
    let mut aij2: ::core::ffi::c_double = 0.5f64 / aij;
    i = 1 as ::core::ffi::c_int;
    while i < nmax {
        *gx.offset(((i + 1 as ::core::ffi::c_int) * di) as isize) = i
            as ::core::ffi::c_double * aij2
            * *gx.offset(((i - 1 as ::core::ffi::c_int) * di) as isize)
            + rijrx[0 as ::core::ffi::c_int as usize] * *gx.offset((i * di) as isize);
        *gy.offset(((i + 1 as ::core::ffi::c_int) * di) as isize) = i
            as ::core::ffi::c_double * aij2
            * *gy.offset(((i - 1 as ::core::ffi::c_int) * di) as isize)
            + rijrx[1 as ::core::ffi::c_int as usize] * *gy.offset((i * di) as isize);
        *gz.offset(((i + 1 as ::core::ffi::c_int) * di) as isize) = i
            as ::core::ffi::c_double * aij2
            * *gz.offset(((i - 1 as ::core::ffi::c_int) * di) as isize)
            + rijrx[2 as ::core::ffi::c_int as usize] * *gz.offset((i * di) as isize);
        i += 1;
    }
    j = 1 as ::core::ffi::c_int;
    while j <= lj {
        ptr = dj * j;
        i = 0 as ::core::ffi::c_int;
        n = ptr;
        while i <= nmax - j {
            *gx.offset(n as isize) = *gx.offset((n + di - dj) as isize)
                + *rirj.offset(0 as ::core::ffi::c_int as isize)
                    * *gx.offset((n - dj) as isize);
            *gy.offset(n as isize) = *gy.offset((n + di - dj) as isize)
                + *rirj.offset(1 as ::core::ffi::c_int as isize)
                    * *gy.offset((n - dj) as isize);
            *gz.offset(n as isize) = *gz.offset((n + di - dj) as isize)
                + *rirj.offset(2 as ::core::ffi::c_int as isize)
                    * *gz.offset((n - dj) as isize);
            i += 1;
            n += di;
        }
        j += 1;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINTnuc_mod(
    mut aij: ::core::ffi::c_double,
    mut nuc_id: ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut zeta: ::core::ffi::c_double = 0.;
    if nuc_id < 0 as ::core::ffi::c_int {
        zeta = *env.offset(PTR_RINV_ZETA as isize);
    } else if *atm.offset((ATM_SLOTS * nuc_id + 2 as ::core::ffi::c_int) as isize)
        == GAUSSIAN_NUC
    {
        zeta = *env
            .offset(
                *atm.offset((ATM_SLOTS * nuc_id + 3 as ::core::ffi::c_int) as isize)
                    as isize,
            );
    } else {
        zeta = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    }
    if zeta > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        return sqrt(zeta / (aij + zeta))
    } else {
        return 1 as ::core::ffi::c_int as ::core::ffi::c_double
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTg1e_nuc(
    mut g: *mut ::core::ffi::c_double,
    mut envs: *mut CINTEnvVars,
    mut nuc_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut atm: *mut ::core::ffi::c_int = (*envs).atm;
    let mut env: *mut ::core::ffi::c_double = (*envs).env;
    let mut rij: *mut ::core::ffi::c_double = &raw mut (*envs).rij
        as *mut ::core::ffi::c_double;
    let mut gx: *mut ::core::ffi::c_double = g;
    let mut gy: *mut ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut u: [::core::ffi::c_double; 32] = [0.; 32];
    let mut w: *mut ::core::ffi::c_double = gz;
    let mut cr: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut crij: [::core::ffi::c_double; 3] = [0.; 3];
    let mut x: ::core::ffi::c_double = 0.;
    let mut fac1: ::core::ffi::c_double = 0.;
    let mut aij: ::core::ffi::c_double = (*envs).ai[0 as ::core::ffi::c_int as usize]
        + (*envs).aj[0 as ::core::ffi::c_int as usize];
    let mut tau: ::core::ffi::c_double = CINTnuc_mod(aij, nuc_id, atm, env);
    if nuc_id < 0 as ::core::ffi::c_int {
        fac1 = 2 as ::core::ffi::c_int as ::core::ffi::c_double * M_PI
            * (*envs).fac[0 as ::core::ffi::c_int as usize] * tau / aij;
        cr = env.offset(PTR_RINV_ORIG as isize);
    } else if *atm.offset((ATM_SLOTS * nuc_id + 2 as ::core::ffi::c_int) as isize)
        == FRAC_CHARGE_NUC
    {
        fac1 = 2 as ::core::ffi::c_int as ::core::ffi::c_double * M_PI
            * -*env
                .offset(
                    *atm.offset((PTR_FRAC_CHARGE + nuc_id * ATM_SLOTS) as isize) as isize,
                ) * (*envs).fac[0 as ::core::ffi::c_int as usize] * tau / aij;
        cr = env
            .offset(
                *atm.offset((ATM_SLOTS * nuc_id + 1 as ::core::ffi::c_int) as isize)
                    as isize,
            );
    } else {
        fac1 = 2 as ::core::ffi::c_int as ::core::ffi::c_double * M_PI
            * -abs(*atm.offset((CHARGE_OF + nuc_id * ATM_SLOTS) as isize))
                as ::core::ffi::c_double * (*envs).fac[0 as ::core::ffi::c_int as usize]
            * tau / aij;
        cr = env
            .offset(
                *atm.offset((ATM_SLOTS * nuc_id + 1 as ::core::ffi::c_int) as isize)
                    as isize,
            );
    }
    crij[0 as ::core::ffi::c_int as usize] = *cr.offset(0 as ::core::ffi::c_int as isize)
        - *rij.offset(0 as ::core::ffi::c_int as isize);
    crij[1 as ::core::ffi::c_int as usize] = *cr.offset(1 as ::core::ffi::c_int as isize)
        - *rij.offset(1 as ::core::ffi::c_int as isize);
    crij[2 as ::core::ffi::c_int as usize] = *cr.offset(2 as ::core::ffi::c_int as isize)
        - *rij.offset(2 as ::core::ffi::c_int as isize);
    x = aij * tau * tau
        * (crij[0 as ::core::ffi::c_int as usize]
            * crij[0 as ::core::ffi::c_int as usize]
            + crij[1 as ::core::ffi::c_int as usize]
                * crij[1 as ::core::ffi::c_int as usize]
            + crij[2 as ::core::ffi::c_int as usize]
                * crij[2 as ::core::ffi::c_int as usize]);
    CINTrys_roots(nrys_roots, x, &raw mut u as *mut ::core::ffi::c_double, w);
    i = 0 as ::core::ffi::c_int;
    while i < nrys_roots {
        *gx.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *gy.offset(i as isize) = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
        *gz.offset(i as isize) *= fac1;
        i += 1;
    }
    let mut nmax: ::core::ffi::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    if nmax == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    let mut p0x: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p0y: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p0z: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p1x: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p1y: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p1z: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p2x: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p2y: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut p2z: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut lj: ::core::ffi::c_int = 0;
    let mut di: ::core::ffi::c_int = 0;
    let mut dj: ::core::ffi::c_int = 0;
    let mut rx: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    if (*envs).li_ceil > (*envs).lj_ceil {
        lj = (*envs).lj_ceil;
        di = (*envs).g_stride_i;
        dj = (*envs).g_stride_j;
        rx = (*envs).ri;
    } else {
        lj = (*envs).li_ceil;
        di = (*envs).g_stride_j;
        dj = (*envs).g_stride_i;
        rx = (*envs).rj;
    }
    let mut rijrx: ::core::ffi::c_double = *rij.offset(0 as ::core::ffi::c_int as isize)
        - *rx.offset(0 as ::core::ffi::c_int as isize);
    let mut rijry: ::core::ffi::c_double = *rij.offset(1 as ::core::ffi::c_int as isize)
        - *rx.offset(1 as ::core::ffi::c_int as isize);
    let mut rijrz: ::core::ffi::c_double = *rij.offset(2 as ::core::ffi::c_int as isize)
        - *rx.offset(2 as ::core::ffi::c_int as isize);
    let mut aij2: ::core::ffi::c_double = 0.5f64 / aij;
    let mut ru: ::core::ffi::c_double = 0.;
    let mut rt: ::core::ffi::c_double = 0.;
    let mut r0: ::core::ffi::c_double = 0.;
    let mut r1: ::core::ffi::c_double = 0.;
    let mut r2: ::core::ffi::c_double = 0.;
    p0x = gx.offset(di as isize);
    p0y = gy.offset(di as isize);
    p0z = gz.offset(di as isize);
    p1x = gx.offset(-(di as isize));
    p1y = gy.offset(-(di as isize));
    p1z = gz.offset(-(di as isize));
    n = 0 as ::core::ffi::c_int;
    while n < nrys_roots {
        ru = tau * tau * u[n as usize]
            / (1 as ::core::ffi::c_int as ::core::ffi::c_double + u[n as usize]);
        rt = aij2 - aij2 * ru;
        r0 = rijrx + ru * crij[0 as ::core::ffi::c_int as usize];
        r1 = rijry + ru * crij[1 as ::core::ffi::c_int as usize];
        r2 = rijrz + ru * crij[2 as ::core::ffi::c_int as usize];
        *p0x.offset(n as isize) = r0 * *gx.offset(n as isize);
        *p0y.offset(n as isize) = r1 * *gy.offset(n as isize);
        *p0z.offset(n as isize) = r2 * *gz.offset(n as isize);
        i = 1 as ::core::ffi::c_int;
        while i < nmax {
            *p0x.offset((n + i * di) as isize) = i as ::core::ffi::c_double * rt
                * *p1x.offset((n + i * di) as isize)
                + r0 * *gx.offset((n + i * di) as isize);
            *p0y.offset((n + i * di) as isize) = i as ::core::ffi::c_double * rt
                * *p1y.offset((n + i * di) as isize)
                + r1 * *gy.offset((n + i * di) as isize);
            *p0z.offset((n + i * di) as isize) = i as ::core::ffi::c_double * rt
                * *p1z.offset((n + i * di) as isize)
                + r2 * *gz.offset((n + i * di) as isize);
            i += 1;
        }
        n += 1;
    }
    let mut rirjx: ::core::ffi::c_double = (*envs)
        .rirj[0 as ::core::ffi::c_int as usize];
    let mut rirjy: ::core::ffi::c_double = (*envs)
        .rirj[1 as ::core::ffi::c_int as usize];
    let mut rirjz: ::core::ffi::c_double = (*envs)
        .rirj[2 as ::core::ffi::c_int as usize];
    j = 1 as ::core::ffi::c_int;
    while j <= lj {
        p0x = gx.offset((j * dj) as isize);
        p0y = gy.offset((j * dj) as isize);
        p0z = gz.offset((j * dj) as isize);
        p1x = p0x.offset(-(dj as isize));
        p1y = p0y.offset(-(dj as isize));
        p1z = p0z.offset(-(dj as isize));
        p2x = p1x.offset(di as isize);
        p2y = p1y.offset(di as isize);
        p2z = p1z.offset(di as isize);
        i = 0 as ::core::ffi::c_int;
        while i <= nmax - j {
            n = 0 as ::core::ffi::c_int;
            while n < nrys_roots {
                *p0x.offset((n + i * di) as isize) = *p2x.offset((n + i * di) as isize)
                    + rirjx * *p1x.offset((n + i * di) as isize);
                *p0y.offset((n + i * di) as isize) = *p2y.offset((n + i * di) as isize)
                    + rirjy * *p1y.offset((n + i * di) as isize);
                *p0z.offset((n + i * di) as isize) = *p2z.offset((n + i * di) as isize)
                    + rirjz * *p1z.offset((n + i * di) as isize);
                n += 1;
            }
            i += 1;
        }
        j += 1;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1i_1e(
    mut f: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut li: ::core::ffi::c_int,
    mut lj: ::core::ffi::c_int,
    mut lk: ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let ai2: ::core::ffi::c_double = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
        * (*envs).ai[0 as ::core::ffi::c_int as usize];
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let mut gx: *const ::core::ffi::c_double = g;
    let mut gy: *const ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut fx: *mut ::core::ffi::c_double = f;
    let mut fy: *mut ::core::ffi::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut ::core::ffi::c_double = f
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    k = 0 as ::core::ffi::c_int;
    while k <= lk {
        j = 0 as ::core::ffi::c_int;
        while j <= lj {
            ptr = dj * j + dk * k;
            *fx.offset(ptr as isize) = ai2
                * *gx.offset((ptr + 1 as ::core::ffi::c_int) as isize);
            *fy.offset(ptr as isize) = ai2
                * *gy.offset((ptr + 1 as ::core::ffi::c_int) as isize);
            *fz.offset(ptr as isize) = ai2
                * *gz.offset((ptr + 1 as ::core::ffi::c_int) as isize);
            i = 1 as ::core::ffi::c_int;
            while i <= li {
                *fx.offset((ptr + i) as isize) = i as ::core::ffi::c_double
                    * *gx.offset((ptr + i - 1 as ::core::ffi::c_int) as isize)
                    + ai2 * *gx.offset((ptr + i + 1 as ::core::ffi::c_int) as isize);
                *fy.offset((ptr + i) as isize) = i as ::core::ffi::c_double
                    * *gy.offset((ptr + i - 1 as ::core::ffi::c_int) as isize)
                    + ai2 * *gy.offset((ptr + i + 1 as ::core::ffi::c_int) as isize);
                *fz.offset((ptr + i) as isize) = i as ::core::ffi::c_double
                    * *gz.offset((ptr + i - 1 as ::core::ffi::c_int) as isize)
                    + ai2 * *gz.offset((ptr + i + 1 as ::core::ffi::c_int) as isize);
                i += 1;
            }
            j += 1;
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1j_1e(
    mut f: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut li: ::core::ffi::c_int,
    mut lj: ::core::ffi::c_int,
    mut lk: ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let aj2: ::core::ffi::c_double = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
        * (*envs).aj[0 as ::core::ffi::c_int as usize];
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let mut gx: *const ::core::ffi::c_double = g;
    let mut gy: *const ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut fx: *mut ::core::ffi::c_double = f;
    let mut fy: *mut ::core::ffi::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut ::core::ffi::c_double = f
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    k = 0 as ::core::ffi::c_int;
    while k <= lk {
        ptr = dk * k;
        i = ptr;
        while i <= ptr + li {
            *fx.offset(i as isize) = aj2 * *gx.offset((i + dj) as isize);
            *fy.offset(i as isize) = aj2 * *gy.offset((i + dj) as isize);
            *fz.offset(i as isize) = aj2 * *gz.offset((i + dj) as isize);
            i += 1;
        }
        j = 1 as ::core::ffi::c_int;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx.offset(i as isize) = j as ::core::ffi::c_double
                    * *gx.offset((i - dj) as isize)
                    + aj2 * *gx.offset((i + dj) as isize);
                *fy.offset(i as isize) = j as ::core::ffi::c_double
                    * *gy.offset((i - dj) as isize)
                    + aj2 * *gy.offset((i + dj) as isize);
                *fz.offset(i as isize) = j as ::core::ffi::c_double
                    * *gz.offset((i - dj) as isize)
                    + aj2 * *gz.offset((i + dj) as isize);
                i += 1;
            }
            j += 1;
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1k_1e(
    mut f: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut li: ::core::ffi::c_int,
    mut lj: ::core::ffi::c_int,
    mut lk: ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let ak2: ::core::ffi::c_double = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
        * (*envs).ak[0 as ::core::ffi::c_int as usize];
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let mut gx: *const ::core::ffi::c_double = g;
    let mut gy: *const ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut fx: *mut ::core::ffi::c_double = f;
    let mut fy: *mut ::core::ffi::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut ::core::ffi::c_double = f
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    j = 0 as ::core::ffi::c_int;
    while j <= lj {
        ptr = dj * j;
        i = ptr;
        while i <= ptr + li {
            *fx.offset(i as isize) = ak2 * *gx.offset((i + dk) as isize);
            *fy.offset(i as isize) = ak2 * *gy.offset((i + dk) as isize);
            *fz.offset(i as isize) = ak2 * *gz.offset((i + dk) as isize);
            i += 1;
        }
        j += 1;
    }
    k = 1 as ::core::ffi::c_int;
    while k <= lk {
        j = 0 as ::core::ffi::c_int;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx.offset(i as isize) = k as ::core::ffi::c_double
                    * *gx.offset((i - dk) as isize)
                    + ak2 * *gx.offset((i + dk) as isize);
                *fy.offset(i as isize) = k as ::core::ffi::c_double
                    * *gy.offset((i - dk) as isize)
                    + ak2 * *gy.offset((i + dk) as isize);
                *fz.offset(i as isize) = k as ::core::ffi::c_double
                    * *gz.offset((i - dk) as isize)
                    + ak2 * *gz.offset((i + dk) as isize);
                i += 1;
            }
            j += 1;
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1i_1e(
    mut f: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut ri: *mut ::core::ffi::c_double,
    mut li: ::core::ffi::c_int,
    mut lj: ::core::ffi::c_int,
    mut lk: ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let mut gx: *const ::core::ffi::c_double = g;
    let mut gy: *const ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut fx: *mut ::core::ffi::c_double = f;
    let mut fy: *mut ::core::ffi::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut ::core::ffi::c_double = f
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    k = 0 as ::core::ffi::c_int;
    while k <= lk {
        j = 0 as ::core::ffi::c_int;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx.offset(i as isize) = *gx
                    .offset((i + 1 as ::core::ffi::c_int) as isize)
                    + *ri.offset(0 as ::core::ffi::c_int as isize)
                        * *gx.offset(i as isize);
                *fy.offset(i as isize) = *gy
                    .offset((i + 1 as ::core::ffi::c_int) as isize)
                    + *ri.offset(1 as ::core::ffi::c_int as isize)
                        * *gy.offset(i as isize);
                *fz.offset(i as isize) = *gz
                    .offset((i + 1 as ::core::ffi::c_int) as isize)
                    + *ri.offset(2 as ::core::ffi::c_int as isize)
                        * *gz.offset(i as isize);
                i += 1;
            }
            j += 1;
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1j_1e(
    mut f: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut rj: *mut ::core::ffi::c_double,
    mut li: ::core::ffi::c_int,
    mut lj: ::core::ffi::c_int,
    mut lk: ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let mut gx: *const ::core::ffi::c_double = g;
    let mut gy: *const ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut fx: *mut ::core::ffi::c_double = f;
    let mut fy: *mut ::core::ffi::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut ::core::ffi::c_double = f
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    k = 0 as ::core::ffi::c_int;
    while k <= lk {
        j = 0 as ::core::ffi::c_int;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx.offset(i as isize) = *gx.offset((i + dj) as isize)
                    + *rj.offset(0 as ::core::ffi::c_int as isize)
                        * *gx.offset(i as isize);
                *fy.offset(i as isize) = *gy.offset((i + dj) as isize)
                    + *rj.offset(1 as ::core::ffi::c_int as isize)
                        * *gy.offset(i as isize);
                *fz.offset(i as isize) = *gz.offset((i + dj) as isize)
                    + *rj.offset(2 as ::core::ffi::c_int as isize)
                        * *gz.offset(i as isize);
                i += 1;
            }
            j += 1;
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1k_1e(
    mut f: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut rk: *mut ::core::ffi::c_double,
    mut li: ::core::ffi::c_int,
    mut lj: ::core::ffi::c_int,
    mut lk: ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let dk: ::core::ffi::c_int = (*envs).g_stride_k;
    let mut gx: *const ::core::ffi::c_double = g;
    let mut gy: *const ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut fx: *mut ::core::ffi::c_double = f;
    let mut fy: *mut ::core::ffi::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut ::core::ffi::c_double = f
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    k = 0 as ::core::ffi::c_int;
    while k <= lk {
        j = 0 as ::core::ffi::c_int;
        while j <= lj {
            ptr = dj * j + dk * k;
            i = ptr;
            while i <= ptr + li {
                *fx.offset(i as isize) = *gx.offset((i + dk) as isize)
                    + *rk.offset(0 as ::core::ffi::c_int as isize)
                        * *gx.offset(i as isize);
                *fy.offset(i as isize) = *gy.offset((i + dk) as isize)
                    + *rk.offset(1 as ::core::ffi::c_int as isize)
                        * *gy.offset(i as isize);
                *fz.offset(i as isize) = *gz.offset((i + dk) as isize)
                    + *rk.offset(2 as ::core::ffi::c_int as isize)
                        * *gz.offset(i as isize);
                i += 1;
            }
            j += 1;
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTprim_to_ctr(
    mut gc: *mut ::core::ffi::c_double,
    mut nf: ::core::ffi::c_int,
    mut gp: *mut ::core::ffi::c_double,
    mut inc: ::core::ffi::c_int,
    mut nprim: ::core::ffi::c_int,
    mut nctr: ::core::ffi::c_int,
    mut coeff: *mut ::core::ffi::c_double,
) {
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut pgc: *mut ::core::ffi::c_double = gc;
    let mut c: ::core::ffi::c_double = 0.;
    i = 0 as ::core::ffi::c_int;
    while i < inc {
        n = 0 as ::core::ffi::c_int;
        while n < nctr {
            c = *coeff.offset((nprim * n) as isize);
            if c != 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                k = 0 as ::core::ffi::c_int;
                while k < nf {
                    *pgc.offset(k as isize) += c * *gp.offset((k * inc + i) as isize);
                    k += 1;
                }
            }
            pgc = pgc.offset(nf as isize);
            n += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTprim_to_ctr_0(
    mut gc: *mut ::core::ffi::c_double,
    mut gp: *mut ::core::ffi::c_double,
    mut coeff: *mut ::core::ffi::c_double,
    mut nf: size_t,
    mut nprim: ::core::ffi::c_int,
    mut nctr: ::core::ffi::c_int,
    mut non0ctr: ::core::ffi::c_int,
    mut sortedidx: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut n: size_t = 0;
    let mut c0: ::core::ffi::c_double = 0.;
    i = 0 as ::core::ffi::c_int;
    while i < nctr {
        c0 = *coeff.offset((nprim * i) as isize);
        n = 0 as size_t;
        while n < nf {
            *gc.offset(nf.wrapping_mul(i as size_t).wrapping_add(n) as isize) = c0
                * *gp.offset(n as isize);
            n = n.wrapping_add(1);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTprim_to_ctr_1(
    mut gc: *mut ::core::ffi::c_double,
    mut gp: *mut ::core::ffi::c_double,
    mut coeff: *mut ::core::ffi::c_double,
    mut nf: size_t,
    mut nprim: ::core::ffi::c_int,
    mut nctr: ::core::ffi::c_int,
    mut non0ctr: ::core::ffi::c_int,
    mut sortedidx: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: size_t = 0;
    let mut c0: ::core::ffi::c_double = 0.;
    i = 0 as ::core::ffi::c_int;
    while i < non0ctr {
        c0 = *coeff.offset((nprim * *sortedidx.offset(i as isize)) as isize);
        j = *sortedidx.offset(i as isize);
        n = 0 as size_t;
        while n < nf {
            *gc.offset(nf.wrapping_mul(j as size_t).wrapping_add(n) as isize)
                += c0 * *gp.offset(n as isize);
            n = n.wrapping_add(1);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTcommon_fac_sp(
    mut l: ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    match l {
        0 => return 0.282094791773878143f64,
        1 => return 0.488602511902919921f64,
        _ => return 1 as ::core::ffi::c_int as ::core::ffi::c_double,
    };
}
pub const SQRTPI: ::core::ffi::c_double = 1.7724538509055160272981674833411451f64;
pub const IINC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JINC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GSHIFT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const POS_E1: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SLOT_RYS_ROOTS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const TENSOR: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const EXPCUTOFF: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
