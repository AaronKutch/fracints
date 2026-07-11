#!/usr/bin/env python3
# Master constant generator for fracints cos_sin.
# All "accurate" constants are computed in Q320 fixed point with plain integers
# (pi via Machin's formula), so they are good to far beyond 128 fractional bits.
# "fast" minimax-ish coefficients are computed via Chebyshev interpolation in
# f64, which is plenty for their ~1e-8 target accuracy.

import math, random

PREC = 320
ONE = 1 << PREC

def atan_inv(x):
    # atan(1/x) * 2^PREC by alternating series
    total = 0
    term = ONE // x
    k = 0
    while term:
        t = term // (2 * k + 1)
        total += t if k % 2 == 0 else -t
        term //= x * x
        k += 1
    return total

PI = 16 * atan_inv(5) - 4 * atan_inv(239)
# sanity
assert abs(PI / ONE - math.pi) < 1e-15

QPI = PI >> 2                      # pi/4
QPI2 = (QPI * QPI) >> PREC         # (pi/4)^2

# Taylor coefficients:
# cos((pi/4) t) = sum (-1)^k a_k t^(2k),  a_k = (pi/4)^(2k) / (2k)!
# sin((pi/4) t) = t * sum (-1)^k b_k t^(2k), b_k = (pi/4)^(2k+1) / (2k+1)!
def gen_coeffs(first, fac_start):
    # generates until coefficient is below 2^-140
    res = [first]
    a = first
    k = 0
    while True:
        k += 1
        d = (fac_start + 2 * k - 1) * (fac_start + 2 * k)
        a = ((a * QPI2) >> PREC) // d
        if a < (1 << (PREC - 140)):
            break
        res.append(a)
    return res

COS_COEFFS = gen_coeffs(ONE, 0)
SIN_COEFFS = gen_coeffs(QPI, 1)

def isqrt_round(x):
    r = math.isqrt(x)
    if (r + 1) ** 2 - x < x - r * r:
        r += 1
    return r

SQRT_HALF = isqrt_round(1 << (2 * PREC - 1))  # sqrt(1/2) in Q320
INV_PI = ((1 << (2 * PREC)) + PI // 2) // PI  # 1/pi in Q320

def to_q(x, w):
    # round Q320 -> Q(w-1) (returns int, may be 2^(w-1) for x = 1.0 exactly;
    # caller clamps)
    shift = PREC - (w - 1)
    return (x + (1 << (shift - 1))) >> shift

def clamp(x, w):
    m = (1 << (w - 1)) - 1
    return max(-m, min(m, x))

# how many terms a width needs: keep coefficient if it is at least a quarter
# ULP, i.e. >= 2^-(w+1)
def nterms(coeffs, w):
    n = 0
    for a in coeffs:
        if a >= (1 << (PREC - (w + 1))):
            n += 1
    return n

print("terms needed (cos, sin):")
for w in [8, 16, 32, 64, 128]:
    print(f"  fi{w}: {nterms(COS_COEFFS, w)}, {nterms(SIN_COEFFS, w)}")

print()
print("master Q127 tables for gen_constants.rs:")
print("COS:", [clamp(to_q(a, 128), 128) for a in COS_COEFFS])
print("SIN:", [clamp(to_q(b, 128), 128) for b in SIN_COEFFS])
print("SQRT_HALF:", clamp(to_q(SQRT_HALF, 128), 128))
print("INV_PI:", clamp(to_q(INV_PI, 128), 128))

# ---------------- fast (Chebyshev economized) coefficients ----------------
# g(u) = cos((pi/4) sqrt(u)) and h(u) = sin((pi/4) sqrt(u))/sqrt(u) on [0,1],
# both analytic in u.
def cheb_coeffs(f, n_nodes, deg):
    cs = []
    for j in range(deg + 1):
        s = 0.0
        for k in range(n_nodes):
            th = math.pi * (k + 0.5) / n_nodes
            v = math.cos(th)
            s += f((v + 1.0) / 2.0) * math.cos(j * th)
        cs.append(2.0 * s / n_nodes)
    return cs

def poly_mul(p, q):
    r = [0.0] * (len(p) + len(q) - 1)
    for i, a in enumerate(p):
        for j, b in enumerate(q):
            r[i + j] += a * b
    return r

def poly_add(p, q):
    n = max(len(p), len(q))
    return [(p[i] if i < len(p) else 0.0) + (q[i] if i < len(q) else 0.0) for i in range(n)]

def poly_scale(p, s):
    return [a * s for a in p]

def cheb_to_power(cs):
    # T_j(2u - 1) in power basis of u
    t_prev = [1.0]
    t_cur = [-1.0, 2.0]
    res = poly_scale(t_prev, cs[0] / 2.0)
    for j in range(1, len(cs)):
        res = poly_add(res, poly_scale(t_cur, cs[j]))
        t_next = poly_add(poly_scale(poly_mul([-1.0, 2.0], t_cur), 2.0), poly_scale(t_prev, -1.0))
        t_prev, t_cur = t_cur, t_next
    return res

def g(u):
    return math.cos((math.pi / 4.0) * math.sqrt(u))

def h(u):
    if u == 0.0:
        return math.pi / 4.0
    su = math.sqrt(u)
    return math.sin((math.pi / 4.0) * su) / su

def report(name, f, deg):
    p = cheb_to_power(cheb_coeffs(f, 400, deg))
    err = 0.0
    for i in range(4001):
        u = i / 4000.0
        v = 0.0
        for c in reversed(p):
            v = v * u + c
        err = max(err, abs(v - f(u)))
    print(f"{name} deg {deg}: maxerr {err:.3e} coeffs {p}")
    return p

print()
for d in [2, 3, 4]:
    report("cos", g, d)
for d in [2, 3, 4]:
    report("sin", h, d)

# ---------------- emit fast tables as Q127 ----------------
def q127(x):
    return int(round(x * (1 << 127)))

FAST2_COS = report("cos", g, 2)
FAST3_COS = report("cos", g, 3)
FAST2_SIN = report("sin", h, 2)
FAST3_SIN = report("sin", h, 3)
print()
# store magnitudes, signs alternate +,-,+,-
for name, p in [("COS_FAST_DEG2", FAST2_COS), ("COS_FAST_DEG3", FAST3_COS),
                ("SIN_FAST_DEG2", FAST2_SIN), ("SIN_FAST_DEG3", FAST3_SIN)]:
    mags = []
    for i, c in enumerate(p):
        assert (c > 0) == (i % 2 == 0), (name, p)
        mags.append(q127(abs(c)))
    print(f"{name}: {mags}")

# ---------------- reference test vectors ----------------
# cos/sin of (pi * x) and of (x radians) in Q320, rounded to the target width
def cos_sin_fixed(theta):
    # theta in Q320, |theta| <= pi. terms are tracked by magnitude so that
    # floor division decays to exactly zero
    sign = 1 if theta >= 0 else -1
    atheta = abs(theta)
    th2 = (atheta * atheta) >> PREC
    # cos
    total = ONE
    term = ONE
    k = 0
    while term:
        k += 1
        term = ((term * th2) >> PREC) // ((2 * k - 1) * (2 * k))
        total += term if k % 2 == 0 else -term
    c = total
    # sin
    total = atheta
    term = atheta
    k = 0
    while term:
        k += 1
        term = ((term * th2) >> PREC) // ((2 * k) * (2 * k + 1))
        total += term if k % 2 == 0 else -term
    return c, sign * total

def vec_entries(w, xs, radians):
    out = []
    for x in xs:
        if radians:
            theta = x << (PREC - (w - 1))
        else:
            theta = (PI * x) >> (w - 1)
        c, s = cos_sin_fixed(theta)
        out.append((x, clamp(to_q(c, w), w), clamp(to_q(s, w), w)))
    return out

random.seed(0x1729)
def gen_xs(w):
    m = 1 << (w - 1)
    xs = set()
    # eighth-turn multiples including MIN, and ULP-adjacent values around
    # quadrant boundaries
    for k in range(-8, 8):
        b = k * (m >> 3)
        for d in [-1, 0, 1]:
            v = b + d
            if -m <= v < m:
                xs.add(v)
    for _ in range(40):
        xs.add(random.randrange(-m, m))
    return sorted(xs)

def gen_rad_xs(w):
    m = 1 << (w - 1)
    xs = {0, 1, -1, m - 1, -m, -m + 1, m >> 1, -(m >> 1)}
    for _ in range(24):
        xs.add(random.randrange(-m, m))
    return sorted(xs)

lines = []
lines.append("//! Reference test vectors for `cos_sin` implementations, generated by")
lines.append("//! `gen_trig_master.py` (in the `testcrate` directory) with 320-bit fixed")
lines.append("//! point arithmetic, so entries are correctly rounded at their width.")
lines.append("")
lines.append("/// entries are `(x, cos(pi * x), sin(pi * x))` as raw fracint integers")
for w in [32, 64, 128]:
    v = vec_entries(w, gen_xs(w), False)
    lines.append(f"pub const FI{w}_COS_SIN_PI: [(i{w}, i{w}, i{w}); {len(v)}] = [")
    for e in v:
        lines.append(f"    ({e[0]}, {e[1]}, {e[2]}),")
    lines.append("];")
lines.append("")
lines.append("/// entries are `(x, cos(x), sin(x))` as raw fracint integers, `x` in radians")
for w in [64, 128]:
    v = vec_entries(w, gen_rad_xs(w), True)
    lines.append(f"pub const FI{w}_COS_SIN_RAD: [(i{w}, i{w}, i{w}); {len(v)}] = [")
    for e in v:
        lines.append(f"    ({e[0]}, {e[1]}, {e[2]}),")
    lines.append("];")
lines.append("")

with open("trig_vectors.rs", "w") as f:
    f.write("\n".join(lines))
print("\nwrote trig_vectors.rs")
