extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn log(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn CINTg3c1e_index_xyz(idx: *mut ::core::ffi::c_int, envs: *const CINTEnvVars);
    fn CINTg2e_index_xyz(idx: *mut ::core::ffi::c_int, envs: *const CINTEnvVars);
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
    fn CINTinit_int3c2e_EnvVars(
        envs: *mut CINTEnvVars,
        ng: *mut ::core::ffi::c_int,
        shls: *mut ::core::ffi::c_int,
        atm: *mut ::core::ffi::c_int,
        natm: ::core::ffi::c_int,
        bas: *mut ::core::ffi::c_int,
        nbas: ::core::ffi::c_int,
        env: *mut ::core::ffi::c_double,
    );
    fn CINTinit_int2c2e_EnvVars(
        envs: *mut CINTEnvVars,
        ng: *mut ::core::ffi::c_int,
        shls: *mut ::core::ffi::c_int,
        atm: *mut ::core::ffi::c_int,
        natm: ::core::ffi::c_int,
        bas: *mut ::core::ffi::c_int,
        nbas: ::core::ffi::c_int,
        env: *mut ::core::ffi::c_double,
    );
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
    fn CINTinit_int3c1e_EnvVars(
        envs: *mut CINTEnvVars,
        ng: *mut ::core::ffi::c_int,
        shls: *mut ::core::ffi::c_int,
        atm: *mut ::core::ffi::c_int,
        natm: ::core::ffi::c_int,
        bas: *mut ::core::ffi::c_int,
        nbas: ::core::ffi::c_int,
        env: *mut ::core::ffi::c_double,
    );
    fn CINTg1e_index_xyz(idx: *mut ::core::ffi::c_int, envs: *mut CINTEnvVars);
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();
pub const PTR_EXPCUTOFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTR_RANGE_OMEGA: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const ATM_SLOTS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const ANG_OF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BAS_SLOTS: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const ANG_MAX: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const LMAX1: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn CINTinit_2e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut opt0: *mut CINTOpt = malloc(::core::mem::size_of::<CINTOpt>() as size_t)
        as *mut CINTOpt;
    (*opt0).index_xyz_array = ::core::ptr::null_mut::<*mut ::core::ffi::c_int>();
    (*opt0).non0ctr = ::core::ptr::null_mut::<*mut ::core::ffi::c_int>();
    (*opt0).sortedidx = ::core::ptr::null_mut::<*mut ::core::ffi::c_int>();
    (*opt0).nbas = nbas;
    (*opt0).log_max_coeff = ::core::ptr::null_mut::<*mut ::core::ffi::c_double>();
    (*opt0).pairdata = ::core::ptr::null_mut::<*mut PairData>();
    *opt = opt0;
}
#[no_mangle]
pub unsafe extern "C" fn CINTinit_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
}
#[no_mangle]
pub unsafe extern "C" fn CINTdel_2e_optimizer(mut opt: *mut *mut CINTOpt) {
    let mut opt0: *mut CINTOpt = *opt;
    if opt0.is_null() {
        return;
    }
    if !(*opt0).index_xyz_array.is_null() {
        free(
            *(*opt0).index_xyz_array.offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
        );
        free((*opt0).index_xyz_array as *mut ::core::ffi::c_void);
    }
    if !(*opt0).non0ctr.is_null() {
        free(
            *(*opt0).sortedidx.offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
        );
        free((*opt0).sortedidx as *mut ::core::ffi::c_void);
        free(
            *(*opt0).non0ctr.offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
        );
        free((*opt0).non0ctr as *mut ::core::ffi::c_void);
    }
    if !(*opt0).log_max_coeff.is_null() {
        free(
            *(*opt0).log_max_coeff.offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
        );
        free((*opt0).log_max_coeff as *mut ::core::ffi::c_void);
    }
    CINTdel_pairdata_optimizer(opt0);
    free(opt0 as *mut ::core::ffi::c_void);
    *opt = ::core::ptr::null_mut::<CINTOpt>();
}
#[no_mangle]
pub unsafe extern "C" fn CINTdel_optimizer(mut opt: *mut *mut CINTOpt) {
    CINTdel_2e_optimizer(opt);
}
#[no_mangle]
pub unsafe extern "C" fn CINTno_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    *opt = ::core::ptr::null_mut::<CINTOpt>();
}
unsafe extern "C" fn _make_fakebas(
    mut fakebas: *mut ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut max_l: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nbas {
        max_l = if max_l
            > *bas
                .offset((8 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize)
        {
            max_l
        } else {
            *bas.offset((8 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize)
        };
        i += 1;
    }
    let mut fakenbas: ::core::ffi::c_int = max_l + 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < BAS_SLOTS * fakenbas {
        *fakebas.offset(i as isize) = 0 as ::core::ffi::c_int;
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i <= max_l {
        *fakebas.offset((BAS_SLOTS * i + ANG_OF) as isize) = i;
        i += 1;
    }
    return max_l;
}
unsafe extern "C" fn _allocate_index_xyz(
    mut opt: *mut CINTOpt,
    mut max_l: ::core::ffi::c_int,
    mut l_allow: ::core::ffi::c_int,
    mut order: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut cumcart: ::core::ffi::c_int = (l_allow + 1 as ::core::ffi::c_int)
        * (l_allow + 2 as ::core::ffi::c_int) * (l_allow + 3 as ::core::ffi::c_int)
        / 6 as ::core::ffi::c_int;
    let mut ll: size_t = (max_l + 1 as ::core::ffi::c_int) as size_t;
    let mut cc: size_t = cumcart as size_t;
    i = 1 as ::core::ffi::c_int;
    while i < order {
        ll = (ll as ::core::ffi::c_ulong).wrapping_mul(LMAX1 as ::core::ffi::c_ulong)
            as size_t as size_t;
        cc = (cc as ::core::ffi::c_ulong).wrapping_mul(cumcart as ::core::ffi::c_ulong)
            as size_t as size_t;
        i += 1;
    }
    let mut buf: *mut ::core::ffi::c_int = malloc(
        (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
            .wrapping_mul(cc)
            .wrapping_mul(3 as size_t),
    ) as *mut ::core::ffi::c_int;
    let mut ppbuf: *mut *mut ::core::ffi::c_int = malloc(
        (::core::mem::size_of::<*mut ::core::ffi::c_int>() as size_t).wrapping_mul(ll),
    ) as *mut *mut ::core::ffi::c_int;
    let ref mut fresh15 = *ppbuf.offset(0 as ::core::ffi::c_int as isize);
    *fresh15 = buf;
    i = 1 as ::core::ffi::c_int;
    while (i as size_t) < ll {
        let ref mut fresh16 = *ppbuf.offset(i as isize);
        *fresh16 = ::core::ptr::null_mut::<::core::ffi::c_int>();
        i += 1;
    }
    (*opt).index_xyz_array = ppbuf;
    return buf;
}
unsafe extern "C" fn gen_idx(
    mut opt: *mut CINTOpt,
    mut finit: Option<unsafe extern "C" fn() -> ()>,
    mut findex_xyz: Option<unsafe extern "C" fn() -> ()>,
    mut order: ::core::ffi::c_int,
    mut l_allow: ::core::ffi::c_int,
    mut ng: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut ptr: ::core::ffi::c_int = 0;
    let mut fakebas: [::core::ffi::c_int; 128] = [0; 128];
    let mut max_l: ::core::ffi::c_int = _make_fakebas(
        &raw mut fakebas as *mut ::core::ffi::c_int,
        bas,
        nbas,
        env,
    );
    let mut fakenbas: ::core::ffi::c_int = max_l + 1 as ::core::ffi::c_int;
    l_allow = if max_l < l_allow { max_l } else { l_allow };
    let mut buf: *mut ::core::ffi::c_int = _allocate_index_xyz(
        opt,
        max_l,
        l_allow,
        order,
    );
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
    let mut shls: [::core::ffi::c_int; 4] = [0 as ::core::ffi::c_int; 4];
    if order == 2 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i <= l_allow {
            j = 0 as ::core::ffi::c_int;
            while j <= l_allow {
                shls[0 as ::core::ffi::c_int as usize] = i;
                shls[1 as ::core::ffi::c_int as usize] = j;
                ::core::mem::transmute::<
                    _,
                    fn(_, _, _, _, _, _, _, _),
                >(
                    Some(finit.expect("non-null function pointer"))
                        .expect("non-null function pointer"),
                )(
                    &raw mut envs,
                    ng,
                    &raw mut shls as *mut ::core::ffi::c_int,
                    atm,
                    natm,
                    &raw mut fakebas as *mut ::core::ffi::c_int,
                    fakenbas,
                    env,
                );
                ptr = i * LMAX1 + j;
                let ref mut fresh12 = *(*opt).index_xyz_array.offset(ptr as isize);
                *fresh12 = buf;
                ::core::mem::transmute::<
                    _,
                    fn(_, _),
                >(
                    Some(findex_xyz.expect("non-null function pointer"))
                        .expect("non-null function pointer"),
                )(buf, &raw mut envs);
                buf = buf.offset((envs.nf * 3 as ::core::ffi::c_int) as isize);
                j += 1;
            }
            i += 1;
        }
    } else if order == 3 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i <= l_allow {
            j = 0 as ::core::ffi::c_int;
            while j <= l_allow {
                k = 0 as ::core::ffi::c_int;
                while k <= l_allow {
                    shls[0 as ::core::ffi::c_int as usize] = i;
                    shls[1 as ::core::ffi::c_int as usize] = j;
                    shls[2 as ::core::ffi::c_int as usize] = k;
                    ::core::mem::transmute::<
                        _,
                        fn(_, _, _, _, _, _, _, _),
                    >(
                        Some(finit.expect("non-null function pointer"))
                            .expect("non-null function pointer"),
                    )(
                        &raw mut envs,
                        ng,
                        &raw mut shls as *mut ::core::ffi::c_int,
                        atm,
                        natm,
                        &raw mut fakebas as *mut ::core::ffi::c_int,
                        fakenbas,
                        env,
                    );
                    ptr = i * LMAX1 * LMAX1 + j * LMAX1 + k;
                    let ref mut fresh13 = *(*opt).index_xyz_array.offset(ptr as isize);
                    *fresh13 = buf;
                    ::core::mem::transmute::<
                        _,
                        fn(_, _),
                    >(
                        Some(findex_xyz.expect("non-null function pointer"))
                            .expect("non-null function pointer"),
                    )(buf, &raw mut envs);
                    buf = buf.offset((envs.nf * 3 as ::core::ffi::c_int) as isize);
                    k += 1;
                }
                j += 1;
            }
            i += 1;
        }
    } else {
        i = 0 as ::core::ffi::c_int;
        while i <= l_allow {
            j = 0 as ::core::ffi::c_int;
            while j <= l_allow {
                k = 0 as ::core::ffi::c_int;
                while k <= l_allow {
                    l = 0 as ::core::ffi::c_int;
                    while l <= l_allow {
                        shls[0 as ::core::ffi::c_int as usize] = i;
                        shls[1 as ::core::ffi::c_int as usize] = j;
                        shls[2 as ::core::ffi::c_int as usize] = k;
                        shls[3 as ::core::ffi::c_int as usize] = l;
                        ::core::mem::transmute::<
                            _,
                            fn(_, _, _, _, _, _, _, _),
                        >(
                            Some(finit.expect("non-null function pointer"))
                                .expect("non-null function pointer"),
                        )(
                            &raw mut envs,
                            ng,
                            &raw mut shls as *mut ::core::ffi::c_int,
                            atm,
                            natm,
                            &raw mut fakebas as *mut ::core::ffi::c_int,
                            fakenbas,
                            env,
                        );
                        ptr = i * LMAX1 * LMAX1 * LMAX1 + j * LMAX1 * LMAX1 + k * LMAX1
                            + l;
                        let ref mut fresh14 = *(*opt)
                            .index_xyz_array
                            .offset(ptr as isize);
                        *fresh14 = buf;
                        ::core::mem::transmute::<
                            _,
                            fn(_, _),
                        >(
                            Some(findex_xyz.expect("non-null function pointer"))
                                .expect("non-null function pointer"),
                        )(buf, &raw mut envs);
                        buf = buf.offset((envs.nf * 3 as ::core::ffi::c_int) as isize);
                        l += 1;
                    }
                    k += 1;
                }
                j += 1;
            }
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CINTall_1e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut ng: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
    CINTOpt_set_log_maxc(*opt, atm, natm, bas, nbas, env);
    CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
    gen_idx(
        *opt,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTinit_int1e_EnvVars
                    as unsafe extern "C" fn(
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_int, *mut CINTEnvVars) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg1e_index_xyz
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                    ) -> (),
            ),
        ),
        2 as ::core::ffi::c_int,
        ANG_MAX,
        ng,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTall_2e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut ng: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
    CINTOpt_setij(*opt, ng, atm, natm, bas, nbas, env);
    CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
    gen_idx(
        *opt,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTinit_int2e_EnvVars
                    as unsafe extern "C" fn(
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_int, *const CINTEnvVars) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg2e_index_xyz
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_int,
                        *const CINTEnvVars,
                    ) -> (),
            ),
        ),
        4 as ::core::ffi::c_int,
        6 as ::core::ffi::c_int,
        ng,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTall_3c2e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut ng: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
    CINTOpt_setij(*opt, ng, atm, natm, bas, nbas, env);
    CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
    gen_idx(
        *opt,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTinit_int3c2e_EnvVars
                    as unsafe extern "C" fn(
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_int, *const CINTEnvVars) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg2e_index_xyz
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_int,
                        *const CINTEnvVars,
                    ) -> (),
            ),
        ),
        3 as ::core::ffi::c_int,
        12 as ::core::ffi::c_int,
        ng,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTall_2c2e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut ng: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
    CINTOpt_set_log_maxc(*opt, atm, natm, bas, nbas, env);
    CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
    gen_idx(
        *opt,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTinit_int2c2e_EnvVars
                    as unsafe extern "C" fn(
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_int, *mut CINTEnvVars) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg1e_index_xyz
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                    ) -> (),
            ),
        ),
        2 as ::core::ffi::c_int,
        ANG_MAX,
        ng,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTall_3c1e_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut ng: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
    CINTOpt_setij(*opt, ng, atm, natm, bas, nbas, env);
    CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
    gen_idx(
        *opt,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTinit_int3c1e_EnvVars
                    as unsafe extern "C" fn(
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_int, *const CINTEnvVars) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg3c1e_index_xyz
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_int,
                        *const CINTEnvVars,
                    ) -> (),
            ),
        ),
        3 as ::core::ffi::c_int,
        12 as ::core::ffi::c_int,
        ng,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTall_1e_grids_optimizer(
    mut opt: *mut *mut CINTOpt,
    mut ng: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    CINTinit_2e_optimizer(opt, atm, natm, bas, nbas, env);
    CINTOpt_set_log_maxc(*opt, atm, natm, bas, nbas, env);
    CINTOpt_set_non0coeff(*opt, atm, natm, bas, nbas, env);
    gen_idx(
        *opt,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut CINTEnvVars,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_double,
                ) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTinit_int1e_grids_EnvVars
                    as unsafe extern "C" fn(
                        *mut CINTEnvVars,
                        *mut ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            ),
        ),
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_int, *mut CINTEnvVars) -> (),
            >,
            Option<unsafe extern "C" fn() -> ()>,
        >(
            Some(
                CINTg1e_index_xyz
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_int,
                        *mut CINTEnvVars,
                    ) -> (),
            ),
        ),
        2 as ::core::ffi::c_int,
        ANG_MAX,
        ng,
        atm,
        natm,
        bas,
        nbas,
        env,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CINTOpt_log_max_pgto_coeff(
    mut log_maxc: *mut ::core::ffi::c_double,
    mut coeff: *mut ::core::ffi::c_double,
    mut nprim: ::core::ffi::c_int,
    mut nctr: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut ip: ::core::ffi::c_int = 0;
    let mut maxc: ::core::ffi::c_double = 0.;
    ip = 0 as ::core::ffi::c_int;
    while ip < nprim {
        maxc = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        i = 0 as ::core::ffi::c_int;
        while i < nctr {
            maxc = if maxc > fabs(*coeff.offset((i * nprim + ip) as isize)) {
                maxc
            } else {
                fabs(*coeff.offset((i * nprim + ip) as isize))
            };
            i += 1;
        }
        *log_maxc.offset(ip as isize) = log(maxc);
        ip += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTOpt_set_log_maxc(
    mut opt: *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut iprim: ::core::ffi::c_int = 0;
    let mut ictr: ::core::ffi::c_int = 0;
    let mut ci: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut tot_prim: size_t = 0 as size_t;
    i = 0 as ::core::ffi::c_int;
    while i < nbas {
        tot_prim = (tot_prim as ::core::ffi::c_ulong)
            .wrapping_add(
                *bas.offset((BAS_SLOTS * i + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong,
            ) as size_t as size_t;
        i += 1;
    }
    if tot_prim == 0 as size_t {
        return;
    }
    (*opt).log_max_coeff = malloc(
        (::core::mem::size_of::<*mut ::core::ffi::c_double>() as size_t)
            .wrapping_mul(
                (if nbas > 1 as ::core::ffi::c_int {
                    nbas
                } else {
                    1 as ::core::ffi::c_int
                }) as size_t,
            ),
    ) as *mut *mut ::core::ffi::c_double;
    let mut plog_maxc: *mut ::core::ffi::c_double = malloc(
        (::core::mem::size_of::<::core::ffi::c_double>() as size_t)
            .wrapping_mul(tot_prim),
    ) as *mut ::core::ffi::c_double;
    let ref mut fresh0 = *(*opt).log_max_coeff.offset(0 as ::core::ffi::c_int as isize);
    *fresh0 = plog_maxc;
    i = 0 as ::core::ffi::c_int;
    while i < nbas {
        iprim = *bas.offset((BAS_SLOTS * i + 2 as ::core::ffi::c_int) as isize);
        ictr = *bas.offset((BAS_SLOTS * i + 3 as ::core::ffi::c_int) as isize);
        ci = env
            .offset(
                *bas.offset((BAS_SLOTS * i + 6 as ::core::ffi::c_int) as isize) as isize,
            );
        let ref mut fresh1 = *(*opt).log_max_coeff.offset(i as isize);
        *fresh1 = plog_maxc;
        CINTOpt_log_max_pgto_coeff(plog_maxc, ci, iprim, ictr);
        plog_maxc = plog_maxc.offset(iprim as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTset_pairdata(
    mut pairdata: *mut PairData,
    mut ai: *mut ::core::ffi::c_double,
    mut aj: *mut ::core::ffi::c_double,
    mut ri: *mut ::core::ffi::c_double,
    mut rj: *mut ::core::ffi::c_double,
    mut log_maxci: *mut ::core::ffi::c_double,
    mut log_maxcj: *mut ::core::ffi::c_double,
    mut li_ceil: ::core::ffi::c_int,
    mut lj_ceil: ::core::ffi::c_int,
    mut iprim: ::core::ffi::c_int,
    mut jprim: ::core::ffi::c_int,
    mut rr_ij: ::core::ffi::c_double,
    mut expcutoff: ::core::ffi::c_double,
    mut env: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut ip: ::core::ffi::c_int = 0;
    let mut jp: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut aij: ::core::ffi::c_double = 0.;
    let mut eij: ::core::ffi::c_double = 0.;
    let mut cceij: ::core::ffi::c_double = 0.;
    let mut wj: ::core::ffi::c_double = 0.;
    aij = *ai.offset((iprim - 1 as ::core::ffi::c_int) as isize)
        + *aj.offset((jprim - 1 as ::core::ffi::c_int) as isize);
    let mut log_rr_ij: ::core::ffi::c_double = 1.7f64 - 1.5f64 * log(aij);
    let mut lij: ::core::ffi::c_int = li_ceil + lj_ceil;
    if lij > 0 as ::core::ffi::c_int {
        let mut dist_ij: ::core::ffi::c_double = sqrt(rr_ij);
        let mut omega: ::core::ffi::c_double = *env.offset(PTR_RANGE_OMEGA as isize);
        if omega < 0 as ::core::ffi::c_int as ::core::ffi::c_double {
            let mut r_guess: ::core::ffi::c_double = 8.0f64;
            let mut omega2: ::core::ffi::c_double = omega * omega;
            let mut theta: ::core::ffi::c_double = omega2 / (omega2 + aij);
            log_rr_ij
                += lij as ::core::ffi::c_double
                    * log(dist_ij + theta * r_guess + 1.0f64);
        } else {
            log_rr_ij += lij as ::core::ffi::c_double * log(dist_ij + 1.0f64);
        }
    }
    let mut pdata: *mut PairData = ::core::ptr::null_mut::<PairData>();
    let mut empty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    n = 0 as ::core::ffi::c_int;
    jp = 0 as ::core::ffi::c_int;
    while jp < jprim {
        ip = 0 as ::core::ffi::c_int;
        while ip < iprim {
            aij = 1 as ::core::ffi::c_int as ::core::ffi::c_double
                / (*ai.offset(ip as isize) + *aj.offset(jp as isize));
            eij = rr_ij * *ai.offset(ip as isize) * *aj.offset(jp as isize) * aij;
            cceij = eij - log_rr_ij - *log_maxci.offset(ip as isize)
                - *log_maxcj.offset(jp as isize);
            pdata = pairdata.offset(n as isize);
            (*pdata).cceij = cceij;
            if cceij < expcutoff {
                empty = 0 as ::core::ffi::c_int;
                wj = *aj.offset(jp as isize) * aij;
                (*pdata).rij[0 as ::core::ffi::c_int as usize] = *ri
                    .offset(0 as ::core::ffi::c_int as isize)
                    + wj
                        * (*rj.offset(0 as ::core::ffi::c_int as isize)
                            - *ri.offset(0 as ::core::ffi::c_int as isize));
                (*pdata).rij[1 as ::core::ffi::c_int as usize] = *ri
                    .offset(1 as ::core::ffi::c_int as isize)
                    + wj
                        * (*rj.offset(1 as ::core::ffi::c_int as isize)
                            - *ri.offset(1 as ::core::ffi::c_int as isize));
                (*pdata).rij[2 as ::core::ffi::c_int as usize] = *ri
                    .offset(2 as ::core::ffi::c_int as isize)
                    + wj
                        * (*rj.offset(2 as ::core::ffi::c_int as isize)
                            - *ri.offset(2 as ::core::ffi::c_int as isize));
                (*pdata).eij = exp(-eij);
            } else {
                (*pdata).rij[0 as ::core::ffi::c_int as usize] = 1e18f64;
                (*pdata).rij[1 as ::core::ffi::c_int as usize] = 1e18f64;
                (*pdata).rij[2 as ::core::ffi::c_int as usize] = 1e18f64;
                (*pdata).eij = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            ip += 1;
            n += 1;
        }
        jp += 1;
    }
    return empty;
}
#[no_mangle]
pub unsafe extern "C" fn CINTOpt_setij(
    mut opt: *mut CINTOpt,
    mut ng: *mut ::core::ffi::c_int,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut ip: ::core::ffi::c_int = 0;
    let mut jp: ::core::ffi::c_int = 0;
    let mut iprim: ::core::ffi::c_int = 0;
    let mut jprim: ::core::ffi::c_int = 0;
    let mut li: ::core::ffi::c_int = 0;
    let mut lj: ::core::ffi::c_int = 0;
    let mut ai: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut aj: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut ri: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut rj: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut expcutoff: ::core::ffi::c_double = 0.;
    if *env.offset(PTR_EXPCUTOFF as isize)
        == 0 as ::core::ffi::c_int as ::core::ffi::c_double
    {
        expcutoff = EXPCUTOFF as ::core::ffi::c_double;
    } else {
        expcutoff = if 40 as ::core::ffi::c_int as ::core::ffi::c_double
            > *env.offset(0 as ::core::ffi::c_int as isize)
        {
            40 as ::core::ffi::c_int as ::core::ffi::c_double
        } else {
            *env.offset(0 as ::core::ffi::c_int as isize)
        };
    }
    if (*opt).log_max_coeff.is_null() {
        CINTOpt_set_log_maxc(opt, atm, natm, bas, nbas, env);
    }
    let mut log_max_coeff: *mut *mut ::core::ffi::c_double = (*opt).log_max_coeff;
    let mut log_maxci: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut log_maxcj: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut tot_prim: size_t = 0 as size_t;
    i = 0 as ::core::ffi::c_int;
    while i < nbas {
        tot_prim = (tot_prim as ::core::ffi::c_ulong)
            .wrapping_add(
                *bas.offset((BAS_SLOTS * i + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong,
            ) as size_t as size_t;
        i += 1;
    }
    if tot_prim == 0 as size_t || tot_prim > MAX_PGTO_FOR_PAIRDATA as size_t {
        return;
    }
    (*opt).pairdata = malloc(
        (::core::mem::size_of::<*mut PairData>() as size_t)
            .wrapping_mul(
                (if nbas * nbas > 1 as ::core::ffi::c_int {
                    nbas * nbas
                } else {
                    1 as ::core::ffi::c_int
                }) as size_t,
            ),
    ) as *mut *mut PairData;
    let mut pdata: *mut PairData = malloc(
        (::core::mem::size_of::<PairData>() as size_t)
            .wrapping_mul(tot_prim)
            .wrapping_mul(tot_prim),
    ) as *mut PairData;
    let ref mut fresh2 = *(*opt).pairdata.offset(0 as ::core::ffi::c_int as isize);
    *fresh2 = pdata;
    let mut ijkl_inc: ::core::ffi::c_int = 0;
    if *ng.offset(IINC as isize) + *ng.offset(JINC as isize)
        > *ng.offset(KINC as isize) + *ng.offset(LINC as isize)
    {
        ijkl_inc = *ng.offset(IINC as isize) + *ng.offset(JINC as isize);
    } else {
        ijkl_inc = *ng.offset(KINC as isize) + *ng.offset(LINC as isize);
    }
    let mut empty: ::core::ffi::c_int = 0;
    let mut rr: ::core::ffi::c_double = 0.;
    let mut pdata0: *mut PairData = ::core::ptr::null_mut::<PairData>();
    i = 0 as ::core::ffi::c_int;
    while i < nbas {
        ri = env
            .offset(
                *atm
                    .offset(
                        (ATM_SLOTS
                            * *bas
                                .offset(
                                    (8 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int)
                                        as isize,
                                ) + 1 as ::core::ffi::c_int) as isize,
                    ) as isize,
            );
        ai = env
            .offset(
                *bas.offset((BAS_SLOTS * i + 5 as ::core::ffi::c_int) as isize) as isize,
            );
        iprim = *bas.offset((BAS_SLOTS * i + 2 as ::core::ffi::c_int) as isize);
        li = *bas.offset((BAS_SLOTS * i + 1 as ::core::ffi::c_int) as isize);
        log_maxci = *log_max_coeff.offset(i as isize);
        j = 0 as ::core::ffi::c_int;
        while j <= i {
            rj = env
                .offset(
                    *atm
                        .offset(
                            (ATM_SLOTS
                                * *bas
                                    .offset(
                                        (8 as ::core::ffi::c_int * j + 0 as ::core::ffi::c_int)
                                            as isize,
                                    ) + 1 as ::core::ffi::c_int) as isize,
                        ) as isize,
                );
            aj = env
                .offset(
                    *bas.offset((BAS_SLOTS * j + 5 as ::core::ffi::c_int) as isize)
                        as isize,
                );
            jprim = *bas.offset((BAS_SLOTS * j + 2 as ::core::ffi::c_int) as isize);
            lj = *bas.offset((BAS_SLOTS * j + 1 as ::core::ffi::c_int) as isize);
            log_maxcj = *log_max_coeff.offset(j as isize);
            rr = (*ri.offset(0 as ::core::ffi::c_int as isize)
                - *rj.offset(0 as ::core::ffi::c_int as isize))
                * (*ri.offset(0 as ::core::ffi::c_int as isize)
                    - *rj.offset(0 as ::core::ffi::c_int as isize))
                + (*ri.offset(1 as ::core::ffi::c_int as isize)
                    - *rj.offset(1 as ::core::ffi::c_int as isize))
                    * (*ri.offset(1 as ::core::ffi::c_int as isize)
                        - *rj.offset(1 as ::core::ffi::c_int as isize))
                + (*ri.offset(2 as ::core::ffi::c_int as isize)
                    - *rj.offset(2 as ::core::ffi::c_int as isize))
                    * (*ri.offset(2 as ::core::ffi::c_int as isize)
                        - *rj.offset(2 as ::core::ffi::c_int as isize));
            empty = CINTset_pairdata(
                pdata,
                ai,
                aj,
                ri,
                rj,
                log_maxci,
                log_maxcj,
                li + ijkl_inc,
                lj,
                iprim,
                jprim,
                rr,
                expcutoff,
                env,
            );
            if i == 0 as ::core::ffi::c_int && j == 0 as ::core::ffi::c_int {
                let ref mut fresh3 = *(*opt)
                    .pairdata
                    .offset(0 as ::core::ffi::c_int as isize);
                *fresh3 = pdata;
                pdata = pdata.offset((iprim * jprim) as isize);
            } else if empty == 0 {
                let ref mut fresh4 = *(*opt).pairdata.offset((i * nbas + j) as isize);
                *fresh4 = pdata;
                pdata = pdata.offset((iprim * jprim) as isize);
                if i != j {
                    let ref mut fresh5 = *(*opt)
                        .pairdata
                        .offset((j * nbas + i) as isize);
                    *fresh5 = pdata;
                    pdata0 = *(*opt).pairdata.offset((i * nbas + j) as isize);
                    ip = 0 as ::core::ffi::c_int;
                    while ip < iprim {
                        jp = 0 as ::core::ffi::c_int;
                        while jp < jprim {
                            memcpy(
                                pdata as *mut ::core::ffi::c_void,
                                pdata0.offset((jp * iprim) as isize).offset(ip as isize)
                                    as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<PairData>() as size_t,
                            );
                            jp += 1;
                            pdata = pdata.offset(1);
                        }
                        ip += 1;
                    }
                }
            } else {
                let ref mut fresh6 = *(*opt).pairdata.offset((i * nbas + j) as isize);
                *fresh6 = NOVALUE as *mut PairData;
                let ref mut fresh7 = *(*opt).pairdata.offset((j * nbas + i) as isize);
                *fresh7 = NOVALUE as *mut PairData;
            }
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTdel_pairdata_optimizer(mut cintopt: *mut CINTOpt) {
    if !cintopt.is_null() && !(*cintopt).pairdata.is_null() {
        free(
            *(*cintopt).pairdata.offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
        );
        free((*cintopt).pairdata as *mut ::core::ffi::c_void);
        (*cintopt).pairdata = ::core::ptr::null_mut::<*mut PairData>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTOpt_non0coeff_byshell(
    mut sortedidx: *mut ::core::ffi::c_int,
    mut non0ctr: *mut ::core::ffi::c_int,
    mut ci: *mut ::core::ffi::c_double,
    mut iprim: ::core::ffi::c_int,
    mut ictr: ::core::ffi::c_int,
) {
    let mut ip: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut kp: ::core::ffi::c_int = 0;
    let vla = ictr as usize;
    let mut zeroidx: Vec<::core::ffi::c_int> = ::std::vec::from_elem(0, vla);
    ip = 0 as ::core::ffi::c_int;
    while ip < iprim {
        j = 0 as ::core::ffi::c_int;
        k = 0 as ::core::ffi::c_int;
        kp = 0 as ::core::ffi::c_int;
        while j < ictr {
            if *ci.offset((iprim * j + ip) as isize)
                != 0 as ::core::ffi::c_int as ::core::ffi::c_double
            {
                *sortedidx.offset(k as isize) = j;
                k += 1;
            } else {
                *zeroidx.as_mut_ptr().offset(kp as isize) = j;
                kp += 1;
            }
            j += 1;
        }
        j = 0 as ::core::ffi::c_int;
        while j < kp {
            *sortedidx.offset((k + j) as isize) = *zeroidx
                .as_mut_ptr()
                .offset(j as isize);
            j += 1;
        }
        *non0ctr.offset(ip as isize) = k;
        sortedidx = sortedidx.offset(ictr as isize);
        ip += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CINTOpt_set_non0coeff(
    mut opt: *mut CINTOpt,
    mut atm: *mut ::core::ffi::c_int,
    mut natm: ::core::ffi::c_int,
    mut bas: *mut ::core::ffi::c_int,
    mut nbas: ::core::ffi::c_int,
    mut env: *mut ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut iprim: ::core::ffi::c_int = 0;
    let mut ictr: ::core::ffi::c_int = 0;
    let mut ci: *mut ::core::ffi::c_double = ::core::ptr::null_mut::<
        ::core::ffi::c_double,
    >();
    let mut tot_prim: size_t = 0 as size_t;
    let mut tot_prim_ctr: size_t = 0 as size_t;
    i = 0 as ::core::ffi::c_int;
    while i < nbas {
        tot_prim = (tot_prim as ::core::ffi::c_ulong)
            .wrapping_add(
                *bas.offset((BAS_SLOTS * i + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong,
            ) as size_t as size_t;
        tot_prim_ctr = (tot_prim_ctr as ::core::ffi::c_ulong)
            .wrapping_add(
                (*bas.offset((BAS_SLOTS * i + 2 as ::core::ffi::c_int) as isize)
                    * *bas.offset((BAS_SLOTS * i + 3 as ::core::ffi::c_int) as isize))
                    as ::core::ffi::c_ulong,
            ) as size_t as size_t;
        i += 1;
    }
    if tot_prim == 0 as size_t {
        return;
    }
    (*opt).non0ctr = malloc(
        (::core::mem::size_of::<*mut ::core::ffi::c_int>() as size_t)
            .wrapping_mul(
                (if nbas > 1 as ::core::ffi::c_int {
                    nbas
                } else {
                    1 as ::core::ffi::c_int
                }) as size_t,
            ),
    ) as *mut *mut ::core::ffi::c_int;
    (*opt).sortedidx = malloc(
        (::core::mem::size_of::<*mut ::core::ffi::c_int>() as size_t)
            .wrapping_mul(
                (if nbas > 1 as ::core::ffi::c_int {
                    nbas
                } else {
                    1 as ::core::ffi::c_int
                }) as size_t,
            ),
    ) as *mut *mut ::core::ffi::c_int;
    let mut pnon0ctr: *mut ::core::ffi::c_int = malloc(
        (::core::mem::size_of::<::core::ffi::c_int>() as size_t).wrapping_mul(tot_prim),
    ) as *mut ::core::ffi::c_int;
    let mut psortedidx: *mut ::core::ffi::c_int = malloc(
        (::core::mem::size_of::<::core::ffi::c_int>() as size_t)
            .wrapping_mul(tot_prim_ctr),
    ) as *mut ::core::ffi::c_int;
    let ref mut fresh8 = *(*opt).non0ctr.offset(0 as ::core::ffi::c_int as isize);
    *fresh8 = pnon0ctr;
    let ref mut fresh9 = *(*opt).sortedidx.offset(0 as ::core::ffi::c_int as isize);
    *fresh9 = psortedidx;
    i = 0 as ::core::ffi::c_int;
    while i < nbas {
        iprim = *bas.offset((BAS_SLOTS * i + 2 as ::core::ffi::c_int) as isize);
        ictr = *bas.offset((BAS_SLOTS * i + 3 as ::core::ffi::c_int) as isize);
        ci = env
            .offset(
                *bas.offset((BAS_SLOTS * i + 6 as ::core::ffi::c_int) as isize) as isize,
            );
        let ref mut fresh10 = *(*opt).non0ctr.offset(i as isize);
        *fresh10 = pnon0ctr;
        let ref mut fresh11 = *(*opt).sortedidx.offset(i as isize);
        *fresh11 = psortedidx;
        CINTOpt_non0coeff_byshell(psortedidx, pnon0ctr, ci, iprim, ictr);
        pnon0ctr = pnon0ctr.offset(iprim as isize);
        psortedidx = psortedidx.offset((iprim * ictr) as isize);
        i += 1;
    }
}
pub const NOVALUE: *mut ::core::ffi::c_void = 0xffffffffffffffff as ::core::ffi::c_ulong
    as *mut ::core::ffi::c_void;
pub const MAX_PGTO_FOR_PAIRDATA: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const IINC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const JINC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const KINC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LINC: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const EXPCUTOFF: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
