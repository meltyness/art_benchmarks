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
    const TEST_COUNT : i32 = 1000000;
    
    #[bench]
    fn bench_match_arr_dumb(b: &mut test::Bencher) {
        b.iter(|| {
            for _i in 1..TEST_COUNT {
                if mix_dumb_arr(&[test::black_box(PrimaryColor::Red), test::black_box(PrimaryColor::Blue)])
                    == mix_dumb_arr(&[test::black_box(PrimaryColor::Blue), test::black_box(PrimaryColor::Red)])
                {
                }
            }
        });
    }

    #[bench]
    fn bench_match_dumb(b: &mut test::Bencher) {
        b.iter(|| {
            for _i in 1..TEST_COUNT {
                if mix_dumb(test::black_box(PrimaryColor::Red), test::black_box(PrimaryColor::Blue))
                    == mix_dumb(test::black_box(PrimaryColor::Blue), test::black_box(PrimaryColor::Red))
                {
                }
            }
        });
    }

    #[bench]
    fn bench_mix(b: &mut test::Bencher) {
        b.iter(|| {
            for _i in 1..TEST_COUNT {
                if mix(test::black_box(PrimaryColor::Red), test::black_box(PrimaryColor::Blue))
                    == mix(test::black_box(PrimaryColor::Blue), test::black_box(PrimaryColor::Red))
                {
                }
            }
        });
    }
    
    #[bench]
    fn bench_match_cmp(b: &mut test::Bencher) {
        b.iter(|| {
            for _i in 1..TEST_COUNT {
                if mix_cmp(test::black_box(PrimaryColor::Red), test::black_box(PrimaryColor::Blue))
                    == mix_cmp(test::black_box(PrimaryColor::Blue),test::black_box(PrimaryColor::Red))
                {
                }
            }
        });
    }

    #[bench]
    fn bench_match_u8(b: &mut test::Bencher) {
        b.iter(|| {
            for _i in 1..TEST_COUNT {
                if mix_u8(test::black_box(PrimaryColor::Red), test::black_box(PrimaryColor::Blue))
                    == mix_u8(test::black_box(PrimaryColor::Blue), test::black_box(PrimaryColor::Red))
                {
                }
            }
        });
    }

    #[bench]
    fn bench_match_mix(b: &mut test::Bencher) {
        b.iter(|| {
            for _i in 1..TEST_COUNT {
                if mix_match(test::black_box(PrimaryColor::Red), test::black_box(PrimaryColor::Blue))
                    == mix_match(test::black_box(PrimaryColor::Blue), test::black_box(PrimaryColor::Red))
                {
                }
            }
        });
    }
}

