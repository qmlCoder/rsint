extern "C" {
    fn c2s_sph_2e1(
        fijkl: *mut ::core::ffi::c_double,
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
    fn c2s_si_2e1(
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
    fn CINTall_2e_optimizer(
        opt: *mut *mut CINTOpt,
        ng: *mut ::core::ffi::c_int,
        atm: *mut ::core::ffi::c_int,
        natm: ::core::ffi::c_int,
        bas: *mut ::core::ffi::c_int,
        nbas: ::core::ffi::c_int,
        env: *mut ::core::ffi::c_double,
    );
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
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_ip1(
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
    CINTnabla1i_2e(
        g1,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
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
        match nrys_roots {
            1 => {
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
            }
            2 => {
                s[0 as ::core::ffi::c_int as usize] = *g1
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g1.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 1 as ::core::ffi::c_int) as isize);
                s[1 as ::core::ffi::c_int as usize] = *g0
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 1 as ::core::ffi::c_int) as isize);
                s[2 as ::core::ffi::c_int as usize] = *g0
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iz + 1 as ::core::ffi::c_int) as isize);
            }
            3 => {
                s[0 as ::core::ffi::c_int as usize] = *g1
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g1.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 1 as ::core::ffi::c_int) as isize)
                    + *g1.offset((ix + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 2 as ::core::ffi::c_int) as isize);
                s[1 as ::core::ffi::c_int as usize] = *g0
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 1 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 2 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iy + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 2 as ::core::ffi::c_int) as isize);
                s[2 as ::core::ffi::c_int as usize] = *g0
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iz + 1 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 2 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iz + 2 as ::core::ffi::c_int) as isize);
            }
            4 => {
                s[0 as ::core::ffi::c_int as usize] = *g1
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g1.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 1 as ::core::ffi::c_int) as isize)
                    + *g1.offset((ix + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 2 as ::core::ffi::c_int) as isize)
                    + *g1.offset((ix + 3 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 3 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 3 as ::core::ffi::c_int) as isize);
                s[1 as ::core::ffi::c_int as usize] = *g0
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 1 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 2 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iy + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 2 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 3 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iy + 3 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 3 as ::core::ffi::c_int) as isize);
                s[2 as ::core::ffi::c_int as usize] = *g0
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iz + 1 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 2 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iz + 2 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 3 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 3 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iz + 3 as ::core::ffi::c_int) as isize);
            }
            _ => {
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
            }
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[2 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_ip1_optimizer(
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
pub unsafe extern "C" fn int2e_ip1_cart(
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
            CINTgout2e_int2e_ip1
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
pub unsafe extern "C" fn int2e_ip1_sph(
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
            CINTgout2e_int2e_ip1
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
pub unsafe extern "C" fn cint2e_ip1_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ip1_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ip1_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ip1_sph(
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
pub unsafe extern "C" fn cint2e_ip1_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ip1_cart(
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
pub unsafe extern "C" fn cint2e_ip1_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ip1_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ip1_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ip1_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_ip2(
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
    CINTnabla1k_2e(
        g1,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l,
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
        match nrys_roots {
            1 => {
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
            }
            2 => {
                s[0 as ::core::ffi::c_int as usize] = *g1
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g1.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 1 as ::core::ffi::c_int) as isize);
                s[1 as ::core::ffi::c_int as usize] = *g0
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 1 as ::core::ffi::c_int) as isize);
                s[2 as ::core::ffi::c_int as usize] = *g0
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iz + 1 as ::core::ffi::c_int) as isize);
            }
            3 => {
                s[0 as ::core::ffi::c_int as usize] = *g1
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g1.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 1 as ::core::ffi::c_int) as isize)
                    + *g1.offset((ix + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 2 as ::core::ffi::c_int) as isize);
                s[1 as ::core::ffi::c_int as usize] = *g0
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 1 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 2 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iy + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 2 as ::core::ffi::c_int) as isize);
                s[2 as ::core::ffi::c_int as usize] = *g0
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iz + 1 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 2 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iz + 2 as ::core::ffi::c_int) as isize);
            }
            4 => {
                s[0 as ::core::ffi::c_int as usize] = *g1
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g1.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 1 as ::core::ffi::c_int) as isize)
                    + *g1.offset((ix + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 2 as ::core::ffi::c_int) as isize)
                    + *g1.offset((ix + 3 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 3 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 3 as ::core::ffi::c_int) as isize);
                s[1 as ::core::ffi::c_int as usize] = *g0
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g1.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 1 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 2 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iy + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 2 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 3 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iy + 3 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iz + 3 as ::core::ffi::c_int) as isize);
                s[2 as ::core::ffi::c_int as usize] = *g0
                    .offset((ix + 0 as ::core::ffi::c_int) as isize)
                    * *g0.offset((iy + 0 as ::core::ffi::c_int) as isize)
                    * *g1.offset((iz + 0 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 1 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 1 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iz + 1 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 2 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 2 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iz + 2 as ::core::ffi::c_int) as isize)
                    + *g0.offset((ix + 3 as ::core::ffi::c_int) as isize)
                        * *g0.offset((iy + 3 as ::core::ffi::c_int) as isize)
                        * *g1.offset((iz + 3 as ::core::ffi::c_int) as isize);
            }
            _ => {
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
            }
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset((n * 3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                += s[0 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                += s[1 as ::core::ffi::c_int as usize];
            *gout
                .offset((n * 3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                += s[2 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_ip2_optimizer(
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
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int2e_ip2_cart(
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
        1 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_ip2
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
pub unsafe extern "C" fn int2e_ip2_sph(
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
        1 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_ip2
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
pub unsafe extern "C" fn cint2e_ip2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ip2_cart(
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
pub unsafe extern "C" fn cint2e_ip2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ip2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ip2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ip2_sph(
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
pub unsafe extern "C" fn cint2e_ip2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ip2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ip2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ip2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_ipspsp1(
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
    CINTnabla1j_2e(
        g1,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g2,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g3,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g4,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
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
                ) = s[11 as ::core::ffi::c_int as usize]
                - s[19 as ::core::ffi::c_int as usize];
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
                ) = s[21 as ::core::ffi::c_int as usize]
                - s[5 as ::core::ffi::c_int as usize];
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
                ) = s[7 as ::core::ffi::c_int as usize]
                - s[15 as ::core::ffi::c_int as usize];
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
                )
                += s[11 as ::core::ffi::c_int as usize]
                    - s[19 as ::core::ffi::c_int as usize];
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
                )
                += s[21 as ::core::ffi::c_int as usize]
                    - s[5 as ::core::ffi::c_int as usize];
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
                )
                += s[7 as ::core::ffi::c_int as usize]
                    - s[15 as ::core::ffi::c_int as usize];
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
pub unsafe extern "C" fn int2e_ipspsp1_optimizer(
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
pub unsafe extern "C" fn int2e_ipspsp1_cart(
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
            CINTgout2e_int2e_ipspsp1
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
pub unsafe extern "C" fn int2e_ipspsp1_sph(
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
            CINTgout2e_int2e_ipspsp1
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
pub unsafe extern "C" fn cint2e_ipspsp1_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipspsp1_cart(
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
pub unsafe extern "C" fn cint2e_ipspsp1_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipspsp1_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipspsp1_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipspsp1_sph(
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
pub unsafe extern "C" fn cint2e_ipspsp1_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipspsp1_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipspsp1_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipspsp1_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_ip1spsp2(
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
    CINTnabla1l_2e(
        g1,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l + 1 as ::core::ffi::c_int,
        (*envs).l_l + 0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1k_2e(
        g2,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l,
        envs,
    );
    CINTnabla1k_2e(
        g3,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g4,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
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
                ) = s[5 as ::core::ffi::c_int as usize]
                - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize]
                - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                - s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[14 as ::core::ffi::c_int as usize]
                - s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[15 as ::core::ffi::c_int as usize]
                - s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[10 as ::core::ffi::c_int as usize]
                - s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize]
                + s[13 as ::core::ffi::c_int as usize]
                + s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[23 as ::core::ffi::c_int as usize]
                - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[24 as ::core::ffi::c_int as usize]
                - s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[19 as ::core::ffi::c_int as usize]
                - s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[18 as ::core::ffi::c_int as usize]
                + s[22 as ::core::ffi::c_int as usize]
                + s[26 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[5 as ::core::ffi::c_int as usize]
                    - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += s[6 as ::core::ffi::c_int as usize]
                    - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += s[1 as ::core::ffi::c_int as usize]
                    - s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += s[14 as ::core::ffi::c_int as usize]
                    - s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[15 as ::core::ffi::c_int as usize]
                    - s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += s[10 as ::core::ffi::c_int as usize]
                    - s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += s[9 as ::core::ffi::c_int as usize]
                    + s[13 as ::core::ffi::c_int as usize]
                    + s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += s[23 as ::core::ffi::c_int as usize]
                    - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += s[24 as ::core::ffi::c_int as usize]
                    - s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[19 as ::core::ffi::c_int as usize]
                    - s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[18 as ::core::ffi::c_int as usize]
                    + s[22 as ::core::ffi::c_int as usize]
                    + s[26 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_ip1spsp2_optimizer(
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
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int2e_ip1spsp2_cart(
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
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_ip1spsp2
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
pub unsafe extern "C" fn int2e_ip1spsp2_sph(
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
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_ip1spsp2
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
pub unsafe extern "C" fn cint2e_ip1spsp2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ip1spsp2_cart(
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
pub unsafe extern "C" fn cint2e_ip1spsp2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ip1spsp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ip1spsp2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ip1spsp2_sph(
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
pub unsafe extern "C" fn cint2e_ip1spsp2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ip1spsp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ip1spsp2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ip1spsp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_ipspsp1spsp2(
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
    let mut g16: *mut ::core::ffi::c_double = g15
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g17: *mut ::core::ffi::c_double = g16
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g18: *mut ::core::ffi::c_double = g17
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g19: *mut ::core::ffi::c_double = g18
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g20: *mut ::core::ffi::c_double = g19
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g21: *mut ::core::ffi::c_double = g20
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g22: *mut ::core::ffi::c_double = g21
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g23: *mut ::core::ffi::c_double = g22
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g24: *mut ::core::ffi::c_double = g23
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g25: *mut ::core::ffi::c_double = g24
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g26: *mut ::core::ffi::c_double = g25
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g27: *mut ::core::ffi::c_double = g26
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g28: *mut ::core::ffi::c_double = g27
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g29: *mut ::core::ffi::c_double = g28
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g30: *mut ::core::ffi::c_double = g29
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g31: *mut ::core::ffi::c_double = g30
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    CINTnabla1l_2e(
        g1,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l + 1 as ::core::ffi::c_int,
        (*envs).l_l + 0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1k_2e(
        g2,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l,
        envs,
    );
    CINTnabla1k_2e(
        g3,
        g1,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g4,
        g0,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g5,
        g1,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g6,
        g2,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g7,
        g3,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g8,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g11,
        g3,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g13,
        g5,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g14,
        g6,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g16,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g17,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g18,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g19,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g20,
        g4,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g21,
        g5,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g22,
        g6,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g23,
        g7,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g24,
        g8,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g25,
        g9,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g26,
        g10,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g27,
        g11,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g28,
        g12,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g29,
        g13,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g30,
        g14,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g31,
        g15,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    let mut s: [::core::ffi::c_double; 243] = [0.; 243];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 243 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g31.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g30.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g30.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g29.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g28.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g28.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g29.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g28.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g28.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[9 as ::core::ffi::c_int as usize]
                += *g27.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[10 as ::core::ffi::c_int as usize]
                += *g26.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[11 as ::core::ffi::c_int as usize]
                += *g26.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[12 as ::core::ffi::c_int as usize]
                += *g25.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[13 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[14 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[15 as ::core::ffi::c_int as usize]
                += *g25.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[16 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[17 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[18 as ::core::ffi::c_int as usize]
                += *g27.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[19 as ::core::ffi::c_int as usize]
                += *g26.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[20 as ::core::ffi::c_int as usize]
                += *g26.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[21 as ::core::ffi::c_int as usize]
                += *g25.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[22 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[23 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[24 as ::core::ffi::c_int as usize]
                += *g25.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[25 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[26 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[27 as ::core::ffi::c_int as usize]
                += *g23.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[28 as ::core::ffi::c_int as usize]
                += *g22.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[29 as ::core::ffi::c_int as usize]
                += *g22.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[30 as ::core::ffi::c_int as usize]
                += *g21.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[31 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[32 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[33 as ::core::ffi::c_int as usize]
                += *g21.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[34 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[35 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[36 as ::core::ffi::c_int as usize]
                += *g19.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[37 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[38 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[39 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[40 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[41 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[42 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[43 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[44 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[45 as ::core::ffi::c_int as usize]
                += *g19.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[46 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[47 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[48 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[49 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[50 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[51 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[52 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[53 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[54 as ::core::ffi::c_int as usize]
                += *g23.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[55 as ::core::ffi::c_int as usize]
                += *g22.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[56 as ::core::ffi::c_int as usize]
                += *g22.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[57 as ::core::ffi::c_int as usize]
                += *g21.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[58 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[59 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[60 as ::core::ffi::c_int as usize]
                += *g21.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[61 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[62 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[63 as ::core::ffi::c_int as usize]
                += *g19.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[64 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[65 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[66 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[67 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[68 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[69 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[70 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[71 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[72 as ::core::ffi::c_int as usize]
                += *g19.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[73 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[74 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[75 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[76 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[77 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[78 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[79 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[80 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            s[81 as ::core::ffi::c_int as usize]
                += *g15.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[82 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[83 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[84 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[85 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g19.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[86 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[87 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[88 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[89 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[90 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[91 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g21.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[92 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[93 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g22.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[94 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g23.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[95 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g22.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[96 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[97 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g21.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[98 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[99 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[100 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[101 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[102 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[103 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g19.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[104 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[105 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[106 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[107 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[108 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[109 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g25.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[110 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[111 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g26.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[112 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g27.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[113 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g26.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[114 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[115 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g25.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[116 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[117 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g28.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[118 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g29.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[119 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g28.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[120 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g30.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[121 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g31.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[122 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g30.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[123 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g28.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[124 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g29.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[125 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g28.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[126 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[127 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g25.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[128 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[129 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g26.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[130 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g27.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[131 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g26.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[132 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[133 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g25.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[134 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[135 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[136 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[137 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[138 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[139 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g19.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[140 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[141 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[142 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[143 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[144 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[145 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g21.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[146 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[147 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g22.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[148 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g23.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[149 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g22.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[150 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[151 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g21.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[152 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[153 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[154 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[155 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[156 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[157 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g19.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[158 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[159 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[160 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[161 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            s[162 as ::core::ffi::c_int as usize]
                += *g15.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[163 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[164 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[165 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[166 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[167 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[168 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[169 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[170 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g19.offset((iz + i) as isize);
            s[171 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[172 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[173 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[174 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[175 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[176 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[177 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[178 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[179 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g19.offset((iz + i) as isize);
            s[180 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[181 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[182 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g21.offset((iz + i) as isize);
            s[183 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[184 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[185 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g21.offset((iz + i) as isize);
            s[186 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g22.offset((iz + i) as isize);
            s[187 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g22.offset((iz + i) as isize);
            s[188 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g23.offset((iz + i) as isize);
            s[189 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[190 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[191 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[192 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[193 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[194 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[195 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[196 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[197 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g19.offset((iz + i) as isize);
            s[198 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[199 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[200 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[201 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[202 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[203 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[204 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[205 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[206 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g19.offset((iz + i) as isize);
            s[207 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[208 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[209 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g21.offset((iz + i) as isize);
            s[210 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[211 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[212 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g21.offset((iz + i) as isize);
            s[213 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g22.offset((iz + i) as isize);
            s[214 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g22.offset((iz + i) as isize);
            s[215 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g23.offset((iz + i) as isize);
            s[216 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[217 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[218 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g25.offset((iz + i) as isize);
            s[219 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[220 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[221 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g25.offset((iz + i) as isize);
            s[222 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g26.offset((iz + i) as isize);
            s[223 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g26.offset((iz + i) as isize);
            s[224 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g27.offset((iz + i) as isize);
            s[225 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[226 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[227 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g25.offset((iz + i) as isize);
            s[228 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[229 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[230 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g25.offset((iz + i) as isize);
            s[231 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g26.offset((iz + i) as isize);
            s[232 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g26.offset((iz + i) as isize);
            s[233 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g27.offset((iz + i) as isize);
            s[234 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g28.offset((iz + i) as isize);
            s[235 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g28.offset((iz + i) as isize);
            s[236 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g29.offset((iz + i) as isize);
            s[237 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g28.offset((iz + i) as isize);
            s[238 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g28.offset((iz + i) as isize);
            s[239 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g29.offset((iz + i) as isize);
            s[240 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g30.offset((iz + i) as isize);
            s[241 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g30.offset((iz + i) as isize);
            s[242 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g31.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[104 as ::core::ffi::c_int as usize]
                - s[176 as ::core::ffi::c_int as usize]
                - s[106 as ::core::ffi::c_int as usize]
                + s[178 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[167 as ::core::ffi::c_int as usize]
                - s[23 as ::core::ffi::c_int as usize]
                - s[169 as ::core::ffi::c_int as usize]
                + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[14 as ::core::ffi::c_int as usize]
                - s[86 as ::core::ffi::c_int as usize]
                - s[16 as ::core::ffi::c_int as usize]
                + s[88 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize]
                + s[95 as ::core::ffi::c_int as usize]
                + s[185 as ::core::ffi::c_int as usize]
                - s[7 as ::core::ffi::c_int as usize]
                - s[97 as ::core::ffi::c_int as usize]
                - s[187 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[105 as ::core::ffi::c_int as usize]
                - s[177 as ::core::ffi::c_int as usize]
                - s[101 as ::core::ffi::c_int as usize]
                + s[173 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[168 as ::core::ffi::c_int as usize]
                - s[24 as ::core::ffi::c_int as usize]
                - s[164 as ::core::ffi::c_int as usize]
                + s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[15 as ::core::ffi::c_int as usize]
                - s[87 as ::core::ffi::c_int as usize]
                - s[11 as ::core::ffi::c_int as usize]
                + s[83 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize]
                + s[96 as ::core::ffi::c_int as usize]
                + s[186 as ::core::ffi::c_int as usize]
                - s[2 as ::core::ffi::c_int as usize]
                - s[92 as ::core::ffi::c_int as usize]
                - s[182 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[100 as ::core::ffi::c_int as usize]
                - s[172 as ::core::ffi::c_int as usize]
                - s[102 as ::core::ffi::c_int as usize]
                + s[174 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[163 as ::core::ffi::c_int as usize]
                - s[19 as ::core::ffi::c_int as usize]
                - s[165 as ::core::ffi::c_int as usize]
                + s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[10 as ::core::ffi::c_int as usize]
                - s[82 as ::core::ffi::c_int as usize]
                - s[12 as ::core::ffi::c_int as usize]
                + s[84 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                + s[91 as ::core::ffi::c_int as usize]
                + s[181 as ::core::ffi::c_int as usize]
                - s[3 as ::core::ffi::c_int as usize]
                - s[93 as ::core::ffi::c_int as usize]
                - s[183 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = s[99 as ::core::ffi::c_int as usize]
                - s[171 as ::core::ffi::c_int as usize]
                + s[103 as ::core::ffi::c_int as usize]
                - s[175 as ::core::ffi::c_int as usize]
                + s[107 as ::core::ffi::c_int as usize]
                - s[179 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = s[162 as ::core::ffi::c_int as usize]
                - s[18 as ::core::ffi::c_int as usize]
                + s[166 as ::core::ffi::c_int as usize]
                - s[22 as ::core::ffi::c_int as usize]
                + s[170 as ::core::ffi::c_int as usize]
                - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize]
                - s[81 as ::core::ffi::c_int as usize]
                + s[13 as ::core::ffi::c_int as usize]
                - s[85 as ::core::ffi::c_int as usize]
                + s[17 as ::core::ffi::c_int as usize]
                - s[89 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[90 as ::core::ffi::c_int as usize]
                + s[180 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize]
                + s[94 as ::core::ffi::c_int as usize]
                + s[184 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize]
                + s[98 as ::core::ffi::c_int as usize]
                + s[188 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) = s[131 as ::core::ffi::c_int as usize]
                - s[203 as ::core::ffi::c_int as usize]
                - s[133 as ::core::ffi::c_int as usize]
                + s[205 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) = s[194 as ::core::ffi::c_int as usize]
                - s[50 as ::core::ffi::c_int as usize]
                - s[196 as ::core::ffi::c_int as usize]
                + s[52 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) = s[41 as ::core::ffi::c_int as usize]
                - s[113 as ::core::ffi::c_int as usize]
                - s[43 as ::core::ffi::c_int as usize]
                + s[115 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) = s[32 as ::core::ffi::c_int as usize]
                + s[122 as ::core::ffi::c_int as usize]
                + s[212 as ::core::ffi::c_int as usize]
                - s[34 as ::core::ffi::c_int as usize]
                - s[124 as ::core::ffi::c_int as usize]
                - s[214 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) = s[132 as ::core::ffi::c_int as usize]
                - s[204 as ::core::ffi::c_int as usize]
                - s[128 as ::core::ffi::c_int as usize]
                + s[200 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) = s[195 as ::core::ffi::c_int as usize]
                - s[51 as ::core::ffi::c_int as usize]
                - s[191 as ::core::ffi::c_int as usize]
                + s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) = s[42 as ::core::ffi::c_int as usize]
                - s[114 as ::core::ffi::c_int as usize]
                - s[38 as ::core::ffi::c_int as usize]
                + s[110 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) = s[33 as ::core::ffi::c_int as usize]
                + s[123 as ::core::ffi::c_int as usize]
                + s[213 as ::core::ffi::c_int as usize]
                - s[29 as ::core::ffi::c_int as usize]
                - s[119 as ::core::ffi::c_int as usize]
                - s[209 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) = s[127 as ::core::ffi::c_int as usize]
                - s[199 as ::core::ffi::c_int as usize]
                - s[129 as ::core::ffi::c_int as usize]
                + s[201 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) = s[190 as ::core::ffi::c_int as usize]
                - s[46 as ::core::ffi::c_int as usize]
                - s[192 as ::core::ffi::c_int as usize]
                + s[48 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) = s[37 as ::core::ffi::c_int as usize]
                - s[109 as ::core::ffi::c_int as usize]
                - s[39 as ::core::ffi::c_int as usize]
                + s[111 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                ) = s[28 as ::core::ffi::c_int as usize]
                + s[118 as ::core::ffi::c_int as usize]
                + s[208 as ::core::ffi::c_int as usize]
                - s[30 as ::core::ffi::c_int as usize]
                - s[120 as ::core::ffi::c_int as usize]
                - s[210 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                ) = s[126 as ::core::ffi::c_int as usize]
                - s[198 as ::core::ffi::c_int as usize]
                + s[130 as ::core::ffi::c_int as usize]
                - s[202 as ::core::ffi::c_int as usize]
                + s[134 as ::core::ffi::c_int as usize]
                - s[206 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                ) = s[189 as ::core::ffi::c_int as usize]
                - s[45 as ::core::ffi::c_int as usize]
                + s[193 as ::core::ffi::c_int as usize]
                - s[49 as ::core::ffi::c_int as usize]
                + s[197 as ::core::ffi::c_int as usize]
                - s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                ) = s[36 as ::core::ffi::c_int as usize]
                - s[108 as ::core::ffi::c_int as usize]
                + s[40 as ::core::ffi::c_int as usize]
                - s[112 as ::core::ffi::c_int as usize]
                + s[44 as ::core::ffi::c_int as usize]
                - s[116 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                ) = s[27 as ::core::ffi::c_int as usize]
                + s[117 as ::core::ffi::c_int as usize]
                + s[207 as ::core::ffi::c_int as usize]
                + s[31 as ::core::ffi::c_int as usize]
                + s[121 as ::core::ffi::c_int as usize]
                + s[211 as ::core::ffi::c_int as usize]
                + s[35 as ::core::ffi::c_int as usize]
                + s[125 as ::core::ffi::c_int as usize]
                + s[215 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                ) = s[158 as ::core::ffi::c_int as usize]
                - s[230 as ::core::ffi::c_int as usize]
                - s[160 as ::core::ffi::c_int as usize]
                + s[232 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                ) = s[221 as ::core::ffi::c_int as usize]
                - s[77 as ::core::ffi::c_int as usize]
                - s[223 as ::core::ffi::c_int as usize]
                + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                ) = s[68 as ::core::ffi::c_int as usize]
                - s[140 as ::core::ffi::c_int as usize]
                - s[70 as ::core::ffi::c_int as usize]
                + s[142 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                ) = s[59 as ::core::ffi::c_int as usize]
                + s[149 as ::core::ffi::c_int as usize]
                + s[239 as ::core::ffi::c_int as usize]
                - s[61 as ::core::ffi::c_int as usize]
                - s[151 as ::core::ffi::c_int as usize]
                - s[241 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 36 as ::core::ffi::c_int) as isize,
                ) = s[159 as ::core::ffi::c_int as usize]
                - s[231 as ::core::ffi::c_int as usize]
                - s[155 as ::core::ffi::c_int as usize]
                + s[227 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 37 as ::core::ffi::c_int) as isize,
                ) = s[222 as ::core::ffi::c_int as usize]
                - s[78 as ::core::ffi::c_int as usize]
                - s[218 as ::core::ffi::c_int as usize]
                + s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 38 as ::core::ffi::c_int) as isize,
                ) = s[69 as ::core::ffi::c_int as usize]
                - s[141 as ::core::ffi::c_int as usize]
                - s[65 as ::core::ffi::c_int as usize]
                + s[137 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 39 as ::core::ffi::c_int) as isize,
                ) = s[60 as ::core::ffi::c_int as usize]
                + s[150 as ::core::ffi::c_int as usize]
                + s[240 as ::core::ffi::c_int as usize]
                - s[56 as ::core::ffi::c_int as usize]
                - s[146 as ::core::ffi::c_int as usize]
                - s[236 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 40 as ::core::ffi::c_int) as isize,
                ) = s[154 as ::core::ffi::c_int as usize]
                - s[226 as ::core::ffi::c_int as usize]
                - s[156 as ::core::ffi::c_int as usize]
                + s[228 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 41 as ::core::ffi::c_int) as isize,
                ) = s[217 as ::core::ffi::c_int as usize]
                - s[73 as ::core::ffi::c_int as usize]
                - s[219 as ::core::ffi::c_int as usize]
                + s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 42 as ::core::ffi::c_int) as isize,
                ) = s[64 as ::core::ffi::c_int as usize]
                - s[136 as ::core::ffi::c_int as usize]
                - s[66 as ::core::ffi::c_int as usize]
                + s[138 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 43 as ::core::ffi::c_int) as isize,
                ) = s[55 as ::core::ffi::c_int as usize]
                + s[145 as ::core::ffi::c_int as usize]
                + s[235 as ::core::ffi::c_int as usize]
                - s[57 as ::core::ffi::c_int as usize]
                - s[147 as ::core::ffi::c_int as usize]
                - s[237 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 44 as ::core::ffi::c_int) as isize,
                ) = s[153 as ::core::ffi::c_int as usize]
                - s[225 as ::core::ffi::c_int as usize]
                + s[157 as ::core::ffi::c_int as usize]
                - s[229 as ::core::ffi::c_int as usize]
                + s[161 as ::core::ffi::c_int as usize]
                - s[233 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 45 as ::core::ffi::c_int) as isize,
                ) = s[216 as ::core::ffi::c_int as usize]
                - s[72 as ::core::ffi::c_int as usize]
                + s[220 as ::core::ffi::c_int as usize]
                - s[76 as ::core::ffi::c_int as usize]
                + s[224 as ::core::ffi::c_int as usize]
                - s[80 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 46 as ::core::ffi::c_int) as isize,
                ) = s[63 as ::core::ffi::c_int as usize]
                - s[135 as ::core::ffi::c_int as usize]
                + s[67 as ::core::ffi::c_int as usize]
                - s[139 as ::core::ffi::c_int as usize]
                + s[71 as ::core::ffi::c_int as usize]
                - s[143 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 47 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize]
                + s[144 as ::core::ffi::c_int as usize]
                + s[234 as ::core::ffi::c_int as usize]
                + s[58 as ::core::ffi::c_int as usize]
                + s[148 as ::core::ffi::c_int as usize]
                + s[238 as ::core::ffi::c_int as usize]
                + s[62 as ::core::ffi::c_int as usize]
                + s[152 as ::core::ffi::c_int as usize]
                + s[242 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[104 as ::core::ffi::c_int as usize]
                    - s[176 as ::core::ffi::c_int as usize]
                    - s[106 as ::core::ffi::c_int as usize]
                    + s[178 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += s[167 as ::core::ffi::c_int as usize]
                    - s[23 as ::core::ffi::c_int as usize]
                    - s[169 as ::core::ffi::c_int as usize]
                    + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += s[14 as ::core::ffi::c_int as usize]
                    - s[86 as ::core::ffi::c_int as usize]
                    - s[16 as ::core::ffi::c_int as usize]
                    + s[88 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[5 as ::core::ffi::c_int as usize]
                    + s[95 as ::core::ffi::c_int as usize]
                    + s[185 as ::core::ffi::c_int as usize]
                    - s[7 as ::core::ffi::c_int as usize]
                    - s[97 as ::core::ffi::c_int as usize]
                    - s[187 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += s[105 as ::core::ffi::c_int as usize]
                    - s[177 as ::core::ffi::c_int as usize]
                    - s[101 as ::core::ffi::c_int as usize]
                    + s[173 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[168 as ::core::ffi::c_int as usize]
                    - s[24 as ::core::ffi::c_int as usize]
                    - s[164 as ::core::ffi::c_int as usize]
                    + s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += s[15 as ::core::ffi::c_int as usize]
                    - s[87 as ::core::ffi::c_int as usize]
                    - s[11 as ::core::ffi::c_int as usize]
                    + s[83 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += s[6 as ::core::ffi::c_int as usize]
                    + s[96 as ::core::ffi::c_int as usize]
                    + s[186 as ::core::ffi::c_int as usize]
                    - s[2 as ::core::ffi::c_int as usize]
                    - s[92 as ::core::ffi::c_int as usize]
                    - s[182 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += s[100 as ::core::ffi::c_int as usize]
                    - s[172 as ::core::ffi::c_int as usize]
                    - s[102 as ::core::ffi::c_int as usize]
                    + s[174 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += s[163 as ::core::ffi::c_int as usize]
                    - s[19 as ::core::ffi::c_int as usize]
                    - s[165 as ::core::ffi::c_int as usize]
                    + s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[10 as ::core::ffi::c_int as usize]
                    - s[82 as ::core::ffi::c_int as usize]
                    - s[12 as ::core::ffi::c_int as usize]
                    + s[84 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[1 as ::core::ffi::c_int as usize]
                    + s[91 as ::core::ffi::c_int as usize]
                    + s[181 as ::core::ffi::c_int as usize]
                    - s[3 as ::core::ffi::c_int as usize]
                    - s[93 as ::core::ffi::c_int as usize]
                    - s[183 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                )
                += s[99 as ::core::ffi::c_int as usize]
                    - s[171 as ::core::ffi::c_int as usize]
                    + s[103 as ::core::ffi::c_int as usize]
                    - s[175 as ::core::ffi::c_int as usize]
                    + s[107 as ::core::ffi::c_int as usize]
                    - s[179 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                )
                += s[162 as ::core::ffi::c_int as usize]
                    - s[18 as ::core::ffi::c_int as usize]
                    + s[166 as ::core::ffi::c_int as usize]
                    - s[22 as ::core::ffi::c_int as usize]
                    + s[170 as ::core::ffi::c_int as usize]
                    - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                )
                += s[9 as ::core::ffi::c_int as usize]
                    - s[81 as ::core::ffi::c_int as usize]
                    + s[13 as ::core::ffi::c_int as usize]
                    - s[85 as ::core::ffi::c_int as usize]
                    + s[17 as ::core::ffi::c_int as usize]
                    - s[89 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[90 as ::core::ffi::c_int as usize]
                    + s[180 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize]
                    + s[94 as ::core::ffi::c_int as usize]
                    + s[184 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize]
                    + s[98 as ::core::ffi::c_int as usize]
                    + s[188 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                )
                += s[131 as ::core::ffi::c_int as usize]
                    - s[203 as ::core::ffi::c_int as usize]
                    - s[133 as ::core::ffi::c_int as usize]
                    + s[205 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                )
                += s[194 as ::core::ffi::c_int as usize]
                    - s[50 as ::core::ffi::c_int as usize]
                    - s[196 as ::core::ffi::c_int as usize]
                    + s[52 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                )
                += s[41 as ::core::ffi::c_int as usize]
                    - s[113 as ::core::ffi::c_int as usize]
                    - s[43 as ::core::ffi::c_int as usize]
                    + s[115 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                )
                += s[32 as ::core::ffi::c_int as usize]
                    + s[122 as ::core::ffi::c_int as usize]
                    + s[212 as ::core::ffi::c_int as usize]
                    - s[34 as ::core::ffi::c_int as usize]
                    - s[124 as ::core::ffi::c_int as usize]
                    - s[214 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                )
                += s[132 as ::core::ffi::c_int as usize]
                    - s[204 as ::core::ffi::c_int as usize]
                    - s[128 as ::core::ffi::c_int as usize]
                    + s[200 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                )
                += s[195 as ::core::ffi::c_int as usize]
                    - s[51 as ::core::ffi::c_int as usize]
                    - s[191 as ::core::ffi::c_int as usize]
                    + s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                )
                += s[42 as ::core::ffi::c_int as usize]
                    - s[114 as ::core::ffi::c_int as usize]
                    - s[38 as ::core::ffi::c_int as usize]
                    + s[110 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                )
                += s[33 as ::core::ffi::c_int as usize]
                    + s[123 as ::core::ffi::c_int as usize]
                    + s[213 as ::core::ffi::c_int as usize]
                    - s[29 as ::core::ffi::c_int as usize]
                    - s[119 as ::core::ffi::c_int as usize]
                    - s[209 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                )
                += s[127 as ::core::ffi::c_int as usize]
                    - s[199 as ::core::ffi::c_int as usize]
                    - s[129 as ::core::ffi::c_int as usize]
                    + s[201 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                )
                += s[190 as ::core::ffi::c_int as usize]
                    - s[46 as ::core::ffi::c_int as usize]
                    - s[192 as ::core::ffi::c_int as usize]
                    + s[48 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                )
                += s[37 as ::core::ffi::c_int as usize]
                    - s[109 as ::core::ffi::c_int as usize]
                    - s[39 as ::core::ffi::c_int as usize]
                    + s[111 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                )
                += s[28 as ::core::ffi::c_int as usize]
                    + s[118 as ::core::ffi::c_int as usize]
                    + s[208 as ::core::ffi::c_int as usize]
                    - s[30 as ::core::ffi::c_int as usize]
                    - s[120 as ::core::ffi::c_int as usize]
                    - s[210 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                )
                += s[126 as ::core::ffi::c_int as usize]
                    - s[198 as ::core::ffi::c_int as usize]
                    + s[130 as ::core::ffi::c_int as usize]
                    - s[202 as ::core::ffi::c_int as usize]
                    + s[134 as ::core::ffi::c_int as usize]
                    - s[206 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                )
                += s[189 as ::core::ffi::c_int as usize]
                    - s[45 as ::core::ffi::c_int as usize]
                    + s[193 as ::core::ffi::c_int as usize]
                    - s[49 as ::core::ffi::c_int as usize]
                    + s[197 as ::core::ffi::c_int as usize]
                    - s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                )
                += s[36 as ::core::ffi::c_int as usize]
                    - s[108 as ::core::ffi::c_int as usize]
                    + s[40 as ::core::ffi::c_int as usize]
                    - s[112 as ::core::ffi::c_int as usize]
                    + s[44 as ::core::ffi::c_int as usize]
                    - s[116 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                )
                += s[27 as ::core::ffi::c_int as usize]
                    + s[117 as ::core::ffi::c_int as usize]
                    + s[207 as ::core::ffi::c_int as usize]
                    + s[31 as ::core::ffi::c_int as usize]
                    + s[121 as ::core::ffi::c_int as usize]
                    + s[211 as ::core::ffi::c_int as usize]
                    + s[35 as ::core::ffi::c_int as usize]
                    + s[125 as ::core::ffi::c_int as usize]
                    + s[215 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                )
                += s[158 as ::core::ffi::c_int as usize]
                    - s[230 as ::core::ffi::c_int as usize]
                    - s[160 as ::core::ffi::c_int as usize]
                    + s[232 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                )
                += s[221 as ::core::ffi::c_int as usize]
                    - s[77 as ::core::ffi::c_int as usize]
                    - s[223 as ::core::ffi::c_int as usize]
                    + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                )
                += s[68 as ::core::ffi::c_int as usize]
                    - s[140 as ::core::ffi::c_int as usize]
                    - s[70 as ::core::ffi::c_int as usize]
                    + s[142 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                )
                += s[59 as ::core::ffi::c_int as usize]
                    + s[149 as ::core::ffi::c_int as usize]
                    + s[239 as ::core::ffi::c_int as usize]
                    - s[61 as ::core::ffi::c_int as usize]
                    - s[151 as ::core::ffi::c_int as usize]
                    - s[241 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 36 as ::core::ffi::c_int) as isize,
                )
                += s[159 as ::core::ffi::c_int as usize]
                    - s[231 as ::core::ffi::c_int as usize]
                    - s[155 as ::core::ffi::c_int as usize]
                    + s[227 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 37 as ::core::ffi::c_int) as isize,
                )
                += s[222 as ::core::ffi::c_int as usize]
                    - s[78 as ::core::ffi::c_int as usize]
                    - s[218 as ::core::ffi::c_int as usize]
                    + s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 38 as ::core::ffi::c_int) as isize,
                )
                += s[69 as ::core::ffi::c_int as usize]
                    - s[141 as ::core::ffi::c_int as usize]
                    - s[65 as ::core::ffi::c_int as usize]
                    + s[137 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 39 as ::core::ffi::c_int) as isize,
                )
                += s[60 as ::core::ffi::c_int as usize]
                    + s[150 as ::core::ffi::c_int as usize]
                    + s[240 as ::core::ffi::c_int as usize]
                    - s[56 as ::core::ffi::c_int as usize]
                    - s[146 as ::core::ffi::c_int as usize]
                    - s[236 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 40 as ::core::ffi::c_int) as isize,
                )
                += s[154 as ::core::ffi::c_int as usize]
                    - s[226 as ::core::ffi::c_int as usize]
                    - s[156 as ::core::ffi::c_int as usize]
                    + s[228 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 41 as ::core::ffi::c_int) as isize,
                )
                += s[217 as ::core::ffi::c_int as usize]
                    - s[73 as ::core::ffi::c_int as usize]
                    - s[219 as ::core::ffi::c_int as usize]
                    + s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 42 as ::core::ffi::c_int) as isize,
                )
                += s[64 as ::core::ffi::c_int as usize]
                    - s[136 as ::core::ffi::c_int as usize]
                    - s[66 as ::core::ffi::c_int as usize]
                    + s[138 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 43 as ::core::ffi::c_int) as isize,
                )
                += s[55 as ::core::ffi::c_int as usize]
                    + s[145 as ::core::ffi::c_int as usize]
                    + s[235 as ::core::ffi::c_int as usize]
                    - s[57 as ::core::ffi::c_int as usize]
                    - s[147 as ::core::ffi::c_int as usize]
                    - s[237 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 44 as ::core::ffi::c_int) as isize,
                )
                += s[153 as ::core::ffi::c_int as usize]
                    - s[225 as ::core::ffi::c_int as usize]
                    + s[157 as ::core::ffi::c_int as usize]
                    - s[229 as ::core::ffi::c_int as usize]
                    + s[161 as ::core::ffi::c_int as usize]
                    - s[233 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 45 as ::core::ffi::c_int) as isize,
                )
                += s[216 as ::core::ffi::c_int as usize]
                    - s[72 as ::core::ffi::c_int as usize]
                    + s[220 as ::core::ffi::c_int as usize]
                    - s[76 as ::core::ffi::c_int as usize]
                    + s[224 as ::core::ffi::c_int as usize]
                    - s[80 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 46 as ::core::ffi::c_int) as isize,
                )
                += s[63 as ::core::ffi::c_int as usize]
                    - s[135 as ::core::ffi::c_int as usize]
                    + s[67 as ::core::ffi::c_int as usize]
                    - s[139 as ::core::ffi::c_int as usize]
                    + s[71 as ::core::ffi::c_int as usize]
                    - s[143 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 47 as ::core::ffi::c_int) as isize,
                )
                += s[54 as ::core::ffi::c_int as usize]
                    + s[144 as ::core::ffi::c_int as usize]
                    + s[234 as ::core::ffi::c_int as usize]
                    + s[58 as ::core::ffi::c_int as usize]
                    + s[148 as ::core::ffi::c_int as usize]
                    + s[238 as ::core::ffi::c_int as usize]
                    + s[62 as ::core::ffi::c_int as usize]
                    + s[152 as ::core::ffi::c_int as usize]
                    + s[242 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_ipspsp1spsp2_optimizer(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int2e_ipspsp1spsp2_cart(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_ipspsp1spsp2
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
pub unsafe extern "C" fn int2e_ipspsp1spsp2_sph(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_ipspsp1spsp2
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
pub unsafe extern "C" fn cint2e_ipspsp1spsp2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipspsp1spsp2_cart(
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
pub unsafe extern "C" fn cint2e_ipspsp1spsp2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipspsp1spsp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipspsp1spsp2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipspsp1spsp2_sph(
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
pub unsafe extern "C" fn cint2e_ipspsp1spsp2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipspsp1spsp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipspsp1spsp2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipspsp1spsp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_ipsrsr1(
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
    g1 = g0.offset((*envs).g_stride_j as isize);
    CINTnabla1i_2e(
        g2,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g3,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    g4 = g0.offset((*envs).g_stride_i as isize);
    g5 = g1.offset((*envs).g_stride_i as isize);
    g6 = g2.offset((*envs).g_stride_i as isize);
    g7 = g3.offset((*envs).g_stride_i as isize);
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
                ) = s[11 as ::core::ffi::c_int as usize]
                - s[19 as ::core::ffi::c_int as usize];
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
                ) = s[21 as ::core::ffi::c_int as usize]
                - s[5 as ::core::ffi::c_int as usize];
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
                ) = s[7 as ::core::ffi::c_int as usize]
                - s[15 as ::core::ffi::c_int as usize];
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
                )
                += s[11 as ::core::ffi::c_int as usize]
                    - s[19 as ::core::ffi::c_int as usize];
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
                )
                += s[21 as ::core::ffi::c_int as usize]
                    - s[5 as ::core::ffi::c_int as usize];
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
                )
                += s[7 as ::core::ffi::c_int as usize]
                    - s[15 as ::core::ffi::c_int as usize];
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
pub unsafe extern "C" fn int2e_ipsrsr1_optimizer(
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
pub unsafe extern "C" fn int2e_ipsrsr1_cart(
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
            CINTgout2e_int2e_ipsrsr1
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
pub unsafe extern "C" fn int2e_ipsrsr1_sph(
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
            CINTgout2e_int2e_ipsrsr1
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
pub unsafe extern "C" fn cint2e_ipsrsr1_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipsrsr1_cart(
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
pub unsafe extern "C" fn cint2e_ipsrsr1_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipsrsr1_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipsrsr1_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipsrsr1_sph(
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
pub unsafe extern "C" fn cint2e_ipsrsr1_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipsrsr1_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipsrsr1_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipsrsr1_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_ip1srsr2(
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
    g1 = g0.offset((*envs).g_stride_l as isize);
    g2 = g0.offset((*envs).g_stride_k as isize);
    g3 = g1.offset((*envs).g_stride_k as isize);
    CINTnabla1i_2e(
        g4,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
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
                ) = s[5 as ::core::ffi::c_int as usize]
                - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize]
                - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                - s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[14 as ::core::ffi::c_int as usize]
                - s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[15 as ::core::ffi::c_int as usize]
                - s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[10 as ::core::ffi::c_int as usize]
                - s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize]
                + s[13 as ::core::ffi::c_int as usize]
                + s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[23 as ::core::ffi::c_int as usize]
                - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[24 as ::core::ffi::c_int as usize]
                - s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[19 as ::core::ffi::c_int as usize]
                - s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[18 as ::core::ffi::c_int as usize]
                + s[22 as ::core::ffi::c_int as usize]
                + s[26 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[5 as ::core::ffi::c_int as usize]
                    - s[7 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += s[6 as ::core::ffi::c_int as usize]
                    - s[2 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += s[1 as ::core::ffi::c_int as usize]
                    - s[3 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += s[14 as ::core::ffi::c_int as usize]
                    - s[16 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[15 as ::core::ffi::c_int as usize]
                    - s[11 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += s[10 as ::core::ffi::c_int as usize]
                    - s[12 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += s[9 as ::core::ffi::c_int as usize]
                    + s[13 as ::core::ffi::c_int as usize]
                    + s[17 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += s[23 as ::core::ffi::c_int as usize]
                    - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += s[24 as ::core::ffi::c_int as usize]
                    - s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[19 as ::core::ffi::c_int as usize]
                    - s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 12 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[18 as ::core::ffi::c_int as usize]
                    + s[22 as ::core::ffi::c_int as usize]
                    + s[26 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_ip1srsr2_optimizer(
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
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int2e_ip1srsr2_cart(
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
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_ip1srsr2
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
pub unsafe extern "C" fn int2e_ip1srsr2_sph(
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
        1 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_ip1srsr2
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
pub unsafe extern "C" fn cint2e_ip1srsr2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ip1srsr2_cart(
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
pub unsafe extern "C" fn cint2e_ip1srsr2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ip1srsr2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ip1srsr2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ip1srsr2_sph(
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
pub unsafe extern "C" fn cint2e_ip1srsr2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ip1srsr2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ip1srsr2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ip1srsr2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_ipsrsr1srsr2(
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
    let mut g16: *mut ::core::ffi::c_double = g15
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g17: *mut ::core::ffi::c_double = g16
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g18: *mut ::core::ffi::c_double = g17
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g19: *mut ::core::ffi::c_double = g18
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g20: *mut ::core::ffi::c_double = g19
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g21: *mut ::core::ffi::c_double = g20
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g22: *mut ::core::ffi::c_double = g21
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g23: *mut ::core::ffi::c_double = g22
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g24: *mut ::core::ffi::c_double = g23
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g25: *mut ::core::ffi::c_double = g24
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g26: *mut ::core::ffi::c_double = g25
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g27: *mut ::core::ffi::c_double = g26
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g28: *mut ::core::ffi::c_double = g27
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g29: *mut ::core::ffi::c_double = g28
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g30: *mut ::core::ffi::c_double = g29
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    let mut g31: *mut ::core::ffi::c_double = g30
        .offset(((*envs).g_size * 3 as ::core::ffi::c_int) as isize);
    g1 = g0.offset((*envs).g_stride_l as isize);
    g2 = g0.offset((*envs).g_stride_k as isize);
    g3 = g1.offset((*envs).g_stride_k as isize);
    g4 = g0.offset((*envs).g_stride_j as isize);
    g5 = g1.offset((*envs).g_stride_j as isize);
    g6 = g2.offset((*envs).g_stride_j as isize);
    g7 = g3.offset((*envs).g_stride_j as isize);
    CINTnabla1i_2e(
        g8,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g11,
        g3,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g4,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g13,
        g5,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g14,
        g6,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g15,
        g7,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    g16 = g0.offset((*envs).g_stride_i as isize);
    g17 = g1.offset((*envs).g_stride_i as isize);
    g18 = g2.offset((*envs).g_stride_i as isize);
    g19 = g3.offset((*envs).g_stride_i as isize);
    g20 = g4.offset((*envs).g_stride_i as isize);
    g21 = g5.offset((*envs).g_stride_i as isize);
    g22 = g6.offset((*envs).g_stride_i as isize);
    g23 = g7.offset((*envs).g_stride_i as isize);
    g24 = g8.offset((*envs).g_stride_i as isize);
    g25 = g9.offset((*envs).g_stride_i as isize);
    g26 = g10.offset((*envs).g_stride_i as isize);
    g27 = g11.offset((*envs).g_stride_i as isize);
    g28 = g12.offset((*envs).g_stride_i as isize);
    g29 = g13.offset((*envs).g_stride_i as isize);
    g30 = g14.offset((*envs).g_stride_i as isize);
    g31 = g15.offset((*envs).g_stride_i as isize);
    let mut s: [::core::ffi::c_double; 243] = [0.; 243];
    n = 0 as ::core::ffi::c_int;
    while n < nf {
        ix = *idx
            .offset((0 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iy = *idx
            .offset((1 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        iz = *idx
            .offset((2 as ::core::ffi::c_int + n * 3 as ::core::ffi::c_int) as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 243 as ::core::ffi::c_int {
            s[i as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            i += 1;
        }
        i = 0 as ::core::ffi::c_int;
        while i < nrys_roots {
            s[0 as ::core::ffi::c_int as usize]
                += *g31.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[1 as ::core::ffi::c_int as usize]
                += *g30.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[2 as ::core::ffi::c_int as usize]
                += *g30.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[3 as ::core::ffi::c_int as usize]
                += *g29.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[4 as ::core::ffi::c_int as usize]
                += *g28.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[5 as ::core::ffi::c_int as usize]
                += *g28.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[6 as ::core::ffi::c_int as usize]
                += *g29.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[7 as ::core::ffi::c_int as usize]
                += *g28.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[8 as ::core::ffi::c_int as usize]
                += *g28.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[9 as ::core::ffi::c_int as usize]
                += *g27.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[10 as ::core::ffi::c_int as usize]
                += *g26.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[11 as ::core::ffi::c_int as usize]
                += *g26.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[12 as ::core::ffi::c_int as usize]
                += *g25.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[13 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[14 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[15 as ::core::ffi::c_int as usize]
                += *g25.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[16 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[17 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[18 as ::core::ffi::c_int as usize]
                += *g27.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[19 as ::core::ffi::c_int as usize]
                += *g26.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[20 as ::core::ffi::c_int as usize]
                += *g26.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[21 as ::core::ffi::c_int as usize]
                += *g25.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[22 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[23 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[24 as ::core::ffi::c_int as usize]
                += *g25.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[25 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[26 as ::core::ffi::c_int as usize]
                += *g24.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[27 as ::core::ffi::c_int as usize]
                += *g23.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[28 as ::core::ffi::c_int as usize]
                += *g22.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[29 as ::core::ffi::c_int as usize]
                += *g22.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[30 as ::core::ffi::c_int as usize]
                += *g21.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[31 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[32 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[33 as ::core::ffi::c_int as usize]
                += *g21.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[34 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[35 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[36 as ::core::ffi::c_int as usize]
                += *g19.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[37 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[38 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[39 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[40 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[41 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[42 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[43 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[44 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[45 as ::core::ffi::c_int as usize]
                += *g19.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[46 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[47 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[48 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[49 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[50 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[51 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[52 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[53 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[54 as ::core::ffi::c_int as usize]
                += *g23.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[55 as ::core::ffi::c_int as usize]
                += *g22.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[56 as ::core::ffi::c_int as usize]
                += *g22.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[57 as ::core::ffi::c_int as usize]
                += *g21.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[58 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[59 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[60 as ::core::ffi::c_int as usize]
                += *g21.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[61 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[62 as ::core::ffi::c_int as usize]
                += *g20.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[63 as ::core::ffi::c_int as usize]
                += *g19.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[64 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[65 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[66 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[67 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[68 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[69 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[70 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[71 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[72 as ::core::ffi::c_int as usize]
                += *g19.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[73 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[74 as ::core::ffi::c_int as usize]
                += *g18.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[75 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[76 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[77 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[78 as ::core::ffi::c_int as usize]
                += *g17.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[79 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[80 as ::core::ffi::c_int as usize]
                += *g16.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            s[81 as ::core::ffi::c_int as usize]
                += *g15.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[82 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[83 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[84 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[85 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g19.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[86 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[87 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[88 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[89 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[90 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[91 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g21.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[92 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[93 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g22.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[94 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g23.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[95 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g22.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[96 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[97 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g21.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[98 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[99 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[100 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[101 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[102 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[103 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g19.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[104 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[105 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[106 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[107 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[108 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[109 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g25.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[110 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[111 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g26.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[112 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g27.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[113 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g26.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[114 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[115 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g25.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[116 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[117 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g28.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[118 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g29.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[119 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g28.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[120 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g30.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[121 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g31.offset((iy + i) as isize)
                    * *g0.offset((iz + i) as isize);
            s[122 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g30.offset((iy + i) as isize)
                    * *g1.offset((iz + i) as isize);
            s[123 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g28.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[124 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g29.offset((iy + i) as isize)
                    * *g2.offset((iz + i) as isize);
            s[125 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g28.offset((iy + i) as isize)
                    * *g3.offset((iz + i) as isize);
            s[126 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[127 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g25.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[128 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[129 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g26.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[130 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g27.offset((iy + i) as isize)
                    * *g4.offset((iz + i) as isize);
            s[131 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g26.offset((iy + i) as isize)
                    * *g5.offset((iz + i) as isize);
            s[132 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[133 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g25.offset((iy + i) as isize)
                    * *g6.offset((iz + i) as isize);
            s[134 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g24.offset((iy + i) as isize)
                    * *g7.offset((iz + i) as isize);
            s[135 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[136 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[137 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[138 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[139 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g19.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[140 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[141 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[142 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[143 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[144 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[145 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g21.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[146 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[147 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g22.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[148 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g23.offset((iy + i) as isize)
                    * *g8.offset((iz + i) as isize);
            s[149 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g22.offset((iy + i) as isize)
                    * *g9.offset((iz + i) as isize);
            s[150 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[151 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g21.offset((iy + i) as isize)
                    * *g10.offset((iz + i) as isize);
            s[152 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g20.offset((iy + i) as isize)
                    * *g11.offset((iz + i) as isize);
            s[153 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[154 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[155 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[156 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[157 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g19.offset((iy + i) as isize)
                    * *g12.offset((iz + i) as isize);
            s[158 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g18.offset((iy + i) as isize)
                    * *g13.offset((iz + i) as isize);
            s[159 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[160 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g17.offset((iy + i) as isize)
                    * *g14.offset((iz + i) as isize);
            s[161 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g16.offset((iy + i) as isize)
                    * *g15.offset((iz + i) as isize);
            s[162 as ::core::ffi::c_int as usize]
                += *g15.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[163 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[164 as ::core::ffi::c_int as usize]
                += *g14.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[165 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[166 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[167 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[168 as ::core::ffi::c_int as usize]
                += *g13.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[169 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[170 as ::core::ffi::c_int as usize]
                += *g12.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g19.offset((iz + i) as isize);
            s[171 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[172 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[173 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[174 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[175 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[176 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[177 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[178 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[179 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g19.offset((iz + i) as isize);
            s[180 as ::core::ffi::c_int as usize]
                += *g11.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[181 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[182 as ::core::ffi::c_int as usize]
                += *g10.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g21.offset((iz + i) as isize);
            s[183 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[184 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[185 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g21.offset((iz + i) as isize);
            s[186 as ::core::ffi::c_int as usize]
                += *g9.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g22.offset((iz + i) as isize);
            s[187 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g22.offset((iz + i) as isize);
            s[188 as ::core::ffi::c_int as usize]
                += *g8.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g23.offset((iz + i) as isize);
            s[189 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[190 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[191 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[192 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[193 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[194 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[195 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[196 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[197 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g19.offset((iz + i) as isize);
            s[198 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[199 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[200 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[201 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[202 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g15.offset((iy + i) as isize)
                    * *g16.offset((iz + i) as isize);
            s[203 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g14.offset((iy + i) as isize)
                    * *g17.offset((iz + i) as isize);
            s[204 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[205 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g13.offset((iy + i) as isize)
                    * *g18.offset((iz + i) as isize);
            s[206 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g12.offset((iy + i) as isize)
                    * *g19.offset((iz + i) as isize);
            s[207 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[208 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[209 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g21.offset((iz + i) as isize);
            s[210 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[211 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g11.offset((iy + i) as isize)
                    * *g20.offset((iz + i) as isize);
            s[212 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g10.offset((iy + i) as isize)
                    * *g21.offset((iz + i) as isize);
            s[213 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g22.offset((iz + i) as isize);
            s[214 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g9.offset((iy + i) as isize)
                    * *g22.offset((iz + i) as isize);
            s[215 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g8.offset((iy + i) as isize)
                    * *g23.offset((iz + i) as isize);
            s[216 as ::core::ffi::c_int as usize]
                += *g7.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[217 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[218 as ::core::ffi::c_int as usize]
                += *g6.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g25.offset((iz + i) as isize);
            s[219 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[220 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[221 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g25.offset((iz + i) as isize);
            s[222 as ::core::ffi::c_int as usize]
                += *g5.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g26.offset((iz + i) as isize);
            s[223 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g26.offset((iz + i) as isize);
            s[224 as ::core::ffi::c_int as usize]
                += *g4.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g27.offset((iz + i) as isize);
            s[225 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[226 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[227 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g25.offset((iz + i) as isize);
            s[228 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[229 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g7.offset((iy + i) as isize)
                    * *g24.offset((iz + i) as isize);
            s[230 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g6.offset((iy + i) as isize)
                    * *g25.offset((iz + i) as isize);
            s[231 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g26.offset((iz + i) as isize);
            s[232 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g5.offset((iy + i) as isize)
                    * *g26.offset((iz + i) as isize);
            s[233 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g4.offset((iy + i) as isize)
                    * *g27.offset((iz + i) as isize);
            s[234 as ::core::ffi::c_int as usize]
                += *g3.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g28.offset((iz + i) as isize);
            s[235 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g28.offset((iz + i) as isize);
            s[236 as ::core::ffi::c_int as usize]
                += *g2.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g29.offset((iz + i) as isize);
            s[237 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g28.offset((iz + i) as isize);
            s[238 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g3.offset((iy + i) as isize)
                    * *g28.offset((iz + i) as isize);
            s[239 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g2.offset((iy + i) as isize)
                    * *g29.offset((iz + i) as isize);
            s[240 as ::core::ffi::c_int as usize]
                += *g1.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g30.offset((iz + i) as isize);
            s[241 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g1.offset((iy + i) as isize)
                    * *g30.offset((iz + i) as isize);
            s[242 as ::core::ffi::c_int as usize]
                += *g0.offset((ix + i) as isize) * *g0.offset((iy + i) as isize)
                    * *g31.offset((iz + i) as isize);
            i += 1;
        }
        if gout_empty != 0 {
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[104 as ::core::ffi::c_int as usize]
                - s[176 as ::core::ffi::c_int as usize]
                - s[106 as ::core::ffi::c_int as usize]
                + s[178 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[167 as ::core::ffi::c_int as usize]
                - s[23 as ::core::ffi::c_int as usize]
                - s[169 as ::core::ffi::c_int as usize]
                + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[14 as ::core::ffi::c_int as usize]
                - s[86 as ::core::ffi::c_int as usize]
                - s[16 as ::core::ffi::c_int as usize]
                + s[88 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize]
                + s[95 as ::core::ffi::c_int as usize]
                + s[185 as ::core::ffi::c_int as usize]
                - s[7 as ::core::ffi::c_int as usize]
                - s[97 as ::core::ffi::c_int as usize]
                - s[187 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[105 as ::core::ffi::c_int as usize]
                - s[177 as ::core::ffi::c_int as usize]
                - s[101 as ::core::ffi::c_int as usize]
                + s[173 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[168 as ::core::ffi::c_int as usize]
                - s[24 as ::core::ffi::c_int as usize]
                - s[164 as ::core::ffi::c_int as usize]
                + s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[15 as ::core::ffi::c_int as usize]
                - s[87 as ::core::ffi::c_int as usize]
                - s[11 as ::core::ffi::c_int as usize]
                + s[83 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize]
                + s[96 as ::core::ffi::c_int as usize]
                + s[186 as ::core::ffi::c_int as usize]
                - s[2 as ::core::ffi::c_int as usize]
                - s[92 as ::core::ffi::c_int as usize]
                - s[182 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[100 as ::core::ffi::c_int as usize]
                - s[172 as ::core::ffi::c_int as usize]
                - s[102 as ::core::ffi::c_int as usize]
                + s[174 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[163 as ::core::ffi::c_int as usize]
                - s[19 as ::core::ffi::c_int as usize]
                - s[165 as ::core::ffi::c_int as usize]
                + s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[10 as ::core::ffi::c_int as usize]
                - s[82 as ::core::ffi::c_int as usize]
                - s[12 as ::core::ffi::c_int as usize]
                + s[84 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                + s[91 as ::core::ffi::c_int as usize]
                + s[181 as ::core::ffi::c_int as usize]
                - s[3 as ::core::ffi::c_int as usize]
                - s[93 as ::core::ffi::c_int as usize]
                - s[183 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = s[99 as ::core::ffi::c_int as usize]
                - s[171 as ::core::ffi::c_int as usize]
                + s[103 as ::core::ffi::c_int as usize]
                - s[175 as ::core::ffi::c_int as usize]
                + s[107 as ::core::ffi::c_int as usize]
                - s[179 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = s[162 as ::core::ffi::c_int as usize]
                - s[18 as ::core::ffi::c_int as usize]
                + s[166 as ::core::ffi::c_int as usize]
                - s[22 as ::core::ffi::c_int as usize]
                + s[170 as ::core::ffi::c_int as usize]
                - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize]
                - s[81 as ::core::ffi::c_int as usize]
                + s[13 as ::core::ffi::c_int as usize]
                - s[85 as ::core::ffi::c_int as usize]
                + s[17 as ::core::ffi::c_int as usize]
                - s[89 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[90 as ::core::ffi::c_int as usize]
                + s[180 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize]
                + s[94 as ::core::ffi::c_int as usize]
                + s[184 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize]
                + s[98 as ::core::ffi::c_int as usize]
                + s[188 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                ) = s[131 as ::core::ffi::c_int as usize]
                - s[203 as ::core::ffi::c_int as usize]
                - s[133 as ::core::ffi::c_int as usize]
                + s[205 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                ) = s[194 as ::core::ffi::c_int as usize]
                - s[50 as ::core::ffi::c_int as usize]
                - s[196 as ::core::ffi::c_int as usize]
                + s[52 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                ) = s[41 as ::core::ffi::c_int as usize]
                - s[113 as ::core::ffi::c_int as usize]
                - s[43 as ::core::ffi::c_int as usize]
                + s[115 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                ) = s[32 as ::core::ffi::c_int as usize]
                + s[122 as ::core::ffi::c_int as usize]
                + s[212 as ::core::ffi::c_int as usize]
                - s[34 as ::core::ffi::c_int as usize]
                - s[124 as ::core::ffi::c_int as usize]
                - s[214 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                ) = s[132 as ::core::ffi::c_int as usize]
                - s[204 as ::core::ffi::c_int as usize]
                - s[128 as ::core::ffi::c_int as usize]
                + s[200 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                ) = s[195 as ::core::ffi::c_int as usize]
                - s[51 as ::core::ffi::c_int as usize]
                - s[191 as ::core::ffi::c_int as usize]
                + s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                ) = s[42 as ::core::ffi::c_int as usize]
                - s[114 as ::core::ffi::c_int as usize]
                - s[38 as ::core::ffi::c_int as usize]
                + s[110 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                ) = s[33 as ::core::ffi::c_int as usize]
                + s[123 as ::core::ffi::c_int as usize]
                + s[213 as ::core::ffi::c_int as usize]
                - s[29 as ::core::ffi::c_int as usize]
                - s[119 as ::core::ffi::c_int as usize]
                - s[209 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                ) = s[127 as ::core::ffi::c_int as usize]
                - s[199 as ::core::ffi::c_int as usize]
                - s[129 as ::core::ffi::c_int as usize]
                + s[201 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                ) = s[190 as ::core::ffi::c_int as usize]
                - s[46 as ::core::ffi::c_int as usize]
                - s[192 as ::core::ffi::c_int as usize]
                + s[48 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                ) = s[37 as ::core::ffi::c_int as usize]
                - s[109 as ::core::ffi::c_int as usize]
                - s[39 as ::core::ffi::c_int as usize]
                + s[111 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                ) = s[28 as ::core::ffi::c_int as usize]
                + s[118 as ::core::ffi::c_int as usize]
                + s[208 as ::core::ffi::c_int as usize]
                - s[30 as ::core::ffi::c_int as usize]
                - s[120 as ::core::ffi::c_int as usize]
                - s[210 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                ) = s[126 as ::core::ffi::c_int as usize]
                - s[198 as ::core::ffi::c_int as usize]
                + s[130 as ::core::ffi::c_int as usize]
                - s[202 as ::core::ffi::c_int as usize]
                + s[134 as ::core::ffi::c_int as usize]
                - s[206 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                ) = s[189 as ::core::ffi::c_int as usize]
                - s[45 as ::core::ffi::c_int as usize]
                + s[193 as ::core::ffi::c_int as usize]
                - s[49 as ::core::ffi::c_int as usize]
                + s[197 as ::core::ffi::c_int as usize]
                - s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                ) = s[36 as ::core::ffi::c_int as usize]
                - s[108 as ::core::ffi::c_int as usize]
                + s[40 as ::core::ffi::c_int as usize]
                - s[112 as ::core::ffi::c_int as usize]
                + s[44 as ::core::ffi::c_int as usize]
                - s[116 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                ) = s[27 as ::core::ffi::c_int as usize]
                + s[117 as ::core::ffi::c_int as usize]
                + s[207 as ::core::ffi::c_int as usize]
                + s[31 as ::core::ffi::c_int as usize]
                + s[121 as ::core::ffi::c_int as usize]
                + s[211 as ::core::ffi::c_int as usize]
                + s[35 as ::core::ffi::c_int as usize]
                + s[125 as ::core::ffi::c_int as usize]
                + s[215 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                ) = s[158 as ::core::ffi::c_int as usize]
                - s[230 as ::core::ffi::c_int as usize]
                - s[160 as ::core::ffi::c_int as usize]
                + s[232 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                ) = s[221 as ::core::ffi::c_int as usize]
                - s[77 as ::core::ffi::c_int as usize]
                - s[223 as ::core::ffi::c_int as usize]
                + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                ) = s[68 as ::core::ffi::c_int as usize]
                - s[140 as ::core::ffi::c_int as usize]
                - s[70 as ::core::ffi::c_int as usize]
                + s[142 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                ) = s[59 as ::core::ffi::c_int as usize]
                + s[149 as ::core::ffi::c_int as usize]
                + s[239 as ::core::ffi::c_int as usize]
                - s[61 as ::core::ffi::c_int as usize]
                - s[151 as ::core::ffi::c_int as usize]
                - s[241 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 36 as ::core::ffi::c_int) as isize,
                ) = s[159 as ::core::ffi::c_int as usize]
                - s[231 as ::core::ffi::c_int as usize]
                - s[155 as ::core::ffi::c_int as usize]
                + s[227 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 37 as ::core::ffi::c_int) as isize,
                ) = s[222 as ::core::ffi::c_int as usize]
                - s[78 as ::core::ffi::c_int as usize]
                - s[218 as ::core::ffi::c_int as usize]
                + s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 38 as ::core::ffi::c_int) as isize,
                ) = s[69 as ::core::ffi::c_int as usize]
                - s[141 as ::core::ffi::c_int as usize]
                - s[65 as ::core::ffi::c_int as usize]
                + s[137 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 39 as ::core::ffi::c_int) as isize,
                ) = s[60 as ::core::ffi::c_int as usize]
                + s[150 as ::core::ffi::c_int as usize]
                + s[240 as ::core::ffi::c_int as usize]
                - s[56 as ::core::ffi::c_int as usize]
                - s[146 as ::core::ffi::c_int as usize]
                - s[236 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 40 as ::core::ffi::c_int) as isize,
                ) = s[154 as ::core::ffi::c_int as usize]
                - s[226 as ::core::ffi::c_int as usize]
                - s[156 as ::core::ffi::c_int as usize]
                + s[228 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 41 as ::core::ffi::c_int) as isize,
                ) = s[217 as ::core::ffi::c_int as usize]
                - s[73 as ::core::ffi::c_int as usize]
                - s[219 as ::core::ffi::c_int as usize]
                + s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 42 as ::core::ffi::c_int) as isize,
                ) = s[64 as ::core::ffi::c_int as usize]
                - s[136 as ::core::ffi::c_int as usize]
                - s[66 as ::core::ffi::c_int as usize]
                + s[138 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 43 as ::core::ffi::c_int) as isize,
                ) = s[55 as ::core::ffi::c_int as usize]
                + s[145 as ::core::ffi::c_int as usize]
                + s[235 as ::core::ffi::c_int as usize]
                - s[57 as ::core::ffi::c_int as usize]
                - s[147 as ::core::ffi::c_int as usize]
                - s[237 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 44 as ::core::ffi::c_int) as isize,
                ) = s[153 as ::core::ffi::c_int as usize]
                - s[225 as ::core::ffi::c_int as usize]
                + s[157 as ::core::ffi::c_int as usize]
                - s[229 as ::core::ffi::c_int as usize]
                + s[161 as ::core::ffi::c_int as usize]
                - s[233 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 45 as ::core::ffi::c_int) as isize,
                ) = s[216 as ::core::ffi::c_int as usize]
                - s[72 as ::core::ffi::c_int as usize]
                + s[220 as ::core::ffi::c_int as usize]
                - s[76 as ::core::ffi::c_int as usize]
                + s[224 as ::core::ffi::c_int as usize]
                - s[80 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 46 as ::core::ffi::c_int) as isize,
                ) = s[63 as ::core::ffi::c_int as usize]
                - s[135 as ::core::ffi::c_int as usize]
                + s[67 as ::core::ffi::c_int as usize]
                - s[139 as ::core::ffi::c_int as usize]
                + s[71 as ::core::ffi::c_int as usize]
                - s[143 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 47 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize]
                + s[144 as ::core::ffi::c_int as usize]
                + s[234 as ::core::ffi::c_int as usize]
                + s[58 as ::core::ffi::c_int as usize]
                + s[148 as ::core::ffi::c_int as usize]
                + s[238 as ::core::ffi::c_int as usize]
                + s[62 as ::core::ffi::c_int as usize]
                + s[152 as ::core::ffi::c_int as usize]
                + s[242 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[104 as ::core::ffi::c_int as usize]
                    - s[176 as ::core::ffi::c_int as usize]
                    - s[106 as ::core::ffi::c_int as usize]
                    + s[178 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += s[167 as ::core::ffi::c_int as usize]
                    - s[23 as ::core::ffi::c_int as usize]
                    - s[169 as ::core::ffi::c_int as usize]
                    + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += s[14 as ::core::ffi::c_int as usize]
                    - s[86 as ::core::ffi::c_int as usize]
                    - s[16 as ::core::ffi::c_int as usize]
                    + s[88 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[5 as ::core::ffi::c_int as usize]
                    + s[95 as ::core::ffi::c_int as usize]
                    + s[185 as ::core::ffi::c_int as usize]
                    - s[7 as ::core::ffi::c_int as usize]
                    - s[97 as ::core::ffi::c_int as usize]
                    - s[187 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += s[105 as ::core::ffi::c_int as usize]
                    - s[177 as ::core::ffi::c_int as usize]
                    - s[101 as ::core::ffi::c_int as usize]
                    + s[173 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[168 as ::core::ffi::c_int as usize]
                    - s[24 as ::core::ffi::c_int as usize]
                    - s[164 as ::core::ffi::c_int as usize]
                    + s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += s[15 as ::core::ffi::c_int as usize]
                    - s[87 as ::core::ffi::c_int as usize]
                    - s[11 as ::core::ffi::c_int as usize]
                    + s[83 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += s[6 as ::core::ffi::c_int as usize]
                    + s[96 as ::core::ffi::c_int as usize]
                    + s[186 as ::core::ffi::c_int as usize]
                    - s[2 as ::core::ffi::c_int as usize]
                    - s[92 as ::core::ffi::c_int as usize]
                    - s[182 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += s[100 as ::core::ffi::c_int as usize]
                    - s[172 as ::core::ffi::c_int as usize]
                    - s[102 as ::core::ffi::c_int as usize]
                    + s[174 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += s[163 as ::core::ffi::c_int as usize]
                    - s[19 as ::core::ffi::c_int as usize]
                    - s[165 as ::core::ffi::c_int as usize]
                    + s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[10 as ::core::ffi::c_int as usize]
                    - s[82 as ::core::ffi::c_int as usize]
                    - s[12 as ::core::ffi::c_int as usize]
                    + s[84 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[1 as ::core::ffi::c_int as usize]
                    + s[91 as ::core::ffi::c_int as usize]
                    + s[181 as ::core::ffi::c_int as usize]
                    - s[3 as ::core::ffi::c_int as usize]
                    - s[93 as ::core::ffi::c_int as usize]
                    - s[183 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                )
                += s[99 as ::core::ffi::c_int as usize]
                    - s[171 as ::core::ffi::c_int as usize]
                    + s[103 as ::core::ffi::c_int as usize]
                    - s[175 as ::core::ffi::c_int as usize]
                    + s[107 as ::core::ffi::c_int as usize]
                    - s[179 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                )
                += s[162 as ::core::ffi::c_int as usize]
                    - s[18 as ::core::ffi::c_int as usize]
                    + s[166 as ::core::ffi::c_int as usize]
                    - s[22 as ::core::ffi::c_int as usize]
                    + s[170 as ::core::ffi::c_int as usize]
                    - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                )
                += s[9 as ::core::ffi::c_int as usize]
                    - s[81 as ::core::ffi::c_int as usize]
                    + s[13 as ::core::ffi::c_int as usize]
                    - s[85 as ::core::ffi::c_int as usize]
                    + s[17 as ::core::ffi::c_int as usize]
                    - s[89 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[90 as ::core::ffi::c_int as usize]
                    + s[180 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize]
                    + s[94 as ::core::ffi::c_int as usize]
                    + s[184 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize]
                    + s[98 as ::core::ffi::c_int as usize]
                    + s[188 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 16 as ::core::ffi::c_int) as isize,
                )
                += s[131 as ::core::ffi::c_int as usize]
                    - s[203 as ::core::ffi::c_int as usize]
                    - s[133 as ::core::ffi::c_int as usize]
                    + s[205 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 17 as ::core::ffi::c_int) as isize,
                )
                += s[194 as ::core::ffi::c_int as usize]
                    - s[50 as ::core::ffi::c_int as usize]
                    - s[196 as ::core::ffi::c_int as usize]
                    + s[52 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 18 as ::core::ffi::c_int) as isize,
                )
                += s[41 as ::core::ffi::c_int as usize]
                    - s[113 as ::core::ffi::c_int as usize]
                    - s[43 as ::core::ffi::c_int as usize]
                    + s[115 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 19 as ::core::ffi::c_int) as isize,
                )
                += s[32 as ::core::ffi::c_int as usize]
                    + s[122 as ::core::ffi::c_int as usize]
                    + s[212 as ::core::ffi::c_int as usize]
                    - s[34 as ::core::ffi::c_int as usize]
                    - s[124 as ::core::ffi::c_int as usize]
                    - s[214 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 20 as ::core::ffi::c_int) as isize,
                )
                += s[132 as ::core::ffi::c_int as usize]
                    - s[204 as ::core::ffi::c_int as usize]
                    - s[128 as ::core::ffi::c_int as usize]
                    + s[200 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 21 as ::core::ffi::c_int) as isize,
                )
                += s[195 as ::core::ffi::c_int as usize]
                    - s[51 as ::core::ffi::c_int as usize]
                    - s[191 as ::core::ffi::c_int as usize]
                    + s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 22 as ::core::ffi::c_int) as isize,
                )
                += s[42 as ::core::ffi::c_int as usize]
                    - s[114 as ::core::ffi::c_int as usize]
                    - s[38 as ::core::ffi::c_int as usize]
                    + s[110 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 23 as ::core::ffi::c_int) as isize,
                )
                += s[33 as ::core::ffi::c_int as usize]
                    + s[123 as ::core::ffi::c_int as usize]
                    + s[213 as ::core::ffi::c_int as usize]
                    - s[29 as ::core::ffi::c_int as usize]
                    - s[119 as ::core::ffi::c_int as usize]
                    - s[209 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as isize,
                )
                += s[127 as ::core::ffi::c_int as usize]
                    - s[199 as ::core::ffi::c_int as usize]
                    - s[129 as ::core::ffi::c_int as usize]
                    + s[201 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 25 as ::core::ffi::c_int) as isize,
                )
                += s[190 as ::core::ffi::c_int as usize]
                    - s[46 as ::core::ffi::c_int as usize]
                    - s[192 as ::core::ffi::c_int as usize]
                    + s[48 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 26 as ::core::ffi::c_int) as isize,
                )
                += s[37 as ::core::ffi::c_int as usize]
                    - s[109 as ::core::ffi::c_int as usize]
                    - s[39 as ::core::ffi::c_int as usize]
                    + s[111 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 27 as ::core::ffi::c_int) as isize,
                )
                += s[28 as ::core::ffi::c_int as usize]
                    + s[118 as ::core::ffi::c_int as usize]
                    + s[208 as ::core::ffi::c_int as usize]
                    - s[30 as ::core::ffi::c_int as usize]
                    - s[120 as ::core::ffi::c_int as usize]
                    - s[210 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 28 as ::core::ffi::c_int) as isize,
                )
                += s[126 as ::core::ffi::c_int as usize]
                    - s[198 as ::core::ffi::c_int as usize]
                    + s[130 as ::core::ffi::c_int as usize]
                    - s[202 as ::core::ffi::c_int as usize]
                    + s[134 as ::core::ffi::c_int as usize]
                    - s[206 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 29 as ::core::ffi::c_int) as isize,
                )
                += s[189 as ::core::ffi::c_int as usize]
                    - s[45 as ::core::ffi::c_int as usize]
                    + s[193 as ::core::ffi::c_int as usize]
                    - s[49 as ::core::ffi::c_int as usize]
                    + s[197 as ::core::ffi::c_int as usize]
                    - s[53 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 30 as ::core::ffi::c_int) as isize,
                )
                += s[36 as ::core::ffi::c_int as usize]
                    - s[108 as ::core::ffi::c_int as usize]
                    + s[40 as ::core::ffi::c_int as usize]
                    - s[112 as ::core::ffi::c_int as usize]
                    + s[44 as ::core::ffi::c_int as usize]
                    - s[116 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 31 as ::core::ffi::c_int) as isize,
                )
                += s[27 as ::core::ffi::c_int as usize]
                    + s[117 as ::core::ffi::c_int as usize]
                    + s[207 as ::core::ffi::c_int as usize]
                    + s[31 as ::core::ffi::c_int as usize]
                    + s[121 as ::core::ffi::c_int as usize]
                    + s[211 as ::core::ffi::c_int as usize]
                    + s[35 as ::core::ffi::c_int as usize]
                    + s[125 as ::core::ffi::c_int as usize]
                    + s[215 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 32 as ::core::ffi::c_int) as isize,
                )
                += s[158 as ::core::ffi::c_int as usize]
                    - s[230 as ::core::ffi::c_int as usize]
                    - s[160 as ::core::ffi::c_int as usize]
                    + s[232 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 33 as ::core::ffi::c_int) as isize,
                )
                += s[221 as ::core::ffi::c_int as usize]
                    - s[77 as ::core::ffi::c_int as usize]
                    - s[223 as ::core::ffi::c_int as usize]
                    + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 34 as ::core::ffi::c_int) as isize,
                )
                += s[68 as ::core::ffi::c_int as usize]
                    - s[140 as ::core::ffi::c_int as usize]
                    - s[70 as ::core::ffi::c_int as usize]
                    + s[142 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 35 as ::core::ffi::c_int) as isize,
                )
                += s[59 as ::core::ffi::c_int as usize]
                    + s[149 as ::core::ffi::c_int as usize]
                    + s[239 as ::core::ffi::c_int as usize]
                    - s[61 as ::core::ffi::c_int as usize]
                    - s[151 as ::core::ffi::c_int as usize]
                    - s[241 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 36 as ::core::ffi::c_int) as isize,
                )
                += s[159 as ::core::ffi::c_int as usize]
                    - s[231 as ::core::ffi::c_int as usize]
                    - s[155 as ::core::ffi::c_int as usize]
                    + s[227 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 37 as ::core::ffi::c_int) as isize,
                )
                += s[222 as ::core::ffi::c_int as usize]
                    - s[78 as ::core::ffi::c_int as usize]
                    - s[218 as ::core::ffi::c_int as usize]
                    + s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 38 as ::core::ffi::c_int) as isize,
                )
                += s[69 as ::core::ffi::c_int as usize]
                    - s[141 as ::core::ffi::c_int as usize]
                    - s[65 as ::core::ffi::c_int as usize]
                    + s[137 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 39 as ::core::ffi::c_int) as isize,
                )
                += s[60 as ::core::ffi::c_int as usize]
                    + s[150 as ::core::ffi::c_int as usize]
                    + s[240 as ::core::ffi::c_int as usize]
                    - s[56 as ::core::ffi::c_int as usize]
                    - s[146 as ::core::ffi::c_int as usize]
                    - s[236 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 40 as ::core::ffi::c_int) as isize,
                )
                += s[154 as ::core::ffi::c_int as usize]
                    - s[226 as ::core::ffi::c_int as usize]
                    - s[156 as ::core::ffi::c_int as usize]
                    + s[228 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 41 as ::core::ffi::c_int) as isize,
                )
                += s[217 as ::core::ffi::c_int as usize]
                    - s[73 as ::core::ffi::c_int as usize]
                    - s[219 as ::core::ffi::c_int as usize]
                    + s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 42 as ::core::ffi::c_int) as isize,
                )
                += s[64 as ::core::ffi::c_int as usize]
                    - s[136 as ::core::ffi::c_int as usize]
                    - s[66 as ::core::ffi::c_int as usize]
                    + s[138 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 43 as ::core::ffi::c_int) as isize,
                )
                += s[55 as ::core::ffi::c_int as usize]
                    + s[145 as ::core::ffi::c_int as usize]
                    + s[235 as ::core::ffi::c_int as usize]
                    - s[57 as ::core::ffi::c_int as usize]
                    - s[147 as ::core::ffi::c_int as usize]
                    - s[237 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 44 as ::core::ffi::c_int) as isize,
                )
                += s[153 as ::core::ffi::c_int as usize]
                    - s[225 as ::core::ffi::c_int as usize]
                    + s[157 as ::core::ffi::c_int as usize]
                    - s[229 as ::core::ffi::c_int as usize]
                    + s[161 as ::core::ffi::c_int as usize]
                    - s[233 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 45 as ::core::ffi::c_int) as isize,
                )
                += s[216 as ::core::ffi::c_int as usize]
                    - s[72 as ::core::ffi::c_int as usize]
                    + s[220 as ::core::ffi::c_int as usize]
                    - s[76 as ::core::ffi::c_int as usize]
                    + s[224 as ::core::ffi::c_int as usize]
                    - s[80 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 46 as ::core::ffi::c_int) as isize,
                )
                += s[63 as ::core::ffi::c_int as usize]
                    - s[135 as ::core::ffi::c_int as usize]
                    + s[67 as ::core::ffi::c_int as usize]
                    - s[139 as ::core::ffi::c_int as usize]
                    + s[71 as ::core::ffi::c_int as usize]
                    - s[143 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 48 as ::core::ffi::c_int + 47 as ::core::ffi::c_int) as isize,
                )
                += s[54 as ::core::ffi::c_int as usize]
                    + s[144 as ::core::ffi::c_int as usize]
                    + s[234 as ::core::ffi::c_int as usize]
                    + s[58 as ::core::ffi::c_int as usize]
                    + s[148 as ::core::ffi::c_int as usize]
                    + s[238 as ::core::ffi::c_int as usize]
                    + s[62 as ::core::ffi::c_int as usize]
                    + s[152 as ::core::ffi::c_int as usize]
                    + s[242 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_ipsrsr1srsr2_optimizer(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int2e_ipsrsr1srsr2_cart(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_ipsrsr1srsr2
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
pub unsafe extern "C" fn int2e_ipsrsr1srsr2_sph(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_ipsrsr1srsr2
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
pub unsafe extern "C" fn cint2e_ipsrsr1srsr2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipsrsr1srsr2_cart(
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
pub unsafe extern "C" fn cint2e_ipsrsr1srsr2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipsrsr1srsr2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipsrsr1srsr2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_ipsrsr1srsr2_sph(
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
pub unsafe extern "C" fn cint2e_ipsrsr1srsr2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipsrsr1srsr2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_ipsrsr1srsr2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_ipsrsr1srsr2_optimizer(opt, atm, natm, bas, nbas, env);
}
