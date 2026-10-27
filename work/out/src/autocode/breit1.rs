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
    fn c2s_si_2e1i(
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
    fn CINTx1j_2e(
        f: *mut ::core::ffi::c_double,
        g: *const ::core::ffi::c_double,
        rj: *const ::core::ffi::c_double,
        li: ::core::ffi::c_int,
        lj: ::core::ffi::c_int,
        lk: ::core::ffi::c_int,
        ll: ::core::ffi::c_int,
        envs: *const CINTEnvVars,
    );
    fn CINTx1l_2e(
        f: *mut ::core::ffi::c_double,
        g: *const ::core::ffi::c_double,
        rl: *const ::core::ffi::c_double,
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
pub unsafe extern "C" fn CINTgout2e_int2e_gauge_r1_ssp1ssp2(
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
        (*envs).j_l + 3 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l + 0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_2e(
        g2,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g3,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTx1j_2e(
        g4,
        g0,
        (*envs).rj,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTx1j_2e(
        g5,
        g1,
        (*envs).rj,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g6,
        g4,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g7,
        g5,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g8.offset(ix as isize) += *g9.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g9.offset(ix as isize) += *g10.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g10,
        g8,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g11,
        g9,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTx1j_2e(
        g12,
        g8,
        (*envs).rj,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTx1j_2e(
        g13,
        g9,
        (*envs).rj,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g14,
        g12,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g15,
        g13,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
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
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[44 as ::core::ffi::c_int as usize]
                - s[68 as ::core::ffi::c_int as usize]
                - s[52 as ::core::ffi::c_int as usize]
                + s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[65 as ::core::ffi::c_int as usize]
                - s[17 as ::core::ffi::c_int as usize]
                - s[73 as ::core::ffi::c_int as usize]
                + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[14 as ::core::ffi::c_int as usize]
                - s[38 as ::core::ffi::c_int as usize]
                - s[22 as ::core::ffi::c_int as usize]
                + s[46 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[11 as ::core::ffi::c_int as usize]
                + s[41 as ::core::ffi::c_int as usize]
                + s[71 as ::core::ffi::c_int as usize]
                - s[19 as ::core::ffi::c_int as usize]
                - s[49 as ::core::ffi::c_int as usize]
                - s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = -s[35 as ::core::ffi::c_int as usize]
                + s[59 as ::core::ffi::c_int as usize]
                + s[51 as ::core::ffi::c_int as usize]
                - s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = -s[56 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize]
                + s[72 as ::core::ffi::c_int as usize]
                - s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = -s[5 as ::core::ffi::c_int as usize]
                + s[29 as ::core::ffi::c_int as usize]
                + s[21 as ::core::ffi::c_int as usize]
                - s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize]
                - s[32 as ::core::ffi::c_int as usize]
                - s[62 as ::core::ffi::c_int as usize]
                + s[18 as ::core::ffi::c_int as usize]
                + s[48 as ::core::ffi::c_int as usize]
                + s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[34 as ::core::ffi::c_int as usize]
                - s[58 as ::core::ffi::c_int as usize]
                - s[42 as ::core::ffi::c_int as usize]
                + s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[55 as ::core::ffi::c_int as usize]
                - s[7 as ::core::ffi::c_int as usize]
                - s[63 as ::core::ffi::c_int as usize]
                + s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize]
                - s[28 as ::core::ffi::c_int as usize]
                - s[12 as ::core::ffi::c_int as usize]
                + s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                + s[31 as ::core::ffi::c_int as usize]
                + s[61 as ::core::ffi::c_int as usize]
                - s[9 as ::core::ffi::c_int as usize]
                - s[39 as ::core::ffi::c_int as usize]
                - s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = s[33 as ::core::ffi::c_int as usize]
                - s[57 as ::core::ffi::c_int as usize]
                + s[43 as ::core::ffi::c_int as usize]
                - s[67 as ::core::ffi::c_int as usize]
                + s[53 as ::core::ffi::c_int as usize]
                - s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize]
                - s[6 as ::core::ffi::c_int as usize]
                + s[64 as ::core::ffi::c_int as usize]
                - s[16 as ::core::ffi::c_int as usize]
                + s[74 as ::core::ffi::c_int as usize]
                - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = s[3 as ::core::ffi::c_int as usize]
                - s[27 as ::core::ffi::c_int as usize]
                + s[13 as ::core::ffi::c_int as usize]
                - s[37 as ::core::ffi::c_int as usize]
                + s[23 as ::core::ffi::c_int as usize]
                - s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[30 as ::core::ffi::c_int as usize]
                + s[60 as ::core::ffi::c_int as usize]
                + s[10 as ::core::ffi::c_int as usize]
                + s[40 as ::core::ffi::c_int as usize]
                + s[70 as ::core::ffi::c_int as usize]
                + s[20 as ::core::ffi::c_int as usize]
                + s[50 as ::core::ffi::c_int as usize]
                + s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[44 as ::core::ffi::c_int as usize]
                    - s[68 as ::core::ffi::c_int as usize]
                    - s[52 as ::core::ffi::c_int as usize]
                    + s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += s[65 as ::core::ffi::c_int as usize]
                    - s[17 as ::core::ffi::c_int as usize]
                    - s[73 as ::core::ffi::c_int as usize]
                    + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += s[14 as ::core::ffi::c_int as usize]
                    - s[38 as ::core::ffi::c_int as usize]
                    - s[22 as ::core::ffi::c_int as usize]
                    + s[46 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[11 as ::core::ffi::c_int as usize]
                    + s[41 as ::core::ffi::c_int as usize]
                    + s[71 as ::core::ffi::c_int as usize]
                    - s[19 as ::core::ffi::c_int as usize]
                    - s[49 as ::core::ffi::c_int as usize]
                    - s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += -s[35 as ::core::ffi::c_int as usize]
                    + s[59 as ::core::ffi::c_int as usize]
                    + s[51 as ::core::ffi::c_int as usize]
                    - s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += -s[56 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize]
                    + s[72 as ::core::ffi::c_int as usize]
                    - s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += -s[5 as ::core::ffi::c_int as usize]
                    + s[29 as ::core::ffi::c_int as usize]
                    + s[21 as ::core::ffi::c_int as usize]
                    - s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += -s[2 as ::core::ffi::c_int as usize]
                    - s[32 as ::core::ffi::c_int as usize]
                    - s[62 as ::core::ffi::c_int as usize]
                    + s[18 as ::core::ffi::c_int as usize]
                    + s[48 as ::core::ffi::c_int as usize]
                    + s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += s[34 as ::core::ffi::c_int as usize]
                    - s[58 as ::core::ffi::c_int as usize]
                    - s[42 as ::core::ffi::c_int as usize]
                    + s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += s[55 as ::core::ffi::c_int as usize]
                    - s[7 as ::core::ffi::c_int as usize]
                    - s[63 as ::core::ffi::c_int as usize]
                    + s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[4 as ::core::ffi::c_int as usize]
                    - s[28 as ::core::ffi::c_int as usize]
                    - s[12 as ::core::ffi::c_int as usize]
                    + s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[1 as ::core::ffi::c_int as usize]
                    + s[31 as ::core::ffi::c_int as usize]
                    + s[61 as ::core::ffi::c_int as usize]
                    - s[9 as ::core::ffi::c_int as usize]
                    - s[39 as ::core::ffi::c_int as usize]
                    - s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                )
                += s[33 as ::core::ffi::c_int as usize]
                    - s[57 as ::core::ffi::c_int as usize]
                    + s[43 as ::core::ffi::c_int as usize]
                    - s[67 as ::core::ffi::c_int as usize]
                    + s[53 as ::core::ffi::c_int as usize]
                    - s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                )
                += s[54 as ::core::ffi::c_int as usize]
                    - s[6 as ::core::ffi::c_int as usize]
                    + s[64 as ::core::ffi::c_int as usize]
                    - s[16 as ::core::ffi::c_int as usize]
                    + s[74 as ::core::ffi::c_int as usize]
                    - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                )
                += s[3 as ::core::ffi::c_int as usize]
                    - s[27 as ::core::ffi::c_int as usize]
                    + s[13 as ::core::ffi::c_int as usize]
                    - s[37 as ::core::ffi::c_int as usize]
                    + s[23 as ::core::ffi::c_int as usize]
                    - s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[30 as ::core::ffi::c_int as usize]
                    + s[60 as ::core::ffi::c_int as usize]
                    + s[10 as ::core::ffi::c_int as usize]
                    + s[40 as ::core::ffi::c_int as usize]
                    + s[70 as ::core::ffi::c_int as usize]
                    + s[20 as ::core::ffi::c_int as usize]
                    + s[50 as ::core::ffi::c_int as usize]
                    + s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_gauge_r1_ssp1ssp2_optimizer(
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
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int2e_gauge_r1_ssp1ssp2_cart(
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
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r1_ssp1ssp2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn int2e_gauge_r1_ssp1ssp2_sph(
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
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r1_ssp1ssp2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn cint2e_gauge_r1_ssp1ssp2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r1_ssp1ssp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r1_ssp1ssp2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r1_ssp1ssp2_sph(
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
pub unsafe extern "C" fn cint2e_gauge_r1_ssp1ssp2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r1_ssp1ssp2_cart(
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
pub unsafe extern "C" fn cint2e_gauge_r1_ssp1ssp2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r1_ssp1ssp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r1_ssp1ssp2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r1_ssp1ssp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_gauge_r1_ssp1sps2(
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
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 3 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g2,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g3,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTx1j_2e(
        g4,
        g0,
        (*envs).rj,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTx1j_2e(
        g5,
        g1,
        (*envs).rj,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g6,
        g4,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g7,
        g5,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g8.offset(ix as isize) += *g9.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g9.offset(ix as isize) += *g10.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g10,
        g8,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g11,
        g9,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTx1j_2e(
        g12,
        g8,
        (*envs).rj,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTx1j_2e(
        g13,
        g9,
        (*envs).rj,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g14,
        g12,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g15,
        g13,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
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
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[44 as ::core::ffi::c_int as usize]
                - s[68 as ::core::ffi::c_int as usize]
                - s[52 as ::core::ffi::c_int as usize]
                + s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[65 as ::core::ffi::c_int as usize]
                - s[17 as ::core::ffi::c_int as usize]
                - s[73 as ::core::ffi::c_int as usize]
                + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[14 as ::core::ffi::c_int as usize]
                - s[38 as ::core::ffi::c_int as usize]
                - s[22 as ::core::ffi::c_int as usize]
                + s[46 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[11 as ::core::ffi::c_int as usize]
                + s[41 as ::core::ffi::c_int as usize]
                + s[71 as ::core::ffi::c_int as usize]
                - s[19 as ::core::ffi::c_int as usize]
                - s[49 as ::core::ffi::c_int as usize]
                - s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = -s[35 as ::core::ffi::c_int as usize]
                + s[59 as ::core::ffi::c_int as usize]
                + s[51 as ::core::ffi::c_int as usize]
                - s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = -s[56 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize]
                + s[72 as ::core::ffi::c_int as usize]
                - s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = -s[5 as ::core::ffi::c_int as usize]
                + s[29 as ::core::ffi::c_int as usize]
                + s[21 as ::core::ffi::c_int as usize]
                - s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = -s[2 as ::core::ffi::c_int as usize]
                - s[32 as ::core::ffi::c_int as usize]
                - s[62 as ::core::ffi::c_int as usize]
                + s[18 as ::core::ffi::c_int as usize]
                + s[48 as ::core::ffi::c_int as usize]
                + s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[34 as ::core::ffi::c_int as usize]
                - s[58 as ::core::ffi::c_int as usize]
                - s[42 as ::core::ffi::c_int as usize]
                + s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[55 as ::core::ffi::c_int as usize]
                - s[7 as ::core::ffi::c_int as usize]
                - s[63 as ::core::ffi::c_int as usize]
                + s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[4 as ::core::ffi::c_int as usize]
                - s[28 as ::core::ffi::c_int as usize]
                - s[12 as ::core::ffi::c_int as usize]
                + s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                + s[31 as ::core::ffi::c_int as usize]
                + s[61 as ::core::ffi::c_int as usize]
                - s[9 as ::core::ffi::c_int as usize]
                - s[39 as ::core::ffi::c_int as usize]
                - s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = -s[33 as ::core::ffi::c_int as usize]
                + s[57 as ::core::ffi::c_int as usize]
                - s[43 as ::core::ffi::c_int as usize]
                + s[67 as ::core::ffi::c_int as usize]
                - s[53 as ::core::ffi::c_int as usize]
                + s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = -s[54 as ::core::ffi::c_int as usize]
                + s[6 as ::core::ffi::c_int as usize]
                - s[64 as ::core::ffi::c_int as usize]
                + s[16 as ::core::ffi::c_int as usize]
                - s[74 as ::core::ffi::c_int as usize]
                + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = -s[3 as ::core::ffi::c_int as usize]
                + s[27 as ::core::ffi::c_int as usize]
                - s[13 as ::core::ffi::c_int as usize]
                + s[37 as ::core::ffi::c_int as usize]
                - s[23 as ::core::ffi::c_int as usize]
                + s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = -s[0 as ::core::ffi::c_int as usize]
                - s[30 as ::core::ffi::c_int as usize]
                - s[60 as ::core::ffi::c_int as usize]
                - s[10 as ::core::ffi::c_int as usize]
                - s[40 as ::core::ffi::c_int as usize]
                - s[70 as ::core::ffi::c_int as usize]
                - s[20 as ::core::ffi::c_int as usize]
                - s[50 as ::core::ffi::c_int as usize]
                - s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[44 as ::core::ffi::c_int as usize]
                    - s[68 as ::core::ffi::c_int as usize]
                    - s[52 as ::core::ffi::c_int as usize]
                    + s[76 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += s[65 as ::core::ffi::c_int as usize]
                    - s[17 as ::core::ffi::c_int as usize]
                    - s[73 as ::core::ffi::c_int as usize]
                    + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += s[14 as ::core::ffi::c_int as usize]
                    - s[38 as ::core::ffi::c_int as usize]
                    - s[22 as ::core::ffi::c_int as usize]
                    + s[46 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[11 as ::core::ffi::c_int as usize]
                    + s[41 as ::core::ffi::c_int as usize]
                    + s[71 as ::core::ffi::c_int as usize]
                    - s[19 as ::core::ffi::c_int as usize]
                    - s[49 as ::core::ffi::c_int as usize]
                    - s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += -s[35 as ::core::ffi::c_int as usize]
                    + s[59 as ::core::ffi::c_int as usize]
                    + s[51 as ::core::ffi::c_int as usize]
                    - s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += -s[56 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize]
                    + s[72 as ::core::ffi::c_int as usize]
                    - s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += -s[5 as ::core::ffi::c_int as usize]
                    + s[29 as ::core::ffi::c_int as usize]
                    + s[21 as ::core::ffi::c_int as usize]
                    - s[45 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += -s[2 as ::core::ffi::c_int as usize]
                    - s[32 as ::core::ffi::c_int as usize]
                    - s[62 as ::core::ffi::c_int as usize]
                    + s[18 as ::core::ffi::c_int as usize]
                    + s[48 as ::core::ffi::c_int as usize]
                    + s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += s[34 as ::core::ffi::c_int as usize]
                    - s[58 as ::core::ffi::c_int as usize]
                    - s[42 as ::core::ffi::c_int as usize]
                    + s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += s[55 as ::core::ffi::c_int as usize]
                    - s[7 as ::core::ffi::c_int as usize]
                    - s[63 as ::core::ffi::c_int as usize]
                    + s[15 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[4 as ::core::ffi::c_int as usize]
                    - s[28 as ::core::ffi::c_int as usize]
                    - s[12 as ::core::ffi::c_int as usize]
                    + s[36 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[1 as ::core::ffi::c_int as usize]
                    + s[31 as ::core::ffi::c_int as usize]
                    + s[61 as ::core::ffi::c_int as usize]
                    - s[9 as ::core::ffi::c_int as usize]
                    - s[39 as ::core::ffi::c_int as usize]
                    - s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                )
                += -s[33 as ::core::ffi::c_int as usize]
                    + s[57 as ::core::ffi::c_int as usize]
                    - s[43 as ::core::ffi::c_int as usize]
                    + s[67 as ::core::ffi::c_int as usize]
                    - s[53 as ::core::ffi::c_int as usize]
                    + s[77 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                )
                += -s[54 as ::core::ffi::c_int as usize]
                    + s[6 as ::core::ffi::c_int as usize]
                    - s[64 as ::core::ffi::c_int as usize]
                    + s[16 as ::core::ffi::c_int as usize]
                    - s[74 as ::core::ffi::c_int as usize]
                    + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                )
                += -s[3 as ::core::ffi::c_int as usize]
                    + s[27 as ::core::ffi::c_int as usize]
                    - s[13 as ::core::ffi::c_int as usize]
                    + s[37 as ::core::ffi::c_int as usize]
                    - s[23 as ::core::ffi::c_int as usize]
                    + s[47 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                )
                += -s[0 as ::core::ffi::c_int as usize]
                    - s[30 as ::core::ffi::c_int as usize]
                    - s[60 as ::core::ffi::c_int as usize]
                    - s[10 as ::core::ffi::c_int as usize]
                    - s[40 as ::core::ffi::c_int as usize]
                    - s[70 as ::core::ffi::c_int as usize]
                    - s[20 as ::core::ffi::c_int as usize]
                    - s[50 as ::core::ffi::c_int as usize]
                    - s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_gauge_r1_ssp1sps2_optimizer(
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
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int2e_gauge_r1_ssp1sps2_cart(
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
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r1_ssp1sps2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn int2e_gauge_r1_ssp1sps2_sph(
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
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r1_ssp1sps2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn cint2e_gauge_r1_ssp1sps2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r1_ssp1sps2_cart(
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
pub unsafe extern "C" fn cint2e_gauge_r1_ssp1sps2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r1_ssp1sps2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r1_ssp1sps2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r1_ssp1sps2_sph(
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
pub unsafe extern "C" fn cint2e_gauge_r1_ssp1sps2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r1_ssp1sps2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r1_ssp1sps2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r1_ssp1sps2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_gauge_r1_sps1ssp2(
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
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l + 0 as ::core::ffi::c_int,
        envs,
    );
    CINTx1j_2e(
        g2,
        g0,
        (*envs).rj,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTx1j_2e(
        g3,
        g1,
        (*envs).rj,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g4,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g4.offset(ix as isize) += *g5.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g5,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g5.offset(ix as isize) += *g6.offset(ix as isize);
        ix += 1;
    }
    CINTx1j_2e(
        g6,
        g4,
        (*envs).rj,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTx1j_2e(
        g7,
        g5,
        (*envs).rj,
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
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = -s[50 as ::core::ffi::c_int as usize]
                + s[68 as ::core::ffi::c_int as usize]
                + s[52 as ::core::ffi::c_int as usize]
                - s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = -s[59 as ::core::ffi::c_int as usize]
                + s[23 as ::core::ffi::c_int as usize]
                + s[61 as ::core::ffi::c_int as usize]
                - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = -s[14 as ::core::ffi::c_int as usize]
                + s[32 as ::core::ffi::c_int as usize]
                + s[16 as ::core::ffi::c_int as usize]
                - s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = -s[5 as ::core::ffi::c_int as usize]
                - s[41 as ::core::ffi::c_int as usize]
                - s[77 as ::core::ffi::c_int as usize]
                + s[7 as ::core::ffi::c_int as usize]
                + s[43 as ::core::ffi::c_int as usize]
                + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[47 as ::core::ffi::c_int as usize]
                - s[65 as ::core::ffi::c_int as usize]
                - s[51 as ::core::ffi::c_int as usize]
                + s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[56 as ::core::ffi::c_int as usize]
                - s[20 as ::core::ffi::c_int as usize]
                - s[60 as ::core::ffi::c_int as usize]
                + s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[11 as ::core::ffi::c_int as usize]
                - s[29 as ::core::ffi::c_int as usize]
                - s[15 as ::core::ffi::c_int as usize]
                + s[33 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize]
                + s[38 as ::core::ffi::c_int as usize]
                + s[74 as ::core::ffi::c_int as usize]
                - s[6 as ::core::ffi::c_int as usize]
                - s[42 as ::core::ffi::c_int as usize]
                - s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = -s[46 as ::core::ffi::c_int as usize]
                + s[64 as ::core::ffi::c_int as usize]
                + s[48 as ::core::ffi::c_int as usize]
                - s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = -s[55 as ::core::ffi::c_int as usize]
                + s[19 as ::core::ffi::c_int as usize]
                + s[57 as ::core::ffi::c_int as usize]
                - s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = -s[10 as ::core::ffi::c_int as usize]
                + s[28 as ::core::ffi::c_int as usize]
                + s[12 as ::core::ffi::c_int as usize]
                - s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = -s[1 as ::core::ffi::c_int as usize]
                - s[37 as ::core::ffi::c_int as usize]
                - s[73 as ::core::ffi::c_int as usize]
                + s[3 as ::core::ffi::c_int as usize]
                + s[39 as ::core::ffi::c_int as usize]
                + s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = -s[45 as ::core::ffi::c_int as usize]
                + s[63 as ::core::ffi::c_int as usize]
                - s[49 as ::core::ffi::c_int as usize]
                + s[67 as ::core::ffi::c_int as usize]
                - s[53 as ::core::ffi::c_int as usize]
                + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = -s[54 as ::core::ffi::c_int as usize]
                + s[18 as ::core::ffi::c_int as usize]
                - s[58 as ::core::ffi::c_int as usize]
                + s[22 as ::core::ffi::c_int as usize]
                - s[62 as ::core::ffi::c_int as usize]
                + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = -s[9 as ::core::ffi::c_int as usize]
                + s[27 as ::core::ffi::c_int as usize]
                - s[13 as ::core::ffi::c_int as usize]
                + s[31 as ::core::ffi::c_int as usize]
                - s[17 as ::core::ffi::c_int as usize]
                + s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = -s[0 as ::core::ffi::c_int as usize]
                - s[36 as ::core::ffi::c_int as usize]
                - s[72 as ::core::ffi::c_int as usize]
                - s[4 as ::core::ffi::c_int as usize]
                - s[40 as ::core::ffi::c_int as usize]
                - s[76 as ::core::ffi::c_int as usize]
                - s[8 as ::core::ffi::c_int as usize]
                - s[44 as ::core::ffi::c_int as usize]
                - s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += -s[50 as ::core::ffi::c_int as usize]
                    + s[68 as ::core::ffi::c_int as usize]
                    + s[52 as ::core::ffi::c_int as usize]
                    - s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += -s[59 as ::core::ffi::c_int as usize]
                    + s[23 as ::core::ffi::c_int as usize]
                    + s[61 as ::core::ffi::c_int as usize]
                    - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += -s[14 as ::core::ffi::c_int as usize]
                    + s[32 as ::core::ffi::c_int as usize]
                    + s[16 as ::core::ffi::c_int as usize]
                    - s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += -s[5 as ::core::ffi::c_int as usize]
                    - s[41 as ::core::ffi::c_int as usize]
                    - s[77 as ::core::ffi::c_int as usize]
                    + s[7 as ::core::ffi::c_int as usize]
                    + s[43 as ::core::ffi::c_int as usize]
                    + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += s[47 as ::core::ffi::c_int as usize]
                    - s[65 as ::core::ffi::c_int as usize]
                    - s[51 as ::core::ffi::c_int as usize]
                    + s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[56 as ::core::ffi::c_int as usize]
                    - s[20 as ::core::ffi::c_int as usize]
                    - s[60 as ::core::ffi::c_int as usize]
                    + s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += s[11 as ::core::ffi::c_int as usize]
                    - s[29 as ::core::ffi::c_int as usize]
                    - s[15 as ::core::ffi::c_int as usize]
                    + s[33 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += s[2 as ::core::ffi::c_int as usize]
                    + s[38 as ::core::ffi::c_int as usize]
                    + s[74 as ::core::ffi::c_int as usize]
                    - s[6 as ::core::ffi::c_int as usize]
                    - s[42 as ::core::ffi::c_int as usize]
                    - s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += -s[46 as ::core::ffi::c_int as usize]
                    + s[64 as ::core::ffi::c_int as usize]
                    + s[48 as ::core::ffi::c_int as usize]
                    - s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += -s[55 as ::core::ffi::c_int as usize]
                    + s[19 as ::core::ffi::c_int as usize]
                    + s[57 as ::core::ffi::c_int as usize]
                    - s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += -s[10 as ::core::ffi::c_int as usize]
                    + s[28 as ::core::ffi::c_int as usize]
                    + s[12 as ::core::ffi::c_int as usize]
                    - s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += -s[1 as ::core::ffi::c_int as usize]
                    - s[37 as ::core::ffi::c_int as usize]
                    - s[73 as ::core::ffi::c_int as usize]
                    + s[3 as ::core::ffi::c_int as usize]
                    + s[39 as ::core::ffi::c_int as usize]
                    + s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                )
                += -s[45 as ::core::ffi::c_int as usize]
                    + s[63 as ::core::ffi::c_int as usize]
                    - s[49 as ::core::ffi::c_int as usize]
                    + s[67 as ::core::ffi::c_int as usize]
                    - s[53 as ::core::ffi::c_int as usize]
                    + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                )
                += -s[54 as ::core::ffi::c_int as usize]
                    + s[18 as ::core::ffi::c_int as usize]
                    - s[58 as ::core::ffi::c_int as usize]
                    + s[22 as ::core::ffi::c_int as usize]
                    - s[62 as ::core::ffi::c_int as usize]
                    + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                )
                += -s[9 as ::core::ffi::c_int as usize]
                    + s[27 as ::core::ffi::c_int as usize]
                    - s[13 as ::core::ffi::c_int as usize]
                    + s[31 as ::core::ffi::c_int as usize]
                    - s[17 as ::core::ffi::c_int as usize]
                    + s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                )
                += -s[0 as ::core::ffi::c_int as usize]
                    - s[36 as ::core::ffi::c_int as usize]
                    - s[72 as ::core::ffi::c_int as usize]
                    - s[4 as ::core::ffi::c_int as usize]
                    - s[40 as ::core::ffi::c_int as usize]
                    - s[76 as ::core::ffi::c_int as usize]
                    - s[8 as ::core::ffi::c_int as usize]
                    - s[44 as ::core::ffi::c_int as usize]
                    - s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_gauge_r1_sps1ssp2_optimizer(
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
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int2e_gauge_r1_sps1ssp2_cart(
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
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r1_sps1ssp2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn int2e_gauge_r1_sps1ssp2_sph(
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
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r1_sps1ssp2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn cint2e_gauge_r1_sps1ssp2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r1_sps1ssp2_cart(
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
pub unsafe extern "C" fn cint2e_gauge_r1_sps1ssp2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r1_sps1ssp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r1_sps1ssp2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r1_sps1ssp2_sph(
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
pub unsafe extern "C" fn cint2e_gauge_r1_sps1ssp2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r1_sps1ssp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r1_sps1ssp2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r1_sps1ssp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_gauge_r1_sps1sps2(
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
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l,
        envs,
    );
    CINTx1j_2e(
        g2,
        g0,
        (*envs).rj,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTx1j_2e(
        g3,
        g1,
        (*envs).rj,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g4,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g4.offset(ix as isize) += *g5.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g5,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g5.offset(ix as isize) += *g6.offset(ix as isize);
        ix += 1;
    }
    CINTx1j_2e(
        g6,
        g4,
        (*envs).rj,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTx1j_2e(
        g7,
        g5,
        (*envs).rj,
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
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = -s[50 as ::core::ffi::c_int as usize]
                + s[68 as ::core::ffi::c_int as usize]
                + s[52 as ::core::ffi::c_int as usize]
                - s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = -s[59 as ::core::ffi::c_int as usize]
                + s[23 as ::core::ffi::c_int as usize]
                + s[61 as ::core::ffi::c_int as usize]
                - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = -s[14 as ::core::ffi::c_int as usize]
                + s[32 as ::core::ffi::c_int as usize]
                + s[16 as ::core::ffi::c_int as usize]
                - s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = -s[5 as ::core::ffi::c_int as usize]
                - s[41 as ::core::ffi::c_int as usize]
                - s[77 as ::core::ffi::c_int as usize]
                + s[7 as ::core::ffi::c_int as usize]
                + s[43 as ::core::ffi::c_int as usize]
                + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[47 as ::core::ffi::c_int as usize]
                - s[65 as ::core::ffi::c_int as usize]
                - s[51 as ::core::ffi::c_int as usize]
                + s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[56 as ::core::ffi::c_int as usize]
                - s[20 as ::core::ffi::c_int as usize]
                - s[60 as ::core::ffi::c_int as usize]
                + s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[11 as ::core::ffi::c_int as usize]
                - s[29 as ::core::ffi::c_int as usize]
                - s[15 as ::core::ffi::c_int as usize]
                + s[33 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[2 as ::core::ffi::c_int as usize]
                + s[38 as ::core::ffi::c_int as usize]
                + s[74 as ::core::ffi::c_int as usize]
                - s[6 as ::core::ffi::c_int as usize]
                - s[42 as ::core::ffi::c_int as usize]
                - s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = -s[46 as ::core::ffi::c_int as usize]
                + s[64 as ::core::ffi::c_int as usize]
                + s[48 as ::core::ffi::c_int as usize]
                - s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = -s[55 as ::core::ffi::c_int as usize]
                + s[19 as ::core::ffi::c_int as usize]
                + s[57 as ::core::ffi::c_int as usize]
                - s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = -s[10 as ::core::ffi::c_int as usize]
                + s[28 as ::core::ffi::c_int as usize]
                + s[12 as ::core::ffi::c_int as usize]
                - s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = -s[1 as ::core::ffi::c_int as usize]
                - s[37 as ::core::ffi::c_int as usize]
                - s[73 as ::core::ffi::c_int as usize]
                + s[3 as ::core::ffi::c_int as usize]
                + s[39 as ::core::ffi::c_int as usize]
                + s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = s[45 as ::core::ffi::c_int as usize]
                - s[63 as ::core::ffi::c_int as usize]
                + s[49 as ::core::ffi::c_int as usize]
                - s[67 as ::core::ffi::c_int as usize]
                + s[53 as ::core::ffi::c_int as usize]
                - s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize]
                - s[18 as ::core::ffi::c_int as usize]
                + s[58 as ::core::ffi::c_int as usize]
                - s[22 as ::core::ffi::c_int as usize]
                + s[62 as ::core::ffi::c_int as usize]
                - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize]
                - s[27 as ::core::ffi::c_int as usize]
                + s[13 as ::core::ffi::c_int as usize]
                - s[31 as ::core::ffi::c_int as usize]
                + s[17 as ::core::ffi::c_int as usize]
                - s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[36 as ::core::ffi::c_int as usize]
                + s[72 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize]
                + s[40 as ::core::ffi::c_int as usize]
                + s[76 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize]
                + s[44 as ::core::ffi::c_int as usize]
                + s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += -s[50 as ::core::ffi::c_int as usize]
                    + s[68 as ::core::ffi::c_int as usize]
                    + s[52 as ::core::ffi::c_int as usize]
                    - s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += -s[59 as ::core::ffi::c_int as usize]
                    + s[23 as ::core::ffi::c_int as usize]
                    + s[61 as ::core::ffi::c_int as usize]
                    - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += -s[14 as ::core::ffi::c_int as usize]
                    + s[32 as ::core::ffi::c_int as usize]
                    + s[16 as ::core::ffi::c_int as usize]
                    - s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += -s[5 as ::core::ffi::c_int as usize]
                    - s[41 as ::core::ffi::c_int as usize]
                    - s[77 as ::core::ffi::c_int as usize]
                    + s[7 as ::core::ffi::c_int as usize]
                    + s[43 as ::core::ffi::c_int as usize]
                    + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += s[47 as ::core::ffi::c_int as usize]
                    - s[65 as ::core::ffi::c_int as usize]
                    - s[51 as ::core::ffi::c_int as usize]
                    + s[69 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[56 as ::core::ffi::c_int as usize]
                    - s[20 as ::core::ffi::c_int as usize]
                    - s[60 as ::core::ffi::c_int as usize]
                    + s[24 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += s[11 as ::core::ffi::c_int as usize]
                    - s[29 as ::core::ffi::c_int as usize]
                    - s[15 as ::core::ffi::c_int as usize]
                    + s[33 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += s[2 as ::core::ffi::c_int as usize]
                    + s[38 as ::core::ffi::c_int as usize]
                    + s[74 as ::core::ffi::c_int as usize]
                    - s[6 as ::core::ffi::c_int as usize]
                    - s[42 as ::core::ffi::c_int as usize]
                    - s[78 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += -s[46 as ::core::ffi::c_int as usize]
                    + s[64 as ::core::ffi::c_int as usize]
                    + s[48 as ::core::ffi::c_int as usize]
                    - s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += -s[55 as ::core::ffi::c_int as usize]
                    + s[19 as ::core::ffi::c_int as usize]
                    + s[57 as ::core::ffi::c_int as usize]
                    - s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += -s[10 as ::core::ffi::c_int as usize]
                    + s[28 as ::core::ffi::c_int as usize]
                    + s[12 as ::core::ffi::c_int as usize]
                    - s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += -s[1 as ::core::ffi::c_int as usize]
                    - s[37 as ::core::ffi::c_int as usize]
                    - s[73 as ::core::ffi::c_int as usize]
                    + s[3 as ::core::ffi::c_int as usize]
                    + s[39 as ::core::ffi::c_int as usize]
                    + s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                )
                += s[45 as ::core::ffi::c_int as usize]
                    - s[63 as ::core::ffi::c_int as usize]
                    + s[49 as ::core::ffi::c_int as usize]
                    - s[67 as ::core::ffi::c_int as usize]
                    + s[53 as ::core::ffi::c_int as usize]
                    - s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                )
                += s[54 as ::core::ffi::c_int as usize]
                    - s[18 as ::core::ffi::c_int as usize]
                    + s[58 as ::core::ffi::c_int as usize]
                    - s[22 as ::core::ffi::c_int as usize]
                    + s[62 as ::core::ffi::c_int as usize]
                    - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                )
                += s[9 as ::core::ffi::c_int as usize]
                    - s[27 as ::core::ffi::c_int as usize]
                    + s[13 as ::core::ffi::c_int as usize]
                    - s[31 as ::core::ffi::c_int as usize]
                    + s[17 as ::core::ffi::c_int as usize]
                    - s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[36 as ::core::ffi::c_int as usize]
                    + s[72 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize]
                    + s[40 as ::core::ffi::c_int as usize]
                    + s[76 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize]
                    + s[44 as ::core::ffi::c_int as usize]
                    + s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_gauge_r1_sps1sps2_optimizer(
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
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int2e_gauge_r1_sps1sps2_cart(
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
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r1_sps1sps2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn int2e_gauge_r1_sps1sps2_sph(
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
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r1_sps1sps2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn cint2e_gauge_r1_sps1sps2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r1_sps1sps2_cart(
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
pub unsafe extern "C" fn cint2e_gauge_r1_sps1sps2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r1_sps1sps2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r1_sps1sps2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r1_sps1sps2_sph(
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
pub unsafe extern "C" fn cint2e_gauge_r1_sps1sps2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r1_sps1sps2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r1_sps1sps2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r1_sps1sps2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_gauge_r2_ssp1ssp2(
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
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l + 0 as ::core::ffi::c_int,
        envs,
    );
    CINTx1l_2e(
        g2,
        g0,
        (*envs).c2rust_unnamed_1.rl,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l + 1 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1l_2e(
        g3,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l + 0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1j_2e(
        g4,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g5,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g6,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g7,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g8.offset(ix as isize) += *g9.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g9.offset(ix as isize) += *g10.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g10,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g11,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g10.offset(ix as isize) += *g11.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g11,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g11.offset(ix as isize) += *g12.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g12,
        g8,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g13,
        g9,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g14,
        g10,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g15,
        g11,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
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
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[50 as ::core::ffi::c_int as usize]
                - s[68 as ::core::ffi::c_int as usize]
                - s[52 as ::core::ffi::c_int as usize]
                + s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[59 as ::core::ffi::c_int as usize]
                - s[23 as ::core::ffi::c_int as usize]
                - s[61 as ::core::ffi::c_int as usize]
                + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[14 as ::core::ffi::c_int as usize]
                - s[32 as ::core::ffi::c_int as usize]
                - s[16 as ::core::ffi::c_int as usize]
                + s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize]
                + s[41 as ::core::ffi::c_int as usize]
                + s[77 as ::core::ffi::c_int as usize]
                - s[7 as ::core::ffi::c_int as usize]
                - s[43 as ::core::ffi::c_int as usize]
                - s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[51 as ::core::ffi::c_int as usize]
                - s[69 as ::core::ffi::c_int as usize]
                - s[47 as ::core::ffi::c_int as usize]
                + s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[60 as ::core::ffi::c_int as usize]
                - s[24 as ::core::ffi::c_int as usize]
                - s[56 as ::core::ffi::c_int as usize]
                + s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[15 as ::core::ffi::c_int as usize]
                - s[33 as ::core::ffi::c_int as usize]
                - s[11 as ::core::ffi::c_int as usize]
                + s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize]
                + s[42 as ::core::ffi::c_int as usize]
                + s[78 as ::core::ffi::c_int as usize]
                - s[2 as ::core::ffi::c_int as usize]
                - s[38 as ::core::ffi::c_int as usize]
                - s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[46 as ::core::ffi::c_int as usize]
                - s[64 as ::core::ffi::c_int as usize]
                - s[48 as ::core::ffi::c_int as usize]
                + s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[55 as ::core::ffi::c_int as usize]
                - s[19 as ::core::ffi::c_int as usize]
                - s[57 as ::core::ffi::c_int as usize]
                + s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[10 as ::core::ffi::c_int as usize]
                - s[28 as ::core::ffi::c_int as usize]
                - s[12 as ::core::ffi::c_int as usize]
                + s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                + s[37 as ::core::ffi::c_int as usize]
                + s[73 as ::core::ffi::c_int as usize]
                - s[3 as ::core::ffi::c_int as usize]
                - s[39 as ::core::ffi::c_int as usize]
                - s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = s[45 as ::core::ffi::c_int as usize]
                - s[63 as ::core::ffi::c_int as usize]
                + s[49 as ::core::ffi::c_int as usize]
                - s[67 as ::core::ffi::c_int as usize]
                + s[53 as ::core::ffi::c_int as usize]
                - s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize]
                - s[18 as ::core::ffi::c_int as usize]
                + s[58 as ::core::ffi::c_int as usize]
                - s[22 as ::core::ffi::c_int as usize]
                + s[62 as ::core::ffi::c_int as usize]
                - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize]
                - s[27 as ::core::ffi::c_int as usize]
                + s[13 as ::core::ffi::c_int as usize]
                - s[31 as ::core::ffi::c_int as usize]
                + s[17 as ::core::ffi::c_int as usize]
                - s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[36 as ::core::ffi::c_int as usize]
                + s[72 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize]
                + s[40 as ::core::ffi::c_int as usize]
                + s[76 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize]
                + s[44 as ::core::ffi::c_int as usize]
                + s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[50 as ::core::ffi::c_int as usize]
                    - s[68 as ::core::ffi::c_int as usize]
                    - s[52 as ::core::ffi::c_int as usize]
                    + s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += s[59 as ::core::ffi::c_int as usize]
                    - s[23 as ::core::ffi::c_int as usize]
                    - s[61 as ::core::ffi::c_int as usize]
                    + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += s[14 as ::core::ffi::c_int as usize]
                    - s[32 as ::core::ffi::c_int as usize]
                    - s[16 as ::core::ffi::c_int as usize]
                    + s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[5 as ::core::ffi::c_int as usize]
                    + s[41 as ::core::ffi::c_int as usize]
                    + s[77 as ::core::ffi::c_int as usize]
                    - s[7 as ::core::ffi::c_int as usize]
                    - s[43 as ::core::ffi::c_int as usize]
                    - s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += s[51 as ::core::ffi::c_int as usize]
                    - s[69 as ::core::ffi::c_int as usize]
                    - s[47 as ::core::ffi::c_int as usize]
                    + s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[60 as ::core::ffi::c_int as usize]
                    - s[24 as ::core::ffi::c_int as usize]
                    - s[56 as ::core::ffi::c_int as usize]
                    + s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += s[15 as ::core::ffi::c_int as usize]
                    - s[33 as ::core::ffi::c_int as usize]
                    - s[11 as ::core::ffi::c_int as usize]
                    + s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += s[6 as ::core::ffi::c_int as usize]
                    + s[42 as ::core::ffi::c_int as usize]
                    + s[78 as ::core::ffi::c_int as usize]
                    - s[2 as ::core::ffi::c_int as usize]
                    - s[38 as ::core::ffi::c_int as usize]
                    - s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += s[46 as ::core::ffi::c_int as usize]
                    - s[64 as ::core::ffi::c_int as usize]
                    - s[48 as ::core::ffi::c_int as usize]
                    + s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += s[55 as ::core::ffi::c_int as usize]
                    - s[19 as ::core::ffi::c_int as usize]
                    - s[57 as ::core::ffi::c_int as usize]
                    + s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[10 as ::core::ffi::c_int as usize]
                    - s[28 as ::core::ffi::c_int as usize]
                    - s[12 as ::core::ffi::c_int as usize]
                    + s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[1 as ::core::ffi::c_int as usize]
                    + s[37 as ::core::ffi::c_int as usize]
                    + s[73 as ::core::ffi::c_int as usize]
                    - s[3 as ::core::ffi::c_int as usize]
                    - s[39 as ::core::ffi::c_int as usize]
                    - s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                )
                += s[45 as ::core::ffi::c_int as usize]
                    - s[63 as ::core::ffi::c_int as usize]
                    + s[49 as ::core::ffi::c_int as usize]
                    - s[67 as ::core::ffi::c_int as usize]
                    + s[53 as ::core::ffi::c_int as usize]
                    - s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                )
                += s[54 as ::core::ffi::c_int as usize]
                    - s[18 as ::core::ffi::c_int as usize]
                    + s[58 as ::core::ffi::c_int as usize]
                    - s[22 as ::core::ffi::c_int as usize]
                    + s[62 as ::core::ffi::c_int as usize]
                    - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                )
                += s[9 as ::core::ffi::c_int as usize]
                    - s[27 as ::core::ffi::c_int as usize]
                    + s[13 as ::core::ffi::c_int as usize]
                    - s[31 as ::core::ffi::c_int as usize]
                    + s[17 as ::core::ffi::c_int as usize]
                    - s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[36 as ::core::ffi::c_int as usize]
                    + s[72 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize]
                    + s[40 as ::core::ffi::c_int as usize]
                    + s[76 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize]
                    + s[44 as ::core::ffi::c_int as usize]
                    + s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_gauge_r2_ssp1ssp2_optimizer(
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
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int2e_gauge_r2_ssp1ssp2_cart(
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
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r2_ssp1ssp2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn int2e_gauge_r2_ssp1ssp2_sph(
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
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r2_ssp1ssp2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn cint2e_gauge_r2_ssp1ssp2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r2_ssp1ssp2_cart(
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
pub unsafe extern "C" fn cint2e_gauge_r2_ssp1ssp2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r2_ssp1ssp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r2_ssp1ssp2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r2_ssp1ssp2_sph(
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
pub unsafe extern "C" fn cint2e_gauge_r2_ssp1ssp2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r2_ssp1ssp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r2_ssp1ssp2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r2_ssp1ssp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_gauge_r2_ssp1sps2(
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
    CINTx1l_2e(
        g1,
        g0,
        (*envs).c2rust_unnamed_1.rl,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l + 1 as ::core::ffi::c_int,
        (*envs).l_l + 0 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1k_2e(
        g2,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l,
        envs,
    );
    CINTnabla1k_2e(
        g3,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 2 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g4,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g5,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g6,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g7,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g8,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g9,
        g0,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g8.offset(ix as isize) += *g9.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g9,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g10,
        g1,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g9.offset(ix as isize) += *g10.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g10,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g11,
        g2,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g10.offset(ix as isize) += *g11.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g11,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g12,
        g3,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g11.offset(ix as isize) += *g12.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g12,
        g8,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g13,
        g9,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g14,
        g10,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1j_2e(
        g15,
        g11,
        (*envs).i_l + 0 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
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
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = -s[50 as ::core::ffi::c_int as usize]
                + s[68 as ::core::ffi::c_int as usize]
                + s[52 as ::core::ffi::c_int as usize]
                - s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = -s[59 as ::core::ffi::c_int as usize]
                + s[23 as ::core::ffi::c_int as usize]
                + s[61 as ::core::ffi::c_int as usize]
                - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = -s[14 as ::core::ffi::c_int as usize]
                + s[32 as ::core::ffi::c_int as usize]
                + s[16 as ::core::ffi::c_int as usize]
                - s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = -s[5 as ::core::ffi::c_int as usize]
                - s[41 as ::core::ffi::c_int as usize]
                - s[77 as ::core::ffi::c_int as usize]
                + s[7 as ::core::ffi::c_int as usize]
                + s[43 as ::core::ffi::c_int as usize]
                + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = -s[51 as ::core::ffi::c_int as usize]
                + s[69 as ::core::ffi::c_int as usize]
                + s[47 as ::core::ffi::c_int as usize]
                - s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = -s[60 as ::core::ffi::c_int as usize]
                + s[24 as ::core::ffi::c_int as usize]
                + s[56 as ::core::ffi::c_int as usize]
                - s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = -s[15 as ::core::ffi::c_int as usize]
                + s[33 as ::core::ffi::c_int as usize]
                + s[11 as ::core::ffi::c_int as usize]
                - s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = -s[6 as ::core::ffi::c_int as usize]
                - s[42 as ::core::ffi::c_int as usize]
                - s[78 as ::core::ffi::c_int as usize]
                + s[2 as ::core::ffi::c_int as usize]
                + s[38 as ::core::ffi::c_int as usize]
                + s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = -s[46 as ::core::ffi::c_int as usize]
                + s[64 as ::core::ffi::c_int as usize]
                + s[48 as ::core::ffi::c_int as usize]
                - s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = -s[55 as ::core::ffi::c_int as usize]
                + s[19 as ::core::ffi::c_int as usize]
                + s[57 as ::core::ffi::c_int as usize]
                - s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = -s[10 as ::core::ffi::c_int as usize]
                + s[28 as ::core::ffi::c_int as usize]
                + s[12 as ::core::ffi::c_int as usize]
                - s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = -s[1 as ::core::ffi::c_int as usize]
                - s[37 as ::core::ffi::c_int as usize]
                - s[73 as ::core::ffi::c_int as usize]
                + s[3 as ::core::ffi::c_int as usize]
                + s[39 as ::core::ffi::c_int as usize]
                + s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = -s[45 as ::core::ffi::c_int as usize]
                + s[63 as ::core::ffi::c_int as usize]
                - s[49 as ::core::ffi::c_int as usize]
                + s[67 as ::core::ffi::c_int as usize]
                - s[53 as ::core::ffi::c_int as usize]
                + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = -s[54 as ::core::ffi::c_int as usize]
                + s[18 as ::core::ffi::c_int as usize]
                - s[58 as ::core::ffi::c_int as usize]
                + s[22 as ::core::ffi::c_int as usize]
                - s[62 as ::core::ffi::c_int as usize]
                + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = -s[9 as ::core::ffi::c_int as usize]
                + s[27 as ::core::ffi::c_int as usize]
                - s[13 as ::core::ffi::c_int as usize]
                + s[31 as ::core::ffi::c_int as usize]
                - s[17 as ::core::ffi::c_int as usize]
                + s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = -s[0 as ::core::ffi::c_int as usize]
                - s[36 as ::core::ffi::c_int as usize]
                - s[72 as ::core::ffi::c_int as usize]
                - s[4 as ::core::ffi::c_int as usize]
                - s[40 as ::core::ffi::c_int as usize]
                - s[76 as ::core::ffi::c_int as usize]
                - s[8 as ::core::ffi::c_int as usize]
                - s[44 as ::core::ffi::c_int as usize]
                - s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += -s[50 as ::core::ffi::c_int as usize]
                    + s[68 as ::core::ffi::c_int as usize]
                    + s[52 as ::core::ffi::c_int as usize]
                    - s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += -s[59 as ::core::ffi::c_int as usize]
                    + s[23 as ::core::ffi::c_int as usize]
                    + s[61 as ::core::ffi::c_int as usize]
                    - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += -s[14 as ::core::ffi::c_int as usize]
                    + s[32 as ::core::ffi::c_int as usize]
                    + s[16 as ::core::ffi::c_int as usize]
                    - s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += -s[5 as ::core::ffi::c_int as usize]
                    - s[41 as ::core::ffi::c_int as usize]
                    - s[77 as ::core::ffi::c_int as usize]
                    + s[7 as ::core::ffi::c_int as usize]
                    + s[43 as ::core::ffi::c_int as usize]
                    + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += -s[51 as ::core::ffi::c_int as usize]
                    + s[69 as ::core::ffi::c_int as usize]
                    + s[47 as ::core::ffi::c_int as usize]
                    - s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += -s[60 as ::core::ffi::c_int as usize]
                    + s[24 as ::core::ffi::c_int as usize]
                    + s[56 as ::core::ffi::c_int as usize]
                    - s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += -s[15 as ::core::ffi::c_int as usize]
                    + s[33 as ::core::ffi::c_int as usize]
                    + s[11 as ::core::ffi::c_int as usize]
                    - s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += -s[6 as ::core::ffi::c_int as usize]
                    - s[42 as ::core::ffi::c_int as usize]
                    - s[78 as ::core::ffi::c_int as usize]
                    + s[2 as ::core::ffi::c_int as usize]
                    + s[38 as ::core::ffi::c_int as usize]
                    + s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += -s[46 as ::core::ffi::c_int as usize]
                    + s[64 as ::core::ffi::c_int as usize]
                    + s[48 as ::core::ffi::c_int as usize]
                    - s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += -s[55 as ::core::ffi::c_int as usize]
                    + s[19 as ::core::ffi::c_int as usize]
                    + s[57 as ::core::ffi::c_int as usize]
                    - s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += -s[10 as ::core::ffi::c_int as usize]
                    + s[28 as ::core::ffi::c_int as usize]
                    + s[12 as ::core::ffi::c_int as usize]
                    - s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += -s[1 as ::core::ffi::c_int as usize]
                    - s[37 as ::core::ffi::c_int as usize]
                    - s[73 as ::core::ffi::c_int as usize]
                    + s[3 as ::core::ffi::c_int as usize]
                    + s[39 as ::core::ffi::c_int as usize]
                    + s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                )
                += -s[45 as ::core::ffi::c_int as usize]
                    + s[63 as ::core::ffi::c_int as usize]
                    - s[49 as ::core::ffi::c_int as usize]
                    + s[67 as ::core::ffi::c_int as usize]
                    - s[53 as ::core::ffi::c_int as usize]
                    + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                )
                += -s[54 as ::core::ffi::c_int as usize]
                    + s[18 as ::core::ffi::c_int as usize]
                    - s[58 as ::core::ffi::c_int as usize]
                    + s[22 as ::core::ffi::c_int as usize]
                    - s[62 as ::core::ffi::c_int as usize]
                    + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                )
                += -s[9 as ::core::ffi::c_int as usize]
                    + s[27 as ::core::ffi::c_int as usize]
                    - s[13 as ::core::ffi::c_int as usize]
                    + s[31 as ::core::ffi::c_int as usize]
                    - s[17 as ::core::ffi::c_int as usize]
                    + s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                )
                += -s[0 as ::core::ffi::c_int as usize]
                    - s[36 as ::core::ffi::c_int as usize]
                    - s[72 as ::core::ffi::c_int as usize]
                    - s[4 as ::core::ffi::c_int as usize]
                    - s[40 as ::core::ffi::c_int as usize]
                    - s[76 as ::core::ffi::c_int as usize]
                    - s[8 as ::core::ffi::c_int as usize]
                    - s[44 as ::core::ffi::c_int as usize]
                    - s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_gauge_r2_ssp1sps2_optimizer(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int2e_gauge_r2_ssp1sps2_cart(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r2_ssp1sps2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn int2e_gauge_r2_ssp1sps2_sph(
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
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r2_ssp1sps2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn cint2e_gauge_r2_ssp1sps2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r2_ssp1sps2_cart(
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
pub unsafe extern "C" fn cint2e_gauge_r2_ssp1sps2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r2_ssp1sps2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r2_ssp1sps2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r2_ssp1sps2_sph(
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
pub unsafe extern "C" fn cint2e_gauge_r2_ssp1sps2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r2_ssp1sps2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r2_ssp1sps2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r2_ssp1sps2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_gauge_r2_sps1ssp2(
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
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l + 0 as ::core::ffi::c_int,
        envs,
    );
    CINTx1l_2e(
        g2,
        g0,
        (*envs).c2rust_unnamed_1.rl,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l + 1 as ::core::ffi::c_int,
        envs,
    );
    CINTnabla1l_2e(
        g3,
        g2,
        (*envs).i_l + 2 as ::core::ffi::c_int,
        (*envs).j_l + 1 as ::core::ffi::c_int,
        (*envs).k_l + 0 as ::core::ffi::c_int,
        (*envs).l_l + 0 as ::core::ffi::c_int,
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
    CINTnabla1i_2e(
        g5,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g4.offset(ix as isize) += *g5.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g5,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g5.offset(ix as isize) += *g6.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g6,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g6.offset(ix as isize) += *g7.offset(ix as isize);
        ix += 1;
    }
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
        g3,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g7.offset(ix as isize) += *g8.offset(ix as isize);
        ix += 1;
    }
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
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = -s[50 as ::core::ffi::c_int as usize]
                + s[68 as ::core::ffi::c_int as usize]
                + s[52 as ::core::ffi::c_int as usize]
                - s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = -s[59 as ::core::ffi::c_int as usize]
                + s[23 as ::core::ffi::c_int as usize]
                + s[61 as ::core::ffi::c_int as usize]
                - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = -s[14 as ::core::ffi::c_int as usize]
                + s[32 as ::core::ffi::c_int as usize]
                + s[16 as ::core::ffi::c_int as usize]
                - s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = -s[5 as ::core::ffi::c_int as usize]
                - s[41 as ::core::ffi::c_int as usize]
                - s[77 as ::core::ffi::c_int as usize]
                + s[7 as ::core::ffi::c_int as usize]
                + s[43 as ::core::ffi::c_int as usize]
                + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = -s[51 as ::core::ffi::c_int as usize]
                + s[69 as ::core::ffi::c_int as usize]
                + s[47 as ::core::ffi::c_int as usize]
                - s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = -s[60 as ::core::ffi::c_int as usize]
                + s[24 as ::core::ffi::c_int as usize]
                + s[56 as ::core::ffi::c_int as usize]
                - s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = -s[15 as ::core::ffi::c_int as usize]
                + s[33 as ::core::ffi::c_int as usize]
                + s[11 as ::core::ffi::c_int as usize]
                - s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = -s[6 as ::core::ffi::c_int as usize]
                - s[42 as ::core::ffi::c_int as usize]
                - s[78 as ::core::ffi::c_int as usize]
                + s[2 as ::core::ffi::c_int as usize]
                + s[38 as ::core::ffi::c_int as usize]
                + s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = -s[46 as ::core::ffi::c_int as usize]
                + s[64 as ::core::ffi::c_int as usize]
                + s[48 as ::core::ffi::c_int as usize]
                - s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = -s[55 as ::core::ffi::c_int as usize]
                + s[19 as ::core::ffi::c_int as usize]
                + s[57 as ::core::ffi::c_int as usize]
                - s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = -s[10 as ::core::ffi::c_int as usize]
                + s[28 as ::core::ffi::c_int as usize]
                + s[12 as ::core::ffi::c_int as usize]
                - s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = -s[1 as ::core::ffi::c_int as usize]
                - s[37 as ::core::ffi::c_int as usize]
                - s[73 as ::core::ffi::c_int as usize]
                + s[3 as ::core::ffi::c_int as usize]
                + s[39 as ::core::ffi::c_int as usize]
                + s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = -s[45 as ::core::ffi::c_int as usize]
                + s[63 as ::core::ffi::c_int as usize]
                - s[49 as ::core::ffi::c_int as usize]
                + s[67 as ::core::ffi::c_int as usize]
                - s[53 as ::core::ffi::c_int as usize]
                + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = -s[54 as ::core::ffi::c_int as usize]
                + s[18 as ::core::ffi::c_int as usize]
                - s[58 as ::core::ffi::c_int as usize]
                + s[22 as ::core::ffi::c_int as usize]
                - s[62 as ::core::ffi::c_int as usize]
                + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = -s[9 as ::core::ffi::c_int as usize]
                + s[27 as ::core::ffi::c_int as usize]
                - s[13 as ::core::ffi::c_int as usize]
                + s[31 as ::core::ffi::c_int as usize]
                - s[17 as ::core::ffi::c_int as usize]
                + s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = -s[0 as ::core::ffi::c_int as usize]
                - s[36 as ::core::ffi::c_int as usize]
                - s[72 as ::core::ffi::c_int as usize]
                - s[4 as ::core::ffi::c_int as usize]
                - s[40 as ::core::ffi::c_int as usize]
                - s[76 as ::core::ffi::c_int as usize]
                - s[8 as ::core::ffi::c_int as usize]
                - s[44 as ::core::ffi::c_int as usize]
                - s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += -s[50 as ::core::ffi::c_int as usize]
                    + s[68 as ::core::ffi::c_int as usize]
                    + s[52 as ::core::ffi::c_int as usize]
                    - s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += -s[59 as ::core::ffi::c_int as usize]
                    + s[23 as ::core::ffi::c_int as usize]
                    + s[61 as ::core::ffi::c_int as usize]
                    - s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += -s[14 as ::core::ffi::c_int as usize]
                    + s[32 as ::core::ffi::c_int as usize]
                    + s[16 as ::core::ffi::c_int as usize]
                    - s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += -s[5 as ::core::ffi::c_int as usize]
                    - s[41 as ::core::ffi::c_int as usize]
                    - s[77 as ::core::ffi::c_int as usize]
                    + s[7 as ::core::ffi::c_int as usize]
                    + s[43 as ::core::ffi::c_int as usize]
                    + s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += -s[51 as ::core::ffi::c_int as usize]
                    + s[69 as ::core::ffi::c_int as usize]
                    + s[47 as ::core::ffi::c_int as usize]
                    - s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += -s[60 as ::core::ffi::c_int as usize]
                    + s[24 as ::core::ffi::c_int as usize]
                    + s[56 as ::core::ffi::c_int as usize]
                    - s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += -s[15 as ::core::ffi::c_int as usize]
                    + s[33 as ::core::ffi::c_int as usize]
                    + s[11 as ::core::ffi::c_int as usize]
                    - s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += -s[6 as ::core::ffi::c_int as usize]
                    - s[42 as ::core::ffi::c_int as usize]
                    - s[78 as ::core::ffi::c_int as usize]
                    + s[2 as ::core::ffi::c_int as usize]
                    + s[38 as ::core::ffi::c_int as usize]
                    + s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += -s[46 as ::core::ffi::c_int as usize]
                    + s[64 as ::core::ffi::c_int as usize]
                    + s[48 as ::core::ffi::c_int as usize]
                    - s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += -s[55 as ::core::ffi::c_int as usize]
                    + s[19 as ::core::ffi::c_int as usize]
                    + s[57 as ::core::ffi::c_int as usize]
                    - s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += -s[10 as ::core::ffi::c_int as usize]
                    + s[28 as ::core::ffi::c_int as usize]
                    + s[12 as ::core::ffi::c_int as usize]
                    - s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += -s[1 as ::core::ffi::c_int as usize]
                    - s[37 as ::core::ffi::c_int as usize]
                    - s[73 as ::core::ffi::c_int as usize]
                    + s[3 as ::core::ffi::c_int as usize]
                    + s[39 as ::core::ffi::c_int as usize]
                    + s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                )
                += -s[45 as ::core::ffi::c_int as usize]
                    + s[63 as ::core::ffi::c_int as usize]
                    - s[49 as ::core::ffi::c_int as usize]
                    + s[67 as ::core::ffi::c_int as usize]
                    - s[53 as ::core::ffi::c_int as usize]
                    + s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                )
                += -s[54 as ::core::ffi::c_int as usize]
                    + s[18 as ::core::ffi::c_int as usize]
                    - s[58 as ::core::ffi::c_int as usize]
                    + s[22 as ::core::ffi::c_int as usize]
                    - s[62 as ::core::ffi::c_int as usize]
                    + s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                )
                += -s[9 as ::core::ffi::c_int as usize]
                    + s[27 as ::core::ffi::c_int as usize]
                    - s[13 as ::core::ffi::c_int as usize]
                    + s[31 as ::core::ffi::c_int as usize]
                    - s[17 as ::core::ffi::c_int as usize]
                    + s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                )
                += -s[0 as ::core::ffi::c_int as usize]
                    - s[36 as ::core::ffi::c_int as usize]
                    - s[72 as ::core::ffi::c_int as usize]
                    - s[4 as ::core::ffi::c_int as usize]
                    - s[40 as ::core::ffi::c_int as usize]
                    - s[76 as ::core::ffi::c_int as usize]
                    - s[8 as ::core::ffi::c_int as usize]
                    - s[44 as ::core::ffi::c_int as usize]
                    - s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_gauge_r2_sps1ssp2_optimizer(
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
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int2e_gauge_r2_sps1ssp2_cart(
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
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r2_sps1ssp2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn int2e_gauge_r2_sps1ssp2_sph(
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
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r2_sps1ssp2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn cint2e_gauge_r2_sps1ssp2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r2_sps1ssp2_cart(
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
pub unsafe extern "C" fn cint2e_gauge_r2_sps1ssp2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r2_sps1ssp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r2_sps1ssp2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r2_sps1ssp2_sph(
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
pub unsafe extern "C" fn cint2e_gauge_r2_sps1ssp2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r2_sps1ssp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r2_sps1ssp2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r2_sps1ssp2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout2e_int2e_gauge_r2_sps1sps2(
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
    CINTx1l_2e(
        g1,
        g0,
        (*envs).c2rust_unnamed_1.rl,
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
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g5,
        g0,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g4.offset(ix as isize) += *g5.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g5,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g6,
        g1,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g5.offset(ix as isize) += *g6.offset(ix as isize);
        ix += 1;
    }
    CINTnabla1j_2e(
        g6,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    CINTnabla1i_2e(
        g7,
        g2,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g6.offset(ix as isize) += *g7.offset(ix as isize);
        ix += 1;
    }
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
        g3,
        (*envs).i_l + 1 as ::core::ffi::c_int,
        (*envs).j_l + 0 as ::core::ffi::c_int,
        (*envs).k_l,
        (*envs).l_l,
        envs,
    );
    ix = 0 as ::core::ffi::c_int;
    while ix < (*envs).g_size * 3 as ::core::ffi::c_int {
        *g7.offset(ix as isize) += *g8.offset(ix as isize);
        ix += 1;
    }
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
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) = s[50 as ::core::ffi::c_int as usize]
                - s[68 as ::core::ffi::c_int as usize]
                - s[52 as ::core::ffi::c_int as usize]
                + s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                ) = s[59 as ::core::ffi::c_int as usize]
                - s[23 as ::core::ffi::c_int as usize]
                - s[61 as ::core::ffi::c_int as usize]
                + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                ) = s[14 as ::core::ffi::c_int as usize]
                - s[32 as ::core::ffi::c_int as usize]
                - s[16 as ::core::ffi::c_int as usize]
                + s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                ) = s[5 as ::core::ffi::c_int as usize]
                + s[41 as ::core::ffi::c_int as usize]
                + s[77 as ::core::ffi::c_int as usize]
                - s[7 as ::core::ffi::c_int as usize]
                - s[43 as ::core::ffi::c_int as usize]
                - s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                ) = s[51 as ::core::ffi::c_int as usize]
                - s[69 as ::core::ffi::c_int as usize]
                - s[47 as ::core::ffi::c_int as usize]
                + s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                ) = s[60 as ::core::ffi::c_int as usize]
                - s[24 as ::core::ffi::c_int as usize]
                - s[56 as ::core::ffi::c_int as usize]
                + s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                ) = s[15 as ::core::ffi::c_int as usize]
                - s[33 as ::core::ffi::c_int as usize]
                - s[11 as ::core::ffi::c_int as usize]
                + s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                ) = s[6 as ::core::ffi::c_int as usize]
                + s[42 as ::core::ffi::c_int as usize]
                + s[78 as ::core::ffi::c_int as usize]
                - s[2 as ::core::ffi::c_int as usize]
                - s[38 as ::core::ffi::c_int as usize]
                - s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) = s[46 as ::core::ffi::c_int as usize]
                - s[64 as ::core::ffi::c_int as usize]
                - s[48 as ::core::ffi::c_int as usize]
                + s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                ) = s[55 as ::core::ffi::c_int as usize]
                - s[19 as ::core::ffi::c_int as usize]
                - s[57 as ::core::ffi::c_int as usize]
                + s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                ) = s[10 as ::core::ffi::c_int as usize]
                - s[28 as ::core::ffi::c_int as usize]
                - s[12 as ::core::ffi::c_int as usize]
                + s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                ) = s[1 as ::core::ffi::c_int as usize]
                + s[37 as ::core::ffi::c_int as usize]
                + s[73 as ::core::ffi::c_int as usize]
                - s[3 as ::core::ffi::c_int as usize]
                - s[39 as ::core::ffi::c_int as usize]
                - s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                ) = s[45 as ::core::ffi::c_int as usize]
                - s[63 as ::core::ffi::c_int as usize]
                + s[49 as ::core::ffi::c_int as usize]
                - s[67 as ::core::ffi::c_int as usize]
                + s[53 as ::core::ffi::c_int as usize]
                - s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                ) = s[54 as ::core::ffi::c_int as usize]
                - s[18 as ::core::ffi::c_int as usize]
                + s[58 as ::core::ffi::c_int as usize]
                - s[22 as ::core::ffi::c_int as usize]
                + s[62 as ::core::ffi::c_int as usize]
                - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                ) = s[9 as ::core::ffi::c_int as usize]
                - s[27 as ::core::ffi::c_int as usize]
                + s[13 as ::core::ffi::c_int as usize]
                - s[31 as ::core::ffi::c_int as usize]
                + s[17 as ::core::ffi::c_int as usize]
                - s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                ) = s[0 as ::core::ffi::c_int as usize]
                + s[36 as ::core::ffi::c_int as usize]
                + s[72 as ::core::ffi::c_int as usize]
                + s[4 as ::core::ffi::c_int as usize]
                + s[40 as ::core::ffi::c_int as usize]
                + s[76 as ::core::ffi::c_int as usize]
                + s[8 as ::core::ffi::c_int as usize]
                + s[44 as ::core::ffi::c_int as usize]
                + s[80 as ::core::ffi::c_int as usize];
        } else {
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                )
                += s[50 as ::core::ffi::c_int as usize]
                    - s[68 as ::core::ffi::c_int as usize]
                    - s[52 as ::core::ffi::c_int as usize]
                    + s[70 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                )
                += s[59 as ::core::ffi::c_int as usize]
                    - s[23 as ::core::ffi::c_int as usize]
                    - s[61 as ::core::ffi::c_int as usize]
                    + s[25 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                )
                += s[14 as ::core::ffi::c_int as usize]
                    - s[32 as ::core::ffi::c_int as usize]
                    - s[16 as ::core::ffi::c_int as usize]
                    + s[34 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
                )
                += s[5 as ::core::ffi::c_int as usize]
                    + s[41 as ::core::ffi::c_int as usize]
                    + s[77 as ::core::ffi::c_int as usize]
                    - s[7 as ::core::ffi::c_int as usize]
                    - s[43 as ::core::ffi::c_int as usize]
                    - s[79 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                )
                += s[51 as ::core::ffi::c_int as usize]
                    - s[69 as ::core::ffi::c_int as usize]
                    - s[47 as ::core::ffi::c_int as usize]
                    + s[65 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                )
                += s[60 as ::core::ffi::c_int as usize]
                    - s[24 as ::core::ffi::c_int as usize]
                    - s[56 as ::core::ffi::c_int as usize]
                    + s[20 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
                )
                += s[15 as ::core::ffi::c_int as usize]
                    - s[33 as ::core::ffi::c_int as usize]
                    - s[11 as ::core::ffi::c_int as usize]
                    + s[29 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
                )
                += s[6 as ::core::ffi::c_int as usize]
                    + s[42 as ::core::ffi::c_int as usize]
                    + s[78 as ::core::ffi::c_int as usize]
                    - s[2 as ::core::ffi::c_int as usize]
                    - s[38 as ::core::ffi::c_int as usize]
                    - s[74 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                )
                += s[46 as ::core::ffi::c_int as usize]
                    - s[64 as ::core::ffi::c_int as usize]
                    - s[48 as ::core::ffi::c_int as usize]
                    + s[66 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                )
                += s[55 as ::core::ffi::c_int as usize]
                    - s[19 as ::core::ffi::c_int as usize]
                    - s[57 as ::core::ffi::c_int as usize]
                    + s[21 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                )
                += s[10 as ::core::ffi::c_int as usize]
                    - s[28 as ::core::ffi::c_int as usize]
                    - s[12 as ::core::ffi::c_int as usize]
                    + s[30 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                )
                += s[1 as ::core::ffi::c_int as usize]
                    + s[37 as ::core::ffi::c_int as usize]
                    + s[73 as ::core::ffi::c_int as usize]
                    - s[3 as ::core::ffi::c_int as usize]
                    - s[39 as ::core::ffi::c_int as usize]
                    - s[75 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                )
                += s[45 as ::core::ffi::c_int as usize]
                    - s[63 as ::core::ffi::c_int as usize]
                    + s[49 as ::core::ffi::c_int as usize]
                    - s[67 as ::core::ffi::c_int as usize]
                    + s[53 as ::core::ffi::c_int as usize]
                    - s[71 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                )
                += s[54 as ::core::ffi::c_int as usize]
                    - s[18 as ::core::ffi::c_int as usize]
                    + s[58 as ::core::ffi::c_int as usize]
                    - s[22 as ::core::ffi::c_int as usize]
                    + s[62 as ::core::ffi::c_int as usize]
                    - s[26 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                )
                += s[9 as ::core::ffi::c_int as usize]
                    - s[27 as ::core::ffi::c_int as usize]
                    + s[13 as ::core::ffi::c_int as usize]
                    - s[31 as ::core::ffi::c_int as usize]
                    + s[17 as ::core::ffi::c_int as usize]
                    - s[35 as ::core::ffi::c_int as usize];
            *gout
                .offset(
                    (n * 16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize,
                )
                += s[0 as ::core::ffi::c_int as usize]
                    + s[36 as ::core::ffi::c_int as usize]
                    + s[72 as ::core::ffi::c_int as usize]
                    + s[4 as ::core::ffi::c_int as usize]
                    + s[40 as ::core::ffi::c_int as usize]
                    + s[76 as ::core::ffi::c_int as usize]
                    + s[8 as ::core::ffi::c_int as usize]
                    + s[44 as ::core::ffi::c_int as usize]
                    + s[80 as ::core::ffi::c_int as usize];
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int2e_gauge_r2_sps1sps2_optimizer(
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
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
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
pub unsafe extern "C" fn int2e_gauge_r2_sps1sps2_cart(
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
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r2_sps1sps2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn int2e_gauge_r2_sps1sps2_sph(
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
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
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
            CINTgout2e_int2e_gauge_r2_sps1sps2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_int,
                    *mut CINTEnvVars,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    );
    envs.common_factor *= -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
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
pub unsafe extern "C" fn cint2e_gauge_r2_sps1sps2_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r2_sps1sps2_cart(
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
pub unsafe extern "C" fn cint2e_gauge_r2_sps1sps2_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r2_sps1sps2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r2_sps1sps2_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int2e_gauge_r2_sps1sps2_sph(
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
pub unsafe extern "C" fn cint2e_gauge_r2_sps1sps2_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r2_sps1sps2_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint2e_gauge_r2_sps1sps2_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int2e_gauge_r2_sps1sps2_optimizer(opt, atm, natm, bas, nbas, env);
}
