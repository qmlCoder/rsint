extern "C" {
    fn c2s_sph_1e_grids(
        out: *mut ::core::ffi::c_double,
        gctr: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut ::core::ffi::c_double,
    );
    fn c2s_cart_1e_grids(
        out: *mut ::core::ffi::c_double,
        gctr: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut ::core::ffi::c_double,
    );
    fn CINTinit_int1e_grids_EnvVars(
        envs: *mut CINTEnvVars,
        ng: *mut ::core::ffi::c_int,
        shls: *mut ::core::ffi::c_int,
        atm: *mut ::core::ffi::c_int,
        natm: ::core::ffi::c_int,
        bas: *mut ::core::ffi::c_int,
        nbas: ::core::ffi::c_int,
        env: *mut ::core::ffi::c_double,
    );
    fn CINTnabla1i_grids(
        f: *mut ::core::ffi::c_double,
        g: *mut ::core::ffi::c_double,
        li: ::core::ffi::c_int,
        lj: ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
    );
    fn CINTnabla1j_grids(
        f: *mut ::core::ffi::c_double,
        g: *mut ::core::ffi::c_double,
        li: ::core::ffi::c_int,
        lj: ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
    );
    fn CINTall_1e_grids_optimizer(
        opt: *mut *mut CINTOpt,
        ng: *mut ::core::ffi::c_int,
        atm: *mut ::core::ffi::c_int,
        natm: ::core::ffi::c_int,
        bas: *mut ::core::ffi::c_int,
        nbas: ::core::ffi::c_int,
        env: *mut ::core::ffi::c_double,
    );
    fn CINT1e_grids_drv(
        out: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut ::core::ffi::c_double,
        f_c2s: Option<unsafe extern "C" fn() -> ()>,
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
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_grids_ip(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut ngrids: ::core::ffi::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: ::core::ffi::c_int = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as ::core::ffi::c_int
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as ::core::ffi::c_int
    };
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ig: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut s: [::core::ffi::c_double; 312] = [0.; 312];
    CINTnabla1i_grids(g1, g0, (*envs).i_l + 0 as ::core::ffi::c_int, (*envs).j_l, envs);
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
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                s[(ig + i * GRID_BLKSIZE) as usize] = 0 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                ig += 1;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                s[(ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as usize]
                    += *g1.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as usize]
                    += *g0.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g1.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as usize]
                    += *g0.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g1.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                ig += 1;
            }
            i += 1;
        }
        if gout_empty != 0 {
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as usize];
                ig += 1;
            }
        } else {
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as usize];
                ig += 1;
            }
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_grids_ip_optimizer(
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
    CINTall_1e_grids_optimizer(
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
pub unsafe extern "C" fn int1e_grids_ip_cart(
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
    CINTinit_int1e_grids_EnvVars(
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
            CINTgout1e_int1e_grids_ip
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_grids_drv(
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
                c2s_cart_1e_grids
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_grids_ip_sph(
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
    CINTinit_int1e_grids_EnvVars(
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
            CINTgout1e_int1e_grids_ip
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_grids_drv(
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
                c2s_sph_1e_grids
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_grids_ip_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_grids_ip_cart(
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
pub unsafe extern "C" fn cint1e_grids_ip_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_grids_ip_sph(
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
pub unsafe extern "C" fn CINTgout1e_int1e_grids_ipvip(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut ngrids: ::core::ffi::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: ::core::ffi::c_int = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as ::core::ffi::c_int
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as ::core::ffi::c_int
    };
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ig: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut s: [::core::ffi::c_double; 936] = [0.; 936];
    CINTnabla1j_grids(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_grids(g2, g0, (*envs).i_l + 0 as ::core::ffi::c_int, (*envs).j_l, envs);
    CINTnabla1i_grids(g3, g1, (*envs).i_l + 0 as ::core::ffi::c_int, (*envs).j_l, envs);
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
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                s[(ig + i * GRID_BLKSIZE) as usize] = 0 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                ig += 1;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                s[(ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as usize]
                    += *g3.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as usize]
                    += *g2.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g1.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as usize]
                    += *g2.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g1.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 3 as ::core::ffi::c_int) as usize]
                    += *g1.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g2.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 4 as ::core::ffi::c_int) as usize]
                    += *g0.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g3.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 5 as ::core::ffi::c_int) as usize]
                    += *g0.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g2.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g1.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 6 as ::core::ffi::c_int) as usize]
                    += *g1.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g2.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 7 as ::core::ffi::c_int) as usize]
                    += *g0.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g1.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g2.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 8 as ::core::ffi::c_int) as usize]
                    += *g0.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g3.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                ig += 1;
            }
            i += 1;
        }
        if gout_empty != 0 {
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 3 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 4 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 5 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 6 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 7 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 8 as ::core::ffi::c_int) as usize];
                ig += 1;
            }
        } else {
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 3 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 4 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 5 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 6 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 7 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 8 as ::core::ffi::c_int) as usize];
                ig += 1;
            }
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_grids_ipvip_optimizer(
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
        0 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
    ];
    CINTall_1e_grids_optimizer(
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
pub unsafe extern "C" fn int1e_grids_ipvip_cart(
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
    CINTinit_int1e_grids_EnvVars(
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
            CINTgout1e_int1e_grids_ipvip
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_grids_drv(
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
                c2s_cart_1e_grids
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_grids_ipvip_sph(
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
    CINTinit_int1e_grids_EnvVars(
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
            CINTgout1e_int1e_grids_ipvip
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_grids_drv(
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
                c2s_sph_1e_grids
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_grids_ipvip_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_grids_ipvip_sph(
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
pub unsafe extern "C" fn cint1e_grids_ipvip_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_grids_ipvip_cart(
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
pub unsafe extern "C" fn CINTgout1e_int1e_grids_spvsp(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut ngrids: ::core::ffi::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: ::core::ffi::c_int = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as ::core::ffi::c_int
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as ::core::ffi::c_int
    };
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ig: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut s: [::core::ffi::c_double; 936] = [0.; 936];
    CINTnabla1j_grids(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_grids(g2, g0, (*envs).i_l + 0 as ::core::ffi::c_int, (*envs).j_l, envs);
    CINTnabla1i_grids(g3, g1, (*envs).i_l + 0 as ::core::ffi::c_int, (*envs).j_l, envs);
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
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                s[(ig + i * GRID_BLKSIZE) as usize] = 0 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                ig += 1;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                s[(ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as usize]
                    += *g3.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as usize]
                    += *g2.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g1.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as usize]
                    += *g2.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g1.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 3 as ::core::ffi::c_int) as usize]
                    += *g1.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g2.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 4 as ::core::ffi::c_int) as usize]
                    += *g0.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g3.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 5 as ::core::ffi::c_int) as usize]
                    += *g0.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g2.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g1.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 6 as ::core::ffi::c_int) as usize]
                    += *g1.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g2.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 7 as ::core::ffi::c_int) as usize]
                    += *g0.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g1.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g2.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 8 as ::core::ffi::c_int) as usize]
                    += *g0.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g3.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                ig += 1;
            }
            i += 1;
        }
        if gout_empty != 0 {
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 5 as ::core::ffi::c_int) as usize]
                    - s[(ig + GRID_BLKSIZE * 7 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 6 as ::core::ffi::c_int) as usize]
                    - s[(ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as usize]
                    - s[(ig + GRID_BLKSIZE * 3 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as usize]
                    + s[(ig + GRID_BLKSIZE * 4 as ::core::ffi::c_int) as usize]
                    + s[(ig + GRID_BLKSIZE * 8 as ::core::ffi::c_int) as usize];
                ig += 1;
            }
        } else {
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int))
                            as isize,
                    )
                    += s[(ig + GRID_BLKSIZE * 5 as ::core::ffi::c_int) as usize]
                        - s[(ig + GRID_BLKSIZE * 7 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                            as isize,
                    )
                    += s[(ig + GRID_BLKSIZE * 6 as ::core::ffi::c_int) as usize]
                        - s[(ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int))
                            as isize,
                    )
                    += s[(ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as usize]
                        - s[(ig + GRID_BLKSIZE * 3 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int))
                            as isize,
                    )
                    += s[(ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as usize]
                        + s[(ig + GRID_BLKSIZE * 4 as ::core::ffi::c_int) as usize]
                        + s[(ig + GRID_BLKSIZE * 8 as ::core::ffi::c_int) as usize];
                ig += 1;
            }
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_grids_spvsp_optimizer(
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
    CINTall_1e_grids_optimizer(
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
pub unsafe extern "C" fn int1e_grids_spvsp_cart(
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
    CINTinit_int1e_grids_EnvVars(
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
            CINTgout1e_int1e_grids_spvsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_grids_drv(
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
                c2s_cart_1e_grids
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_grids_spvsp_sph(
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
    CINTinit_int1e_grids_EnvVars(
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
            CINTgout1e_int1e_grids_spvsp
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_grids_drv(
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
                c2s_sph_1e_grids
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_grids_spvsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_grids_spvsp_cart(
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
pub unsafe extern "C" fn cint1e_grids_spvsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_grids_spvsp_sph(
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
pub unsafe extern "C" fn CINTgout1e_int1e_grids_ipip(
    mut gout: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut idx: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut gout_empty: ::core::ffi::c_int,
) {
    let mut ngrids: ::core::ffi::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: ::core::ffi::c_int = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as ::core::ffi::c_int
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as ::core::ffi::c_int
    };
    let mut nrys_roots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut ix: ::core::ffi::c_int = 0;
    let mut iy: ::core::ffi::c_int = 0;
    let mut iz: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut ig: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut s: [::core::ffi::c_double; 936] = [0.; 936];
    CINTnabla1i_grids(g1, g0, (*envs).i_l + 1 as ::core::ffi::c_int, (*envs).j_l, envs);
    CINTnabla1i_grids(g2, g0, (*envs).i_l + 0 as ::core::ffi::c_int, (*envs).j_l, envs);
    CINTnabla1i_grids(g3, g1, (*envs).i_l + 0 as ::core::ffi::c_int, (*envs).j_l, envs);
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
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                s[(ig + i * GRID_BLKSIZE) as usize] = 0 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                ig += 1;
            }
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                s[(ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as usize]
                    += *g3.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as usize]
                    += *g2.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g1.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as usize]
                    += *g2.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g1.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 3 as ::core::ffi::c_int) as usize]
                    += *g1.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g2.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 4 as ::core::ffi::c_int) as usize]
                    += *g0.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g3.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 5 as ::core::ffi::c_int) as usize]
                    += *g0.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g2.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g1.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 6 as ::core::ffi::c_int) as usize]
                    += *g1.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g2.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 7 as ::core::ffi::c_int) as usize]
                    += *g0.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g1.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g2.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                s[(ig + GRID_BLKSIZE * 8 as ::core::ffi::c_int) as usize]
                    += *g0.offset((ix + ig + i * GRID_BLKSIZE) as isize)
                        * *g0.offset((iy + ig + i * GRID_BLKSIZE) as isize)
                        * *g3.offset((iz + ig + i * GRID_BLKSIZE) as isize);
                ig += 1;
            }
            i += 1;
        }
        if gout_empty != 0 {
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 3 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 6 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 4 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 7 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 5 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int))
                            as isize,
                    ) = s[(ig + GRID_BLKSIZE * 8 as ::core::ffi::c_int) as usize];
                ig += 1;
            }
        } else {
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 3 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 6 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 4 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 7 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 5 as ::core::ffi::c_int) as usize];
                *gout
                    .offset(
                        (ig
                            + bgrids
                                * (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int))
                            as isize,
                    ) += s[(ig + GRID_BLKSIZE * 8 as ::core::ffi::c_int) as usize];
                ig += 1;
            }
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_grids_ipip_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
    ];
    CINTall_1e_grids_optimizer(
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
pub unsafe extern "C" fn int1e_grids_ipip_cart(
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
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
    CINTinit_int1e_grids_EnvVars(
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
            CINTgout1e_int1e_grids_ipip
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_grids_drv(
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
                c2s_cart_1e_grids
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn int1e_grids_ipip_sph(
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
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
    CINTinit_int1e_grids_EnvVars(
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
            CINTgout1e_int1e_grids_ipip
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT1e_grids_drv(
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
                c2s_sph_1e_grids
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_grids_ipip_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_grids_ipip_cart(
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
pub unsafe extern "C" fn cint1e_grids_ipip_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_grids_ipip_sph(
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
pub const GRID_BLKSIZE: ::core::ffi::c_int = 104 as ::core::ffi::c_int;
