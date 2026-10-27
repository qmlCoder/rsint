extern "C" {
    fn CINTcgto_spinor(
        bas_id: ::core::ffi::c_int,
        bas: *const ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn CINTinit_int1e_EnvVars(
        envs: *mut CINTEnvVars,
        ng: *mut ::core::ffi::c_int,
        shls: *mut ::core::ffi::c_int,
        atm: *mut ::core::ffi::c_int,
        natm: ::core::ffi::c_int,
        bas: *mut ::core::ffi::c_int,
        nbas: ::core::ffi::c_int,
        env: *mut ::core::ffi::c_double,
    );
    fn CINTnabla1i_1e(
        f: *mut ::core::ffi::c_double,
        g: *mut ::core::ffi::c_double,
        li: ::core::ffi::c_int,
        lj: ::core::ffi::c_int,
        lk: ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
    );
    fn CINTnabla1j_1e(
        f: *mut ::core::ffi::c_double,
        g: *mut ::core::ffi::c_double,
        li: ::core::ffi::c_int,
        lj: ::core::ffi::c_int,
        lk: ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
    );
    fn CINTx1i_1e(
        f: *mut ::core::ffi::c_double,
        g: *mut ::core::ffi::c_double,
        ri: *mut ::core::ffi::c_double,
        li: ::core::ffi::c_int,
        lj: ::core::ffi::c_int,
        lk: ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
    );
    fn CINTx1j_1e(
        f: *mut ::core::ffi::c_double,
        g: *mut ::core::ffi::c_double,
        rj: *mut ::core::ffi::c_double,
        li: ::core::ffi::c_int,
        lj: ::core::ffi::c_int,
        lk: ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
    );
    fn c2s_sph_1e(
        opij: *mut ::core::ffi::c_double,
        gctr: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut ::core::ffi::c_double,
    );
    fn c2s_cart_1e(
        opij: *mut ::core::ffi::c_double,
        gctr: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut ::core::ffi::c_double,
    );
    fn c2s_dset0(
        out: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        counts: *mut ::core::ffi::c_int,
    );
    fn CINTnabla1i_2e(
        f: *mut ::core::ffi::c_double,
        g: *const ::core::ffi::c_double,
        li: ::core::ffi::c_int,
        lj: ::core::ffi::c_int,
        lk: ::core::ffi::c_int,
        ll: ::core::ffi::c_int,
        envs: *const CINTEnvVars,
    );
    fn CINTnabla1j_2e(
        f: *mut ::core::ffi::c_double,
        g: *const ::core::ffi::c_double,
        li: ::core::ffi::c_int,
        lj: ::core::ffi::c_int,
        lk: ::core::ffi::c_int,
        ll: ::core::ffi::c_int,
        envs: *const CINTEnvVars,
    );
    fn CINTx1i_2e(
        f: *mut ::core::ffi::c_double,
        g: *const ::core::ffi::c_double,
        ri: *const ::core::ffi::c_double,
        li: ::core::ffi::c_int,
        lj: ::core::ffi::c_int,
        lk: ::core::ffi::c_int,
        ll: ::core::ffi::c_int,
        envs: *const CINTEnvVars,
    );
    fn CINTall_1e_optimizer(
        opt: *mut *mut CINTOpt,
        ng: *mut ::core::ffi::c_int,
        atm: *mut ::core::ffi::c_int,
        natm: ::core::ffi::c_int,
        bas: *mut ::core::ffi::c_int,
        nbas: ::core::ffi::c_int,
        env: *mut ::core::ffi::c_double,
    );
    fn CINT1e_drv(
        out: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut ::core::ffi::c_double,
        f_c2s: Option<unsafe extern "C" fn() -> ()>,
        int1e_type: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const PTR_COMMON_ORIG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_sigma(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut s: [::core::ffi::c_double; 1] = [0.; 1];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        s[0 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = -s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = -s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = -s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) += -s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) += -s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) += -s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sigma_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sigma_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_sigma
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sigma_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_sigma
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_sigma_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_sigma_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_sigma_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_sigma_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_spsigmasp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut s: [::core::ffi::c_double; 9] = [0.; 9];
    CINTnabla1j_1e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g2,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g3,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        s[0 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[1 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[2 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[3 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[4 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[5 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[6 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[7 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[8 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = -s[0 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
                * s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
                * s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
                * s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = -s[4 as ::core::ffi::c_int as usize]
                + s[0 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
                * s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
                * s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
                * s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = -s[8 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize]
                + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += -s[0 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
                    * s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
                    * s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
                    * s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += -s[4 as ::core::ffi::c_int as usize]
                    + s[0 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
                    * s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
                    * s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
                    * s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += -s[8 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize]
                    + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spsigmasp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spsigmasp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_spsigmasp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spsigmasp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_spsigmasp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_spsigmasp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_spsigmasp_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_spsigmasp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_spsigmasp_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_srsr(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut s: [::core::ffi::c_double; 9] = [0.; 9];
    g1 = g0.offset((*envs).g_stride_j as isize);
    g2 = g0.offset((*envs).g_stride_i as isize);
    g3 = g1.offset((*envs).g_stride_i as isize);
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        s[0 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[1 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[2 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[3 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[4 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[5 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[6 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[7 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[8 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize]
                - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize]
                - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                - s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[5 as ::core::ffi::c_int as usize]
                    - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[6 as ::core::ffi::c_int as usize]
                    - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize]
                    - s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_srsr_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_srsr_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_srsr
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_srsr_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_srsr
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_srsr_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_srsr_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_srsr_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_srsr_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_sr(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut s: [::core::ffi::c_double; 3] = [0.; 3];
    g1 = g0.offset((*envs).g_stride_i as isize);
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        s[0 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[1 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[2 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = -s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += -s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sr_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sr_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_sr
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sr_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_sr
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_sr_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_sr_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_sr_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_sr_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_srsp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut s: [::core::ffi::c_double; 9] = [0.; 9];
    CINTnabla1j_1e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    g2 = g0.offset((*envs).g_stride_i as isize);
    g3 = g1.offset((*envs).g_stride_i as isize);
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        s[0 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[1 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[2 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[3 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[4 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[5 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[6 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[7 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[8 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = -s[5 as ::core::ffi::c_int as usize]
                + s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = -s[6 as ::core::ffi::c_int as usize]
                + s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = -s[1 as ::core::ffi::c_int as usize]
                + s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = -s[0 as ::core::ffi::c_int as usize]
                - s[4 as ::core::ffi::c_int as usize]
                - s[8 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += -s[5 as ::core::ffi::c_int as usize]
                    + s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += -s[6 as ::core::ffi::c_int as usize]
                    + s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += -s[1 as ::core::ffi::c_int as usize]
                    + s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += -s[0 as ::core::ffi::c_int as usize]
                    - s[4 as ::core::ffi::c_int as usize]
                    - s[8 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_srsp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_srsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_srsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_srsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_srsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_srsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_srsp_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_srsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_srsp_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_spsp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut s: [::core::ffi::c_double; 9] = [0.; 9];
    CINTnabla1j_1e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g2,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g3,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        s[0 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[1 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[2 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[3 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[4 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[5 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[6 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[7 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[8 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spsp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_spsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_spsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_spsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_spsp_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_spsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_spsp_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_sp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut s: [::core::ffi::c_double; 3] = [0.; 3];
    CINTnabla1i_1e(
        g1,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        s[0 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[1 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[2 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        } else {
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_sp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_sp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_sp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_sp_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_sp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_sp_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_spnucsp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g2,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g3,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 9] = [0.; 9];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 9 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize]
                - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize]
                - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                - s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[5 as ::core::ffi::c_int as usize]
                    - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[6 as ::core::ffi::c_int as usize]
                    - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize]
                    - s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spnucsp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spnucsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_spnucsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        2 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spnucsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_spnucsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        2 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_spnucsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_spnucsp_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_spnucsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_spnucsp_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_sprinvsp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g2,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g3,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 9] = [0.; 9];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 9 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize]
                - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize]
                - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                - s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[5 as ::core::ffi::c_int as usize]
                    - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[6 as ::core::ffi::c_int as usize]
                    - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize]
                    - s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sprinvsp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sprinvsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_sprinvsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sprinvsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_sprinvsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_sprinvsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_sprinvsp_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_sprinvsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_sprinvsp_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_srnucsr(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    g1 = g0.offset((*envs).g_stride_j as isize);
    g2 = g0.offset((*envs).g_stride_i as isize);
    g3 = g1.offset((*envs).g_stride_i as isize);
    let mut s: [::core::ffi::c_double; 9] = [0.; 9];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 9 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize]
                - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize]
                - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                - s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[5 as ::core::ffi::c_int as usize]
                    - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[6 as ::core::ffi::c_int as usize]
                    - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize]
                    - s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_srnucsr_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_srnucsr_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_srnucsr
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        2 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_srnucsr_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_srnucsr
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        2 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_srnucsr_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_srnucsr_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_srnucsr_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_srnucsr_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_sprsp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g4: *mut ::core::ffi::c_double = g3
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g5: *mut ::core::ffi::c_double = g4
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g6: *mut ::core::ffi::c_double = g5
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g7: *mut ::core::ffi::c_double = g6
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut drj: [::core::ffi::c_double; 3] = [0.; 3];
    drj[0 as ::core::ffi::c_int as usize] = *(*envs)
        .rj
        .offset(0 as ::core::ffi::c_int as isize)
        - *(*envs).env.offset((PTR_COMMON_ORIG + 0 as ::core::ffi::c_int) as isize);
    drj[1 as ::core::ffi::c_int as usize] = *(*envs)
        .rj
        .offset(1 as ::core::ffi::c_int as isize)
        - *(*envs).env.offset((PTR_COMMON_ORIG + 1 as ::core::ffi::c_int) as isize);
    drj[2 as ::core::ffi::c_int as usize] = *(*envs)
        .rj
        .offset(2 as ::core::ffi::c_int as isize)
        - *(*envs).env.offset((PTR_COMMON_ORIG + 2 as ::core::ffi::c_int) as isize);
    let mut s: [::core::ffi::c_double; 27] = [0.; 27];
    CINTnabla1j_1e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTx1j_1e(
        g2,
        g0,
        &raw mut drj as *mut ::core::ffi::c_double,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_1e(
        g3,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g4,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g5,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g6,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g7,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        s[0 as ::core::ffi::c_int as usize] = *g7
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[1 as ::core::ffi::c_int as usize] = *g6
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[2 as ::core::ffi::c_int as usize] = *g6
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[3 as ::core::ffi::c_int as usize] = *g5
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[4 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[5 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[6 as ::core::ffi::c_int as usize] = *g5
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[7 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[8 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[9 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[10 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[11 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[12 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[13 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g7.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[14 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[15 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[16 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[17 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[18 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[19 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[20 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[21 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[22 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[23 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[24 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[25 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[26 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g7.offset((iz + 0 as ::core::ffi::c_int) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[18 as ::core::ffi::c_int as usize]
                - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                - s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[10 as ::core::ffi::c_int as usize]
                + s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[14 as ::core::ffi::c_int as usize]
                - s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize]
                - s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize]
                + s[13 as ::core::ffi::c_int as usize]
                + s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[17 as ::core::ffi::c_int as usize]
                - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[24 as ::core::ffi::c_int as usize]
                - s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize]
                + s[16 as ::core::ffi::c_int as usize]
                + s[26 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += s[18 as ::core::ffi::c_int as usize]
                    - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += s[1 as ::core::ffi::c_int as usize]
                    - s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[10 as ::core::ffi::c_int as usize]
                    + s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += s[14 as ::core::ffi::c_int as usize]
                    - s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += s[4 as ::core::ffi::c_int as usize]
                    - s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += s[3 as ::core::ffi::c_int as usize]
                    + s[13 as ::core::ffi::c_int as usize]
                    + s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += s[17 as ::core::ffi::c_int as usize]
                    - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += s[24 as ::core::ffi::c_int as usize]
                    - s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[6 as ::core::ffi::c_int as usize]
                    + s[16 as ::core::ffi::c_int as usize]
                    + s[26 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sprsp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sprsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_sprsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sprsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_sprsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_sprsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_sprsp_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_sprsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_sprsp_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_govlp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut rirj: [::core::ffi::c_double; 3] = [0.; 3];
    let mut c: [::core::ffi::c_double; 3] = [0.; 3];
    rirj[0 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(0 as ::core::ffi::c_int as isize)
        - *(*envs).rj.offset(0 as ::core::ffi::c_int as isize);
    rirj[1 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(1 as ::core::ffi::c_int as isize)
        - *(*envs).rj.offset(1 as ::core::ffi::c_int as isize);
    rirj[2 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(2 as ::core::ffi::c_int as isize)
        - *(*envs).rj.offset(2 as ::core::ffi::c_int as isize);
    c[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double * rirj[0 as ::core::ffi::c_int as usize];
    c[1 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double * rirj[1 as ::core::ffi::c_int as usize];
    c[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double * rirj[2 as ::core::ffi::c_int as usize];
    let mut s: [::core::ffi::c_double; 3] = [0.; 3];
    CINTx1i_1e(
        g1,
        g0,
        (*envs).ri as *mut ::core::ffi::c_double,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        s[0 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[1 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[2 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[2 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[0 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[1 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[0 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += c[1 as ::core::ffi::c_int as usize]
                    * s[2 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += c[2 as ::core::ffi::c_int as usize]
                    * s[0 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += c[0 as ::core::ffi::c_int as usize]
                    * s[1 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[0 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_govlp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_govlp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_govlp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    if !out.is_null()
        && *envs.shls.offset(0 as ::core::ffi::c_int as isize)
            == *envs.shls.offset(1 as ::core::ffi::c_int as isize)
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut nout: ::core::ffi::c_int = 0;
        let mut counts: [::core::ffi::c_int; 4] = [0; 4];
        counts[0 as ::core::ffi::c_int as usize] = envs.nfi
            * envs.x_ctr[0 as ::core::ffi::c_int as usize];
        counts[1 as ::core::ffi::c_int as usize] = envs.nfj
            * envs.x_ctr[1 as ::core::ffi::c_int as usize];
        counts[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        counts[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        if dims.is_null() {
            dims = &raw mut counts as *mut ::core::ffi::c_int;
        }
        nout = *dims.offset(0 as ::core::ffi::c_int as isize)
            * *dims.offset(1 as ::core::ffi::c_int as isize);
        i = 0 as ::core::ffi::c_int;
        while i < envs.ncomp_e1 * envs.ncomp_tensor {
            c2s_dset0(
                out.offset((nout * i) as isize),
                dims,
                &raw mut counts as *mut ::core::ffi::c_int,
            );
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_govlp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_govlp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    if !out.is_null()
        && *envs.shls.offset(0 as ::core::ffi::c_int as isize)
            == *envs.shls.offset(1 as ::core::ffi::c_int as isize)
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut nout: ::core::ffi::c_int = 0;
        let mut counts: [::core::ffi::c_int; 4] = [0; 4];
        counts[0 as ::core::ffi::c_int as usize] = (envs.i_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * envs.x_ctr[0 as ::core::ffi::c_int as usize];
        counts[1 as ::core::ffi::c_int as usize] = (envs.j_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * envs.x_ctr[1 as ::core::ffi::c_int as usize];
        counts[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        counts[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        if dims.is_null() {
            dims = &raw mut counts as *mut ::core::ffi::c_int;
        }
        nout = *dims.offset(0 as ::core::ffi::c_int as isize)
            * *dims.offset(1 as ::core::ffi::c_int as isize);
        i = 0 as ::core::ffi::c_int;
        while i < envs.ncomp_e1 * envs.ncomp_tensor {
            c2s_dset0(
                out.offset((nout * i) as isize),
                dims,
                &raw mut counts as *mut ::core::ffi::c_int,
            );
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_govlp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_govlp_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_govlp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_govlp_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_gnuc(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut rirj: [::core::ffi::c_double; 3] = [0.; 3];
    let mut c: [::core::ffi::c_double; 3] = [0.; 3];
    rirj[0 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(0 as ::core::ffi::c_int as isize)
        - *(*envs).rj.offset(0 as ::core::ffi::c_int as isize);
    rirj[1 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(1 as ::core::ffi::c_int as isize)
        - *(*envs).rj.offset(1 as ::core::ffi::c_int as isize);
    rirj[2 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(2 as ::core::ffi::c_int as isize)
        - *(*envs).rj.offset(2 as ::core::ffi::c_int as isize);
    c[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double * rirj[0 as ::core::ffi::c_int as usize];
    c[1 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double * rirj[1 as ::core::ffi::c_int as usize];
    c[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double * rirj[2 as ::core::ffi::c_int as usize];
    CINTx1i_2e(
        g1,
        g0,
        (*envs).ri,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 3] = [0.; 3];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 3 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[2 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[0 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[1 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[0 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += c[1 as ::core::ffi::c_int as usize]
                    * s[2 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += c[2 as ::core::ffi::c_int as usize]
                    * s[0 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += c[0 as ::core::ffi::c_int as usize]
                    * s[1 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[0 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_gnuc_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_gnuc_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_gnuc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    if !out.is_null()
        && *envs.shls.offset(0 as ::core::ffi::c_int as isize)
            == *envs.shls.offset(1 as ::core::ffi::c_int as isize)
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut nout: ::core::ffi::c_int = 0;
        let mut counts: [::core::ffi::c_int; 4] = [0; 4];
        counts[0 as ::core::ffi::c_int as usize] = envs.nfi
            * envs.x_ctr[0 as ::core::ffi::c_int as usize];
        counts[1 as ::core::ffi::c_int as usize] = envs.nfj
            * envs.x_ctr[1 as ::core::ffi::c_int as usize];
        counts[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        counts[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        if dims.is_null() {
            dims = &raw mut counts as *mut ::core::ffi::c_int;
        }
        nout = *dims.offset(0 as ::core::ffi::c_int as isize)
            * *dims.offset(1 as ::core::ffi::c_int as isize);
        i = 0 as ::core::ffi::c_int;
        while i < envs.ncomp_e1 * envs.ncomp_tensor {
            c2s_dset0(
                out.offset((nout * i) as isize),
                dims,
                &raw mut counts as *mut ::core::ffi::c_int,
            );
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        2 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_gnuc_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_gnuc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    if !out.is_null()
        && *envs.shls.offset(0 as ::core::ffi::c_int as isize)
            == *envs.shls.offset(1 as ::core::ffi::c_int as isize)
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut nout: ::core::ffi::c_int = 0;
        let mut counts: [::core::ffi::c_int; 4] = [0; 4];
        counts[0 as ::core::ffi::c_int as usize] = (envs.i_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * envs.x_ctr[0 as ::core::ffi::c_int as usize];
        counts[1 as ::core::ffi::c_int as usize] = (envs.j_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * envs.x_ctr[1 as ::core::ffi::c_int as usize];
        counts[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        counts[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        if dims.is_null() {
            dims = &raw mut counts as *mut ::core::ffi::c_int;
        }
        nout = *dims.offset(0 as ::core::ffi::c_int as isize)
            * *dims.offset(1 as ::core::ffi::c_int as isize);
        i = 0 as ::core::ffi::c_int;
        while i < envs.ncomp_e1 * envs.ncomp_tensor {
            c2s_dset0(
                out.offset((nout * i) as isize),
                dims,
                &raw mut counts as *mut ::core::ffi::c_int,
            );
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        2 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_gnuc_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_gnuc_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_gnuc_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_gnuc_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_cg_sa10sa01(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut dri: [::core::ffi::c_double; 3] = [0.; 3];
    dri[0 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(0 as ::core::ffi::c_int as isize)
        - *(*envs).env.offset((PTR_COMMON_ORIG + 0 as ::core::ffi::c_int) as isize);
    dri[1 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(1 as ::core::ffi::c_int as isize)
        - *(*envs).env.offset((PTR_COMMON_ORIG + 1 as ::core::ffi::c_int) as isize);
    dri[2 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(2 as ::core::ffi::c_int as isize)
        - *(*envs).env.offset((PTR_COMMON_ORIG + 2 as ::core::ffi::c_int) as isize);
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g2,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g1.offset(ix as isize) += *g2.offset(ix as isize);
        ix += 1;
    }
    CINTx1i_2e(
        g2,
        g0,
        &raw mut dri as *mut ::core::ffi::c_double,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTx1i_2e(
        g3,
        g1,
        &raw mut dri as *mut ::core::ffi::c_double,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 9] = [0.; 9];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 9 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = -s[7 as ::core::ffi::c_int as usize]
                + s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = -s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = -s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = -s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize]
                + s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize]
                + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) = -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                ) = -s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                ) = -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                ) = -s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                ) = -s[3 as ::core::ffi::c_int as usize]
                + s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize]
                + s[0 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += -s[7 as ::core::ffi::c_int as usize]
                    + s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[8 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) += s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) += -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) += -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) += -s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) += -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) += -s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) += -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) += -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) += -s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) += -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                )
                += -s[2 as ::core::ffi::c_int as usize]
                    + s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                )
                += s[8 as ::core::ffi::c_int as usize]
                    + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) += -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) += s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                ) += -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                ) += -s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                ) += -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                ) += -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                ) += -s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                )
                += -s[3 as ::core::ffi::c_int as usize]
                    + s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                )
                += s[4 as ::core::ffi::c_int as usize]
                    + s[0 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_cg_sa10sa01_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_cg_sa10sa01_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_cg_sa10sa01
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_cg_sa10sa01_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_cg_sa10sa01
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_cg_sa10sa01_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_cg_sa10sa01_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_cg_sa10sa01_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_cg_sa10sa01_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_cg_sa10sp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut dri: [::core::ffi::c_double; 3] = [0.; 3];
    dri[0 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(0 as ::core::ffi::c_int as isize)
        - *(*envs).env.offset((PTR_COMMON_ORIG + 0 as ::core::ffi::c_int) as isize);
    dri[1 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(1 as ::core::ffi::c_int as isize)
        - *(*envs).env.offset((PTR_COMMON_ORIG + 1 as ::core::ffi::c_int) as isize);
    dri[2 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(2 as ::core::ffi::c_int as isize)
        - *(*envs).env.offset((PTR_COMMON_ORIG + 2 as ::core::ffi::c_int) as isize);
    let mut s: [::core::ffi::c_double; 9] = [0.; 9];
    CINTnabla1j_1e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTx1i_1e(
        g2,
        g0,
        &raw mut dri as *mut ::core::ffi::c_double,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTx1i_1e(
        g3,
        g1,
        &raw mut dri as *mut ::core::ffi::c_double,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        s[0 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[1 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[2 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[3 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[4 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[5 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[6 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[7 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[8 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = -s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize]
                - s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = -s[6 as ::core::ffi::c_int as usize]
                + s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = -s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize]
                + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize]
                - s[1 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[8 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) += -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) += -s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[7 as ::core::ffi::c_int as usize]
                    - s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) += -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) += -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += -s[6 as ::core::ffi::c_int as usize]
                    + s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) += -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) += -s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[4 as ::core::ffi::c_int as usize]
                    + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[3 as ::core::ffi::c_int as usize]
                    - s[1 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_cg_sa10sp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_cg_sa10sp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_cg_sa10sp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_cg_sa10sp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_cg_sa10sp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_cg_sa10sp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_cg_sa10sp_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_cg_sa10sp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_cg_sa10sp_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_cg_sa10nucsp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut dri: [::core::ffi::c_double; 3] = [0.; 3];
    dri[0 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(0 as ::core::ffi::c_int as isize)
        - *(*envs).env.offset((PTR_COMMON_ORIG + 0 as ::core::ffi::c_int) as isize);
    dri[1 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(1 as ::core::ffi::c_int as isize)
        - *(*envs).env.offset((PTR_COMMON_ORIG + 1 as ::core::ffi::c_int) as isize);
    dri[2 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(2 as ::core::ffi::c_int as isize)
        - *(*envs).env.offset((PTR_COMMON_ORIG + 2 as ::core::ffi::c_int) as isize);
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTx1i_2e(
        g2,
        g0,
        &raw mut dri as *mut ::core::ffi::c_double,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTx1i_2e(
        g3,
        g1,
        &raw mut dri as *mut ::core::ffi::c_double,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 9] = [0.; 9];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 9 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = -s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize]
                - s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = -s[6 as ::core::ffi::c_int as usize]
                + s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = -s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize]
                + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize]
                - s[1 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[8 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) += -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) += -s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[7 as ::core::ffi::c_int as usize]
                    - s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) += -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) += -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += -s[6 as ::core::ffi::c_int as usize]
                    + s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) += -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) += -s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[4 as ::core::ffi::c_int as usize]
                    + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[3 as ::core::ffi::c_int as usize]
                    - s[1 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_cg_sa10nucsp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_cg_sa10nucsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_cg_sa10nucsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        2 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_cg_sa10nucsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_cg_sa10nucsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        2 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_cg_sa10nucsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_cg_sa10nucsp_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_cg_sa10nucsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_cg_sa10nucsp_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_giao_sa10sa01(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g2,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g1.offset(ix as isize) += *g2.offset(ix as isize);
        ix += 1;
    }
    g2 = g0.offset((*envs).g_stride_i as isize);
    g3 = g1.offset((*envs).g_stride_i as isize);
    let mut s: [::core::ffi::c_double; 9] = [0.; 9];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 9 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = -s[7 as ::core::ffi::c_int as usize]
                + s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = -s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = -s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = -s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize]
                + s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize]
                + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) = -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                ) = -s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                ) = -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                ) = -s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                ) = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                ) = -s[3 as ::core::ffi::c_int as usize]
                + s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize]
                + s[0 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += -s[7 as ::core::ffi::c_int as usize]
                    + s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[8 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) += s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) += -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) += -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) += -s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) += -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) += -s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) += -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) += -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) += -s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) += -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                )
                += -s[2 as ::core::ffi::c_int as usize]
                    + s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                )
                += s[8 as ::core::ffi::c_int as usize]
                    + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) += -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) += s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                ) += -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                ) += -s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                ) += -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                ) += -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                ) += -s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                ) += 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                )
                += -s[3 as ::core::ffi::c_int as usize]
                    + s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                )
                += s[4 as ::core::ffi::c_int as usize]
                    + s[0 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_giao_sa10sa01_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_giao_sa10sa01_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_giao_sa10sa01
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_giao_sa10sa01_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_giao_sa10sa01
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_giao_sa10sa01_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_giao_sa10sa01_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_giao_sa10sa01_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_giao_sa10sa01_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_giao_sa10sp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut s: [::core::ffi::c_double; 9] = [0.; 9];
    CINTnabla1j_1e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    g2 = g0.offset((*envs).g_stride_i as isize);
    g3 = g1.offset((*envs).g_stride_i as isize);
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        s[0 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[1 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[2 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[3 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[4 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[5 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[6 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[7 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[8 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = -s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize]
                - s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = -s[6 as ::core::ffi::c_int as usize]
                + s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = -s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize]
                + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize]
                - s[1 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[8 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) += -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) += -s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[7 as ::core::ffi::c_int as usize]
                    - s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) += -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) += -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += -s[6 as ::core::ffi::c_int as usize]
                    + s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) += -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) += -s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[4 as ::core::ffi::c_int as usize]
                    + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[3 as ::core::ffi::c_int as usize]
                    - s[1 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_giao_sa10sp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_giao_sa10sp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_giao_sa10sp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_giao_sa10sp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_giao_sa10sp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_giao_sa10sp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_giao_sa10sp_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_giao_sa10sp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_giao_sa10sp_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_giao_sa10nucsp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    g2 = g0.offset((*envs).g_stride_i as isize);
    g3 = g1.offset((*envs).g_stride_i as isize);
    let mut s: [::core::ffi::c_double; 9] = [0.; 9];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 9 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = -s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize]
                - s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = -s[6 as ::core::ffi::c_int as usize]
                + s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = -s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize]
                + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize]
                - s[1 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[8 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) += -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) += -s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[7 as ::core::ffi::c_int as usize]
                    - s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) += -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) += -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += -s[6 as ::core::ffi::c_int as usize]
                    + s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) += -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) += -s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[4 as ::core::ffi::c_int as usize]
                    + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[3 as ::core::ffi::c_int as usize]
                    - s[1 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_giao_sa10nucsp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_giao_sa10nucsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_giao_sa10nucsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        2 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_giao_sa10nucsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_giao_sa10nucsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        2 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_giao_sa10nucsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_giao_sa10nucsp_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_giao_sa10nucsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_giao_sa10nucsp_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_sa01sp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_2e(
        g2,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g3,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g2.offset(ix as isize) += *g3.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g3,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 9] = [0.; 9];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 9 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = -s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize]
                - s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = -s[6 as ::core::ffi::c_int as usize]
                + s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = -s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize]
                + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize]
                - s[1 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[8 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) += -s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) += -s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[7 as ::core::ffi::c_int as usize]
                    - s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) += -s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) += -s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += -s[6 as ::core::ffi::c_int as usize]
                    + s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) += -s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) += -s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[4 as ::core::ffi::c_int as usize]
                    + s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[3 as ::core::ffi::c_int as usize]
                    - s[1 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sa01sp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sa01sp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_sa01sp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_sa01sp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_sa01sp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_sa01sp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_sa01sp_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_sa01sp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_sa01sp_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_spgsp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g4: *mut ::core::ffi::c_double = g3
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g5: *mut ::core::ffi::c_double = g4
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g6: *mut ::core::ffi::c_double = g5
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g7: *mut ::core::ffi::c_double = g6
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut rirj: [::core::ffi::c_double; 3] = [0.; 3];
    let mut c: [::core::ffi::c_double; 3] = [0.; 3];
    rirj[0 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(0 as ::core::ffi::c_int as isize)
        - *(*envs).rj.offset(0 as ::core::ffi::c_int as isize);
    rirj[1 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(1 as ::core::ffi::c_int as isize)
        - *(*envs).rj.offset(1 as ::core::ffi::c_int as isize);
    rirj[2 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(2 as ::core::ffi::c_int as isize)
        - *(*envs).rj.offset(2 as ::core::ffi::c_int as isize);
    c[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double * rirj[0 as ::core::ffi::c_int as usize];
    c[1 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double * rirj[1 as ::core::ffi::c_int as usize];
    c[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double * rirj[2 as ::core::ffi::c_int as usize];
    let mut s: [::core::ffi::c_double; 27] = [0.; 27];
    CINTnabla1j_1e(
        g1,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTx1i_1e(
        g2,
        g0,
        (*envs).ri as *mut ::core::ffi::c_double,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTx1i_1e(
        g3,
        g1,
        (*envs).ri as *mut ::core::ffi::c_double,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g4,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g5,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g6,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g7,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        s[0 as ::core::ffi::c_int as usize] = *g7
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[1 as ::core::ffi::c_int as usize] = *g6
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[2 as ::core::ffi::c_int as usize] = *g6
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[3 as ::core::ffi::c_int as usize] = *g5
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[4 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[5 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[6 as ::core::ffi::c_int as usize] = *g5
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[7 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[8 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[9 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[10 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[11 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[12 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[13 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g7.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[14 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[15 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[16 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[17 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[18 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[19 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[20 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[21 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[22 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[23 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[24 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[25 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[26 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g7.offset((iz + 0 as ::core::ffi::c_int) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[17 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[14 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[25 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[24 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = -c[2 as ::core::ffi::c_int as usize]
                * s[4 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[6 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[3 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[16 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[13 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[26 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = -c[0 as ::core::ffi::c_int as usize]
                * s[17 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[18 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[24 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[2 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[1 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[0 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[6 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[10 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[16 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[20 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[14 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = -c[1 as ::core::ffi::c_int as usize]
                * s[18 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[4 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[1 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[12 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[3 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[0 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[13 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[10 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[23 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[20 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += c[1 as ::core::ffi::c_int as usize]
                    * s[17 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[14 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[25 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += c[1 as ::core::ffi::c_int as usize]
                    * s[24 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += -c[2 as ::core::ffi::c_int as usize]
                    * s[4 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += c[1 as ::core::ffi::c_int as usize]
                    * s[6 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[3 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[16 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[13 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[26 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += -c[0 as ::core::ffi::c_int as usize]
                    * s[17 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += c[2 as ::core::ffi::c_int as usize]
                    * s[18 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[24 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[2 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += c[2 as ::core::ffi::c_int as usize]
                    * s[1 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += c[2 as ::core::ffi::c_int as usize]
                    * s[0 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[6 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[10 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[16 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[20 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += c[0 as ::core::ffi::c_int as usize]
                    * s[14 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += -c[1 as ::core::ffi::c_int as usize]
                    * s[18 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += c[0 as ::core::ffi::c_int as usize]
                    * s[4 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[1 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[12 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += c[0 as ::core::ffi::c_int as usize]
                    * s[3 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[0 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[13 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[10 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[23 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[20 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spgsp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spgsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_spgsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    if !out.is_null()
        && *envs.shls.offset(0 as ::core::ffi::c_int as isize)
            == *envs.shls.offset(1 as ::core::ffi::c_int as isize)
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut nout: ::core::ffi::c_int = 0;
        let mut counts: [::core::ffi::c_int; 4] = [0; 4];
        counts[0 as ::core::ffi::c_int as usize] = envs.nfi
            * envs.x_ctr[0 as ::core::ffi::c_int as usize];
        counts[1 as ::core::ffi::c_int as usize] = envs.nfj
            * envs.x_ctr[1 as ::core::ffi::c_int as usize];
        counts[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        counts[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        if dims.is_null() {
            dims = &raw mut counts as *mut ::core::ffi::c_int;
        }
        nout = *dims.offset(0 as ::core::ffi::c_int as isize)
            * *dims.offset(1 as ::core::ffi::c_int as isize);
        i = 0 as ::core::ffi::c_int;
        while i < envs.ncomp_e1 * envs.ncomp_tensor {
            c2s_dset0(
                out.offset((nout * i) as isize),
                dims,
                &raw mut counts as *mut ::core::ffi::c_int,
            );
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spgsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_spgsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    if !out.is_null()
        && *envs.shls.offset(0 as ::core::ffi::c_int as isize)
            == *envs.shls.offset(1 as ::core::ffi::c_int as isize)
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut nout: ::core::ffi::c_int = 0;
        let mut counts: [::core::ffi::c_int; 4] = [0; 4];
        counts[0 as ::core::ffi::c_int as usize] = (envs.i_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * envs.x_ctr[0 as ::core::ffi::c_int as usize];
        counts[1 as ::core::ffi::c_int as usize] = (envs.j_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * envs.x_ctr[1 as ::core::ffi::c_int as usize];
        counts[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        counts[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        if dims.is_null() {
            dims = &raw mut counts as *mut ::core::ffi::c_int;
        }
        nout = *dims.offset(0 as ::core::ffi::c_int as isize)
            * *dims.offset(1 as ::core::ffi::c_int as isize);
        i = 0 as ::core::ffi::c_int;
        while i < envs.ncomp_e1 * envs.ncomp_tensor {
            c2s_dset0(
                out.offset((nout * i) as isize),
                dims,
                &raw mut counts as *mut ::core::ffi::c_int,
            );
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_spgsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_spgsp_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_spgsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_spgsp_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_spgnucsp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g4: *mut ::core::ffi::c_double = g3
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g5: *mut ::core::ffi::c_double = g4
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g6: *mut ::core::ffi::c_double = g5
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g7: *mut ::core::ffi::c_double = g6
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut rirj: [::core::ffi::c_double; 3] = [0.; 3];
    let mut c: [::core::ffi::c_double; 3] = [0.; 3];
    rirj[0 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(0 as ::core::ffi::c_int as isize)
        - *(*envs).rj.offset(0 as ::core::ffi::c_int as isize);
    rirj[1 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(1 as ::core::ffi::c_int as isize)
        - *(*envs).rj.offset(1 as ::core::ffi::c_int as isize);
    rirj[2 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(2 as ::core::ffi::c_int as isize)
        - *(*envs).rj.offset(2 as ::core::ffi::c_int as isize);
    c[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double * rirj[0 as ::core::ffi::c_int as usize];
    c[1 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double * rirj[1 as ::core::ffi::c_int as usize];
    c[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double * rirj[2 as ::core::ffi::c_int as usize];
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTx1i_2e(
        g2,
        g0,
        (*envs).ri,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTx1i_2e(
        g3,
        g1,
        (*envs).ri,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g4,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 27] = [0.; 27];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 27 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[9 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[10 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[11 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[12 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[13 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[14 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[15 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[16 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[17 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[18 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[19 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[20 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[21 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[22 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[23 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[24 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[25 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[26 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[17 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[14 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[25 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[24 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[21 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[8 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[7 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[4 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[15 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[6 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[3 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[16 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[13 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[26 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[11 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[17 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[19 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[18 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[24 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[2 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[1 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[7 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[9 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[0 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[6 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[10 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[16 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[20 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[14 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[11 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[22 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[19 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[21 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[18 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[5 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[4 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[1 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[12 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[3 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[0 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[13 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[10 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[23 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[20 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += c[1 as ::core::ffi::c_int as usize]
                    * s[17 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[14 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[25 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += c[1 as ::core::ffi::c_int as usize]
                    * s[24 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[21 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[8 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += c[1 as ::core::ffi::c_int as usize]
                    * s[7 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[4 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[15 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += c[1 as ::core::ffi::c_int as usize]
                    * s[6 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[3 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[16 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[13 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[26 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += c[2 as ::core::ffi::c_int as usize]
                    * s[11 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[17 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[19 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += c[2 as ::core::ffi::c_int as usize]
                    * s[18 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[24 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[2 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += c[2 as ::core::ffi::c_int as usize]
                    * s[1 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[7 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[9 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += c[2 as ::core::ffi::c_int as usize]
                    * s[0 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[6 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[10 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[16 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[20 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += c[0 as ::core::ffi::c_int as usize]
                    * s[14 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[11 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[22 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[19 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += c[0 as ::core::ffi::c_int as usize]
                    * s[21 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[18 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[5 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += c[0 as ::core::ffi::c_int as usize]
                    * s[4 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[1 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[12 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += c[0 as ::core::ffi::c_int as usize]
                    * s[3 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[0 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[13 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[10 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[23 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[20 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spgnucsp_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spgnucsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_spgnucsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    if !out.is_null()
        && *envs.shls.offset(0 as ::core::ffi::c_int as isize)
            == *envs.shls.offset(1 as ::core::ffi::c_int as isize)
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut nout: ::core::ffi::c_int = 0;
        let mut counts: [::core::ffi::c_int; 4] = [0; 4];
        counts[0 as ::core::ffi::c_int as usize] = envs.nfi
            * envs.x_ctr[0 as ::core::ffi::c_int as usize];
        counts[1 as ::core::ffi::c_int as usize] = envs.nfj
            * envs.x_ctr[1 as ::core::ffi::c_int as usize];
        counts[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        counts[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        if dims.is_null() {
            dims = &raw mut counts as *mut ::core::ffi::c_int;
        }
        nout = *dims.offset(0 as ::core::ffi::c_int as isize)
            * *dims.offset(1 as ::core::ffi::c_int as isize);
        i = 0 as ::core::ffi::c_int;
        while i < envs.ncomp_e1 * envs.ncomp_tensor {
            c2s_dset0(
                out.offset((nout * i) as isize),
                dims,
                &raw mut counts as *mut ::core::ffi::c_int,
            );
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        2 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spgnucsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_spgnucsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    if !out.is_null()
        && *envs.shls.offset(0 as ::core::ffi::c_int as isize)
            == *envs.shls.offset(1 as ::core::ffi::c_int as isize)
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut nout: ::core::ffi::c_int = 0;
        let mut counts: [::core::ffi::c_int; 4] = [0; 4];
        counts[0 as ::core::ffi::c_int as usize] = (envs.i_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * envs.x_ctr[0 as ::core::ffi::c_int as usize];
        counts[1 as ::core::ffi::c_int as usize] = (envs.j_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * envs.x_ctr[1 as ::core::ffi::c_int as usize];
        counts[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        counts[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        if dims.is_null() {
            dims = &raw mut counts as *mut ::core::ffi::c_int;
        }
        nout = *dims.offset(0 as ::core::ffi::c_int as isize)
            * *dims.offset(1 as ::core::ffi::c_int as isize);
        i = 0 as ::core::ffi::c_int;
        while i < envs.ncomp_e1 * envs.ncomp_tensor {
            c2s_dset0(
                out.offset((nout * i) as isize),
                dims,
                &raw mut counts as *mut ::core::ffi::c_int,
            );
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        2 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_spgnucsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_spgnucsp_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_spgnucsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_spgnucsp_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_spgsa01(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g4: *mut ::core::ffi::c_double = g3
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g5: *mut ::core::ffi::c_double = g4
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g6: *mut ::core::ffi::c_double = g5
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g7: *mut ::core::ffi::c_double = g6
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut rirj: [::core::ffi::c_double; 3] = [0.; 3];
    let mut c: [::core::ffi::c_double; 3] = [0.; 3];
    rirj[0 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(0 as ::core::ffi::c_int as isize)
        - *(*envs).rj.offset(0 as ::core::ffi::c_int as isize);
    rirj[1 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(1 as ::core::ffi::c_int as isize)
        - *(*envs).rj.offset(1 as ::core::ffi::c_int as isize);
    rirj[2 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(2 as ::core::ffi::c_int as isize)
        - *(*envs).rj.offset(2 as ::core::ffi::c_int as isize);
    c[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double * rirj[0 as ::core::ffi::c_int as usize];
    c[1 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double * rirj[1 as ::core::ffi::c_int as usize];
    c[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double * rirj[2 as ::core::ffi::c_int as usize];
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g2,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g1.offset(ix as isize) += *g2.offset(ix as isize);
        ix += 1;
    }
    CINTx1i_2e(
        g2,
        g0,
        (*envs).ri,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTx1i_2e(
        g3,
        g1,
        (*envs).ri,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g4,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 27] = [0.; 27];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 27 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[9 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[10 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[11 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[12 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[13 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[14 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[15 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[16 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[17 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[18 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[19 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[20 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[21 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[22 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[23 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[24 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[25 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[26 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = -c[1 as ::core::ffi::c_int as usize]
                * s[16 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[13 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[26 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[7 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[8 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[17 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[14 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[25 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[15 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = -c[1 as ::core::ffi::c_int as usize]
                * s[26 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[23 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[6 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[17 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[14 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = -c[1 as ::core::ffi::c_int as usize]
                * s[8 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[5 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[24 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[24 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[25 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = -c[1 as ::core::ffi::c_int as usize]
                * s[6 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[3 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[16 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[13 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = c[1 as ::core::ffi::c_int as usize]
                * s[7 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[4 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[15 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = -c[2 as ::core::ffi::c_int as usize]
                * s[10 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[16 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[20 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[1 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[2 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[11 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[17 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[19 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[9 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) = -c[2 as ::core::ffi::c_int as usize]
                * s[20 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[26 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[0 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[11 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) = -c[2 as ::core::ffi::c_int as usize]
                * s[2 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[8 as ::core::ffi::c_int as usize]
                + c[2 as ::core::ffi::c_int as usize]
                    * s[18 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[18 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[19 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) = -c[2 as ::core::ffi::c_int as usize]
                * s[0 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[6 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[10 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) = c[2 as ::core::ffi::c_int as usize]
                * s[1 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[7 as ::core::ffi::c_int as usize]
                - c[2 as ::core::ffi::c_int as usize]
                    * s[9 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) = -c[0 as ::core::ffi::c_int as usize]
                * s[13 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[10 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[23 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[4 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[5 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[14 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[11 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[22 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[19 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[12 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                ) = -c[0 as ::core::ffi::c_int as usize]
                * s[23 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[20 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[3 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[14 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                ) = -c[0 as ::core::ffi::c_int as usize]
                * s[5 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[2 as ::core::ffi::c_int as usize]
                + c[0 as ::core::ffi::c_int as usize]
                    * s[21 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[18 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[21 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[18 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[22 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[19 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                ) = -c[0 as ::core::ffi::c_int as usize]
                * s[3 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[0 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[13 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[10 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                ) = c[0 as ::core::ffi::c_int as usize]
                * s[4 as ::core::ffi::c_int as usize]
                - c[1 as ::core::ffi::c_int as usize]
                    * s[1 as ::core::ffi::c_int as usize]
                - c[0 as ::core::ffi::c_int as usize]
                    * s[12 as ::core::ffi::c_int as usize]
                + c[1 as ::core::ffi::c_int as usize]
                    * s[9 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += -c[1 as ::core::ffi::c_int as usize]
                    * s[16 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[13 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[26 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += c[1 as ::core::ffi::c_int as usize]
                    * s[7 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += c[1 as ::core::ffi::c_int as usize]
                    * s[8 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += c[1 as ::core::ffi::c_int as usize]
                    * s[17 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[14 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[25 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += c[1 as ::core::ffi::c_int as usize]
                    * s[15 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += -c[1 as ::core::ffi::c_int as usize]
                    * s[26 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[23 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[6 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += c[1 as ::core::ffi::c_int as usize]
                    * s[17 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[14 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += -c[1 as ::core::ffi::c_int as usize]
                    * s[8 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[5 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[24 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += c[1 as ::core::ffi::c_int as usize]
                    * s[24 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += c[1 as ::core::ffi::c_int as usize]
                    * s[25 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += -c[1 as ::core::ffi::c_int as usize]
                    * s[6 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[3 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[16 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[13 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += c[1 as ::core::ffi::c_int as usize]
                    * s[7 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[4 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[15 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                )
                += -c[2 as ::core::ffi::c_int as usize]
                    * s[10 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[16 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[20 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                )
                += c[2 as ::core::ffi::c_int as usize]
                    * s[1 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                )
                += c[2 as ::core::ffi::c_int as usize]
                    * s[2 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                )
                += c[2 as ::core::ffi::c_int as usize]
                    * s[11 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[17 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[19 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                )
                += c[2 as ::core::ffi::c_int as usize]
                    * s[9 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                )
                += -c[2 as ::core::ffi::c_int as usize]
                    * s[20 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[26 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[0 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                )
                += c[2 as ::core::ffi::c_int as usize]
                    * s[11 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                )
                += -c[2 as ::core::ffi::c_int as usize]
                    * s[2 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[8 as ::core::ffi::c_int as usize]
                    + c[2 as ::core::ffi::c_int as usize]
                        * s[18 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                )
                += c[2 as ::core::ffi::c_int as usize]
                    * s[18 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                )
                += c[2 as ::core::ffi::c_int as usize]
                    * s[19 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                )
                += -c[2 as ::core::ffi::c_int as usize]
                    * s[0 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[6 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[10 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                )
                += c[2 as ::core::ffi::c_int as usize]
                    * s[1 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[7 as ::core::ffi::c_int as usize]
                    - c[2 as ::core::ffi::c_int as usize]
                        * s[9 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                )
                += -c[0 as ::core::ffi::c_int as usize]
                    * s[13 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[10 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[23 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                )
                += c[0 as ::core::ffi::c_int as usize]
                    * s[4 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                )
                += c[0 as ::core::ffi::c_int as usize]
                    * s[5 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                )
                += c[0 as ::core::ffi::c_int as usize]
                    * s[14 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[11 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[22 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[19 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                )
                += c[0 as ::core::ffi::c_int as usize]
                    * s[12 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                )
                += -c[0 as ::core::ffi::c_int as usize]
                    * s[23 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[20 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[3 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                )
                += c[0 as ::core::ffi::c_int as usize]
                    * s[14 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                )
                += -c[0 as ::core::ffi::c_int as usize]
                    * s[5 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[2 as ::core::ffi::c_int as usize]
                    + c[0 as ::core::ffi::c_int as usize]
                        * s[21 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[18 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                )
                += c[0 as ::core::ffi::c_int as usize]
                    * s[21 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[18 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                )
                += c[0 as ::core::ffi::c_int as usize]
                    * s[22 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[19 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                )
                += -c[0 as ::core::ffi::c_int as usize]
                    * s[3 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[0 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[13 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[10 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                )
                += c[0 as ::core::ffi::c_int as usize]
                    * s[4 as ::core::ffi::c_int as usize]
                    - c[1 as ::core::ffi::c_int as usize]
                        * s[1 as ::core::ffi::c_int as usize]
                    - c[0 as ::core::ffi::c_int as usize]
                        * s[12 as ::core::ffi::c_int as usize]
                    + c[1 as ::core::ffi::c_int as usize]
                        * s[9 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spgsa01_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        3 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
    ];
    CINTall_1e_optimizer(
        opt,
        &raw mut ng as *mut ::core::ffi::c_int,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spgsa01_cart(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        3 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_spgsa01
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    if !out.is_null()
        && *envs.shls.offset(0 as ::core::ffi::c_int as isize)
            == *envs.shls.offset(1 as ::core::ffi::c_int as isize)
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut nout: ::core::ffi::c_int = 0;
        let mut counts: [::core::ffi::c_int; 4] = [0; 4];
        counts[0 as ::core::ffi::c_int as usize] = envs.nfi
            * envs.x_ctr[0 as ::core::ffi::c_int as usize];
        counts[1 as ::core::ffi::c_int as usize] = envs.nfj
            * envs.x_ctr[1 as ::core::ffi::c_int as usize];
        counts[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        counts[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        if dims.is_null() {
            dims = &raw mut counts as *mut ::core::ffi::c_int;
        }
        nout = *dims.offset(0 as ::core::ffi::c_int as isize)
            * *dims.offset(1 as ::core::ffi::c_int as isize);
        i = 0 as ::core::ffi::c_int;
        while i < envs.ncomp_e1 * envs.ncomp_tensor {
            c2s_dset0(
                out.offset((nout * i) as isize),
                dims,
                &raw mut counts as *mut ::core::ffi::c_int,
            );
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_cart_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_spgsa01_sph(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ng: [::core::ffi::c_int; 8] = [
        3 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
    ];
    let mut envs: CINTEnvVars = CINTEnvVars {
        atm: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        bas: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        env: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        shls: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        natm: 0,
        nbas: 0,
        i_l: 0,
        j_l: 0,
        k_l: 0,
        l_l: 0,
        nfi: 0,
        nfj: 0,
        c2rust_unnamed: C2RustUnnamed_1 { nfk: 0 },
        c2rust_unnamed_0: C2RustUnnamed_0 { nfl: 0 },
        nf: 0,
        rys_order: 0,
        x_ctr: [0; 4],
        gbits: 0,
        ncomp_e1: 0,
        ncomp_e2: 0,
        ncomp_tensor: 0,
        li_ceil: 0,
        lj_ceil: 0,
        lk_ceil: 0,
        ll_ceil: 0,
        g_stride_i: 0,
        g_stride_k: 0,
        g_stride_l: 0,
        g_stride_j: 0,
        nrys_roots: 0,
        g_size: 0,
        g2d_ijmax: 0,
        g2d_klmax: 0,
        common_factor: 0.,
        expcutoff: 0.,
        rirj: [0.; 3],
        rkrl: [0.; 3],
        rx_in_rijrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rx_in_rklrx: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        ri: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rj: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        rk: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        c2rust_unnamed_1: C2RustUnnamed {
            rl: ::core::ptr::null_mut::<::core::ffi::c_double>(),
        },
        f_g0_2e: None,
        f_g0_2d4d: None,
        f_gout: None,
        opt: ::core::ptr::null_mut::<CINTOpt>(),
        idx: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        ai: [0.; 1],
        aj: [0.; 1],
        ak: [0.; 1],
        al: [0.; 1],
        fac: [0.; 1],
        rij: [0.; 3],
        rkl: [0.; 3],
    };
    CINTinit_int1e_EnvVars(
        &raw mut envs,
        &raw mut ng as *mut ::core::ffi::c_int,
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
    envs.f_gout = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_int,
                *mut CINTEnvVars,
                ::core::ffi::c_int,
            ) -> (),
        >,
        Option<unsafe extern "C" fn() -> ()>,
    >(
        Some(
            CINTgout1e_int1e_spgsa01
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= 0.5f64;
    if !out.is_null()
        && *envs.shls.offset(0 as ::core::ffi::c_int as isize)
            == *envs.shls.offset(1 as ::core::ffi::c_int as isize)
    {
        let mut i: ::core::ffi::c_int = 0;
        let mut nout: ::core::ffi::c_int = 0;
        let mut counts: [::core::ffi::c_int; 4] = [0; 4];
        counts[0 as ::core::ffi::c_int as usize] = (envs.i_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * envs.x_ctr[0 as ::core::ffi::c_int as usize];
        counts[1 as ::core::ffi::c_int as usize] = (envs.j_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * envs.x_ctr[1 as ::core::ffi::c_int as usize];
        counts[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        counts[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        if dims.is_null() {
            dims = &raw mut counts as *mut ::core::ffi::c_int;
        }
        nout = *dims.offset(0 as ::core::ffi::c_int as isize)
            * *dims.offset(1 as ::core::ffi::c_int as isize);
        i = 0 as ::core::ffi::c_int;
        while i < envs.ncomp_e1 * envs.ncomp_tensor {
            c2s_dset0(
                out.offset((nout * i) as isize),
                dims,
                &raw mut counts as *mut ::core::ffi::c_int,
            );
            i += 1;
        }
        return 0 as ::core::ffi::c_int;
    }
    return CINT1e_drv(
        out,
        dims,
        &raw mut envs,
        cache,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                c2s_sph_1e
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_spgsa01_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_spgsa01_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_spgsa01_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_spgsa01_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        ::core::ptr::null_mut::<CINTOpt>(),
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
