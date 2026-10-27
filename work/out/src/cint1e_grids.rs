extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn CINTcgto_spinor(
        bas_id: ::core::ffi::c_int,
        bas: *const ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn CINTOpt_log_max_pgto_coeff(
        log_maxc: *mut ::core::ffi::c_double,
        coeff: *mut ::core::ffi::c_double,
        nprim: ::core::ffi::c_int,
        nctr: ::core::ffi::c_int,
    );
    fn CINTOpt_non0coeff_byshell(
        sortedidx: *mut ::core::ffi::c_int,
        non0ctr: *mut ::core::ffi::c_int,
        ci: *mut ::core::ffi::c_double,
        iprim: ::core::ffi::c_int,
        ictr: ::core::ffi::c_int,
    );
    fn CINTset_pairdata(
        pairdata: *mut PairData,
        ai: *mut ::core::ffi::c_double,
        aj: *mut ::core::ffi::c_double,
        ri: *mut ::core::ffi::c_double,
        rj: *mut ::core::ffi::c_double,
        log_maxci: *mut ::core::ffi::c_double,
        log_maxcj: *mut ::core::ffi::c_double,
        li_ceil: ::core::ffi::c_int,
        lj_ceil: ::core::ffi::c_int,
        iprim: ::core::ffi::c_int,
        jprim: ::core::ffi::c_int,
        rr_ij: ::core::ffi::c_double,
        expcutoff: ::core::ffi::c_double,
        env: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
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
    fn CINTg0_1e_grids(
        g: *mut ::core::ffi::c_double,
        cutoff: ::core::ffi::c_double,
        envs: *mut CINTEnvVars,
        cache: *mut ::core::ffi::c_double,
        gridsT: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_int;
    fn CINTgout1e_grids(
        gout: *mut ::core::ffi::c_double,
        g: *mut ::core::ffi::c_double,
        idx: *mut ::core::ffi::c_int,
        envs: *mut CINTEnvVars,
        gout_empty: ::core::ffi::c_int,
    );
    fn CINTg1e_index_xyz(idx: *mut ::core::ffi::c_int, envs: *mut CINTEnvVars);
    fn CINTprim_to_ctr_0(
        gc: *mut ::core::ffi::c_double,
        gp: *mut ::core::ffi::c_double,
        coeff: *mut ::core::ffi::c_double,
        nf: size_t,
        nprim: ::core::ffi::c_int,
        nctr: ::core::ffi::c_int,
        non0ctr: ::core::ffi::c_int,
        sortedidx: *mut ::core::ffi::c_int,
    );
    fn CINTprim_to_ctr_1(
        gc: *mut ::core::ffi::c_double,
        gp: *mut ::core::ffi::c_double,
        coeff: *mut ::core::ffi::c_double,
        nf: size_t,
        nprim: ::core::ffi::c_int,
        nctr: ::core::ffi::c_int,
        non0ctr: ::core::ffi::c_int,
        sortedidx: *mut ::core::ffi::c_int,
    );
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
    fn c2s_grids_dset0(
        out: *mut ::core::ffi::c_double,
        dims: *mut ::core::ffi::c_int,
        counts: *mut ::core::ffi::c_int,
    );
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const BAS_SLOTS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn CINT1e_grids_loop(
    mut gctr: *mut ::core::ffi::c_double,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut shls: *mut ::core::ffi::c_int = (*envs).shls;
    let mut bas: *mut ::core::ffi::c_int = (*envs).bas;
    let mut env: *mut ::core::ffi::c_double = (*envs).env;
    let mut i_sh: ::core::ffi::c_int = *shls.offset(0 as ::core::ffi::c_int as isize);
    let mut j_sh: ::core::ffi::c_int = *shls.offset(1 as ::core::ffi::c_int as isize);
    let mut i_ctr: ::core::ffi::c_int = (*envs).x_ctr[0 as ::core::ffi::c_int as usize];
    let mut j_ctr: ::core::ffi::c_int = (*envs).x_ctr[1 as ::core::ffi::c_int as usize];
    let mut i_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * i_sh + 2 as ::core::ffi::c_int) as isize);
    let mut j_prim: ::core::ffi::c_int = *bas
        .offset((BAS_SLOTS * j_sh + 2 as ::core::ffi::c_int) as isize);
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut n_comp: ::core::ffi::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut ngrids: ::core::ffi::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut grids: *mut ::core::ffi::c_double = (*envs).c2rust_unnamed_1.grids;
    let mut ai: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * i_sh + 5 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut aj: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * j_sh + 5 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut ci: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * i_sh + 6 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut cj: *mut ::core::ffi::c_double = env
        .offset(
            *bas.offset((BAS_SLOTS * j_sh + 6 as ::core::ffi::c_int) as isize) as isize,
        );
    let mut expcutoff: ::core::ffi::c_double = (*envs).expcutoff;
    let mut log_maxci: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut log_maxcj: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut pdata_base: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut pdata_ij: *mut PairData = ::core::ptr::null_mut::<PairData>();
    log_maxci = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
    cache = log_maxci.offset((i_prim + j_prim) as isize);
    pdata_base = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut PairData;
    cache = pdata_base.offset((i_prim * j_prim) as isize) as *mut ::core::ffi::c_double;
    log_maxcj = log_maxci.offset(i_prim as isize);
    CINTOpt_log_max_pgto_coeff(log_maxci, ci, i_prim, i_ctr);
    CINTOpt_log_max_pgto_coeff(log_maxcj, cj, j_prim, j_ctr);
    if CINTset_pairdata(
        pdata_base,
        ai,
        aj,
        (*envs).ri,
        (*envs).rj,
        log_maxci,
        log_maxcj,
        (*envs).li_ceil,
        (*envs).lj_ceil,
        i_prim,
        j_prim,
        (*envs).rirj[0 as ::core::ffi::c_int as usize]
            * (*envs).rirj[0 as ::core::ffi::c_int as usize]
            + (*envs).rirj[1 as ::core::ffi::c_int as usize]
                * (*envs).rirj[1 as ::core::ffi::c_int as usize]
            + (*envs).rirj[2 as ::core::ffi::c_int as usize]
                * (*envs).rirj[2 as ::core::ffi::c_int as usize],
        expcutoff,
        env,
    ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut fac1i: ::core::ffi::c_double = 0.;
    let mut fac1j: ::core::ffi::c_double = 0.;
    let mut expij: ::core::ffi::c_double = 0.;
    let mut cutoff: ::core::ffi::c_double = 0.;
    let mut rij: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut ip: ::core::ffi::c_int = 0;
    let mut jp: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut grids_offset: ::core::ffi::c_int = 0;
    let mut bgrids: ::core::ffi::c_int = 0;
    let mut empty: [::core::ffi::c_int; 4] = [
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut gempty: *mut ::core::ffi::c_int = (&raw mut empty as *mut ::core::ffi::c_int)
        .offset(0 as ::core::ffi::c_int as isize);
    let mut iempty: *mut ::core::ffi::c_int = (&raw mut empty as *mut ::core::ffi::c_int)
        .offset(1 as ::core::ffi::c_int as isize);
    let mut jempty: *mut ::core::ffi::c_int = (&raw mut empty as *mut ::core::ffi::c_int)
        .offset(2 as ::core::ffi::c_int as isize);
    let mut all_empty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut idx: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    idx = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
    cache = idx.offset((nf * 3 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_double;
    CINTg1e_index_xyz(idx, envs);
    let mut non0ctri: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut non0ctrj: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut non0idxi: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut non0idxj: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    non0ctri = ((cache as uintptr_t).wrapping_add(7 as ::core::ffi::c_int as uintptr_t)
        & (8 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
    cache = non0ctri.offset((i_prim + j_prim + i_prim * i_ctr + j_prim * j_ctr) as isize)
        as *mut ::core::ffi::c_double;
    non0ctrj = non0ctri.offset(i_prim as isize);
    non0idxi = non0ctrj.offset(j_prim as isize);
    non0idxj = non0idxi.offset((i_prim * i_ctr) as isize);
    CINTOpt_non0coeff_byshell(non0idxi, non0ctri, ci, i_prim, i_ctr);
    CINTOpt_non0coeff_byshell(non0idxj, non0ctrj, cj, j_prim, j_ctr);
    let nc: ::core::ffi::c_int = i_ctr * j_ctr;
    let leng: ::core::ffi::c_int = (*envs).g_size * 3 as ::core::ffi::c_int
        * (((1 as ::core::ffi::c_int) << (*envs).gbits) + 1 as ::core::ffi::c_int);
    let lenj: ::core::ffi::c_int = GRID_BLKSIZE * nf * nc * n_comp;
    let leni: ::core::ffi::c_int = GRID_BLKSIZE * nf * i_ctr * n_comp;
    let len0: ::core::ffi::c_int = GRID_BLKSIZE * nf * n_comp;
    let len: ::core::ffi::c_int = leng + lenj + leni + len0;
    let mut gridsT: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    gridsT = ((cache as uintptr_t).wrapping_add(63 as ::core::ffi::c_int as uintptr_t)
        & (64 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
    cache = gridsT
        .offset((len + 104 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize);
    let mut g: *mut ::core::ffi::c_double = gridsT
        .offset((GRID_BLKSIZE * 3 as ::core::ffi::c_int) as isize);
    let mut g1: *mut ::core::ffi::c_double = g.offset(leng as isize);
    let mut gout: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut gctri: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut gctrj: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    if n_comp == 1 as ::core::ffi::c_int {
        gctrj = gctr;
    } else {
        gctrj = g1;
        g1 = g1.offset(lenj as isize);
    }
    if j_ctr == 1 as ::core::ffi::c_int {
        gctri = gctrj;
        iempty = jempty;
    } else {
        gctri = g1;
        g1 = g1.offset(leni as isize);
    }
    if i_ctr == 1 as ::core::ffi::c_int {
        gout = gctri;
        gempty = iempty;
    } else {
        gout = g1;
    }
    grids_offset = 0 as ::core::ffi::c_int;
    while grids_offset < ngrids {
        (*envs).c2rust_unnamed.grids_offset = grids_offset;
        bgrids = if ngrids - grids_offset < 104 as ::core::ffi::c_int {
            ngrids - grids_offset
        } else {
            104 as ::core::ffi::c_int
        };
        i = 0 as ::core::ffi::c_int;
        while i < bgrids {
            *gridsT.offset((i + GRID_BLKSIZE * 0 as ::core::ffi::c_int) as isize) = *grids
                .offset(
                    ((grids_offset + i) * 3 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int) as isize,
                );
            *gridsT.offset((i + GRID_BLKSIZE * 1 as ::core::ffi::c_int) as isize) = *grids
                .offset(
                    ((grids_offset + i) * 3 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as isize,
                );
            *gridsT.offset((i + GRID_BLKSIZE * 2 as ::core::ffi::c_int) as isize) = *grids
                .offset(
                    ((grids_offset + i) * 3 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int) as isize,
                );
            i += 1;
        }
        empty[0 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        empty[1 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        empty[2 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
        if n_comp == 1 as ::core::ffi::c_int {
            gctrj = gctr.offset((grids_offset * nf * nc) as isize);
            if j_ctr == 1 as ::core::ffi::c_int {
                gctri = gctrj;
            }
            if i_ctr == 1 as ::core::ffi::c_int {
                gout = gctri;
            }
        }
        pdata_ij = pdata_base;
        jp = 0 as ::core::ffi::c_int;
        while jp < j_prim {
            (*envs).aj[0 as ::core::ffi::c_int as usize] = *aj.offset(jp as isize);
            if j_ctr == 1 as ::core::ffi::c_int {
                fac1j = (*envs).common_factor * *cj.offset(jp as isize);
            } else {
                fac1j = (*envs).common_factor;
                *iempty = 1 as ::core::ffi::c_int;
            }
            ip = 0 as ::core::ffi::c_int;
            while ip < i_prim {
                if !((*pdata_ij).cceij > expcutoff) {
                    (*envs).ai[0 as ::core::ffi::c_int as usize] = *ai
                        .offset(ip as isize);
                    expij = (*pdata_ij).eij;
                    cutoff = expcutoff - (*pdata_ij).cceij;
                    rij = &raw mut (*pdata_ij).rij as *mut ::core::ffi::c_double;
                    (*envs).rij[0 as ::core::ffi::c_int as usize] = *rij
                        .offset(0 as ::core::ffi::c_int as isize);
                    (*envs).rij[1 as ::core::ffi::c_int as usize] = *rij
                        .offset(1 as ::core::ffi::c_int as isize);
                    (*envs).rij[2 as ::core::ffi::c_int as usize] = *rij
                        .offset(2 as ::core::ffi::c_int as isize);
                    if i_ctr == 1 as ::core::ffi::c_int {
                        fac1i = fac1j * *ci.offset(ip as isize) * expij;
                    } else {
                        fac1i = fac1j * expij;
                    }
                    (*envs).fac[0 as ::core::ffi::c_int as usize] = fac1i;
                    CINTg0_1e_grids(g, cutoff, envs, cache, gridsT);
                    ::core::mem::transmute::<
                        _,
                        fn(_, _, _, _, _),
                    >(
                        Some((*envs).f_gout.expect("non-null function pointer"))
                            .expect("non-null function pointer"),
                    )(gout, g, idx, envs, *gempty);
                    if i_ctr > 1 as ::core::ffi::c_int {
                        if *iempty != 0 {
                            CINTprim_to_ctr_0(
                                gctri,
                                gout,
                                ci.offset(ip as isize),
                                (bgrids * nf * n_comp) as size_t,
                                i_prim,
                                i_ctr,
                                *non0ctri.offset(ip as isize),
                                non0idxi.offset((ip * i_ctr) as isize),
                            );
                        } else {
                            CINTprim_to_ctr_1(
                                gctri,
                                gout,
                                ci.offset(ip as isize),
                                (bgrids * nf * n_comp) as size_t,
                                i_prim,
                                i_ctr,
                                *non0ctri.offset(ip as isize),
                                non0idxi.offset((ip * i_ctr) as isize),
                            );
                        }
                    }
                    *iempty = 0 as ::core::ffi::c_int;
                }
                ip += 1;
                pdata_ij = pdata_ij.offset(1);
            }
            if *iempty == 0 {
                if j_ctr > 1 as ::core::ffi::c_int {
                    if *jempty != 0 {
                        CINTprim_to_ctr_0(
                            gctrj,
                            gctri,
                            cj.offset(jp as isize),
                            (bgrids * nf * i_ctr * n_comp) as size_t,
                            j_prim,
                            j_ctr,
                            *non0ctrj.offset(jp as isize),
                            non0idxj.offset((jp * j_ctr) as isize),
                        );
                    } else {
                        CINTprim_to_ctr_1(
                            gctrj,
                            gctri,
                            cj.offset(jp as isize),
                            (bgrids * nf * i_ctr * n_comp) as size_t,
                            j_prim,
                            j_ctr,
                            *non0ctrj.offset(jp as isize),
                            non0idxj.offset((jp * j_ctr) as isize),
                        );
                    }
                }
                *jempty = 0 as ::core::ffi::c_int;
            }
            jp += 1;
        }
        if n_comp > 1 as ::core::ffi::c_int && *jempty == 0 {
            _transpose_comps(
                gctr.offset((grids_offset * nf * nc) as isize),
                gctrj,
                bgrids,
                nf * nc,
                ngrids,
                n_comp,
            );
        }
        all_empty &= *jempty;
        grids_offset += GRID_BLKSIZE;
    }
    return (all_empty == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn _transpose_comps(
    mut gctr: *mut ::core::ffi::c_double,
    mut gctrj: *mut ::core::ffi::c_double,
    mut bgrids: ::core::ffi::c_int,
    mut dij: ::core::ffi::c_int,
    mut ngrids: ::core::ffi::c_int,
    mut n_comp: ::core::ffi::c_int,
) {
    let mut n: ::core::ffi::c_int = 0;
    let mut ic: ::core::ffi::c_int = 0;
    let mut ig: ::core::ffi::c_int = 0;
    let mut pgctr: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut pgctrj: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    ic = 0 as ::core::ffi::c_int;
    while ic < n_comp {
        pgctr = gctr.offset((ic * dij * ngrids) as isize);
        n = 0 as ::core::ffi::c_int;
        while n < dij {
            pgctrj = gctrj.offset(((n * n_comp + ic) * bgrids) as isize);
            ig = 0 as ::core::ffi::c_int;
            while ig < bgrids {
                *pgctr.offset((ig + n * bgrids) as isize) = *pgctrj.offset(ig as isize);
                ig += 1;
            }
            n += 1;
        }
        ic += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn int1e_grids_cache_size(mut envs: *mut CINTEnvVars) -> size_t {
    let mut bas: *mut ::core::ffi::c_int = (*envs).bas;
    let mut shls: *mut ::core::ffi::c_int = (*envs).shls;
    let mut x_ctr: *mut ::core::ffi::c_int = &raw mut (*envs).x_ctr
        as *mut ::core::ffi::c_int;
    let mut ngrids: ::core::ffi::c_int = (*envs).c2rust_unnamed_0.ngrids;
    let mut nroots: ::core::ffi::c_int = (*envs).nrys_roots;
    let mut nf: ::core::ffi::c_int = (*envs).nf;
    let mut nc: ::core::ffi::c_int = ngrids * nf
        * *x_ctr.offset(0 as ::core::ffi::c_int as isize)
        * *x_ctr.offset(1 as ::core::ffi::c_int as isize);
    let mut n_comp: ::core::ffi::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut i_prim: ::core::ffi::c_int = *bas
        .offset(
            (BAS_SLOTS * *shls.offset(0 as ::core::ffi::c_int as isize)
                + 2 as ::core::ffi::c_int) as isize,
        );
    let mut j_prim: ::core::ffi::c_int = *bas
        .offset(
            (BAS_SLOTS * *shls.offset(1 as ::core::ffi::c_int as isize)
                + 2 as ::core::ffi::c_int) as isize,
        );
    let mut pdata_size: ::core::ffi::c_int = i_prim * j_prim * 5 as ::core::ffi::c_int
        + i_prim * *x_ctr.offset(0 as ::core::ffi::c_int as isize)
        + j_prim * *x_ctr.offset(1 as ::core::ffi::c_int as isize)
        + (i_prim + j_prim) * 2 as ::core::ffi::c_int
        + (*envs).nf * 3 as ::core::ffi::c_int;
    let mut leng: size_t = ((*envs).g_size * 3 as ::core::ffi::c_int
        * (((1 as ::core::ffi::c_int) << (*envs).gbits) + 1 as ::core::ffi::c_int))
        as size_t;
    let mut len0: size_t = (GRID_BLKSIZE * nf * n_comp) as size_t;
    let mut leni: size_t = len0
        .wrapping_mul(*x_ctr.offset(0 as ::core::ffi::c_int as isize) as size_t);
    let mut lenj: size_t = leni
        .wrapping_mul(*x_ctr.offset(1 as ::core::ffi::c_int as isize) as size_t);
    let mut cache_size: size_t = if ((nc * n_comp) as size_t)
        .wrapping_add(leng)
        .wrapping_add(len0)
        .wrapping_add(leni)
        .wrapping_add(lenj)
        .wrapping_add(pdata_size as size_t)
        .wrapping_add(
            (104 as ::core::ffi::c_int
                * (if n_comp > nroots + 10 as ::core::ffi::c_int {
                    n_comp
                } else {
                    nroots + 10 as ::core::ffi::c_int
                })) as size_t,
        )
        > (nc * n_comp
            + 104 as ::core::ffi::c_int * nf * 8 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as size_t
    {
        ((nc * n_comp) as size_t)
            .wrapping_add(leng)
            .wrapping_add(len0)
            .wrapping_add(leni)
            .wrapping_add(lenj)
            .wrapping_add(pdata_size as size_t)
            .wrapping_add(
                (104 as ::core::ffi::c_int
                    * (if n_comp > nroots + 10 as ::core::ffi::c_int {
                        n_comp
                    } else {
                        nroots + 10 as ::core::ffi::c_int
                    })) as size_t,
            )
    } else {
        (nc * n_comp
            + 104 as ::core::ffi::c_int * nf * 8 as ::core::ffi::c_int
                * 2 as ::core::ffi::c_int) as size_t
    };
    return cache_size.wrapping_add(32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn CINT1e_grids_drv(
    mut out: *mut ::core::ffi::c_double,
    mut dims: *mut ::core::ffi::c_int,
    mut envs: *mut CINTEnvVars,
    mut cache: *mut ::core::ffi::c_double,
    mut f_c2s: Option<unsafe extern "C" fn() -> ()>,
) -> ::core::ffi::c_int {
    if out.is_null() {
        return int1e_grids_cache_size(envs) as ::core::ffi::c_int;
    }
    let mut x_ctr: *mut ::core::ffi::c_int = &raw mut (*envs).x_ctr
        as *mut ::core::ffi::c_int;
    let mut ngrids_nf: ::core::ffi::c_int = (*envs).c2rust_unnamed_0.ngrids * (*envs).nf;
    let mut nc: ::core::ffi::c_int = ngrids_nf
        * *x_ctr.offset(0 as ::core::ffi::c_int as isize)
        * *x_ctr.offset(1 as ::core::ffi::c_int as isize);
    let mut n_comp: ::core::ffi::c_int = (*envs).ncomp_e1 * (*envs).ncomp_tensor;
    let mut stack: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    if cache.is_null() {
        let mut cache_size: size_t = int1e_grids_cache_size(envs);
        stack = malloc(
            (::core::mem::size_of::<::core::ffi::c_double>() as size_t)
                .wrapping_mul(cache_size),
        ) as *mut ::core::ffi::c_double;
        cache = stack;
    }
    let mut gctr: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    gctr = ((cache as uintptr_t).wrapping_add(63 as ::core::ffi::c_int as uintptr_t)
        & (64 as ::core::ffi::c_int as uintptr_t).wrapping_neg())
        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_double;
    cache = gctr.offset((nc * n_comp) as isize);
    let mut has_value: ::core::ffi::c_int = CINT1e_grids_loop(gctr, envs, cache);
    let mut counts: [::core::ffi::c_int; 4] = [0; 4];
    if dims.is_null() {
        dims = &raw mut counts as *mut ::core::ffi::c_int;
    }
    if f_c2s
        == ::core::mem::transmute::<
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
        )
    {
        counts[0 as ::core::ffi::c_int as usize] = ((*envs).i_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * *x_ctr.offset(0 as ::core::ffi::c_int as isize);
        counts[1 as ::core::ffi::c_int as usize] = ((*envs).j_l * 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) * *x_ctr.offset(1 as ::core::ffi::c_int as isize);
        counts[2 as ::core::ffi::c_int as usize] = (*envs).c2rust_unnamed_0.ngrids;
        counts[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
    } else if f_c2s
        == ::core::mem::transmute::<
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
        )
    {
        counts[0 as ::core::ffi::c_int as usize] = (*envs).nfi
            * *x_ctr.offset(0 as ::core::ffi::c_int as isize);
        counts[1 as ::core::ffi::c_int as usize] = (*envs).nfj
            * *x_ctr.offset(1 as ::core::ffi::c_int as isize);
        counts[2 as ::core::ffi::c_int as usize] = (*envs).c2rust_unnamed_0.ngrids;
        counts[3 as ::core::ffi::c_int as usize] = 1 as ::core::ffi::c_int;
    }
    let mut nout: ::core::ffi::c_int = *dims.offset(0 as ::core::ffi::c_int as isize)
        * *dims.offset(1 as ::core::ffi::c_int as isize)
        * *dims.offset(2 as ::core::ffi::c_int as isize);
    let mut n: ::core::ffi::c_int = 0;
    if has_value != 0 {
        n = 0 as ::core::ffi::c_int;
        while n < n_comp {
            ::core::mem::transmute::<
                _,
                fn(_, _, _, _, _),
            >(
                Some(f_c2s.expect("non-null function pointer"))
                    .expect("non-null function pointer"),
            )(
                out.offset((nout * n) as isize),
                gctr.offset((nc * n) as isize),
                dims,
                envs,
                cache,
            );
            n += 1;
        }
    } else {
        n = 0 as ::core::ffi::c_int;
        while n < n_comp {
            c2s_grids_dset0(
                out.offset((nout * n) as isize),
                dims,
                &raw mut counts as *mut ::core::ffi::c_int,
            );
            n += 1;
        }
    }
    if !stack.is_null() {
        free(stack as *mut ::core::ffi::c_void);
    }
    return has_value;
}
#[no_mangle]
pub unsafe extern "C" fn int1e_grids_sph(
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
        1 as ::core::ffi::c_int,
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
            CINTgout1e_grids
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
pub unsafe extern "C" fn int1e_grids_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    *opt = ::core::ptr::null_mut::<CINTOpt>();
}
#[no_mangle]
pub unsafe extern "C" fn int1e_grids_cart(
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
        1 as ::core::ffi::c_int,
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
            CINTgout1e_grids
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
pub unsafe extern "C" fn cint1e_grids_cart(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int1e_grids_cart(
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
pub unsafe extern "C" fn cint1e_grids_cart_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int1e_grids_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_grids_sph(
    mut out: *mut ::core::ffi::c_double,
    mut shls: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
    mut opt: *mut CINTOpt,
) -> ::core::ffi::c_int {
    return int1e_grids_sph(
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
pub unsafe extern "C" fn cint1e_grids_sph_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int1e_grids_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn cint1e_grids_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    int1e_grids_optimizer(opt, atm, natm, bas, nbas, env);
}
pub const GRID_BLKSIZE: ::core::ffi::c_int = 104 as ::core::ffi::c_int;
