extern "C" {
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
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
    fn CINTcommon_fac_sp(l: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn CINTrys_roots(
        nroots: ::core::ffi::c_int,
        x: ::core::ffi::c_double,
        u: *mut ::core::ffi::c_double,
        w: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn CINTsr_rys_roots(
        nroots: ::core::ffi::c_int,
        x: ::core::ffi::c_double,
        lower: ::core::ffi::c_double,
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
pub type uintptr_t = usize;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 73] = unsafe {
    ::core::mem::transmute::<
        [u8; 73],
        [::core::ffi::c_char; 73],
    >(*b"int CINTg0_1e_grids(double *, double, CINTEnvVars *, double *, double *)\0")
};
pub const PTR_RINV_ZETA: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const PTR_RANGE_OMEGA: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PTR_GRIDS: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const M_PI: ::core::ffi::c_double = 3.14159265358979323846f64;
#[no_mangle]
pub unsafe extern "C" fn CINTinit_int1e_grids_EnvVars(
    mut envs: *mut CINTEnvVars,
    mut ng: *mut ::core::ffi::c_int,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    CINTinit_int1e_EnvVars(envs, ng, shls, atm, natm, bas, nbas, env);
    let mut ngrids: ::core::ffi::c_int = *shls.offset(3 as ::core::ffi::c_int as isize)
        - *shls.offset(2 as ::core::ffi::c_int as isize);
    let mut grids: *mut ::core::ffi::c_double = env
        .offset(*env.offset(PTR_GRIDS as isize) as size_t as isize)
        .offset(
            (*shls.offset(2 as ::core::ffi::c_int as isize) * 3 as ::core::ffi::c_int)
                as isize,
        );
    (*envs).c2rust_unnamed_0.ngrids = ngrids;
    (*envs).c2rust_unnamed_1.grids = grids;
    (*envs).common_factor = 2 as ::core::ffi::c_int as ::core::ffi::c_double * M_PI
        * CINTcommon_fac_sp((*envs).i_l) * CINTcommon_fac_sp((*envs).j_l);
    let mut rys_order: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut nroots: ::core::ffi::c_int = rys_order;
    let mut omega: ::core::ffi::c_double = *env.offset(PTR_RANGE_OMEGA as isize);
    if omega < 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && rys_order <= 3 as ::core::ffi::c_int
    {
        nroots *= 2 as ::core::ffi::c_int;
    }
    (*envs).rys_order = rys_order;
    (*envs).nrys_roots = nroots;
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
    (*envs).g_stride_i = GRID_BLKSIZE * nroots;
    (*envs).g_stride_j = GRID_BLKSIZE * nroots * dli;
    (*envs).g_size = GRID_BLKSIZE * nroots * dli * dlj;
    (*envs).g_stride_k = (*envs).g_size;
    (*envs).g_stride_l = (*envs).g_size;
}
#[no_mangle]
pub unsafe extern "C" fn CINTg0_1e_grids(
    mut g: *mut ::core::ffi::c_double,
    mut cutoff: ::core::ffi::c_double,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut ::core::ffi::c_double,
    mut gridsT: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ngrids: ::core::ffi::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: ::core::ffi::c_int = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as ::core::ffi::c_int
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as ::core::ffi::c_int
    };
    let mut nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut gx: *mut ::core::ffi::c_double = g;
    let mut gy: *mut ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *mut ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut w: *mut ::core::ffi::c_double = gz;
    let mut rij: *mut ::core::ffi::c_double = &raw mut (*envs).rij
        as *mut ::core::ffi::c_double;
    let mut ubuf: [::core::ffi::c_double; 32] = [0.; 32];
    let mut wbuf: [::core::ffi::c_double; 32] = [0.; 32];
    let mut u: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    u = ((cache as uintptr_t).wrapping_add(63 as ::core::ffi::c_int as uintptr_t)
        & (64 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
    cache = u.offset((104 as ::core::ffi::c_int * nroots) as isize);
    let mut rijrg: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    rijrg = ((cache as uintptr_t).wrapping_add(63 as ::core::ffi::c_int as uintptr_t)
        & (64 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
    cache = rijrg.offset((104 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize);
    let mut aij: ::core::ffi::c_double = (*envs).ai[0 as ::core::ffi::c_int as usize]
        + (*envs).aj[0 as ::core::ffi::c_int as usize];
    let mut n: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut ig: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_double = 0.;
    let mut fac1: ::core::ffi::c_double = 0.;
    i = 0 as ::core::ffi::c_int;
    while i < nroots {
        ig = 0 as ::core::ffi::c_int;
        while ig < bgrids {
            *gx.offset((ig + GRID_BLKSIZE * i) as isize) = 1 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            *gy.offset((ig + GRID_BLKSIZE * i) as isize) = 1 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            ig += 1;
        }
        i += 1;
    }
    ig = 0 as ::core::ffi::c_int;
    while ig < bgrids {
        *rijrg.offset((ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as isize) = *gridsT
            .offset((ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as isize)
            - *rij.offset(0 as ::core::ffi::c_int as isize);
        *rijrg.offset((ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as isize) = *gridsT
            .offset((ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as isize)
            - *rij.offset(1 as ::core::ffi::c_int as isize);
        *rijrg.offset((ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as isize) = *gridsT
            .offset((ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as isize)
            - *rij.offset(2 as ::core::ffi::c_int as isize);
        ig += 1;
    }
    let mut omega: ::core::ffi::c_double = *(*envs).env.offset(PTR_RANGE_OMEGA as isize);
    let mut zeta: ::core::ffi::c_double = *(*envs).env.offset(PTR_RINV_ZETA as isize);
    let mut omega2: ::core::ffi::c_double = 0.;
    let mut theta: ::core::ffi::c_double = 0.;
    let mut sqrt_theta: ::core::ffi::c_double = 0.;
    let mut a0: ::core::ffi::c_double = 0.;
    let mut tau2: ::core::ffi::c_double = 0.;
    '_c2rust_label: {
        if zeta >= 0 as ::core::ffi::c_int as ::core::ffi::c_double {} else {
            __assert_fail(
                b"zeta >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/fly/code/rsint/src_work/g1e_grids.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                102 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    if omega == 0.0f64 && zeta == 0.0f64 {
        fac1 = (*envs).fac[0 as ::core::ffi::c_int as usize] / aij;
        ig = 0 as ::core::ffi::c_int;
        while ig < bgrids {
            x = aij
                * (*rijrg.offset((ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as isize)
                    * *rijrg
                        .offset((ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as isize)
                    + *rijrg
                        .offset((ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as isize)
                        * *rijrg
                            .offset(
                                (ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as isize,
                            )
                    + *rijrg
                        .offset((ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as isize)
                        * *rijrg
                            .offset(
                                (ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as isize,
                            ));
            CINTrys_roots(
                nroots,
                x,
                &raw mut ubuf as *mut ::core::ffi::c_double,
                &raw mut wbuf as *mut ::core::ffi::c_double,
            );
            i = 0 as ::core::ffi::c_int;
            while i < nroots {
                *u.offset((ig + GRID_BLKSIZE * i) as isize) = ubuf[i as usize]
                    / (ubuf[i as usize]
                        + 1 as ::core::ffi::c_int as ::core::ffi::c_double);
                *w.offset((ig + GRID_BLKSIZE * i) as isize) = wbuf[i as usize] * fac1;
                i += 1;
            }
            ig += 1;
        }
    } else if omega < 0.0f64 {
        a0 = aij;
        fac1 = (*envs).fac[0 as ::core::ffi::c_int as usize] / aij;
        if zeta == 0.0f64 {
            tau2 = 1.0f64;
            omega2 = omega * omega;
            theta = omega2 / (omega2 + aij);
        } else {
            tau2 = zeta / (zeta + aij);
            a0 *= tau2;
            fac1 *= sqrt(tau2);
            omega2 = omega * omega;
            theta = omega2 / (omega2 + a0);
        }
        sqrt_theta = sqrt(theta);
        let mut temp_cutoff: ::core::ffi::c_double = if cutoff
            < 40 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            cutoff
        } else {
            40 as ::core::ffi::c_int as ::core::ffi::c_double
        };
        let mut rorder: ::core::ffi::c_int = (*envs).rys_order;
        let mut tau_theta: ::core::ffi::c_double = 0.;
        let mut fac_theta: ::core::ffi::c_double = 0.;
        ig = 0 as ::core::ffi::c_int;
        while ig < bgrids {
            x = a0
                * (*rijrg.offset((ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as isize)
                    * *rijrg
                        .offset((ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as isize)
                    + *rijrg
                        .offset((ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as isize)
                        * *rijrg
                            .offset(
                                (ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as isize,
                            )
                    + *rijrg
                        .offset((ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as isize)
                        * *rijrg
                            .offset(
                                (ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as isize,
                            ));
            if theta * x > temp_cutoff {
                i = 0 as ::core::ffi::c_int;
                while i < nroots {
                    *u.offset((ig + GRID_BLKSIZE * i) as isize) = 0 as ::core::ffi::c_int
                        as ::core::ffi::c_double;
                    *w.offset((ig + GRID_BLKSIZE * i) as isize) = 0 as ::core::ffi::c_int
                        as ::core::ffi::c_double;
                    i += 1;
                }
            } else if rorder == nroots {
                CINTsr_rys_roots(
                    nroots,
                    x,
                    sqrt_theta,
                    &raw mut ubuf as *mut ::core::ffi::c_double,
                    &raw mut wbuf as *mut ::core::ffi::c_double,
                );
                i = 0 as ::core::ffi::c_int;
                while i < nroots {
                    *u.offset((ig + GRID_BLKSIZE * i) as isize) = ubuf[i as usize]
                        / (ubuf[i as usize]
                            + 1 as ::core::ffi::c_int as ::core::ffi::c_double) * tau2;
                    *w.offset((ig + GRID_BLKSIZE * i) as isize) = wbuf[i as usize]
                        * fac1;
                    i += 1;
                }
            } else {
                tau_theta = tau2 * theta;
                fac_theta = fac1 * -sqrt_theta;
                CINTrys_roots(
                    rorder,
                    x,
                    &raw mut ubuf as *mut ::core::ffi::c_double,
                    &raw mut wbuf as *mut ::core::ffi::c_double,
                );
                CINTrys_roots(
                    rorder,
                    theta * x,
                    (&raw mut ubuf as *mut ::core::ffi::c_double)
                        .offset(rorder as isize),
                    (&raw mut wbuf as *mut ::core::ffi::c_double).offset(rorder as isize),
                );
                i = 0 as ::core::ffi::c_int;
                while i < rorder {
                    *u.offset((ig + GRID_BLKSIZE * i) as isize) = ubuf[i as usize]
                        / (ubuf[i as usize]
                            + 1 as ::core::ffi::c_int as ::core::ffi::c_double) * tau2;
                    *w.offset((ig + GRID_BLKSIZE * i) as isize) = wbuf[i as usize]
                        * fac1;
                    *u.offset((ig + GRID_BLKSIZE * (i + rorder)) as isize) = ubuf[(i
                        + rorder) as usize]
                        / (ubuf[(i + rorder) as usize]
                            + 1 as ::core::ffi::c_int as ::core::ffi::c_double)
                        * tau_theta;
                    *w.offset((ig + GRID_BLKSIZE * (i + rorder)) as isize) = wbuf[(i
                        + rorder) as usize] * fac_theta;
                    i += 1;
                }
            }
            ig += 1;
        }
    } else {
        a0 = aij;
        fac1 = (*envs).fac[0 as ::core::ffi::c_int as usize] / aij;
        if zeta == 0.0f64 {
            omega2 = omega * omega;
            theta = omega2 / (omega2 + aij);
            a0 *= theta;
            fac1 *= sqrt(theta);
        } else if omega == 0.0f64 {
            theta = zeta / (zeta + aij);
            a0 *= theta;
            fac1 *= sqrt(theta);
        } else {
            omega2 = omega * omega;
            theta = omega2 * zeta / (omega2 * zeta + (zeta + omega2) * aij);
            a0 *= theta;
            fac1 *= sqrt(theta);
        }
        ig = 0 as ::core::ffi::c_int;
        while ig < bgrids {
            x = a0
                * (*rijrg.offset((ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as isize)
                    * *rijrg
                        .offset((ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as isize)
                    + *rijrg
                        .offset((ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as isize)
                        * *rijrg
                            .offset(
                                (ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as isize,
                            )
                    + *rijrg
                        .offset((ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as isize)
                        * *rijrg
                            .offset(
                                (ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as isize,
                            ));
            CINTrys_roots(
                nroots,
                x,
                &raw mut ubuf as *mut ::core::ffi::c_double,
                &raw mut wbuf as *mut ::core::ffi::c_double,
            );
            i = 0 as ::core::ffi::c_int;
            while i < nroots {
                *u.offset((ig + GRID_BLKSIZE * i) as isize) = ubuf[i as usize]
                    / (ubuf[i as usize]
                        + 1 as ::core::ffi::c_int as ::core::ffi::c_double) * theta;
                *w.offset((ig + GRID_BLKSIZE * i) as isize) = wbuf[i as usize] * fac1;
                i += 1;
            }
            ig += 1;
        }
    }
    let mut nmax: ::core::ffi::c_int = (*envs).li_ceil + (*envs).lj_ceil;
    if nmax == 0 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    let mut rirj: *mut ::core::ffi::c_double = &raw mut (*envs).rirj
        as *mut ::core::ffi::c_double;
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
    let mut t2: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    t2 = ((cache as uintptr_t).wrapping_add(63 as ::core::ffi::c_int as uintptr_t)
        & (64 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
    cache = t2.offset((104 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize);
    let mut rirgx: *mut ::core::ffi::c_double = t2.offset(GRID_BLKSIZE as isize);
    let mut rirgy: *mut ::core::ffi::c_double = rirgx.offset(GRID_BLKSIZE as isize);
    let mut rirgz: *mut ::core::ffi::c_double = rirgy.offset(GRID_BLKSIZE as isize);
    let mut aij2: ::core::ffi::c_double = 0.5f64 / aij;
    let mut tx: ::core::ffi::c_double = 0.;
    let mut ty: ::core::ffi::c_double = 0.;
    let mut tz: ::core::ffi::c_double = 0.;
    n = 0 as ::core::ffi::c_int;
    while n < nroots {
        p0x = gx.offset((GRID_BLKSIZE * n) as isize);
        p0y = gy.offset((GRID_BLKSIZE * n) as isize);
        p0z = gz.offset((GRID_BLKSIZE * n) as isize);
        p1x = p0x.offset(di as isize);
        p1y = p0y.offset(di as isize);
        p1z = p0z.offset(di as isize);
        ig = 0 as ::core::ffi::c_int;
        while ig < bgrids {
            *rirgx.offset(ig as isize) = rijrx[0 as ::core::ffi::c_int as usize]
                + *u.offset((ig + GRID_BLKSIZE * n) as isize)
                    * *rijrg
                        .offset((ig + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as isize);
            *rirgy.offset(ig as isize) = rijrx[1 as ::core::ffi::c_int as usize]
                + *u.offset((ig + GRID_BLKSIZE * n) as isize)
                    * *rijrg
                        .offset((ig + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as isize);
            *rirgz.offset(ig as isize) = rijrx[2 as ::core::ffi::c_int as usize]
                + *u.offset((ig + GRID_BLKSIZE * n) as isize)
                    * *rijrg
                        .offset((ig + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as isize);
            *p1x.offset(ig as isize) = *rirgx.offset(ig as isize)
                * *p0x.offset(ig as isize);
            *p1y.offset(ig as isize) = *rirgy.offset(ig as isize)
                * *p0y.offset(ig as isize);
            *p1z.offset(ig as isize) = *rirgz.offset(ig as isize)
                * *p0z.offset(ig as isize);
            ig += 1;
        }
        if nmax > 0 as ::core::ffi::c_int {
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                *t2.offset(ig as isize) = aij2
                    * (1 as ::core::ffi::c_int as ::core::ffi::c_double
                        - *u.offset((ig + GRID_BLKSIZE * n) as isize));
                ig += 1;
            }
        }
        i = 1 as ::core::ffi::c_int;
        while i < nmax {
            p0x = gx.offset((GRID_BLKSIZE * n) as isize).offset((i * di) as isize);
            p0y = gy.offset((GRID_BLKSIZE * n) as isize).offset((i * di) as isize);
            p0z = gz.offset((GRID_BLKSIZE * n) as isize).offset((i * di) as isize);
            p1x = p0x.offset(di as isize);
            p1y = p0y.offset(di as isize);
            p1z = p0z.offset(di as isize);
            p2x = p0x.offset(-(di as isize));
            p2y = p0y.offset(-(di as isize));
            p2z = p0z.offset(-(di as isize));
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                *p1x.offset(ig as isize) = i as ::core::ffi::c_double
                    * *t2.offset(ig as isize) * *p2x.offset(ig as isize)
                    + *rirgx.offset(ig as isize) * *p0x.offset(ig as isize);
                *p1y.offset(ig as isize) = i as ::core::ffi::c_double
                    * *t2.offset(ig as isize) * *p2y.offset(ig as isize)
                    + *rirgy.offset(ig as isize) * *p0y.offset(ig as isize);
                *p1z.offset(ig as isize) = i as ::core::ffi::c_double
                    * *t2.offset(ig as isize) * *p2z.offset(ig as isize)
                    + *rirgz.offset(ig as isize) * *p0z.offset(ig as isize);
                ig += 1;
            }
            i += 1;
        }
        n += 1;
    }
    j = 1 as ::core::ffi::c_int;
    while j <= lj {
        i = 0 as ::core::ffi::c_int;
        while i <= nmax - j {
            p0x = gx.offset((j * dj) as isize).offset((i * di) as isize);
            p0y = gy.offset((j * dj) as isize).offset((i * di) as isize);
            p0z = gz.offset((j * dj) as isize).offset((i * di) as isize);
            p1x = p0x.offset(-(dj as isize));
            p1y = p0y.offset(-(dj as isize));
            p1z = p0z.offset(-(dj as isize));
            p2x = p1x.offset(di as isize);
            p2y = p1y.offset(di as isize);
            p2z = p1z.offset(di as isize);
            n = 0 as ::core::ffi::c_int;
            while n < nroots {
                ig = 0 as ::core::ffi::c_int;
                while ig < bgrids {
                    *p0x.offset((ig + GRID_BLKSIZE * n) as isize) = *p2x
                        .offset((ig + GRID_BLKSIZE * n) as isize)
                        + *rirj.offset(0 as ::core::ffi::c_int as isize)
                            * *p1x.offset((ig + GRID_BLKSIZE * n) as isize);
                    *p0y.offset((ig + GRID_BLKSIZE * n) as isize) = *p2y
                        .offset((ig + GRID_BLKSIZE * n) as isize)
                        + *rirj.offset(1 as ::core::ffi::c_int as isize)
                            * *p1y.offset((ig + GRID_BLKSIZE * n) as isize);
                    *p0z.offset((ig + GRID_BLKSIZE * n) as isize) = *p2z
                        .offset((ig + GRID_BLKSIZE * n) as isize)
                        + *rirj.offset(2 as ::core::ffi::c_int as isize)
                            * *p1z.offset((ig + GRID_BLKSIZE * n) as isize);
                    ig += 1;
                }
                n += 1;
            }
            i += 1;
        }
        j += 1;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CINTgout1e_grids(
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
    let mut nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut i: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ig: ::core::ffi::c_int = 0;
    let mut gx: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut gy: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut gz: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut s: [::core::ffi::c_double; 104] = [0.; 104];
    if gout_empty != 0 {
        n = 0 as ::core::ffi::c_int;
        while n < nf {
            gx = g.offset(*idx.offset(0 as ::core::ffi::c_int as isize) as isize);
            gy = g.offset(*idx.offset(1 as ::core::ffi::c_int as isize) as isize);
            gz = g.offset(*idx.offset(2 as ::core::ffi::c_int as isize) as isize);
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                s[ig as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                ig += 1;
            }
            i = 0 as ::core::ffi::c_int;
            while i < nroots {
                ig = 0 as ::core::ffi::c_int;
                while ig < bgrids {
                    s[ig as usize]
                        += *gx.offset((ig + GRID_BLKSIZE * i) as isize)
                            * *gy.offset((ig + GRID_BLKSIZE * i) as isize)
                            * *gz.offset((ig + GRID_BLKSIZE * i) as isize);
                    ig += 1;
                }
                i += 1;
            }
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                *gout.offset((ig + bgrids * n) as isize) = s[ig as usize];
                ig += 1;
            }
            n += 1;
            idx = idx.offset(3 as ::core::ffi::c_int as isize);
        }
    } else {
        n = 0 as ::core::ffi::c_int;
        while n < nf {
            gx = g.offset(*idx.offset(0 as ::core::ffi::c_int as isize) as isize);
            gy = g.offset(*idx.offset(1 as ::core::ffi::c_int as isize) as isize);
            gz = g.offset(*idx.offset(2 as ::core::ffi::c_int as isize) as isize);
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                s[ig as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                ig += 1;
            }
            i = 0 as ::core::ffi::c_int;
            while i < nroots {
                ig = 0 as ::core::ffi::c_int;
                while ig < bgrids {
                    s[ig as usize]
                        += *gx.offset((ig + GRID_BLKSIZE * i) as isize)
                            * *gy.offset((ig + GRID_BLKSIZE * i) as isize)
                            * *gz.offset((ig + GRID_BLKSIZE * i) as isize);
                    ig += 1;
                }
                i += 1;
            }
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                *gout.offset((ig + bgrids * n) as isize) += s[ig as usize];
                ig += 1;
            }
            n += 1;
            idx = idx.offset(3 as ::core::ffi::c_int as isize);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1i_grids(
    mut f: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut li: ::core::ffi::c_int,
    mut lj: ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let mut ngrids: ::core::ffi::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: ::core::ffi::c_int = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as ::core::ffi::c_int
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as ::core::ffi::c_int
    };
    let mut nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let di: ::core::ffi::c_int = (*envs).g_stride_i;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let ai2: ::core::ffi::c_double = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
        * (*envs).ai[0 as ::core::ffi::c_int as usize];
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ig: ::core::ffi::c_int = 0;
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
        n = 0 as ::core::ffi::c_int;
        while n < nroots {
            ptr = dj * j + n * GRID_BLKSIZE;
            ig = ptr;
            while ig < ptr + bgrids {
                *fx.offset(ig as isize) = ai2 * *gx.offset((ig + di) as isize);
                *fy.offset(ig as isize) = ai2 * *gy.offset((ig + di) as isize);
                *fz.offset(ig as isize) = ai2 * *gz.offset((ig + di) as isize);
                ig += 1;
            }
            n += 1;
        }
        i = 1 as ::core::ffi::c_int;
        while i <= li {
            n = 0 as ::core::ffi::c_int;
            while n < nroots {
                ptr = dj * j + di * i + n * GRID_BLKSIZE;
                ig = ptr;
                while ig < ptr + bgrids {
                    *fx.offset(ig as isize) = i as ::core::ffi::c_double
                        * *gx.offset((ig - di) as isize)
                        + ai2 * *gx.offset((ig + di) as isize);
                    *fy.offset(ig as isize) = i as ::core::ffi::c_double
                        * *gy.offset((ig - di) as isize)
                        + ai2 * *gy.offset((ig + di) as isize);
                    *fz.offset(ig as isize) = i as ::core::ffi::c_double
                        * *gz.offset((ig - di) as isize)
                        + ai2 * *gz.offset((ig + di) as isize);
                    ig += 1;
                }
                n += 1;
            }
            i += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTnabla1j_grids(
    mut f: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut li: ::core::ffi::c_int,
    mut lj: ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let mut ngrids: ::core::ffi::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: ::core::ffi::c_int = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as ::core::ffi::c_int
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as ::core::ffi::c_int
    };
    let mut nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let di: ::core::ffi::c_int = (*envs).g_stride_i;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
    let aj2: ::core::ffi::c_double = -(2 as ::core::ffi::c_int) as ::core::ffi::c_double
        * (*envs).aj[0 as ::core::ffi::c_int as usize];
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ig: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let mut gx: *const ::core::ffi::c_double = g;
    let mut gy: *const ::core::ffi::c_double = g.offset((*envs).g_size as isize);
    let mut gz: *const ::core::ffi::c_double = g
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    let mut fx: *mut ::core::ffi::c_double = f;
    let mut fy: *mut ::core::ffi::c_double = f.offset((*envs).g_size as isize);
    let mut fz: *mut ::core::ffi::c_double = f
        .offset(((*envs).g_size * 2 as ::core::ffi::c_int) as isize);
    i = 0 as ::core::ffi::c_int;
    while i <= li {
        n = 0 as ::core::ffi::c_int;
        while n < nroots {
            ptr = di * i + n * GRID_BLKSIZE;
            ig = ptr;
            while ig < ptr + bgrids {
                *fx.offset(ig as isize) = aj2 * *gx.offset((ig + dj) as isize);
                *fy.offset(ig as isize) = aj2 * *gy.offset((ig + dj) as isize);
                *fz.offset(ig as isize) = aj2 * *gz.offset((ig + dj) as isize);
                ig += 1;
            }
            n += 1;
        }
        i += 1;
    }
    j = 1 as ::core::ffi::c_int;
    while j <= lj {
        i = 0 as ::core::ffi::c_int;
        while i <= li {
            n = 0 as ::core::ffi::c_int;
            while n < nroots {
                ptr = dj * j + di * i + n * GRID_BLKSIZE;
                ig = ptr;
                while ig < ptr + bgrids {
                    *fx.offset(ig as isize) = j as ::core::ffi::c_double
                        * *gx.offset((ig - dj) as isize)
                        + aj2 * *gx.offset((ig + dj) as isize);
                    *fy.offset(ig as isize) = j as ::core::ffi::c_double
                        * *gy.offset((ig - dj) as isize)
                        + aj2 * *gy.offset((ig + dj) as isize);
                    *fz.offset(ig as isize) = j as ::core::ffi::c_double
                        * *gz.offset((ig - dj) as isize)
                        + aj2 * *gz.offset((ig + dj) as isize);
                    ig += 1;
                }
                n += 1;
            }
            i += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1i_grids(
    mut f: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut ri: *mut ::core::ffi::c_double,
    mut li: ::core::ffi::c_int,
    mut lj: ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let mut ngrids: ::core::ffi::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: ::core::ffi::c_int = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as ::core::ffi::c_int
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as ::core::ffi::c_int
    };
    let mut nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ig: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let di: ::core::ffi::c_int = (*envs).g_stride_i;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
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
        i = 0 as ::core::ffi::c_int;
        while i <= li {
            n = 0 as ::core::ffi::c_int;
            while n < nroots {
                ptr = dj * j + di * i + n * GRID_BLKSIZE;
                ig = ptr;
                while ig < ptr + bgrids {
                    *fx.offset(ig as isize) = *gx.offset((ig + di) as isize)
                        + *ri.offset(0 as ::core::ffi::c_int as isize)
                            * *gx.offset(ig as isize);
                    *fy.offset(ig as isize) = *gy.offset((ig + di) as isize)
                        + *ri.offset(1 as ::core::ffi::c_int as isize)
                            * *gy.offset(ig as isize);
                    *fz.offset(ig as isize) = *gz.offset((ig + di) as isize)
                        + *ri.offset(2 as ::core::ffi::c_int as isize)
                            * *gz.offset(ig as isize);
                    ig += 1;
                }
                n += 1;
            }
            i += 1;
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTx1j_grids(
    mut f: *mut ::core::ffi::c_double,
    mut g: *mut ::core::ffi::c_double,
    mut rj: *mut ::core::ffi::c_double,
    mut li: ::core::ffi::c_int,
    mut lj: ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
) {
    let mut ngrids: ::core::ffi::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut bgrids: ::core::ffi::c_int = if ngrids - (*envs).c2rust_unnamed.grids_offset
        < 104 as ::core::ffi::c_int
    {
        ngrids - (*envs).c2rust_unnamed.grids_offset
    } else {
        104 as ::core::ffi::c_int
    };
    let mut nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut ig: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let di: ::core::ffi::c_int = (*envs).g_stride_i;
    let dj: ::core::ffi::c_int = (*envs).g_stride_j;
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
        i = 0 as ::core::ffi::c_int;
        while i <= li {
            n = 0 as ::core::ffi::c_int;
            while n < nroots {
                ptr = dj * j + di * i + n * GRID_BLKSIZE;
                ig = ptr;
                while ig < ptr + bgrids {
                    *fx.offset(ig as isize) = *gx.offset((ig + dj) as isize)
                        + *rj.offset(0 as ::core::ffi::c_int as isize)
                            * *gx.offset(ig as isize);
                    *fy.offset(ig as isize) = *gy.offset((ig + dj) as isize)
                        + *rj.offset(1 as ::core::ffi::c_int as isize)
                            * *gy.offset(ig as isize);
                    *fz.offset(ig as isize) = *gz.offset((ig + dj) as isize)
                        + *rj.offset(2 as ::core::ffi::c_int as isize)
                            * *gz.offset(ig as isize);
                    ig += 1;
                }
                n += 1;
            }
            i += 1;
        }
        j += 1;
    }
}
pub const GRID_BLKSIZE: ::core::ffi::c_int = 104 as ::core::ffi::c_int;
