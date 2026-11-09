extern "C" {
    fn ceil(__x: f64) -> f64;
    fn calloc(__nmemb: usize, __size: usize) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
}
#[repr(C)]
pub struct GFC {
    pub M: u64,
    pub r: u64,
    pub a: u64,
    pub b: u64,
    pub speck_exp: *mut u32,
}
pub const SPECK_ROUNDS: i32 = 27;
pub const SPECK_KEY_LEN: i32 = 4;
pub(crate) fn speck_expand(K: &[u32], S: &mut [u32]) {
    let mut i: u32;
    let mut b: u32 = (&K[0..])[0];
    let mut a: [u32; 3] = [0; 3];
    i = 0;
    while i < (SPECK_KEY_LEN - 1) as u32 {
        a[i as usize] = (&K[i.wrapping_add(1) as usize..])[0];
        i = i.wrapping_add(1);
    }
    (&mut S[0..])[0] = b;
    i = 0;
    while i < (SPECK_ROUNDS - 1) as u32 {
        a[i.wrapping_rem((4 - 1) as u32) as usize] = a[i.wrapping_rem((4 - 1) as u32) as usize]
            >> 8
            | a[i.wrapping_rem((4 - 1) as u32) as usize]
                << ::core::mem::size_of::<u32>()
                    .wrapping_mul(8)
                    .wrapping_sub(8);
        a[i.wrapping_rem((4 - 1) as u32) as usize] =
            a[i.wrapping_rem((4 - 1) as u32) as usize].wrapping_add(b);
        a[i.wrapping_rem((4 - 1) as u32) as usize] = a[i.wrapping_rem((4 - 1) as u32) as usize] ^ i;
        b = b << 3
            | b >> ::core::mem::size_of::<u32>()
                .wrapping_mul(8)
                .wrapping_sub(3);
        b ^= a[i.wrapping_rem((4 - 1) as u32) as usize];
        (&mut S[i.wrapping_add(1) as usize..])[0] = b;
        i = i.wrapping_add(1);
    }
}
pub(crate) fn speck_encrypt(pt: &[u32], ct: &mut [u32], K: &[u32]) {
    let mut i: u32;
    (&mut ct[0..])[0] = (&pt[0..])[0];
    (&mut ct[1..])[0] = (&pt[1..])[0];
    i = 0;
    while i < SPECK_ROUNDS as u32 {
        (&mut ct[1..])[0] = (&ct[1..])[0] >> 8
            | (&ct[1..])[0]
                << ::core::mem::size_of::<u32>()
                    .wrapping_mul(8)
                    .wrapping_sub(8);
        (&mut ct[1..])[0] = (&ct[1..])[0].wrapping_add((&ct[0..])[0]);
        (&mut ct[1..])[0] = (&ct[1..])[0] ^ (&K[i as usize..])[0];
        (&mut ct[0..])[0] = (&ct[0..])[0] << 3
            | (&ct[0..])[0]
                >> ::core::mem::size_of::<u32>()
                    .wrapping_mul(8)
                    .wrapping_sub(3);
        (&mut ct[0..])[0] = (&ct[0..])[0] ^ (&ct[1..])[0];
        i = i.wrapping_add(1);
    }
}
pub(crate) unsafe fn F(gfc: &GFC, j: u64, R: u64) -> u64 {
    let mut enc: u64 = 0;
    let key: &[u32] = std::slice::from_raw_parts(
        gfc.speck_exp
            .offset(j.wrapping_sub(1).wrapping_mul(SPECK_ROUNDS as u64) as isize),
        1_000_000,
    );
    speck_encrypt(
        std::slice::from_raw_parts(&raw const R as *const _, 1_000_000),
        std::slice::from_raw_parts_mut(&raw mut enc as *mut _, 1_000_000),
        key,
    );
    enc
}
pub(crate) unsafe fn fe(gfc: &GFC, m: u64) -> u64 {
    let mut L: u64;
    let mut R: u64;
    let mut tmp: u64;
    L = m.wrapping_rem(gfc.a);
    R = m.wrapping_div(gfc.a);
    let mut j: u64 = 1;
    while j <= gfc.r {
        if j & 1 != 0 {
            tmp = L.wrapping_add(F(gfc, j, R)).wrapping_rem(gfc.a);
        } else {
            tmp = L.wrapping_add(F(gfc, j, R)).wrapping_rem(gfc.b);
        }
        L = R;
        R = tmp;
        j = j.wrapping_add(1);
    }
    if gfc.r & 1 != 0 {
        gfc.a.wrapping_mul(L).wrapping_add(R)
    } else {
        gfc.a.wrapping_mul(R).wrapping_add(L)
    }
}
pub(crate) unsafe fn fe_inv(gfc: &GFC, m: u64) -> u64 {
    let mut L: u64;
    let mut R: u64;
    let mut tmp: u64;
    if gfc.r & 1 != 0 {
        R = m.wrapping_rem(gfc.a);
        L = m.wrapping_div(gfc.a);
    } else {
        L = m.wrapping_rem(gfc.a);
        R = m.wrapping_div(gfc.a);
    }
    let mut j: u64 = gfc.r;
    while j >= 1 {
        if j & 1 != 0 {
            tmp = R
                .wrapping_add(gfc.a)
                .wrapping_sub(F(gfc, j, L).wrapping_rem(gfc.a))
                .wrapping_rem(gfc.a);
        } else {
            tmp = R
                .wrapping_add(gfc.b)
                .wrapping_sub(F(gfc, j, L).wrapping_rem(gfc.b))
                .wrapping_rem(gfc.b);
        }
        R = L;
        L = tmp;
        j = j.wrapping_sub(1);
    }
    gfc.a.wrapping_mul(R).wrapping_add(L)
}
#[no_mangle]
pub unsafe extern "C" fn gfc_init(range: u64, rounds: u64, seed: u64) -> *const GFC {
    let mut gfc: Box<GFC> = Box::new(GFC {
        M: <u64 as Default>::default(),
        r: <u64 as Default>::default(),
        a: <u64 as Default>::default(),
        b: <u64 as Default>::default(),
        speck_exp: std::ptr::null_mut::<u32>(),
    });
    if false {
        return std::ptr::null();
    }
    gfc.M = range;
    gfc.r = rounds;
    let tmp: f64 = ceil((range as f64).sqrt()) as f64;
    gfc.a = tmp as u64;
    gfc.b = tmp as u64;
    gfc.speck_exp = calloc(
        (gfc.r as usize).wrapping_mul(SPECK_ROUNDS as usize),
        ::core::mem::size_of::<u32>(),
    ) as *mut u32;
    let mut i: u64 = 0;
    while i < gfc.r {
        let key: [u64; 2] = [seed, i];
        speck_expand(
            bytemuck::cast_slice::<_, u32>(&key),
            std::slice::from_raw_parts_mut(
                gfc.speck_exp
                    .offset(i.wrapping_mul(SPECK_ROUNDS as u64) as isize),
                1_000_000,
            ),
        );
        i = i.wrapping_add(1);
    }
    Box::into_raw(gfc) as *const GFC
}
#[no_mangle]
pub unsafe extern "C" fn gfc_destroy(gfc: *mut GFC) {
    free((*gfc).speck_exp as *mut core::ffi::c_void);
    {
        let __crat_raw_free = gfc as *mut core::ffi::c_void;
        if !__crat_raw_free.is_null() {
            drop(Box::from_raw(__crat_raw_free as *mut GFC));
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gfc_decrypt(gfc: &GFC, m: u64) -> u64 {
    let mut c: u64 = fe_inv(gfc, m);
    while c >= gfc.M {
        c = fe_inv(gfc, c);
    }
    c
}
#[no_mangle]
pub unsafe extern "C" fn gfc_encrypt(gfc: &GFC, m: u64) -> u64 {
    let mut c: u64 = fe(gfc, m);
    while c >= gfc.M {
        c = fe(gfc, c);
    }
    c
}
