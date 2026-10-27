extern "C" {
    fn CINTcommon_fac_sp(l: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn CINTg0_2e(
        g: *mut ::core::ffi::c_double,
        rij: *mut ::core::ffi::c_double,
        rkl: *mut ::core::ffi::c_double,
        cutoff: ::core::ffi::c_double,
        envs: *mut CINTEnvVars,
    ) -> ::core::ffi::c_int;
    fn CINTg0_2e_2d(
        g: *mut ::core::ffi::c_double,
        bc: *mut Rys2eT,
        envs: *mut CINTEnvVars,
    );
    fn CINTg0_2e_2d4d_unrolled(
        g: *mut ::core::ffi::c_double,
        bc: *mut Rys2eT,
        envs: *mut CINTEnvVars,
    );
    fn CINTsrg0_2e_2d4d_unrolled(
        g: *mut ::core::ffi::c_double,
        bc: *mut Rys2eT,
        envs: *mut CINTEnvVars,
    );
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rys2eT {
    pub c00x: [::core::ffi::c_double; 32],
    pub c00y: [::core::ffi::c_double; 32],
    pub c00z: [::core::ffi::c_double; 32],
    pub c0px: [::core::ffi::c_double; 32],
    pub c0py: [::core::ffi::c_double; 32],
    pub c0pz: [::core::ffi::c_double; 32],
    pub b01: [::core::ffi::c_double; 32],
    pub b00: [::core::ffi::c_double; 32],
    pub b10: [::core::ffi::c_double; 32],
}
pub const M_PI: ::core::ffi::c_double = 3.14159265358979323846f64;
pub const PTR_EXPCUTOFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTR_RANGE_OMEGA: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const ATM_SLOTS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const BAS_SLOTS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn CINTinit_int2c2e_EnvVars(
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
    let k_sh: ::core::ffi::c_int = *shls.offset(1 as ::core::ffi::c_int as isize);
    (*envs).i_l = *bas.offset((BAS_SLOTS * i_sh + 1 as ::core::ffi::c_int) as isize);
    (*envs).j_l = 0 as ::core::ffi::c_int;
    (*envs).k_l = *bas.offset((BAS_SLOTS * k_sh + 1 as ::core::ffi::c_int) as isize);
    (*envs).l_l = 0 as ::core::ffi::c_int;
    (*envs).x_ctr[0 as ::core::ffi::c_int as usize] = *bas
        .offset((BAS_SLOTS * i_sh + 3 as ::core::ffi::c_int) as isize);
    (*envs).x_ctr[1 as ::core::ffi::c_int as usize] = *bas
        .offset((BAS_SLOTS * k_sh + 3 as ::core::ffi::c_int) as isize);
    (*envs).x_ctr[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
    (*envs).x_ctr[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
    (*envs).nfi = ((*envs).i_l + 1 as ::core::ffi::c_int)
        * ((*envs).i_l + 2 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
    (*envs).nfj = 1 as ::core::ffi::c_int;
    (*envs).c2rust_unnamed.nfk = ((*envs).k_l + 1 as ::core::ffi::c_int)
        * ((*envs).k_l + 2 as ::core::ffi::c_int) / 2 as ::core::ffi::c_int;
    (*envs).c2rust_unnamed_0.nfl = 1 as ::core::ffi::c_int;
    (*envs).nf = (*envs).nfi * (*envs).c2rust_unnamed.nfk;
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
    (*envs).rk = env
        .offset(
            *atm
                .offset(
                    (ATM_SLOTS
                        * *bas
                            .offset(
                                (8 as ::core::ffi::c_int * k_sh + 0 as ::core::ffi::c_int)
                                    as isize,
                            ) + 1 as ::core::ffi::c_int) as isize,
                ) as isize,
        );
    (*envs).common_factor = M_PI * M_PI * M_PI
        * 2 as ::core::ffi::c_int as ::core::ffi::c_double / SQRTPI
        * CINTcommon_fac_sp((*envs).i_l) * CINTcommon_fac_sp((*envs).k_l);
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
    (*envs).gbits = *ng.offset(GSHIFT as isize);
    (*envs).ncomp_e1 = *ng.offset(POS_E1 as isize);
    (*envs).ncomp_e2 = *ng.offset(POS_E2 as isize);
    (*envs).ncomp_tensor = *ng.offset(TENSOR as isize);
    (*envs).li_ceil = (*envs).i_l + *ng.offset(IINC as isize);
    (*envs).lj_ceil = 0 as ::core::ffi::c_int;
    (*envs).lk_ceil = (*envs).k_l + *ng.offset(KINC as isize);
    (*envs).ll_ceil = 0 as ::core::ffi::c_int;
    let mut rys_order: ::core::ffi::c_int = ((*envs).li_ceil + (*envs).lk_ceil)
        / 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    let mut nrys_roots: ::core::ffi::c_int = rys_order;
    let mut omega: ::core::ffi::c_double = *env.offset(PTR_RANGE_OMEGA as isize);
    if omega < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && rys_order <= 3 as ::core::ffi::c_int
    {
        nrys_roots *= 2 as ::core::ffi::c_int;
    }
    (*envs).rys_order = rys_order;
    (*envs).nrys_roots = nrys_roots;
    let mut dli: ::core::ffi::c_int = (*envs).li_ceil + 1 as ::core::ffi::c_int;
    let mut dlk: ::core::ffi::c_int = (*envs).lk_ceil + 1 as ::core::ffi::c_int;
    (*envs).g_stride_i = nrys_roots;
    (*envs).g_stride_k = nrys_roots * dli;
    (*envs).g_stride_l = (*envs).g_stride_k;
    (*envs).g_size = nrys_roots * dli * dlk;
    (*envs).aj[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    (*envs).al[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    (*envs).rij[0 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(0 as ::core::ffi::c_int as isize);
    (*envs).rij[1 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(1 as ::core::ffi::c_int as isize);
    (*envs).rij[2 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(2 as ::core::ffi::c_int as isize);
    (*envs).rkl[0 as ::core::ffi::c_int as usize] = *(*envs)
        .rk
        .offset(0 as ::core::ffi::c_int as isize);
    (*envs).rkl[1 as ::core::ffi::c_int as usize] = *(*envs)
        .rk
        .offset(1 as ::core::ffi::c_int as isize);
    (*envs).rkl[2 as ::core::ffi::c_int as usize] = *(*envs)
        .rk
        .offset(2 as ::core::ffi::c_int as isize);
    (*envs).g2d_ijmax = (*envs).g_stride_i;
    (*envs).g2d_klmax = (*envs).g_stride_k;
    (*envs).rkrl[0 as ::core::ffi::c_int as usize] = *(*envs)
        .rk
        .offset(0 as ::core::ffi::c_int as isize);
    (*envs).rkrl[1 as ::core::ffi::c_int as usize] = *(*envs)
        .rk
        .offset(1 as ::core::ffi::c_int as isize);
    (*envs).rkrl[2 as ::core::ffi::c_int as usize] = *(*envs)
        .rk
        .offset(2 as ::core::ffi::c_int as isize);
    (*envs).rirj[0 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(0 as ::core::ffi::c_int as isize);
    (*envs).rirj[1 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(1 as ::core::ffi::c_int as isize);
    (*envs).rirj[2 as ::core::ffi::c_int as usize] = *(*envs)
        .ri
        .offset(2 as ::core::ffi::c_int as isize);
    (*envs).rx_in_rklrx = (*envs).rk;
    (*envs).rx_in_rijrx = (*envs).ri;
    if rys_order <= 2 as ::core::ffi::c_int {
        (*envs).f_g0_2d4d = ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut Rys2eT,
                    *mut CINTEnvVars,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg0_2e_2d4d_unrolled
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
            ),
        );
        if rys_order != nrys_roots {
            (*envs).f_g0_2d4d = ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
                >,
                Option<unsafe extern "C" fn() -> ()>,
            >(
                Some(
                    CINTsrg0_2e_2d4d_unrolled
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_double,
                            *mut Rys2eT,
                            *mut CINTEnvVars,
                        ) -> (),
                ),
            );
        }
    } else {
        (*envs).f_g0_2d4d = ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut Rys2eT,
                    *mut CINTEnvVars,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg0_2e_2d
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut Rys2eT,
                        *mut CINTEnvVars,
                    ) -> (),
            ),
        );
    }
    (*envs).f_g0_2e = ::core::mem::transmute::<
        Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                *mut ::core::ffi::c_double,
                ::core::ffi::c_double,
                *mut CINTEnvVars,
            ) -> ::core::ffi::c_int,
        >,
        Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    >(
        Some(
            CINTg0_2e
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    *mut ::core::ffi::c_double,
                    ::core::ffi::c_double,
                    *mut CINTEnvVars,
                ) -> ::core::ffi::c_int,
        ),
    );
    (*envs).j_l = (*envs).k_l;
    (*envs).nfj = (*envs).c2rust_unnamed.nfk;
    (*envs).g_stride_j = (*envs).g_stride_k;
}
pub const SQRTPI: ::core::ffi::c_double = 1.7724538509055160272981674833411451f64;
pub const IINC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const KINC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const GSHIFT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const POS_E1: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const POS_E2: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const TENSOR: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const EXPCUTOFF: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
