#[cfg(test)]
mod tests {
    use crate::kinds::*;
    use crate::utils::*;

    #[test]
    fn cmp_works() {
        use crate::PrimaryColor::*;

        mix_cmp(Red, Blue);
        mix_cmp(Yellow, Red);
        mix_cmp(Blue, Yellow);
    }
    const test_count : i32 = 1000000;
    
    #[bench]
    fn bench_mix(b: &mut test::Bencher) {
        b.iter(|| {
            for _i in (1..test_count) {
                if mix(PrimaryColor::Red, PrimaryColor::Blue)
                    == mix(PrimaryColor::Blue, PrimaryColor::Red)
                {
                    test::black_box(1000);
                }
            }
        });
    }
    
    #[bench]
    fn bench_match_cmp(b: &mut test::Bencher) {
        b.iter(|| {
            for _i in (1..test_count) {
                if mix_cmp(PrimaryColor::Red, PrimaryColor::Blue)
                    == mix_cmp(PrimaryColor::Blue, PrimaryColor::Red)
                {
                    test::black_box(1000);
                }
            }
        });
    }

    #[bench]
    fn bench_match_u8(b: &mut test::Bencher) {
        b.iter(|| {
            for _i in (1..test_count) {
                if mix_u8(PrimaryColor::Red, PrimaryColor::Blue)
                    == mix_u8(PrimaryColor::Blue, PrimaryColor::Red)
                {
                    test::black_box(1000);
                }
            }
        });
    }

    #[bench]
    fn bench_match_mix(b: &mut test::Bencher) {
        b.iter(|| {
            for _i in (1..test_count) {
                if mix_match(PrimaryColor::Red, PrimaryColor::Blue)
                    == mix_match(PrimaryColor::Blue, PrimaryColor::Red)
                {
                    test::black_box(1000);
                }
            }
        });
    }
}

