extern "C" {
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
    fn c2s_sph_2e1(
        fijkl: *mut ::core::ffi::c_double,
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
    fn c2s_cart_2e1(
        fijkl: *mut ::core::ffi::c_double,
        gctr: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut ::core::ffi::c_double,
    );
    fn c2s_sf_2e1(
        opij: *mut ::core::ffi::c_double,
        gctr: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        cache: *mut ::core::ffi::c_double,
    );
    fn CINTinit_int2e_EnvVars(
        envs: *mut CINTEnvVars,
        ng: *mut ::core::ffi::c_int,
        shls: *mut ::core::ffi::c_int,
        atm: *mut ::core::ffi::c_int,
        natm: ::core::ffi::c_int,
        bas: *mut ::core::ffi::c_int,
        nbas: ::core::ffi::c_int,
        env: *mut ::core::ffi::c_double,
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
    fn CINTnabla1k_2e(
        f: *mut ::core::ffi::c_double,
        g: *const ::core::ffi::c_double,
        li: ::core::ffi::c_int,
        lj: ::core::ffi::c_int,
        lk: ::core::ffi::c_int,
        ll: ::core::ffi::c_int,
        envs: *const CINTEnvVars,
    );
    fn CINTnabla1l_2e(
        f: *mut ::core::ffi::c_double,
        g: *const ::core::ffi::c_double,
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
    fn CINTall_2e_optimizer(
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
    fn CINT2e_drv(
        out: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        opt: *mut CINTOpt,
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
pub const PTR_COMMON_ORIG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_ipipovlp(
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
    CINTnabla1i_1e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
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
                    (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                += s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                += s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                += s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                += s[8 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ipipovlp_optimizer(
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
        1 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int1e_ipipovlp_cart(
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
        1 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipipovlp
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
pub unsafe extern "C" fn int1e_ipipovlp_sph(
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
        1 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipipovlp
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
pub unsafe extern "C" fn cint1e_ipipovlp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipipovlp_cart(
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
pub unsafe extern "C" fn cint1e_ipipovlp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipipovlp_sph(
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
pub unsafe extern "C" fn CINTgout1e_int1e_ipovlpip(
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
                    (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                += s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                += s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                += s[8 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ipovlpip_optimizer(
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
pub unsafe extern "C" fn int1e_ipovlpip_cart(
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
            CINTgout1e_int1e_ipovlpip
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
pub unsafe extern "C" fn int1e_ipovlpip_sph(
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
            CINTgout1e_int1e_ipovlpip
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
pub unsafe extern "C" fn cint1e_ipovlpip_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipovlpip_cart(
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
pub unsafe extern "C" fn cint1e_ipovlpip_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipovlpip_sph(
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
pub unsafe extern "C" fn CINTgout1e_int1e_ipipkin(
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
    let mut g8: *mut ::core::ffi::c_double = g7
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g9: *mut ::core::ffi::c_double = g8
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g10: *mut ::core::ffi::c_double = g9
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g11: *mut ::core::ffi::c_double = g10
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g12: *mut ::core::ffi::c_double = g11
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g13: *mut ::core::ffi::c_double = g12
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g14: *mut ::core::ffi::c_double = g13
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g15: *mut ::core::ffi::c_double = g14
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut s: [::core::ffi::c_double; 81] = [0.; 81];
    CINTnabla1j_1e(
        g1,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_1e(
        g2,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_1e(
        g3,
        g2,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g4,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g5,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g6,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g7,
        g3,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g10,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g11,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g12,
        g4,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g13,
        g5,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g14,
        g6,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g15,
        g7,
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
        s[0 as ::core::ffi::c_int as usize] = *g15
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[1 as ::core::ffi::c_int as usize] = *g14
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[2 as ::core::ffi::c_int as usize] = *g14
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[3 as ::core::ffi::c_int as usize] = *g13
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[4 as ::core::ffi::c_int as usize] = *g12
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[5 as ::core::ffi::c_int as usize] = *g12
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[6 as ::core::ffi::c_int as usize] = *g13
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[7 as ::core::ffi::c_int as usize] = *g12
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[8 as ::core::ffi::c_int as usize] = *g12
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[9 as ::core::ffi::c_int as usize] = *g11
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[10 as ::core::ffi::c_int as usize] = *g10
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[11 as ::core::ffi::c_int as usize] = *g10
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[12 as ::core::ffi::c_int as usize] = *g9
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[13 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g7.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[14 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[15 as ::core::ffi::c_int as usize] = *g9
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[16 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[17 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[18 as ::core::ffi::c_int as usize] = *g11
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[19 as ::core::ffi::c_int as usize] = *g10
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[20 as ::core::ffi::c_int as usize] = *g10
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[21 as ::core::ffi::c_int as usize] = *g9
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[22 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[23 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[24 as ::core::ffi::c_int as usize] = *g9
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[25 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[26 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g7.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[27 as ::core::ffi::c_int as usize] = *g7
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[28 as ::core::ffi::c_int as usize] = *g6
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[29 as ::core::ffi::c_int as usize] = *g6
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[30 as ::core::ffi::c_int as usize] = *g5
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[31 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g11.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[32 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[33 as ::core::ffi::c_int as usize] = *g5
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[34 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[35 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[36 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[37 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g13.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[38 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[39 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g14.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[40 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g15.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[41 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g14.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[42 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[43 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g13.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[44 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[45 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[46 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[47 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[48 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[49 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g11.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[50 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[51 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[52 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[53 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g7.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[54 as ::core::ffi::c_int as usize] = *g7
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[55 as ::core::ffi::c_int as usize] = *g6
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[56 as ::core::ffi::c_int as usize] = *g6
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[57 as ::core::ffi::c_int as usize] = *g5
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[58 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[59 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[60 as ::core::ffi::c_int as usize] = *g5
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[61 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[62 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g11.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[63 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[64 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[65 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[66 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[67 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g7.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[68 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[69 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[70 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[71 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g11.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[72 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[73 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[74 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g13.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[75 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[76 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[77 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g13.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[78 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g14.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[79 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g14.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[80 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g15.offset((iz + 0 as ::core::ffi::c_int) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = -s[0 as ::core::ffi::c_int as usize]
                - s[4 as ::core::ffi::c_int as usize]
                - s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = -s[27 as ::core::ffi::c_int as usize]
                - s[31 as ::core::ffi::c_int as usize]
                - s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = -s[54 as ::core::ffi::c_int as usize]
                - s[58 as ::core::ffi::c_int as usize]
                - s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = -s[9 as ::core::ffi::c_int as usize]
                - s[13 as ::core::ffi::c_int as usize]
                - s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = -s[36 as ::core::ffi::c_int as usize]
                - s[40 as ::core::ffi::c_int as usize]
                - s[44 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = -s[63 as ::core::ffi::c_int as usize]
                - s[67 as ::core::ffi::c_int as usize]
                - s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = -s[18 as ::core::ffi::c_int as usize]
                - s[22 as ::core::ffi::c_int as usize]
                - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = -s[45 as ::core::ffi::c_int as usize]
                - s[49 as ::core::ffi::c_int as usize]
                - s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = -s[72 as ::core::ffi::c_int as usize]
                - s[76 as ::core::ffi::c_int as usize]
                - s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += -s[0 as ::core::ffi::c_int as usize]
                    - s[4 as ::core::ffi::c_int as usize]
                    - s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += -s[27 as ::core::ffi::c_int as usize]
                    - s[31 as ::core::ffi::c_int as usize]
                    - s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += -s[54 as ::core::ffi::c_int as usize]
                    - s[58 as ::core::ffi::c_int as usize]
                    - s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += -s[9 as ::core::ffi::c_int as usize]
                    - s[13 as ::core::ffi::c_int as usize]
                    - s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                += -s[36 as ::core::ffi::c_int as usize]
                    - s[40 as ::core::ffi::c_int as usize]
                    - s[44 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                += -s[63 as ::core::ffi::c_int as usize]
                    - s[67 as ::core::ffi::c_int as usize]
                    - s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                += -s[18 as ::core::ffi::c_int as usize]
                    - s[22 as ::core::ffi::c_int as usize]
                    - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                += -s[45 as ::core::ffi::c_int as usize]
                    - s[49 as ::core::ffi::c_int as usize]
                    - s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                += -s[72 as ::core::ffi::c_int as usize]
                    - s[76 as ::core::ffi::c_int as usize]
                    - s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ipipkin_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int1e_ipipkin_cart(
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
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipipkin
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
pub unsafe extern "C" fn int1e_ipipkin_sph(
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
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipipkin
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
pub unsafe extern "C" fn cint1e_ipipkin_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipipkin_cart(
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
pub unsafe extern "C" fn cint1e_ipipkin_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipipkin_sph(
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
pub unsafe extern "C" fn CINTgout1e_int1e_ipkinip(
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
    let mut g8: *mut ::core::ffi::c_double = g7
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g9: *mut ::core::ffi::c_double = g8
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g10: *mut ::core::ffi::c_double = g9
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g11: *mut ::core::ffi::c_double = g10
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g12: *mut ::core::ffi::c_double = g11
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g13: *mut ::core::ffi::c_double = g12
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g14: *mut ::core::ffi::c_double = g13
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g15: *mut ::core::ffi::c_double = g14
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut s: [::core::ffi::c_double; 81] = [0.; 81];
    CINTnabla1j_1e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_1e(
        g2,
        g0,
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
    CINTnabla1j_1e(
        g4,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_1e(
        g5,
        g4,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_1e(
        g6,
        g4,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_1e(
        g7,
        g6,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g10,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g11,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g12,
        g4,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g13,
        g5,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g14,
        g6,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g15,
        g7,
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
        s[0 as ::core::ffi::c_int as usize] = *g15
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[1 as ::core::ffi::c_int as usize] = *g14
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[2 as ::core::ffi::c_int as usize] = *g14
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[3 as ::core::ffi::c_int as usize] = *g13
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[4 as ::core::ffi::c_int as usize] = *g12
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[5 as ::core::ffi::c_int as usize] = *g12
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[6 as ::core::ffi::c_int as usize] = *g13
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[7 as ::core::ffi::c_int as usize] = *g12
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[8 as ::core::ffi::c_int as usize] = *g12
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[9 as ::core::ffi::c_int as usize] = *g11
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[10 as ::core::ffi::c_int as usize] = *g10
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[11 as ::core::ffi::c_int as usize] = *g10
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[12 as ::core::ffi::c_int as usize] = *g9
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[13 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g7.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[14 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[15 as ::core::ffi::c_int as usize] = *g9
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[16 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[17 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[18 as ::core::ffi::c_int as usize] = *g11
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[19 as ::core::ffi::c_int as usize] = *g10
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[20 as ::core::ffi::c_int as usize] = *g10
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[21 as ::core::ffi::c_int as usize] = *g9
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[22 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[23 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[24 as ::core::ffi::c_int as usize] = *g9
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[25 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[26 as ::core::ffi::c_int as usize] = *g8
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g7.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[27 as ::core::ffi::c_int as usize] = *g7
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[28 as ::core::ffi::c_int as usize] = *g6
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[29 as ::core::ffi::c_int as usize] = *g6
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[30 as ::core::ffi::c_int as usize] = *g5
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[31 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g11.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[32 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[33 as ::core::ffi::c_int as usize] = *g5
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[34 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[35 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[36 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[37 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g13.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[38 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[39 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g14.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[40 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g15.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[41 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g14.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[42 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[43 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g13.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[44 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[45 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[46 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[47 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[48 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[49 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g11.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[50 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[51 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[52 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[53 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g7.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[54 as ::core::ffi::c_int as usize] = *g7
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[55 as ::core::ffi::c_int as usize] = *g6
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[56 as ::core::ffi::c_int as usize] = *g6
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[57 as ::core::ffi::c_int as usize] = *g5
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[58 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[59 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[60 as ::core::ffi::c_int as usize] = *g5
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[61 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[62 as ::core::ffi::c_int as usize] = *g4
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g11.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[63 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[64 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[65 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[66 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[67 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g7.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g8.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[68 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g6.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g9.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[69 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[70 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g5.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g10.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[71 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g4.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g11.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[72 as ::core::ffi::c_int as usize] = *g3
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[73 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[74 as ::core::ffi::c_int as usize] = *g2
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g13.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[75 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[76 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g3.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g12.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[77 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g2.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g13.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[78 as ::core::ffi::c_int as usize] = *g1
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g14.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[79 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g14.offset((iz + 0 as ::core::ffi::c_int) as isize);
        s[80 as ::core::ffi::c_int as usize] = *g0
            .offset((ix + 0 as ::core::ffi::c_int) as isize)
            * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
            * *g15.offset((iz + 0 as ::core::ffi::c_int) as isize);
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = -s[0 as ::core::ffi::c_int as usize]
                - s[12 as ::core::ffi::c_int as usize]
                - s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = -s[1 as ::core::ffi::c_int as usize]
                - s[13 as ::core::ffi::c_int as usize]
                - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize]
                - s[14 as ::core::ffi::c_int as usize]
                - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = -s[27 as ::core::ffi::c_int as usize]
                - s[39 as ::core::ffi::c_int as usize]
                - s[51 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = -s[28 as ::core::ffi::c_int as usize]
                - s[40 as ::core::ffi::c_int as usize]
                - s[52 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = -s[29 as ::core::ffi::c_int as usize]
                - s[41 as ::core::ffi::c_int as usize]
                - s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = -s[54 as ::core::ffi::c_int as usize]
                - s[66 as ::core::ffi::c_int as usize]
                - s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = -s[55 as ::core::ffi::c_int as usize]
                - s[67 as ::core::ffi::c_int as usize]
                - s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = -s[56 as ::core::ffi::c_int as usize]
                - s[68 as ::core::ffi::c_int as usize]
                - s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += -s[0 as ::core::ffi::c_int as usize]
                    - s[12 as ::core::ffi::c_int as usize]
                    - s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += -s[1 as ::core::ffi::c_int as usize]
                    - s[13 as ::core::ffi::c_int as usize]
                    - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += -s[2 as ::core::ffi::c_int as usize]
                    - s[14 as ::core::ffi::c_int as usize]
                    - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += -s[27 as ::core::ffi::c_int as usize]
                    - s[39 as ::core::ffi::c_int as usize]
                    - s[51 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                += -s[28 as ::core::ffi::c_int as usize]
                    - s[40 as ::core::ffi::c_int as usize]
                    - s[52 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                += -s[29 as ::core::ffi::c_int as usize]
                    - s[41 as ::core::ffi::c_int as usize]
                    - s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                += -s[54 as ::core::ffi::c_int as usize]
                    - s[66 as ::core::ffi::c_int as usize]
                    - s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                += -s[55 as ::core::ffi::c_int as usize]
                    - s[67 as ::core::ffi::c_int as usize]
                    - s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                += -s[56 as ::core::ffi::c_int as usize]
                    - s[68 as ::core::ffi::c_int as usize]
                    - s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ipkinip_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int1e_ipkinip_cart(
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
        3 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipkinip
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
pub unsafe extern "C" fn int1e_ipkinip_sph(
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
        3 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipkinip
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
pub unsafe extern "C" fn cint1e_ipkinip_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipkinip_cart(
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
pub unsafe extern "C" fn cint1e_ipkinip_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipkinip_sph(
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
pub unsafe extern "C" fn CINTgout1e_int1e_ipipnuc(
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
    CINTnabla1i_2e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
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
                    (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                += s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                += s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                += s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                += s[8 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ipipnuc_optimizer(
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
pub unsafe extern "C" fn int1e_ipipnuc_cart(
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
            CINTgout1e_int1e_ipipnuc
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
pub unsafe extern "C" fn int1e_ipipnuc_sph(
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
            CINTgout1e_int1e_ipipnuc
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
pub unsafe extern "C" fn cint1e_ipipnuc_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipipnuc_sph(
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
pub unsafe extern "C" fn cint1e_ipipnuc_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipipnuc_cart(
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
pub unsafe extern "C" fn CINTgout1e_int1e_ipnucip(
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
                    (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                += s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                += s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                += s[8 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ipnucip_optimizer(
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
pub unsafe extern "C" fn int1e_ipnucip_cart(
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
            CINTgout1e_int1e_ipnucip
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
pub unsafe extern "C" fn int1e_ipnucip_sph(
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
            CINTgout1e_int1e_ipnucip
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
pub unsafe extern "C" fn cint1e_ipnucip_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipnucip_cart(
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
pub unsafe extern "C" fn cint1e_ipnucip_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipnucip_sph(
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
pub unsafe extern "C" fn CINTgout1e_int1e_ipiprinv(
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
    CINTnabla1i_2e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
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
                    (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                += s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                += s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                += s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                += s[8 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ipiprinv_optimizer(
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
pub unsafe extern "C" fn int1e_ipiprinv_cart(
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
            CINTgout1e_int1e_ipiprinv
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
pub unsafe extern "C" fn int1e_ipiprinv_sph(
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
            CINTgout1e_int1e_ipiprinv
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
pub unsafe extern "C" fn cint1e_ipiprinv_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipiprinv_sph(
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
pub unsafe extern "C" fn cint1e_ipiprinv_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipiprinv_cart(
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
pub unsafe extern "C" fn CINTgout1e_int1e_iprinvip(
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
                    (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                += s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                += s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                += s[8 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_iprinvip_optimizer(
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
pub unsafe extern "C" fn int1e_iprinvip_cart(
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
            CINTgout1e_int1e_iprinvip
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
pub unsafe extern "C" fn int1e_iprinvip_sph(
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
            CINTgout1e_int1e_iprinvip
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
pub unsafe extern "C" fn cint1e_iprinvip_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_iprinvip_sph(
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
pub unsafe extern "C" fn cint1e_iprinvip_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_iprinvip_cart(
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
pub unsafe extern "C" fn CINTgout1e_int1e_ipipr(
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
    CINTx1j_1e(
        g1,
        g0,
        &raw mut drj as *mut ::core::ffi::c_double,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g2,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_1e(
        g3,
        g1,
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
                    (n * 27 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[10 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[18 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[19 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = s[13 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = s[14 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) = s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) = s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) = s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) = s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) = s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) = s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) = s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) = s[26 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) += s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) += s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) += s[10 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) += s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) += s[18 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) += s[19 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) += s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) += s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) += s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) += s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) += s[13 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) += s[14 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) += s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) += s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) += s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) += s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) += s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) += s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) += s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) += s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) += s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) += s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) += s[26 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ipipr_optimizer(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int1e_ipipr_cart(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipipr
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
pub unsafe extern "C" fn int1e_ipipr_sph(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipipr
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
pub unsafe extern "C" fn cint1e_ipipr_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipipr_cart(
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
pub unsafe extern "C" fn cint1e_ipipr_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipipr_sph(
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
pub unsafe extern "C" fn CINTgout1e_int1e_iprip(
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
                    (n * 27 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[10 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = s[13 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = s[14 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) = s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) = s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) = s[18 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) = s[19 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) = s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) = s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) = s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) = s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) = s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) = s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) = s[26 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) += s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) += s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) += s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) += s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) += s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) += s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) += s[10 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) += s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) += s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) += s[13 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) += s[14 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) += s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) += s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) += s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) += s[18 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) += s[19 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) += s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) += s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) += s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) += s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) += s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) += s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 27 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) += s[26 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_iprip_optimizer(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int1e_iprip_cart(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_iprip
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
pub unsafe extern "C" fn int1e_iprip_sph(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        27 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_iprip
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
pub unsafe extern "C" fn cint1e_iprip_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_iprip_cart(
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
pub unsafe extern "C" fn cint1e_iprip_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_iprip_sph(
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
pub unsafe extern "C" fn CINTgout2e_int2e_ipip1(
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
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1i_2e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g2,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g3,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
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
                    (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                += s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                += s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                += s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                += s[8 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_ipip1_optimizer(
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
        1 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
    ];
    CINTall_2e_optimizer(
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
pub unsafe extern "C" fn int2e_ipip1_cart(
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
        1 as ::core::ffi::c_int,
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
    CINTinit_int2e_EnvVars(
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
            CINTgout2e_int2e_ipip1
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT2e_drv(
        out,
        dims,
        &raw mut envs,
        opt,
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
                c2s_cart_2e1
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
pub unsafe extern "C" fn int2e_ipip1_sph(
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
        1 as ::core::ffi::c_int,
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
    CINTinit_int2e_EnvVars(
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
            CINTgout2e_int2e_ipip1
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT2e_drv(
        out,
        dims,
        &raw mut envs,
        opt,
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
                c2s_sph_2e1
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
pub unsafe extern "C" fn cint2e_ipip1_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipip1_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipip1_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipip1_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipip1_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipip1_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipip1_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipip1_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipip1_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipip1_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_ipvip1(
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
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
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
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g2,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g3,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
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
                    (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                += s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                += s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                += s[8 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_ipvip1_optimizer(
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
        9 as ::core::ffi::c_int,
    ];
    CINTall_2e_optimizer(
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
pub unsafe extern "C" fn int2e_ipvip1_cart(
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
    CINTinit_int2e_EnvVars(
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
            CINTgout2e_int2e_ipvip1
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT2e_drv(
        out,
        dims,
        &raw mut envs,
        opt,
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
                c2s_cart_2e1
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
pub unsafe extern "C" fn int2e_ipvip1_sph(
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
    CINTinit_int2e_EnvVars(
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
            CINTgout2e_int2e_ipvip1
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT2e_drv(
        out,
        dims,
        &raw mut envs,
        opt,
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
                c2s_sph_2e1
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
pub unsafe extern "C" fn cint2e_ipvip1_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipvip1_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipvip1_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipvip1_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipvip1_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipvip1_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipvip1_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipvip1_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipvip1_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipvip1_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_ip1ip2(
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
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut g0: *mut ::core::ffi::c_double = g;
    let mut g1: *mut ::core::ffi::c_double = g0
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g2: *mut ::core::ffi::c_double = g1
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g3: *mut ::core::ffi::c_double = g2
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1k_2e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g2,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g3,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
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
                    (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                += s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                += s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                += s[8 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_ip1ip2_optimizer(
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
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        9 as ::core::ffi::c_int,
    ];
    CINTall_2e_optimizer(
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
pub unsafe extern "C" fn int2e_ip1ip2_cart(
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
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
    CINTinit_int2e_EnvVars(
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
            CINTgout2e_int2e_ip1ip2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT2e_drv(
        out,
        dims,
        &raw mut envs,
        opt,
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
                c2s_cart_2e1
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
pub unsafe extern "C" fn int2e_ip1ip2_sph(
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
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
    CINTinit_int2e_EnvVars(
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
            CINTgout2e_int2e_ip1ip2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT2e_drv(
        out,
        dims,
        &raw mut envs,
        opt,
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
                c2s_sph_2e1
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
pub unsafe extern "C" fn cint2e_ip1ip2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ip1ip2_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ip1ip2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ip1ip2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ip1ip2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ip1ip2_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ip1ip2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ip1ip2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ip1ip2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ip1ip2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_int1e_ipippnucp(
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
    let mut g8: *mut ::core::ffi::c_double = g7
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g9: *mut ::core::ffi::c_double = g8
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g10: *mut ::core::ffi::c_double = g9
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g11: *mut ::core::ffi::c_double = g10
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g12: *mut ::core::ffi::c_double = g11
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g13: *mut ::core::ffi::c_double = g12
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g14: *mut ::core::ffi::c_double = g13
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g15: *mut ::core::ffi::c_double = g14
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 3 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g2,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g3,
        g1,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g4,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g3,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g11,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g13,
        g5,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g14,
        g6,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 81] = [0.; 81];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 81 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g15.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[9 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[10 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[11 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[12 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[13 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[14 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[15 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[16 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[17 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[18 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[19 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[20 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[21 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[22 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[23 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[24 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[25 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[26 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[27 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[28 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[29 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[30 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[31 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[32 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[33 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[34 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[35 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[36 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[37 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[38 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[39 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[40 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[41 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[42 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[43 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[44 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[45 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[46 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[47 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[48 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[49 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[50 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[51 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[52 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[53 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[54 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[55 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[56 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[57 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[58 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[59 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[60 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[61 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[62 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[63 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[64 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[65 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[66 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[67 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[68 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[69 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[70 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[71 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[72 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[73 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[74 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[75 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[76 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[77 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[78 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[79 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[80 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[27 as ::core::ffi::c_int as usize]
                + s[31 as ::core::ffi::c_int as usize]
                + s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize]
                + s[58 as ::core::ffi::c_int as usize]
                + s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize]
                + s[13 as ::core::ffi::c_int as usize]
                + s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[36 as ::core::ffi::c_int as usize]
                + s[40 as ::core::ffi::c_int as usize]
                + s[44 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[63 as ::core::ffi::c_int as usize]
                + s[67 as ::core::ffi::c_int as usize]
                + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[18 as ::core::ffi::c_int as usize]
                + s[22 as ::core::ffi::c_int as usize]
                + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[45 as ::core::ffi::c_int as usize]
                + s[49 as ::core::ffi::c_int as usize]
                + s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[72 as ::core::ffi::c_int as usize]
                + s[76 as ::core::ffi::c_int as usize]
                + s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[27 as ::core::ffi::c_int as usize]
                    + s[31 as ::core::ffi::c_int as usize]
                    + s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[54 as ::core::ffi::c_int as usize]
                    + s[58 as ::core::ffi::c_int as usize]
                    + s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[9 as ::core::ffi::c_int as usize]
                    + s[13 as ::core::ffi::c_int as usize]
                    + s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                += s[36 as ::core::ffi::c_int as usize]
                    + s[40 as ::core::ffi::c_int as usize]
                    + s[44 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                += s[63 as ::core::ffi::c_int as usize]
                    + s[67 as ::core::ffi::c_int as usize]
                    + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                += s[18 as ::core::ffi::c_int as usize]
                    + s[22 as ::core::ffi::c_int as usize]
                    + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                += s[45 as ::core::ffi::c_int as usize]
                    + s[49 as ::core::ffi::c_int as usize]
                    + s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                += s[72 as ::core::ffi::c_int as usize]
                    + s[76 as ::core::ffi::c_int as usize]
                    + s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ipippnucp_optimizer(
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
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int1e_ipippnucp_cart(
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
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipippnucp
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
pub unsafe extern "C" fn int1e_ipippnucp_sph(
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
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipippnucp
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
pub unsafe extern "C" fn cint1e_ipippnucp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipippnucp_cart(
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
pub unsafe extern "C" fn cint1e_ipippnucp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipippnucp_sph(
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
pub unsafe extern "C" fn CINTgout1e_int1e_ippnucpip(
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
    let mut g8: *mut ::core::ffi::c_double = g7
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g9: *mut ::core::ffi::c_double = g8
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g10: *mut ::core::ffi::c_double = g9
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g11: *mut ::core::ffi::c_double = g10
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g12: *mut ::core::ffi::c_double = g11
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g13: *mut ::core::ffi::c_double = g12
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g14: *mut ::core::ffi::c_double = g13
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g15: *mut ::core::ffi::c_double = g14
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_2e(
        g2,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_2e(
        g3,
        g2,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g4,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g3,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g11,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g13,
        g5,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g14,
        g6,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 81] = [0.; 81];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 81 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g15.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[9 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[10 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[11 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[12 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[13 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[14 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[15 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[16 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[17 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[18 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[19 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[20 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[21 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[22 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[23 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[24 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[25 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[26 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[27 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[28 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[29 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[30 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[31 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[32 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[33 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[34 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[35 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[36 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[37 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[38 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[39 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[40 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[41 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[42 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[43 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[44 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[45 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[46 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[47 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[48 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[49 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[50 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[51 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[52 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[53 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[54 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[55 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[56 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[57 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[58 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[59 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[60 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[61 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[62 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[63 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[64 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[65 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[66 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[67 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[68 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[69 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[70 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[71 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[72 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[73 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[74 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[75 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[76 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[77 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[78 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[79 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[80 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[12 as ::core::ffi::c_int as usize]
                + s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[27 as ::core::ffi::c_int as usize]
                + s[39 as ::core::ffi::c_int as usize]
                + s[51 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize]
                + s[66 as ::core::ffi::c_int as usize]
                + s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                + s[13 as ::core::ffi::c_int as usize]
                + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[28 as ::core::ffi::c_int as usize]
                + s[40 as ::core::ffi::c_int as usize]
                + s[52 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[55 as ::core::ffi::c_int as usize]
                + s[67 as ::core::ffi::c_int as usize]
                + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize]
                + s[14 as ::core::ffi::c_int as usize]
                + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[29 as ::core::ffi::c_int as usize]
                + s[41 as ::core::ffi::c_int as usize]
                + s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[56 as ::core::ffi::c_int as usize]
                + s[68 as ::core::ffi::c_int as usize]
                + s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize]
                    + s[12 as ::core::ffi::c_int as usize]
                    + s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[27 as ::core::ffi::c_int as usize]
                    + s[39 as ::core::ffi::c_int as usize]
                    + s[51 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[54 as ::core::ffi::c_int as usize]
                    + s[66 as ::core::ffi::c_int as usize]
                    + s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize]
                    + s[13 as ::core::ffi::c_int as usize]
                    + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                += s[28 as ::core::ffi::c_int as usize]
                    + s[40 as ::core::ffi::c_int as usize]
                    + s[52 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                += s[55 as ::core::ffi::c_int as usize]
                    + s[67 as ::core::ffi::c_int as usize]
                    + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                += s[2 as ::core::ffi::c_int as usize]
                    + s[14 as ::core::ffi::c_int as usize]
                    + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                += s[29 as ::core::ffi::c_int as usize]
                    + s[41 as ::core::ffi::c_int as usize]
                    + s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                += s[56 as ::core::ffi::c_int as usize]
                    + s[68 as ::core::ffi::c_int as usize]
                    + s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ippnucpip_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int1e_ippnucpip_cart(
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
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ippnucpip
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
pub unsafe extern "C" fn int1e_ippnucpip_sph(
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
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ippnucpip
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
pub unsafe extern "C" fn cint1e_ippnucpip_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ippnucpip_cart(
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
pub unsafe extern "C" fn cint1e_ippnucpip_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ippnucpip_sph(
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
pub unsafe extern "C" fn CINTgout1e_int1e_ipipprinvp(
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
    let mut g8: *mut ::core::ffi::c_double = g7
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g9: *mut ::core::ffi::c_double = g8
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g10: *mut ::core::ffi::c_double = g9
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g11: *mut ::core::ffi::c_double = g10
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g12: *mut ::core::ffi::c_double = g11
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g13: *mut ::core::ffi::c_double = g12
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g14: *mut ::core::ffi::c_double = g13
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g15: *mut ::core::ffi::c_double = g14
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 3 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g2,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g3,
        g1,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g4,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g3,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g11,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g13,
        g5,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g14,
        g6,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 81] = [0.; 81];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 81 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g15.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[9 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[10 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[11 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[12 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[13 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[14 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[15 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[16 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[17 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[18 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[19 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[20 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[21 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[22 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[23 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[24 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[25 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[26 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[27 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[28 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[29 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[30 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[31 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[32 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[33 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[34 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[35 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[36 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[37 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[38 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[39 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[40 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[41 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[42 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[43 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[44 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[45 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[46 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[47 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[48 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[49 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[50 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[51 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[52 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[53 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[54 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[55 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[56 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[57 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[58 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[59 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[60 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[61 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[62 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[63 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[64 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[65 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[66 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[67 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[68 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[69 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[70 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[71 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[72 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[73 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[74 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[75 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[76 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[77 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[78 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[79 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[80 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[27 as ::core::ffi::c_int as usize]
                + s[31 as ::core::ffi::c_int as usize]
                + s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize]
                + s[58 as ::core::ffi::c_int as usize]
                + s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize]
                + s[13 as ::core::ffi::c_int as usize]
                + s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[36 as ::core::ffi::c_int as usize]
                + s[40 as ::core::ffi::c_int as usize]
                + s[44 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[63 as ::core::ffi::c_int as usize]
                + s[67 as ::core::ffi::c_int as usize]
                + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[18 as ::core::ffi::c_int as usize]
                + s[22 as ::core::ffi::c_int as usize]
                + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[45 as ::core::ffi::c_int as usize]
                + s[49 as ::core::ffi::c_int as usize]
                + s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[72 as ::core::ffi::c_int as usize]
                + s[76 as ::core::ffi::c_int as usize]
                + s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[27 as ::core::ffi::c_int as usize]
                    + s[31 as ::core::ffi::c_int as usize]
                    + s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[54 as ::core::ffi::c_int as usize]
                    + s[58 as ::core::ffi::c_int as usize]
                    + s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[9 as ::core::ffi::c_int as usize]
                    + s[13 as ::core::ffi::c_int as usize]
                    + s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                += s[36 as ::core::ffi::c_int as usize]
                    + s[40 as ::core::ffi::c_int as usize]
                    + s[44 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                += s[63 as ::core::ffi::c_int as usize]
                    + s[67 as ::core::ffi::c_int as usize]
                    + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                += s[18 as ::core::ffi::c_int as usize]
                    + s[22 as ::core::ffi::c_int as usize]
                    + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                += s[45 as ::core::ffi::c_int as usize]
                    + s[49 as ::core::ffi::c_int as usize]
                    + s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                += s[72 as ::core::ffi::c_int as usize]
                    + s[76 as ::core::ffi::c_int as usize]
                    + s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ipipprinvp_optimizer(
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
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int1e_ipipprinvp_cart(
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
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipipprinvp
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
pub unsafe extern "C" fn int1e_ipipprinvp_sph(
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
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipipprinvp
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
pub unsafe extern "C" fn cint1e_ipipprinvp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipipprinvp_cart(
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
pub unsafe extern "C" fn cint1e_ipipprinvp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipipprinvp_sph(
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
pub unsafe extern "C" fn CINTgout1e_int1e_ipprinvpip(
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
    let mut g8: *mut ::core::ffi::c_double = g7
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g9: *mut ::core::ffi::c_double = g8
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g10: *mut ::core::ffi::c_double = g9
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g11: *mut ::core::ffi::c_double = g10
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g12: *mut ::core::ffi::c_double = g11
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g13: *mut ::core::ffi::c_double = g12
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g14: *mut ::core::ffi::c_double = g13
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g15: *mut ::core::ffi::c_double = g14
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_2e(
        g2,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_2e(
        g3,
        g2,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g4,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g3,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g11,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g13,
        g5,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g14,
        g6,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 81] = [0.; 81];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 81 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g15.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[9 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[10 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[11 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[12 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[13 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[14 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[15 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[16 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[17 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[18 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[19 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[20 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[21 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[22 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[23 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[24 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[25 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[26 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[27 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[28 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[29 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[30 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[31 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[32 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[33 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[34 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[35 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[36 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[37 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[38 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[39 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[40 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[41 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[42 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[43 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[44 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[45 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[46 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[47 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[48 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[49 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[50 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[51 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[52 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[53 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[54 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[55 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[56 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[57 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[58 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[59 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[60 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[61 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[62 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[63 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[64 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[65 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[66 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[67 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[68 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[69 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[70 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[71 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[72 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[73 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[74 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[75 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[76 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[77 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[78 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[79 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[80 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[12 as ::core::ffi::c_int as usize]
                + s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[27 as ::core::ffi::c_int as usize]
                + s[39 as ::core::ffi::c_int as usize]
                + s[51 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize]
                + s[66 as ::core::ffi::c_int as usize]
                + s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                + s[13 as ::core::ffi::c_int as usize]
                + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[28 as ::core::ffi::c_int as usize]
                + s[40 as ::core::ffi::c_int as usize]
                + s[52 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[55 as ::core::ffi::c_int as usize]
                + s[67 as ::core::ffi::c_int as usize]
                + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize]
                + s[14 as ::core::ffi::c_int as usize]
                + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[29 as ::core::ffi::c_int as usize]
                + s[41 as ::core::ffi::c_int as usize]
                + s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[56 as ::core::ffi::c_int as usize]
                + s[68 as ::core::ffi::c_int as usize]
                + s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize]
                    + s[12 as ::core::ffi::c_int as usize]
                    + s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[27 as ::core::ffi::c_int as usize]
                    + s[39 as ::core::ffi::c_int as usize]
                    + s[51 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[54 as ::core::ffi::c_int as usize]
                    + s[66 as ::core::ffi::c_int as usize]
                    + s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize]
                    + s[13 as ::core::ffi::c_int as usize]
                    + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                += s[28 as ::core::ffi::c_int as usize]
                    + s[40 as ::core::ffi::c_int as usize]
                    + s[52 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                += s[55 as ::core::ffi::c_int as usize]
                    + s[67 as ::core::ffi::c_int as usize]
                    + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                += s[2 as ::core::ffi::c_int as usize]
                    + s[14 as ::core::ffi::c_int as usize]
                    + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                += s[29 as ::core::ffi::c_int as usize]
                    + s[41 as ::core::ffi::c_int as usize]
                    + s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 9 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                += s[56 as ::core::ffi::c_int as usize]
                    + s[68 as ::core::ffi::c_int as usize]
                    + s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ipprinvpip_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int1e_ipprinvpip_cart(
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
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipprinvpip
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
pub unsafe extern "C" fn int1e_ipprinvpip_sph(
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
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipprinvpip
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
pub unsafe extern "C" fn cint1e_ipprinvpip_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipprinvpip_cart(
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
pub unsafe extern "C" fn cint1e_ipprinvpip_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipprinvpip_sph(
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
pub unsafe extern "C" fn CINTgout1e_int1e_ipipspnucsp(
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
    let mut g8: *mut ::core::ffi::c_double = g7
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g9: *mut ::core::ffi::c_double = g8
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g10: *mut ::core::ffi::c_double = g9
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g11: *mut ::core::ffi::c_double = g10
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g12: *mut ::core::ffi::c_double = g11
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g13: *mut ::core::ffi::c_double = g12
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g14: *mut ::core::ffi::c_double = g13
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g15: *mut ::core::ffi::c_double = g14
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 3 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g2,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g3,
        g1,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g4,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g3,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g11,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g13,
        g5,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g14,
        g6,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 81] = [0.; 81];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 81 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g15.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[9 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[10 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[11 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[12 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[13 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[14 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[15 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[16 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[17 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[18 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[19 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[20 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[21 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[22 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[23 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[24 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[25 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[26 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[27 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[28 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[29 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[30 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[31 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[32 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[33 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[34 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[35 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[36 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[37 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[38 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[39 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[40 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[41 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[42 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[43 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[44 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[45 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[46 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[47 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[48 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[49 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[50 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[51 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[52 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[53 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[54 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[55 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[56 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[57 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[58 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[59 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[60 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[61 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[62 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[63 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[64 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[65 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[66 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[67 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[68 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[69 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[70 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[71 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[72 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[73 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[74 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[75 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[76 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[77 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[78 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[79 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[80 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[29 as ::core::ffi::c_int as usize]
                - s[55 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize]
                - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                - s[27 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[28 as ::core::ffi::c_int as usize]
                + s[56 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[32 as ::core::ffi::c_int as usize]
                - s[58 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[57 as ::core::ffi::c_int as usize]
                - s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize]
                - s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize]
                + s[31 as ::core::ffi::c_int as usize]
                + s[59 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[35 as ::core::ffi::c_int as usize]
                - s[61 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[60 as ::core::ffi::c_int as usize]
                - s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize]
                - s[33 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize]
                + s[34 as ::core::ffi::c_int as usize]
                + s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = s[38 as ::core::ffi::c_int as usize]
                - s[64 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = s[63 as ::core::ffi::c_int as usize]
                - s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = s[10 as ::core::ffi::c_int as usize]
                - s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize]
                + s[37 as ::core::ffi::c_int as usize]
                + s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) = s[41 as ::core::ffi::c_int as usize]
                - s[67 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) = s[66 as ::core::ffi::c_int as usize]
                - s[14 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) = s[13 as ::core::ffi::c_int as usize]
                - s[39 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) = s[12 as ::core::ffi::c_int as usize]
                + s[40 as ::core::ffi::c_int as usize]
                + s[68 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) = s[44 as ::core::ffi::c_int as usize]
                - s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) = s[69 as ::core::ffi::c_int as usize]
                - s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) = s[16 as ::core::ffi::c_int as usize]
                - s[42 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) = s[15 as ::core::ffi::c_int as usize]
                + s[43 as ::core::ffi::c_int as usize]
                + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) = s[47 as ::core::ffi::c_int as usize]
                - s[73 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) = s[72 as ::core::ffi::c_int as usize]
                - s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) = s[19 as ::core::ffi::c_int as usize]
                - s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                ) = s[18 as ::core::ffi::c_int as usize]
                + s[46 as ::core::ffi::c_int as usize]
                + s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                ) = s[50 as ::core::ffi::c_int as usize]
                - s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                ) = s[75 as ::core::ffi::c_int as usize]
                - s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                ) = s[22 as ::core::ffi::c_int as usize]
                - s[48 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                ) = s[21 as ::core::ffi::c_int as usize]
                + s[49 as ::core::ffi::c_int as usize]
                + s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                ) = s[53 as ::core::ffi::c_int as usize]
                - s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                ) = s[78 as ::core::ffi::c_int as usize]
                - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                ) = s[25 as ::core::ffi::c_int as usize]
                - s[51 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                ) = s[24 as ::core::ffi::c_int as usize]
                + s[52 as ::core::ffi::c_int as usize]
                + s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[29 as ::core::ffi::c_int as usize]
                    - s[55 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += s[54 as ::core::ffi::c_int as usize]
                    - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += s[1 as ::core::ffi::c_int as usize]
                    - s[27 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[28 as ::core::ffi::c_int as usize]
                    + s[56 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += s[32 as ::core::ffi::c_int as usize]
                    - s[58 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[57 as ::core::ffi::c_int as usize]
                    - s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += s[4 as ::core::ffi::c_int as usize]
                    - s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += s[3 as ::core::ffi::c_int as usize]
                    + s[31 as ::core::ffi::c_int as usize]
                    + s[59 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += s[35 as ::core::ffi::c_int as usize]
                    - s[61 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += s[60 as ::core::ffi::c_int as usize]
                    - s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[7 as ::core::ffi::c_int as usize]
                    - s[33 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[6 as ::core::ffi::c_int as usize]
                    + s[34 as ::core::ffi::c_int as usize]
                    + s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                )
                += s[38 as ::core::ffi::c_int as usize]
                    - s[64 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                )
                += s[63 as ::core::ffi::c_int as usize]
                    - s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                )
                += s[10 as ::core::ffi::c_int as usize]
                    - s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                )
                += s[9 as ::core::ffi::c_int as usize]
                    + s[37 as ::core::ffi::c_int as usize]
                    + s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                )
                += s[41 as ::core::ffi::c_int as usize]
                    - s[67 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                )
                += s[66 as ::core::ffi::c_int as usize]
                    - s[14 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                )
                += s[13 as ::core::ffi::c_int as usize]
                    - s[39 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                )
                += s[12 as ::core::ffi::c_int as usize]
                    + s[40 as ::core::ffi::c_int as usize]
                    + s[68 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                )
                += s[44 as ::core::ffi::c_int as usize]
                    - s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                )
                += s[69 as ::core::ffi::c_int as usize]
                    - s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                )
                += s[16 as ::core::ffi::c_int as usize]
                    - s[42 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                )
                += s[15 as ::core::ffi::c_int as usize]
                    + s[43 as ::core::ffi::c_int as usize]
                    + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                )
                += s[47 as ::core::ffi::c_int as usize]
                    - s[73 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                )
                += s[72 as ::core::ffi::c_int as usize]
                    - s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                )
                += s[19 as ::core::ffi::c_int as usize]
                    - s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                )
                += s[18 as ::core::ffi::c_int as usize]
                    + s[46 as ::core::ffi::c_int as usize]
                    + s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                )
                += s[50 as ::core::ffi::c_int as usize]
                    - s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                )
                += s[75 as ::core::ffi::c_int as usize]
                    - s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                )
                += s[22 as ::core::ffi::c_int as usize]
                    - s[48 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                )
                += s[21 as ::core::ffi::c_int as usize]
                    + s[49 as ::core::ffi::c_int as usize]
                    + s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                )
                += s[53 as ::core::ffi::c_int as usize]
                    - s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                )
                += s[78 as ::core::ffi::c_int as usize]
                    - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                )
                += s[25 as ::core::ffi::c_int as usize]
                    - s[51 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                )
                += s[24 as ::core::ffi::c_int as usize]
                    + s[52 as ::core::ffi::c_int as usize]
                    + s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ipipspnucsp_optimizer(
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
        4 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int1e_ipipspnucsp_cart(
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
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipipspnucsp
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
pub unsafe extern "C" fn int1e_ipipspnucsp_sph(
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
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipipspnucsp
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
pub unsafe extern "C" fn cint1e_ipipspnucsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipipspnucsp_cart(
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
pub unsafe extern "C" fn cint1e_ipipspnucsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipipspnucsp_sph(
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
pub unsafe extern "C" fn CINTgout1e_int1e_ipspnucspip(
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
    let mut g8: *mut ::core::ffi::c_double = g7
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g9: *mut ::core::ffi::c_double = g8
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g10: *mut ::core::ffi::c_double = g9
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g11: *mut ::core::ffi::c_double = g10
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g12: *mut ::core::ffi::c_double = g11
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g13: *mut ::core::ffi::c_double = g12
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g14: *mut ::core::ffi::c_double = g13
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g15: *mut ::core::ffi::c_double = g14
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_2e(
        g2,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_2e(
        g3,
        g2,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g4,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g3,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g11,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g13,
        g5,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g14,
        g6,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 81] = [0.; 81];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 81 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g15.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[9 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[10 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[11 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[12 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[13 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[14 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[15 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[16 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[17 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[18 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[19 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[20 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[21 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[22 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[23 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[24 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[25 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[26 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[27 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[28 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[29 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[30 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[31 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[32 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[33 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[34 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[35 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[36 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[37 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[38 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[39 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[40 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[41 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[42 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[43 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[44 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[45 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[46 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[47 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[48 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[49 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[50 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[51 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[52 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[53 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[54 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[55 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[56 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[57 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[58 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[59 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[60 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[61 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[62 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[63 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[64 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[65 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[66 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[67 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[68 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[69 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[70 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[71 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[72 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[73 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[74 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[75 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[76 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[77 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[78 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[79 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[80 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[33 as ::core::ffi::c_int as usize]
                - s[57 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize]
                - s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize]
                - s[27 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[30 as ::core::ffi::c_int as usize]
                + s[60 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[34 as ::core::ffi::c_int as usize]
                - s[58 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[55 as ::core::ffi::c_int as usize]
                - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize]
                - s[28 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                + s[31 as ::core::ffi::c_int as usize]
                + s[61 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[35 as ::core::ffi::c_int as usize]
                - s[59 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[56 as ::core::ffi::c_int as usize]
                - s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize]
                - s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize]
                + s[32 as ::core::ffi::c_int as usize]
                + s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = s[42 as ::core::ffi::c_int as usize]
                - s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = s[63 as ::core::ffi::c_int as usize]
                - s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = s[12 as ::core::ffi::c_int as usize]
                - s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize]
                + s[39 as ::core::ffi::c_int as usize]
                + s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) = s[43 as ::core::ffi::c_int as usize]
                - s[67 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) = s[64 as ::core::ffi::c_int as usize]
                - s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) = s[13 as ::core::ffi::c_int as usize]
                - s[37 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) = s[10 as ::core::ffi::c_int as usize]
                + s[40 as ::core::ffi::c_int as usize]
                + s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) = s[44 as ::core::ffi::c_int as usize]
                - s[68 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) = s[65 as ::core::ffi::c_int as usize]
                - s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) = s[14 as ::core::ffi::c_int as usize]
                - s[38 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) = s[11 as ::core::ffi::c_int as usize]
                + s[41 as ::core::ffi::c_int as usize]
                + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) = s[51 as ::core::ffi::c_int as usize]
                - s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) = s[72 as ::core::ffi::c_int as usize]
                - s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) = s[21 as ::core::ffi::c_int as usize]
                - s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                ) = s[18 as ::core::ffi::c_int as usize]
                + s[48 as ::core::ffi::c_int as usize]
                + s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                ) = s[52 as ::core::ffi::c_int as usize]
                - s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                ) = s[73 as ::core::ffi::c_int as usize]
                - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                ) = s[22 as ::core::ffi::c_int as usize]
                - s[46 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                ) = s[19 as ::core::ffi::c_int as usize]
                + s[49 as ::core::ffi::c_int as usize]
                + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                ) = s[53 as ::core::ffi::c_int as usize]
                - s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                ) = s[74 as ::core::ffi::c_int as usize]
                - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                ) = s[23 as ::core::ffi::c_int as usize]
                - s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                ) = s[20 as ::core::ffi::c_int as usize]
                + s[50 as ::core::ffi::c_int as usize]
                + s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[33 as ::core::ffi::c_int as usize]
                    - s[57 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += s[54 as ::core::ffi::c_int as usize]
                    - s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += s[3 as ::core::ffi::c_int as usize]
                    - s[27 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[30 as ::core::ffi::c_int as usize]
                    + s[60 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += s[34 as ::core::ffi::c_int as usize]
                    - s[58 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[55 as ::core::ffi::c_int as usize]
                    - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += s[4 as ::core::ffi::c_int as usize]
                    - s[28 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += s[1 as ::core::ffi::c_int as usize]
                    + s[31 as ::core::ffi::c_int as usize]
                    + s[61 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += s[35 as ::core::ffi::c_int as usize]
                    - s[59 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += s[56 as ::core::ffi::c_int as usize]
                    - s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[5 as ::core::ffi::c_int as usize]
                    - s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[2 as ::core::ffi::c_int as usize]
                    + s[32 as ::core::ffi::c_int as usize]
                    + s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                )
                += s[42 as ::core::ffi::c_int as usize]
                    - s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                )
                += s[63 as ::core::ffi::c_int as usize]
                    - s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                )
                += s[12 as ::core::ffi::c_int as usize]
                    - s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                )
                += s[9 as ::core::ffi::c_int as usize]
                    + s[39 as ::core::ffi::c_int as usize]
                    + s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                )
                += s[43 as ::core::ffi::c_int as usize]
                    - s[67 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                )
                += s[64 as ::core::ffi::c_int as usize]
                    - s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                )
                += s[13 as ::core::ffi::c_int as usize]
                    - s[37 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                )
                += s[10 as ::core::ffi::c_int as usize]
                    + s[40 as ::core::ffi::c_int as usize]
                    + s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                )
                += s[44 as ::core::ffi::c_int as usize]
                    - s[68 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                )
                += s[65 as ::core::ffi::c_int as usize]
                    - s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                )
                += s[14 as ::core::ffi::c_int as usize]
                    - s[38 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                )
                += s[11 as ::core::ffi::c_int as usize]
                    + s[41 as ::core::ffi::c_int as usize]
                    + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                )
                += s[51 as ::core::ffi::c_int as usize]
                    - s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                )
                += s[72 as ::core::ffi::c_int as usize]
                    - s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                )
                += s[21 as ::core::ffi::c_int as usize]
                    - s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                )
                += s[18 as ::core::ffi::c_int as usize]
                    + s[48 as ::core::ffi::c_int as usize]
                    + s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                )
                += s[52 as ::core::ffi::c_int as usize]
                    - s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                )
                += s[73 as ::core::ffi::c_int as usize]
                    - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                )
                += s[22 as ::core::ffi::c_int as usize]
                    - s[46 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                )
                += s[19 as ::core::ffi::c_int as usize]
                    + s[49 as ::core::ffi::c_int as usize]
                    + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                )
                += s[53 as ::core::ffi::c_int as usize]
                    - s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                )
                += s[74 as ::core::ffi::c_int as usize]
                    - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                )
                += s[23 as ::core::ffi::c_int as usize]
                    - s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                )
                += s[20 as ::core::ffi::c_int as usize]
                    + s[50 as ::core::ffi::c_int as usize]
                    + s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ipspnucspip_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int1e_ipspnucspip_cart(
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
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipspnucspip
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
pub unsafe extern "C" fn int1e_ipspnucspip_sph(
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
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipspnucspip
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
pub unsafe extern "C" fn cint1e_ipspnucspip_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipspnucspip_cart(
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
pub unsafe extern "C" fn cint1e_ipspnucspip_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipspnucspip_sph(
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
pub unsafe extern "C" fn CINTgout1e_int1e_ipipsprinvsp(
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
    let mut g8: *mut ::core::ffi::c_double = g7
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g9: *mut ::core::ffi::c_double = g8
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g10: *mut ::core::ffi::c_double = g9
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g11: *mut ::core::ffi::c_double = g10
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g12: *mut ::core::ffi::c_double = g11
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g13: *mut ::core::ffi::c_double = g12
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g14: *mut ::core::ffi::c_double = g13
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g15: *mut ::core::ffi::c_double = g14
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 3 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g2,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g3,
        g1,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g4,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g3,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g11,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g13,
        g5,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g14,
        g6,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 81] = [0.; 81];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 81 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g15.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[9 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[10 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[11 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[12 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[13 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[14 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[15 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[16 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[17 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[18 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[19 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[20 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[21 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[22 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[23 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[24 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[25 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[26 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[27 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[28 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[29 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[30 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[31 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[32 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[33 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[34 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[35 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[36 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[37 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[38 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[39 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[40 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[41 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[42 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[43 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[44 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[45 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[46 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[47 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[48 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[49 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[50 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[51 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[52 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[53 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[54 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[55 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[56 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[57 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[58 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[59 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[60 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[61 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[62 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[63 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[64 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[65 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[66 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[67 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[68 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[69 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[70 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[71 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[72 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[73 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[74 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[75 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[76 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[77 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[78 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[79 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[80 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[29 as ::core::ffi::c_int as usize]
                - s[55 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize]
                - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                - s[27 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[28 as ::core::ffi::c_int as usize]
                + s[56 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[32 as ::core::ffi::c_int as usize]
                - s[58 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[57 as ::core::ffi::c_int as usize]
                - s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize]
                - s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize]
                + s[31 as ::core::ffi::c_int as usize]
                + s[59 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[35 as ::core::ffi::c_int as usize]
                - s[61 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[60 as ::core::ffi::c_int as usize]
                - s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize]
                - s[33 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize]
                + s[34 as ::core::ffi::c_int as usize]
                + s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = s[38 as ::core::ffi::c_int as usize]
                - s[64 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = s[63 as ::core::ffi::c_int as usize]
                - s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = s[10 as ::core::ffi::c_int as usize]
                - s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize]
                + s[37 as ::core::ffi::c_int as usize]
                + s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) = s[41 as ::core::ffi::c_int as usize]
                - s[67 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) = s[66 as ::core::ffi::c_int as usize]
                - s[14 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) = s[13 as ::core::ffi::c_int as usize]
                - s[39 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) = s[12 as ::core::ffi::c_int as usize]
                + s[40 as ::core::ffi::c_int as usize]
                + s[68 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) = s[44 as ::core::ffi::c_int as usize]
                - s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) = s[69 as ::core::ffi::c_int as usize]
                - s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) = s[16 as ::core::ffi::c_int as usize]
                - s[42 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) = s[15 as ::core::ffi::c_int as usize]
                + s[43 as ::core::ffi::c_int as usize]
                + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) = s[47 as ::core::ffi::c_int as usize]
                - s[73 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) = s[72 as ::core::ffi::c_int as usize]
                - s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) = s[19 as ::core::ffi::c_int as usize]
                - s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                ) = s[18 as ::core::ffi::c_int as usize]
                + s[46 as ::core::ffi::c_int as usize]
                + s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                ) = s[50 as ::core::ffi::c_int as usize]
                - s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                ) = s[75 as ::core::ffi::c_int as usize]
                - s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                ) = s[22 as ::core::ffi::c_int as usize]
                - s[48 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                ) = s[21 as ::core::ffi::c_int as usize]
                + s[49 as ::core::ffi::c_int as usize]
                + s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                ) = s[53 as ::core::ffi::c_int as usize]
                - s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                ) = s[78 as ::core::ffi::c_int as usize]
                - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                ) = s[25 as ::core::ffi::c_int as usize]
                - s[51 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                ) = s[24 as ::core::ffi::c_int as usize]
                + s[52 as ::core::ffi::c_int as usize]
                + s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[29 as ::core::ffi::c_int as usize]
                    - s[55 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += s[54 as ::core::ffi::c_int as usize]
                    - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += s[1 as ::core::ffi::c_int as usize]
                    - s[27 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[28 as ::core::ffi::c_int as usize]
                    + s[56 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += s[32 as ::core::ffi::c_int as usize]
                    - s[58 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[57 as ::core::ffi::c_int as usize]
                    - s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += s[4 as ::core::ffi::c_int as usize]
                    - s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += s[3 as ::core::ffi::c_int as usize]
                    + s[31 as ::core::ffi::c_int as usize]
                    + s[59 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += s[35 as ::core::ffi::c_int as usize]
                    - s[61 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += s[60 as ::core::ffi::c_int as usize]
                    - s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[7 as ::core::ffi::c_int as usize]
                    - s[33 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[6 as ::core::ffi::c_int as usize]
                    + s[34 as ::core::ffi::c_int as usize]
                    + s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                )
                += s[38 as ::core::ffi::c_int as usize]
                    - s[64 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                )
                += s[63 as ::core::ffi::c_int as usize]
                    - s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                )
                += s[10 as ::core::ffi::c_int as usize]
                    - s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                )
                += s[9 as ::core::ffi::c_int as usize]
                    + s[37 as ::core::ffi::c_int as usize]
                    + s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                )
                += s[41 as ::core::ffi::c_int as usize]
                    - s[67 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                )
                += s[66 as ::core::ffi::c_int as usize]
                    - s[14 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                )
                += s[13 as ::core::ffi::c_int as usize]
                    - s[39 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                )
                += s[12 as ::core::ffi::c_int as usize]
                    + s[40 as ::core::ffi::c_int as usize]
                    + s[68 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                )
                += s[44 as ::core::ffi::c_int as usize]
                    - s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                )
                += s[69 as ::core::ffi::c_int as usize]
                    - s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                )
                += s[16 as ::core::ffi::c_int as usize]
                    - s[42 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                )
                += s[15 as ::core::ffi::c_int as usize]
                    + s[43 as ::core::ffi::c_int as usize]
                    + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                )
                += s[47 as ::core::ffi::c_int as usize]
                    - s[73 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                )
                += s[72 as ::core::ffi::c_int as usize]
                    - s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                )
                += s[19 as ::core::ffi::c_int as usize]
                    - s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                )
                += s[18 as ::core::ffi::c_int as usize]
                    + s[46 as ::core::ffi::c_int as usize]
                    + s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                )
                += s[50 as ::core::ffi::c_int as usize]
                    - s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                )
                += s[75 as ::core::ffi::c_int as usize]
                    - s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                )
                += s[22 as ::core::ffi::c_int as usize]
                    - s[48 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                )
                += s[21 as ::core::ffi::c_int as usize]
                    + s[49 as ::core::ffi::c_int as usize]
                    + s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                )
                += s[53 as ::core::ffi::c_int as usize]
                    - s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                )
                += s[78 as ::core::ffi::c_int as usize]
                    - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                )
                += s[25 as ::core::ffi::c_int as usize]
                    - s[51 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                )
                += s[24 as ::core::ffi::c_int as usize]
                    + s[52 as ::core::ffi::c_int as usize]
                    + s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ipipsprinvsp_optimizer(
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
        4 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int1e_ipipsprinvsp_cart(
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
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipipsprinvsp
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
pub unsafe extern "C" fn int1e_ipipsprinvsp_sph(
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
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipipsprinvsp
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
pub unsafe extern "C" fn cint1e_ipipsprinvsp_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipipsprinvsp_cart(
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
pub unsafe extern "C" fn cint1e_ipipsprinvsp_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipipsprinvsp_sph(
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
pub unsafe extern "C" fn CINTgout1e_int1e_ipsprinvspip(
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
    let mut g8: *mut ::core::ffi::c_double = g7
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g9: *mut ::core::ffi::c_double = g8
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g10: *mut ::core::ffi::c_double = g9
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g11: *mut ::core::ffi::c_double = g10
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g12: *mut ::core::ffi::c_double = g11
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g13: *mut ::core::ffi::c_double = g12
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g14: *mut ::core::ffi::c_double = g13
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g15: *mut ::core::ffi::c_double = g14
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_2e(
        g2,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_2e(
        g3,
        g2,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g4,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g3,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g11,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g13,
        g5,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g14,
        g6,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        envs,
    );
    let mut s: [::core::ffi::c_double; 81] = [0.; 81];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 81 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g15.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[9 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[10 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[11 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[12 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[13 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[14 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[15 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[16 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[17 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[18 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[19 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[20 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[21 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[22 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[23 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[24 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[25 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[26 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[27 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[28 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[29 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[30 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[31 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[32 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[33 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[34 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[35 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[36 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[37 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[38 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[39 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[40 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[41 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[42 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[43 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[44 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[45 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[46 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[47 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[48 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[49 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[50 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[51 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[52 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[53 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[54 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[55 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[56 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[57 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[58 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[59 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[60 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[61 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[62 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[63 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[64 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[65 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[66 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[67 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[68 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[69 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[70 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[71 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[72 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[73 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[74 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[75 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[76 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[77 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[78 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[79 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[80 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[33 as ::core::ffi::c_int as usize]
                - s[57 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize]
                - s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize]
                - s[27 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[30 as ::core::ffi::c_int as usize]
                + s[60 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[34 as ::core::ffi::c_int as usize]
                - s[58 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[55 as ::core::ffi::c_int as usize]
                - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize]
                - s[28 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                + s[31 as ::core::ffi::c_int as usize]
                + s[61 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[35 as ::core::ffi::c_int as usize]
                - s[59 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[56 as ::core::ffi::c_int as usize]
                - s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize]
                - s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize]
                + s[32 as ::core::ffi::c_int as usize]
                + s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = s[42 as ::core::ffi::c_int as usize]
                - s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = s[63 as ::core::ffi::c_int as usize]
                - s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = s[12 as ::core::ffi::c_int as usize]
                - s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize]
                + s[39 as ::core::ffi::c_int as usize]
                + s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) = s[43 as ::core::ffi::c_int as usize]
                - s[67 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) = s[64 as ::core::ffi::c_int as usize]
                - s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) = s[13 as ::core::ffi::c_int as usize]
                - s[37 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) = s[10 as ::core::ffi::c_int as usize]
                + s[40 as ::core::ffi::c_int as usize]
                + s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) = s[44 as ::core::ffi::c_int as usize]
                - s[68 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) = s[65 as ::core::ffi::c_int as usize]
                - s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) = s[14 as ::core::ffi::c_int as usize]
                - s[38 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) = s[11 as ::core::ffi::c_int as usize]
                + s[41 as ::core::ffi::c_int as usize]
                + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) = s[51 as ::core::ffi::c_int as usize]
                - s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) = s[72 as ::core::ffi::c_int as usize]
                - s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) = s[21 as ::core::ffi::c_int as usize]
                - s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                ) = s[18 as ::core::ffi::c_int as usize]
                + s[48 as ::core::ffi::c_int as usize]
                + s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                ) = s[52 as ::core::ffi::c_int as usize]
                - s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                ) = s[73 as ::core::ffi::c_int as usize]
                - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                ) = s[22 as ::core::ffi::c_int as usize]
                - s[46 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                ) = s[19 as ::core::ffi::c_int as usize]
                + s[49 as ::core::ffi::c_int as usize]
                + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                ) = s[53 as ::core::ffi::c_int as usize]
                - s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                ) = s[74 as ::core::ffi::c_int as usize]
                - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                ) = s[23 as ::core::ffi::c_int as usize]
                - s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                ) = s[20 as ::core::ffi::c_int as usize]
                + s[50 as ::core::ffi::c_int as usize]
                + s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[33 as ::core::ffi::c_int as usize]
                    - s[57 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += s[54 as ::core::ffi::c_int as usize]
                    - s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += s[3 as ::core::ffi::c_int as usize]
                    - s[27 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[30 as ::core::ffi::c_int as usize]
                    + s[60 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += s[34 as ::core::ffi::c_int as usize]
                    - s[58 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[55 as ::core::ffi::c_int as usize]
                    - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += s[4 as ::core::ffi::c_int as usize]
                    - s[28 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += s[1 as ::core::ffi::c_int as usize]
                    + s[31 as ::core::ffi::c_int as usize]
                    + s[61 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += s[35 as ::core::ffi::c_int as usize]
                    - s[59 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += s[56 as ::core::ffi::c_int as usize]
                    - s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[5 as ::core::ffi::c_int as usize]
                    - s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[2 as ::core::ffi::c_int as usize]
                    + s[32 as ::core::ffi::c_int as usize]
                    + s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                )
                += s[42 as ::core::ffi::c_int as usize]
                    - s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                )
                += s[63 as ::core::ffi::c_int as usize]
                    - s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                )
                += s[12 as ::core::ffi::c_int as usize]
                    - s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                )
                += s[9 as ::core::ffi::c_int as usize]
                    + s[39 as ::core::ffi::c_int as usize]
                    + s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                )
                += s[43 as ::core::ffi::c_int as usize]
                    - s[67 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                )
                += s[64 as ::core::ffi::c_int as usize]
                    - s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                )
                += s[13 as ::core::ffi::c_int as usize]
                    - s[37 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                )
                += s[10 as ::core::ffi::c_int as usize]
                    + s[40 as ::core::ffi::c_int as usize]
                    + s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                )
                += s[44 as ::core::ffi::c_int as usize]
                    - s[68 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                )
                += s[65 as ::core::ffi::c_int as usize]
                    - s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                )
                += s[14 as ::core::ffi::c_int as usize]
                    - s[38 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                )
                += s[11 as ::core::ffi::c_int as usize]
                    + s[41 as ::core::ffi::c_int as usize]
                    + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                )
                += s[51 as ::core::ffi::c_int as usize]
                    - s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                )
                += s[72 as ::core::ffi::c_int as usize]
                    - s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                )
                += s[21 as ::core::ffi::c_int as usize]
                    - s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                )
                += s[18 as ::core::ffi::c_int as usize]
                    + s[48 as ::core::ffi::c_int as usize]
                    + s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                )
                += s[52 as ::core::ffi::c_int as usize]
                    - s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                )
                += s[73 as ::core::ffi::c_int as usize]
                    - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                )
                += s[22 as ::core::ffi::c_int as usize]
                    - s[46 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                )
                += s[19 as ::core::ffi::c_int as usize]
                    + s[49 as ::core::ffi::c_int as usize]
                    + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                )
                += s[53 as ::core::ffi::c_int as usize]
                    - s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                )
                += s[74 as ::core::ffi::c_int as usize]
                    - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                )
                += s[23 as ::core::ffi::c_int as usize]
                    - s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 36 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                )
                += s[20 as ::core::ffi::c_int as usize]
                    + s[50 as ::core::ffi::c_int as usize]
                    + s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_ipsprinvspip_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut ng: [::core::ffi::c_int; 8] = [
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int1e_ipsprinvspip_cart(
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
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipsprinvspip
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
pub unsafe extern "C" fn int1e_ipsprinvspip_sph(
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
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout1e_int1e_ipsprinvspip
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
pub unsafe extern "C" fn cint1e_ipsprinvspip_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipsprinvspip_sph(
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
pub unsafe extern "C" fn cint1e_ipsprinvspip_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    return int1e_ipsprinvspip_cart(
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
pub unsafe extern "C" fn CINTgout2e_int2e_ipip1ipip2(
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
    let mut i: ::core::ffi::c_int = 0;
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
    let mut g8: *mut ::core::ffi::c_double = g7
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g9: *mut ::core::ffi::c_double = g8
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g10: *mut ::core::ffi::c_double = g9
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g11: *mut ::core::ffi::c_double = g10
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g12: *mut ::core::ffi::c_double = g11
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g13: *mut ::core::ffi::c_double = g12
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g14: *mut ::core::ffi::c_double = g13
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g15: *mut ::core::ffi::c_double = g14
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1k_2e(
        g1,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l + 1 as ::core::ffi::c_int,
        (*envs).l_l,
        envs,
    );
    CINTnabla1k_2e(
        g2,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l,
        envs,
    );
    CINTnabla1k_2e(
        g3,
        g1,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g4,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g3,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g11,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g13,
        g5,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g14,
        g6,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    let mut s: [::core::ffi::c_double; 81] = [0.; 81];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 81 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g15.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[9 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[10 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[11 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[12 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[13 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[14 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[15 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[16 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[17 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[18 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[19 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[20 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[21 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[22 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[23 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[24 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[25 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[26 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[27 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[28 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[29 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[30 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[31 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[32 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[33 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[34 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[35 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[36 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[37 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[38 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[39 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[40 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[41 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[42 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[43 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[44 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[45 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[46 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[47 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[48 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[49 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[50 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[51 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[52 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[53 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[54 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[55 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[56 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[57 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[58 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[59 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[60 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[61 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[62 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[63 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[64 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[65 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[66 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[67 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[68 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[69 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[70 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[71 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[72 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[73 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[74 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[75 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[76 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[77 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[78 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[79 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[80 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[27 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[33 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = s[28 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = s[31 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) = s[32 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) = s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) = s[57 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) = s[60 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) = s[55 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) = s[58 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) = s[61 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) = s[56 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) = s[59 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) = s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                ) = s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                ) = s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                ) = s[10 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                ) = s[13 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                ) = s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                ) = s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                ) = s[14 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                ) = s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 36 as ::core::ffi::c_int) as isize,
                ) = s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 37 as ::core::ffi::c_int) as isize,
                ) = s[39 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 38 as ::core::ffi::c_int) as isize,
                ) = s[42 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 39 as ::core::ffi::c_int) as isize,
                ) = s[37 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 40 as ::core::ffi::c_int) as isize,
                ) = s[40 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 41 as ::core::ffi::c_int) as isize,
                ) = s[43 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 42 as ::core::ffi::c_int) as isize,
                ) = s[38 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 43 as ::core::ffi::c_int) as isize,
                ) = s[41 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 44 as ::core::ffi::c_int) as isize,
                ) = s[44 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 45 as ::core::ffi::c_int) as isize,
                ) = s[63 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 46 as ::core::ffi::c_int) as isize,
                ) = s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 47 as ::core::ffi::c_int) as isize,
                ) = s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 48 as ::core::ffi::c_int) as isize,
                ) = s[64 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 49 as ::core::ffi::c_int) as isize,
                ) = s[67 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 50 as ::core::ffi::c_int) as isize,
                ) = s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 51 as ::core::ffi::c_int) as isize,
                ) = s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 52 as ::core::ffi::c_int) as isize,
                ) = s[68 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 53 as ::core::ffi::c_int) as isize,
                ) = s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 54 as ::core::ffi::c_int) as isize,
                ) = s[18 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 55 as ::core::ffi::c_int) as isize,
                ) = s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 56 as ::core::ffi::c_int) as isize,
                ) = s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 57 as ::core::ffi::c_int) as isize,
                ) = s[19 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 58 as ::core::ffi::c_int) as isize,
                ) = s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 59 as ::core::ffi::c_int) as isize,
                ) = s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 60 as ::core::ffi::c_int) as isize,
                ) = s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 61 as ::core::ffi::c_int) as isize,
                ) = s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 62 as ::core::ffi::c_int) as isize,
                ) = s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 63 as ::core::ffi::c_int) as isize,
                ) = s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 64 as ::core::ffi::c_int) as isize,
                ) = s[48 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 65 as ::core::ffi::c_int) as isize,
                ) = s[51 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 66 as ::core::ffi::c_int) as isize,
                ) = s[46 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 67 as ::core::ffi::c_int) as isize,
                ) = s[49 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 68 as ::core::ffi::c_int) as isize,
                ) = s[52 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 69 as ::core::ffi::c_int) as isize,
                ) = s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 70 as ::core::ffi::c_int) as isize,
                ) = s[50 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 71 as ::core::ffi::c_int) as isize,
                ) = s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 72 as ::core::ffi::c_int) as isize,
                ) = s[72 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 73 as ::core::ffi::c_int) as isize,
                ) = s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 74 as ::core::ffi::c_int) as isize,
                ) = s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 75 as ::core::ffi::c_int) as isize,
                ) = s[73 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 76 as ::core::ffi::c_int) as isize,
                ) = s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 77 as ::core::ffi::c_int) as isize,
                ) = s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 78 as ::core::ffi::c_int) as isize,
                ) = s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 79 as ::core::ffi::c_int) as isize,
                ) = s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 80 as ::core::ffi::c_int) as isize,
                ) = s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) += s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) += s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) += s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) += s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) += s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) += s[27 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) += s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) += s[33 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) += s[28 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) += s[31 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) += s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) += s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) += s[32 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) += s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) += s[54 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) += s[57 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) += s[60 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) += s[55 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) += s[58 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) += s[61 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) += s[56 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) += s[59 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) += s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                ) += s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                ) += s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                ) += s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                ) += s[10 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                ) += s[13 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                ) += s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                ) += s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                ) += s[14 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                ) += s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 36 as ::core::ffi::c_int) as isize,
                ) += s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 37 as ::core::ffi::c_int) as isize,
                ) += s[39 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 38 as ::core::ffi::c_int) as isize,
                ) += s[42 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 39 as ::core::ffi::c_int) as isize,
                ) += s[37 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 40 as ::core::ffi::c_int) as isize,
                ) += s[40 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 41 as ::core::ffi::c_int) as isize,
                ) += s[43 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 42 as ::core::ffi::c_int) as isize,
                ) += s[38 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 43 as ::core::ffi::c_int) as isize,
                ) += s[41 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 44 as ::core::ffi::c_int) as isize,
                ) += s[44 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 45 as ::core::ffi::c_int) as isize,
                ) += s[63 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 46 as ::core::ffi::c_int) as isize,
                ) += s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 47 as ::core::ffi::c_int) as isize,
                ) += s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 48 as ::core::ffi::c_int) as isize,
                ) += s[64 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 49 as ::core::ffi::c_int) as isize,
                ) += s[67 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 50 as ::core::ffi::c_int) as isize,
                ) += s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 51 as ::core::ffi::c_int) as isize,
                ) += s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 52 as ::core::ffi::c_int) as isize,
                ) += s[68 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 53 as ::core::ffi::c_int) as isize,
                ) += s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 54 as ::core::ffi::c_int) as isize,
                ) += s[18 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 55 as ::core::ffi::c_int) as isize,
                ) += s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 56 as ::core::ffi::c_int) as isize,
                ) += s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 57 as ::core::ffi::c_int) as isize,
                ) += s[19 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 58 as ::core::ffi::c_int) as isize,
                ) += s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 59 as ::core::ffi::c_int) as isize,
                ) += s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 60 as ::core::ffi::c_int) as isize,
                ) += s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 61 as ::core::ffi::c_int) as isize,
                ) += s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 62 as ::core::ffi::c_int) as isize,
                ) += s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 63 as ::core::ffi::c_int) as isize,
                ) += s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 64 as ::core::ffi::c_int) as isize,
                ) += s[48 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 65 as ::core::ffi::c_int) as isize,
                ) += s[51 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 66 as ::core::ffi::c_int) as isize,
                ) += s[46 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 67 as ::core::ffi::c_int) as isize,
                ) += s[49 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 68 as ::core::ffi::c_int) as isize,
                ) += s[52 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 69 as ::core::ffi::c_int) as isize,
                ) += s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 70 as ::core::ffi::c_int) as isize,
                ) += s[50 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 71 as ::core::ffi::c_int) as isize,
                ) += s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 72 as ::core::ffi::c_int) as isize,
                ) += s[72 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 73 as ::core::ffi::c_int) as isize,
                ) += s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 74 as ::core::ffi::c_int) as isize,
                ) += s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 75 as ::core::ffi::c_int) as isize,
                ) += s[73 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 76 as ::core::ffi::c_int) as isize,
                ) += s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 77 as ::core::ffi::c_int) as isize,
                ) += s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 78 as ::core::ffi::c_int) as isize,
                ) += s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 79 as ::core::ffi::c_int) as isize,
                ) += s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 80 as ::core::ffi::c_int) as isize,
                ) += s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_ipip1ipip2_optimizer(
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
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        81 as ::core::ffi::c_int,
    ];
    CINTall_2e_optimizer(
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
pub unsafe extern "C" fn int2e_ipip1ipip2_cart(
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
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        81 as ::core::ffi::c_int,
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
    CINTinit_int2e_EnvVars(
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
            CINTgout2e_int2e_ipip1ipip2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT2e_drv(
        out,
        dims,
        &raw mut envs,
        opt,
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
                c2s_cart_2e1
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
pub unsafe extern "C" fn int2e_ipip1ipip2_sph(
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
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        81 as ::core::ffi::c_int,
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
    CINTinit_int2e_EnvVars(
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
            CINTgout2e_int2e_ipip1ipip2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT2e_drv(
        out,
        dims,
        &raw mut envs,
        opt,
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
                c2s_sph_2e1
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
pub unsafe extern "C" fn cint2e_ipip1ipip2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipip1ipip2_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipip1ipip2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipip1ipip2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipip1ipip2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipip1ipip2_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipip1ipip2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipip1ipip2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipip1ipip2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipip1ipip2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_ipvip1ipvip2(
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
    let mut i: ::core::ffi::c_int = 0;
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
    let mut g8: *mut ::core::ffi::c_double = g7
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g9: *mut ::core::ffi::c_double = g8
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g10: *mut ::core::ffi::c_double = g9
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g11: *mut ::core::ffi::c_double = g10
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g12: *mut ::core::ffi::c_double = g11
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g13: *mut ::core::ffi::c_double = g12
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g14: *mut ::core::ffi::c_double = g13
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g15: *mut ::core::ffi::c_double = g14
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1l_2e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l + 1 as ::core::ffi::c_int,
        (*envs).l_l + 0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1k_2e(
        g2,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l,
        envs,
    );
    CINTnabla1k_2e(
        g3,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g4,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g5,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g6,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g7,
        g3,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g11,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g13,
        g5,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g14,
        g6,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    let mut s: [::core::ffi::c_double; 81] = [0.; 81];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 81 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g15.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[9 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[10 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[11 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[12 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[13 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[14 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[15 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[16 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[17 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[18 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[19 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[20 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[21 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[22 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[23 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[24 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[25 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[26 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[27 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[28 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[29 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[30 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[31 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[32 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[33 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[34 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[35 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[36 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[37 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[38 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[39 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[40 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[41 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[42 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[43 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[44 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[45 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[46 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[47 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[48 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[49 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[50 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[51 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[52 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[53 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[54 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[55 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[56 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[57 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[58 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[59 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[60 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[61 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[62 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[63 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[64 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[65 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[66 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[67 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[68 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[69 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[70 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[71 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[72 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[73 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[74 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[75 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[76 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[77 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[78 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[79 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[80 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[10 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = s[13 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = s[14 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) = s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) = s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) = s[18 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) = s[19 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) = s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) = s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) = s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) = s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) = s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) = s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) = s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                ) = s[27 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                ) = s[28 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                ) = s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                ) = s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                ) = s[31 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                ) = s[32 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                ) = s[33 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                ) = s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                ) = s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 36 as ::core::ffi::c_int) as isize,
                ) = s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 37 as ::core::ffi::c_int) as isize,
                ) = s[37 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 38 as ::core::ffi::c_int) as isize,
                ) = s[38 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 39 as ::core::ffi::c_int) as isize,
                ) = s[39 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 40 as ::core::ffi::c_int) as isize,
                ) = s[40 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 41 as ::core::ffi::c_int) as isize,
                ) = s[41 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 42 as ::core::ffi::c_int) as isize,
                ) = s[42 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 43 as ::core::ffi::c_int) as isize,
                ) = s[43 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 44 as ::core::ffi::c_int) as isize,
                ) = s[44 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 45 as ::core::ffi::c_int) as isize,
                ) = s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 46 as ::core::ffi::c_int) as isize,
                ) = s[46 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 47 as ::core::ffi::c_int) as isize,
                ) = s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 48 as ::core::ffi::c_int) as isize,
                ) = s[48 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 49 as ::core::ffi::c_int) as isize,
                ) = s[49 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 50 as ::core::ffi::c_int) as isize,
                ) = s[50 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 51 as ::core::ffi::c_int) as isize,
                ) = s[51 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 52 as ::core::ffi::c_int) as isize,
                ) = s[52 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 53 as ::core::ffi::c_int) as isize,
                ) = s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 54 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 55 as ::core::ffi::c_int) as isize,
                ) = s[55 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 56 as ::core::ffi::c_int) as isize,
                ) = s[56 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 57 as ::core::ffi::c_int) as isize,
                ) = s[57 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 58 as ::core::ffi::c_int) as isize,
                ) = s[58 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 59 as ::core::ffi::c_int) as isize,
                ) = s[59 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 60 as ::core::ffi::c_int) as isize,
                ) = s[60 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 61 as ::core::ffi::c_int) as isize,
                ) = s[61 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 62 as ::core::ffi::c_int) as isize,
                ) = s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 63 as ::core::ffi::c_int) as isize,
                ) = s[63 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 64 as ::core::ffi::c_int) as isize,
                ) = s[64 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 65 as ::core::ffi::c_int) as isize,
                ) = s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 66 as ::core::ffi::c_int) as isize,
                ) = s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 67 as ::core::ffi::c_int) as isize,
                ) = s[67 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 68 as ::core::ffi::c_int) as isize,
                ) = s[68 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 69 as ::core::ffi::c_int) as isize,
                ) = s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 70 as ::core::ffi::c_int) as isize,
                ) = s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 71 as ::core::ffi::c_int) as isize,
                ) = s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 72 as ::core::ffi::c_int) as isize,
                ) = s[72 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 73 as ::core::ffi::c_int) as isize,
                ) = s[73 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 74 as ::core::ffi::c_int) as isize,
                ) = s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 75 as ::core::ffi::c_int) as isize,
                ) = s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 76 as ::core::ffi::c_int) as isize,
                ) = s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 77 as ::core::ffi::c_int) as isize,
                ) = s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 78 as ::core::ffi::c_int) as isize,
                ) = s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 79 as ::core::ffi::c_int) as isize,
                ) = s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 80 as ::core::ffi::c_int) as isize,
                ) = s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) += s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) += s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) += s[4 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) += s[5 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) += s[6 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) += s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) += s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) += s[9 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) += s[10 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) += s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) += s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) += s[13 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) += s[14 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) += s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) += s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) += s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) += s[18 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) += s[19 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) += s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) += s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) += s[22 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) += s[23 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) += s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) += s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) += s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                ) += s[27 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                ) += s[28 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                ) += s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                ) += s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                ) += s[31 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                ) += s[32 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                ) += s[33 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                ) += s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                ) += s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 36 as ::core::ffi::c_int) as isize,
                ) += s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 37 as ::core::ffi::c_int) as isize,
                ) += s[37 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 38 as ::core::ffi::c_int) as isize,
                ) += s[38 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 39 as ::core::ffi::c_int) as isize,
                ) += s[39 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 40 as ::core::ffi::c_int) as isize,
                ) += s[40 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 41 as ::core::ffi::c_int) as isize,
                ) += s[41 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 42 as ::core::ffi::c_int) as isize,
                ) += s[42 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 43 as ::core::ffi::c_int) as isize,
                ) += s[43 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 44 as ::core::ffi::c_int) as isize,
                ) += s[44 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 45 as ::core::ffi::c_int) as isize,
                ) += s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 46 as ::core::ffi::c_int) as isize,
                ) += s[46 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 47 as ::core::ffi::c_int) as isize,
                ) += s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 48 as ::core::ffi::c_int) as isize,
                ) += s[48 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 49 as ::core::ffi::c_int) as isize,
                ) += s[49 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 50 as ::core::ffi::c_int) as isize,
                ) += s[50 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 51 as ::core::ffi::c_int) as isize,
                ) += s[51 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 52 as ::core::ffi::c_int) as isize,
                ) += s[52 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 53 as ::core::ffi::c_int) as isize,
                ) += s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 54 as ::core::ffi::c_int) as isize,
                ) += s[54 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 55 as ::core::ffi::c_int) as isize,
                ) += s[55 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 56 as ::core::ffi::c_int) as isize,
                ) += s[56 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 57 as ::core::ffi::c_int) as isize,
                ) += s[57 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 58 as ::core::ffi::c_int) as isize,
                ) += s[58 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 59 as ::core::ffi::c_int) as isize,
                ) += s[59 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 60 as ::core::ffi::c_int) as isize,
                ) += s[60 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 61 as ::core::ffi::c_int) as isize,
                ) += s[61 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 62 as ::core::ffi::c_int) as isize,
                ) += s[62 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 63 as ::core::ffi::c_int) as isize,
                ) += s[63 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 64 as ::core::ffi::c_int) as isize,
                ) += s[64 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 65 as ::core::ffi::c_int) as isize,
                ) += s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 66 as ::core::ffi::c_int) as isize,
                ) += s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 67 as ::core::ffi::c_int) as isize,
                ) += s[67 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 68 as ::core::ffi::c_int) as isize,
                ) += s[68 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 69 as ::core::ffi::c_int) as isize,
                ) += s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 70 as ::core::ffi::c_int) as isize,
                ) += s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 71 as ::core::ffi::c_int) as isize,
                ) += s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 72 as ::core::ffi::c_int) as isize,
                ) += s[72 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 73 as ::core::ffi::c_int) as isize,
                ) += s[73 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 74 as ::core::ffi::c_int) as isize,
                ) += s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 75 as ::core::ffi::c_int) as isize,
                ) += s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 76 as ::core::ffi::c_int) as isize,
                ) += s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 77 as ::core::ffi::c_int) as isize,
                ) += s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 78 as ::core::ffi::c_int) as isize,
                ) += s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 79 as ::core::ffi::c_int) as isize,
                ) += s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 81 as ::core::ffi::c_int + 80 as ::core::ffi::c_int) as isize,
                ) += s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_ipvip1ipvip2_optimizer(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        81 as ::core::ffi::c_int,
    ];
    CINTall_2e_optimizer(
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
pub unsafe extern "C" fn int2e_ipvip1ipvip2_cart(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        81 as ::core::ffi::c_int,
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
    CINTinit_int2e_EnvVars(
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
            CINTgout2e_int2e_ipvip1ipvip2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT2e_drv(
        out,
        dims,
        &raw mut envs,
        opt,
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
                c2s_cart_2e1
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
pub unsafe extern "C" fn int2e_ipvip1ipvip2_sph(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        81 as ::core::ffi::c_int,
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
    CINTinit_int2e_EnvVars(
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
            CINTgout2e_int2e_ipvip1ipvip2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    return CINT2e_drv(
        out,
        dims,
        &raw mut envs,
        opt,
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
                c2s_sph_2e1
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
pub unsafe extern "C" fn cint2e_ipvip1ipvip2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipvip1ipvip2_cart(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipvip1ipvip2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipvip1ipvip2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipvip1ipvip2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipvip1ipvip2_sph(
        out,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        shls,
        atm,
        natm,
        bas,
        nbas,
        env,
        opt,
        ::core::ptr::null_mut::<::core::ffi::c_double>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipvip1ipvip2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipvip1ipvip2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipvip1ipvip2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipvip1ipvip2_optimizer(opt, atm, natm, bas, nbas, env);
}
