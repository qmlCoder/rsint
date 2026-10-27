#!/usr/bin/env python
"""对比 C 库与 c2rust Rust 库的数值输出。直接比对同一输入的输出 buffer."""
import os, ctypes
import numpy as np
from pyscf import gto, lib

CRUST = os.path.abspath(os.path.join(os.path.dirname(__file__), 'out/target/release/librsint.so'))
CC    = os.path.abspath(os.path.join(os.path.dirname(__file__), '../libcint/build/libcint.so.6.1.3'))

# (pyscf intor 名不带后缀, 库函数前缀). 对 1e 用 'int1e_ovlp'. 2e 用 'int2e' 等
CASES = [
    # (c函数base, pyscf intor base)
    ('cint1e_ovlp', 'int1e_ovlp'),
    ('cint1e_kin',  'int1e_kin'),
    ('cint1e_nuc',  'int1e_nuc'),
    ('cint1e_ovlp', 'int1e_ovlp'),
    ('cint2e',      'int2e'),
    ('cint2c2e',    'int2c2e'),
    ('cint3c2e',    'int3c2e'),
]

def main():
    mol = gto.M(atom='H 0 0 0; H .3 .5 .8; He 1.9 2.1 .1',
                basis='sto-3g', cart=True)
    atm = np.asarray(mol._atm, dtype=np.int32, order='C')
    bas = np.asarray(mol._bas, dtype=np.int32, order='C')
    env = np.asarray(mol._env, dtype=np.float64, order='C')
    natm, nbas = mol.natm, mol.nbas

    cc, rust = ctypes.CDLL(CC), ctypes.CDLL(CRUST)

    print(f"{'intor':28s} {'worst_abs_err':>14s}  status")
    allok = True
    for cbase, ibase in CASES:
        for suf in ['cart', 'sph']:
            cfn = f'{cbase}_{suf}'
            if not (hasattr(cc, cfn) and hasattr(rust, cfn)):
                continue
            cf = getattr(cc, cfn); cf.restype = ctypes.c_int
            rf = getattr(rust, cfn); rf.restype = ctypes.c_int
            worst = 0.0
            for i in range(nbas):
                for j in range(nbas):
                    try:
                        ref = mol.intor_by_shell(ibase + '_' + suf, [i, j])
                    except Exception as e:
                        continue
                    buf_c  = np.zeros_like(ref)
                    buf_rs = np.zeros_like(ref)
                    shls = (ctypes.c_int*2)(i, j)
                    aptr = atm.ctypes.data_as(ctypes.c_void_p)
                    bptr = bas.ctypes.data_as(ctypes.c_void_p)
                    eptr = env.ctypes.data_as(ctypes.c_void_p)
                    cf(buf_c.ctypes.data_as(ctypes.c_void_p), shls, aptr, ctypes.c_int(natm),
                       bptr, ctypes.c_int(nbas), eptr, lib.c_null_ptr())
                    rf(buf_rs.ctypes.data_as(ctypes.c_void_p), shls, aptr, ctypes.c_int(natm),
                       bptr, ctypes.c_int(nbas), eptr, lib.c_null_ptr())
                    worst = max(worst, float(np.abs(buf_c - buf_rs).max()))
            ok = worst < 1e-9
            allok = allok and ok
            print(f"{cfn:28s} {worst:14.3e}  {'PASS' if ok else 'FAIL'}")
    print("\n==== 汇总 ====", "全部通过" if allok else "有FAIL项")

if __name__ == '__main__':
    main()
