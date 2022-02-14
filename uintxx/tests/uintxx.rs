use eint::{Eint, E128, E256, E32};

#[test]
fn test_wrapping_div() {
    let case_list = [[
        E256(
            E128(0xd3e04adfb2db76e8ce58bba4207434a4),
            E128(0x15de88272aefffffffffffffffffffff),
        ),
        E256(
            E128(0x686f332000000000000000000dd2966b),
            E128(0x00000bea6a6af75538be984c83ce8648),
        ),
        E256(
            E128(0x0000000000000000000000000001d5d8),
            E128(0x00000000000000000000000000000000),
        ),
    ]];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let e = case[2];
        let r = lhs.wrapping_div(rhs);
        assert_eq!(r, e);
    }
}

#[test]
fn test_wrapping_div_s() {
    let case_list = [
        [
            E256(
                E128(0x00000000000000000000000000000001),
                E128(0x00000000000000000000000000000000),
            ),
            E256(
                E128(0x00000000000000000000000000000000),
                E128(0x00000000000000000000000000000000),
            ),
            E256(
                E128(0xffffffffffffffffffffffffffffffff),
                E128(0xffffffffffffffffffffffffffffffff),
            ),
        ],
        [
            E256(
                E128(0x00000000000000000000000000000000),
                E128(0x80000000000000000000000000000000),
            ),
            E256(
                E128(0xffffffffffffffffffffffffffffffff),
                E128(0xffffffffffffffffffffffffffffffff),
            ),
            E256(
                E128(0x00000000000000000000000000000000),
                E128(0x80000000000000000000000000000000),
            ),
        ],
        [
            E256(
                E128(0x2c1fb5204d24891731a7445bdf8bcb5c),
                E128(0xea2177d8d51000000000000000000000),
            ),
            E256(
                E128(0x686f332000000000000000000dd2966b),
                E128(0x00000bea6a6af75538be984c83ce8648),
            ),
            E256(
                E128(0xfffffffffffffffffffffffffffe2a28),
                E128(0xffffffffffffffffffffffffffffffff),
            ),
        ],
    ];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let e = case[2];
        let r = lhs.wrapping_div_s(rhs);
        assert_eq!(r, e);
    }
}

#[test]
fn test_wrapping_rem() {
    let case_list = [[
        E256(
            E128(0x00000000000000000000000000000007),
            E128(0x00000000000000000000000000000000),
        ),
        E256(
            E128(0x00000000000000000000000000000002),
            E128(0x00000000000000000000000000000000),
        ),
        E256(
            E128(0x00000000000000000000000000000001),
            E128(0x00000000000000000000000000000000),
        ),
    ]];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let e = case[2];
        let r = lhs.wrapping_rem(rhs);
        assert_eq!(r, e);
    }
}

#[test]
fn test_wrapping_rem_s() {
    let case_list = [
        [
            E256(
                E128(0x00000000000000000000000000000001),
                E128(0x00000000000000000000000000000000),
            ),
            E256(
                E128(0x00000000000000000000000000000000),
                E128(0x00000000000000000000000000000000),
            ),
            E256(
                E128(0x00000000000000000000000000000001),
                E128(0x00000000000000000000000000000000),
            ),
        ],
        [
            E256(
                E128(0x00000000000000000000000000000000),
                E128(0x80000000000000000000000000000000),
            ),
            E256(
                E128(0xffffffffffffffffffffffffffffffff),
                E128(0xffffffffffffffffffffffffffffffff),
            ),
            E256(
                E128(0x00000000000000000000000000000000),
                E128(0x00000000000000000000000000000000),
            ),
        ],
        [
            E256(
                E128(0xfffffffffffffffffffffffffffffff9),
                E128(0xffffffffffffffffffffffffffffffff),
            ),
            E256(
                E128(0x00000000000000000000000000000003),
                E128(0x00000000000000000000000000000000),
            ),
            E256(
                E128(0xffffffffffffffffffffffffffffffff),
                E128(0xffffffffffffffffffffffffffffffff),
            ),
        ],
    ];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let e = case[2];
        let r = lhs.wrapping_rem_s(rhs);
        assert_eq!(r, e);
    }
}

#[test]
fn test_average_add() {
    let case_list = [
        [
            E256(
                E128(0xffffffffffffffffffffffffffffffff),
                E128(0xffffffffffffffffffffffffffffffff),
            ),
            E256(
                E128(0xffffffffffffffffffffffffffffffff),
                E128(0xffffffffffffffffffffffffffffffff),
            ),
            E256(
                E128(0xffffffffffffffffffffffffffffffff),
                E128(0xffffffffffffffffffffffffffffffff),
            ),
        ],
        [
            E256(
                E128(0x00000000000000000000000000000004),
                E128(0x00000000000000000000000000000000),
            ),
            E256(
                E128(0x00000000000000000000000000000006),
                E128(0x00000000000000000000000000000000),
            ),
            E256(
                E128(0x00000000000000000000000000000005),
                E128(0x00000000000000000000000000000000),
            ),
        ],
    ];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let e = case[2];
        let r = lhs.average_add(rhs);
        assert_eq!(r, e);
    }
}

#[test]
fn test_widening_mul_s() {
    let case_list = [
        [E32(0xffffffff), E32(0xffffffff), E32(0x00000001), E32(0x00000000)],
        [E32(0x00000002), E32(0xffffffff), E32(0xfffffffe), E32(0xffffffff)],
        [E32(0x00000002), E32(0x00000002), E32(0x00000004), E32(0x00000000)],
    ];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let elo = case[2];
        let ehi = case[3];
        let (rlo, rhi) = lhs.widening_mul_s(rhs);
        assert_eq!(rlo, elo);
        assert_eq!(rhi, ehi);
    }
}

#[test]
fn test_average_add_s() {
    let case_list = [
        [E32(0xfffffff6), E32(0x00000008), E32(0xffffffff)],
        [E32(0xffffffff), E32(0xffffffff), E32(0xffffffff)],
    ];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let e = case[2];
        let r = lhs.average_add_s(rhs);
        assert_eq!(e, r);
    }
}

#[test]
fn test_average_sub() {
    fn asub(a: u32, b: u32) -> [E32; 3] {
        let c = (a as u64).wrapping_sub(b as u64).wrapping_shr(1) as u32;
        [E32(a), E32(b), E32(c)]
    }
    let case_list = [
        asub(0x00000008, 0x00000000),
        asub(0x00000000, 0x00000008),
        asub(0x80000000, 0x80000000),
        asub(0x80000000, 0x7fffffff),
        asub(0x00000000, 0xffffffff),
        asub(0xffffffff, 0x80000000),
    ];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let e = case[2];
        let r = lhs.average_sub(rhs);
        assert_eq!(e, r);
    }
}

#[test]
fn test_average_sub_s() {
    fn asub(a: u32, b: u32) -> [E32; 3] {
        let c = (a as i32 as i64).wrapping_sub(b as i32 as i64).wrapping_shr(1) as u64 as u32;
        [E32(a), E32(b), E32(c)]
    }
    let case_list = [
        asub(0x00000008, 0x00000000),
        asub(0x00000000, 0x00000008),
        asub(0x80000000, 0x80000000),
        asub(0x80000000, 0x7fffffff),
        asub(0x00000000, 0xffffffff),
        asub(0xffffffff, 0x80000000),
        asub(0x7fffffff, 0x80000000),
    ];
    for case in &case_list {
        let lhs = case[0];
        let rhs = case[1];
        let e = case[2];
        let r = lhs.average_sub_s(rhs);
        assert_eq!(e, r);
    }
}

#[test]
fn test_bug_fix_0() {
    let a = E256(
        E128(0x00000000000022330000000000001122),
        E128(0x00000000000044550000000000003344),
    );
    let b = E256(
        E128(0x00000000000023450000000000001234),
        E128(0x00000000000056780000000000004567),
    );
    let c = E256(
        E128(0x00000000000000bb00000000000000aa),
        E128(0x00000000000000cc00000000000000dd),
    );
    let e = E256(
        E128(0x000000000047f182000000000017771c),
        E128(0x000000000117122b0000000000a16174),
    );
    let r = (a + b) * c;
    assert_eq!(r, e);
}
