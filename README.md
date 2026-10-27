# rsint — libcint 的 C→Rust 转译项目

将 [libcint](https://github.com/sunqm/libcint)（C 语言电子 Gaussian 积分库，v6.1.3）通过
[c2rust](https://github.com/immunant/c2rust) 转译为 Rust，并对转译结果进行数值验证。

## 背景与目标

libcint 是一个开源的解析 Gaussian 积分库，提供 C/Fortran API，用于计算一电子/二电子
积分（Cartesian/实球谐/spinor GTO）。本项目直接用 c2rust 将其机械转译为 Rust，
形成与原始 C 库可对照、数值等价的 Rust 版本，并在此基础上做验证与后续人工优化。

## 当前状态

- **转译**：41 个 C 源文件 → **27 万行 Rust**（`work/out/src/`）
- **构建**：Rust 静态/动态库可编译通过，生成 `cdylib`（`librsint.so`）
- **数值验证**：核心实数积分 C vs Rust **绝对误差为 0**（见下）

### 已验证正确的核心积分（C 库 vs Rust 转译库，逐位一致）

| 函数 | 含义 | 结果 |
|------|------|------|
| `cint1e_ovlp_cart` / `cint1e_ovlp_sph` | 重叠积分 | ✅ 0.000 |
| `cint1e_kin_cart` / `cint1e_kin_sph` | 动能积分 | ✅ 0.000 |
| `cint1e_nuc_cart` / `cint1e_nuc_sph` | 核吸引积分 | ✅ 0.000 |
| `cint2e_cart` / `cint2e_sph` | 双电子库仑 | ✅ 0.000 |
| `cint2c2e_cart` / `cint2c2e_sph` | 2 中心双电子 | ✅ 0.000 |
| `cint3c2e_cart` / `cint3c2e_sph` | 3 中心双电子 | ✅ 0.000 |

### 已知限制

- **复数/spinor 功能缺失**：c2rust 0.22.1 不支持 C 的 `double complex`/`_Complex`
  类型，导致 **479 个复数相关函数被跳过**（spinor 积分、复系数 GTO、部分
  deriv/hess/grad）。这是 c2rust 的能力上限，需要人工移植或接受限制。
- **`misc.c`** 因使用 `_Complex_I`（虚数单位字面量）无法自动转译，整体未包含。
  实数核心路径不依赖它，唯一被引用的 `CINTsquare_dist` 已手写补充
  （见 `work/out/helpers.rs`）。

## 目录结构

```
rsint/
├── libcint/                 # C 原始仓库（参照/测试基准，含 build/libcint.so）
├── src_work/                # 转译输入的 C 源码副本（quadmath/long double 已替换为 double）
├── work/
│   ├── compile_commands.json # c2rust 转译用的编译数据库
│   ├── compare2.py           # C vs Rust 数值对比验证脚本
│   ├── compare_rust_vs_c.py  # （早期验证脚本）
│   └── out/                  # ★ 转译产物 (cargo 项目)
│       ├── lib.rs            # 模块组装入口
│       ├── helpers.rs        # 人工补充的缺失符号 (CINTsquare_dist)
│       ├── Cargo.toml
│       └── src/              # 41 个 .rs 转译模块 + autocode/
├── rust_out/                 # 早期转译试验（无构建文件）
├── rust_out_e/               # 早期带构建文件的转译（含 f128 依赖）
├── rust_test/                # 早期单文件转译试验
├── build_cc_work.json        # 早期编译数据库备份
├── README.md
└── AGENTS.md
```

## 环境要求

- **Rust nightly-2025-02-01**（c2rust 转译产物依赖旧 nightly 的 feature 如
  `raw_ref_op`、`extern_types`、`label_break_value`）
- **c2rust 0.22.1**（`cargo +nightly-2025-02-01 install c2rust --locked`）
- **cmake / gcc / clang**（用于生成 compile_commands.json 与构建 C 参照库）
- **Python 3 + numpy + pyscf + mpmath**（用于数值对比验证）

## 快速开始

### 0. 克隆后准备

`libcint/` 是 git submodule（上游 <https://github.com/sunqm/libcint>），
`src_work/*.dat` 与 libcint 中同名文件逐字节相同、未入库。首次克隆后执行：

```bash
git submodule update --init --recursive
bash scripts/sync_dat.sh
```

### 1. 构建 C 参照库（用于数值对比基准）

```bash
cd libcint
cmake -S . -B build
cmake --build build -j$(nproc)
# 生成 build/libcint.so.6.1.3
```

### 2. 转译（若需要重新生成 Rust 产物）

```bash
# 准备转译输入源（quadmath / long double → double）
# 见下方"转译细节"
c2rust transpile work/compile_commands.json -e -o work/out
```

### 3. 编译 Rust 转译产物

```bash
cd work/out
cargo build --release
# 生成 target/release/librsint.so
```

### 4. 数值验证

```bash
cd work
python3 compare2.py
```

预期输出：所有列出的积分函数 `PASS`，绝对误差 `0.000e+00`。

## 转译关键细节

c2rust 直接转译会遇到两类阻碍，已通过"转译输入源预处理"解决：

1. **quadmath / f128 依赖**：c2rust 把 `__float128`、`long double` 映射为 `f128` crate，
   而 f128 0.2.9 的 `new` 非 const，产生约 2000 个 `E0015` 编译错误。
   **处理**：在 `src_work/` 中把 `__float128`/`_Float128`/`long double` 全部替换为
   `double`，并去掉 `q`/`l` 后缀字面量、将 `sqrtq`→`sqrt`、`erfq`→`erf` 等。
   代价：quad 精度降为 double（对验证正确性足够）。

2. **复数类型不支持**：c2rust 无法处理 `double complex` 与 `_Complex_I`。
   - `misc.c`（用 `_Complex_I`）整体未转译。
   - 复数函数被逐段跳过（`Failed to translate ... Unsupported type Complex`）。

3. **缺失符号补齐**：移除 f128 依赖后，仅 `CINTsquare_dist` 未定义，已在
   `helpers.rs` 手写补充，使 `cdylib` 链接完整。

## 验证方法

`compare2.py` 使用 pyscf 生成分子与 `atm/bas/env` 布局，分别调用 C 库
（`libcint/build/libcint.so`）与 Rust 库（`work/out/target/release/librsint.so`）的同一
积分函数（shell-pair API），比较输出 buffer 的最大绝对误差。6 类核心实数积分全部
达到 0 误差，证明转译在实值路径上数值等价。

## 后续工作建议

- [ ] 系统性跑通 `testsuite/` 中的实数类测试（`test_int1e.py`、`test_int2e.py` 无 spinor 部分）
- [ ] 将对比脚本固化为回归测试（pytest）
- [ ] 人工移植 479 个复数/spinor 函数（工作量大，c2rust 无法自动完成）
- [ ] 结合 `src/autocode/*.rs` 与原始 Lisp 生成脚本，为转译产物做人工语义重构
- [ ] 清理早期试验目录（`rust_out*`、`rust_test`、`build_cc_work.json`）

## 参考

- libcint 原始仓库：<https://github.com/sunqm/libcint>
- c2rust：<https://github.com/immunant/c2rust>
- 原始 README：见 `libcint/README.rst`
