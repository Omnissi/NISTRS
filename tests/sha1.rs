#[cfg(test)]
mod test {

    use std::fs::File;
    use std::io::{Error, Read};
    use std::path::Path;

    use nistrs::BitsData;

    fn equal_results(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    fn load_sequnce() -> std::io::Result<BitsData> {
        let path = Path::new("./tests/files/data.sha1");
        if !path.exists() {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Test sequece not found",
            ));
        }

        let mut data: Vec<u8> = Vec::<u8>::new();
        File::open(path)?.read_to_end(&mut data)?;

        Ok(BitsData::from_binary(data))
    }
    mod sha1 {
        use crate::test::{equal_results, load_sequnce};

        #[test]
        fn test_freq() {
            use nistrs::freq::frequency_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(frequency_test(&res).1, 0.604458));
        }

        #[test]
        fn test_block_freq() {
            use nistrs::block_freq::block_frequency_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(
                block_frequency_test(&res, 128).unwrap().1,
                0.091517
            ));
        }

        #[test]
        fn test_runs() {
            use nistrs::runs::runs_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(runs_test(&res).1, 0.309757));
        }

        #[test]
        fn test_longest_ones_runs() {
            use nistrs::longest_run_of_ones::longest_run_of_ones_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(
                longest_run_of_ones_test(&res).unwrap().1,
                0.657812
            ));
        }

        #[test]
        fn test_rank() {
            use nistrs::rank::rank_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(rank_test(&res).unwrap().1, 0.577829));
        }

        #[test]
        fn test_fft() {
            use nistrs::fft::fft_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(fft_test(&res).1, 0.163062));
        }

        #[test]
        fn test_non_overlapping() {
            use nistrs::non_overlapping_template::non_overlapping_template_test;

            let res = load_sequnce().unwrap();

            let ret = non_overlapping_template_test(&res, 9).unwrap();

            assert!(equal_results(ret[0].1, 0.496601));
            assert!(equal_results(ret[1].1, 0.421114));
            assert!(equal_results(ret[2].1, 0.762228));
            assert!(equal_results(ret[3].1, 0.313147));
            assert!(equal_results(ret[4].1, 0.267553));
            assert!(equal_results(ret[5].1, 0.569810));
            assert!(equal_results(ret[6].1, 0.918072));
            assert!(equal_results(ret[7].1, 0.314578));
            assert!(equal_results(ret[8].1, 0.987461));
            assert!(equal_results(ret[9].1, 0.129969));
            assert!(equal_results(ret[10].1, 0.001239));
            assert!(equal_results(ret[11].1, 0.982678));
            assert!(equal_results(ret[12].1, 0.296665));
            assert!(equal_results(ret[13].1, 0.502599));
            assert!(equal_results(ret[14].1, 0.897621));
            assert!(equal_results(ret[15].1, 0.688362));
            assert!(equal_results(ret[16].1, 0.943372));
            assert!(equal_results(ret[17].1, 0.178395));
            assert!(equal_results(ret[18].1, 0.129886));
            assert!(equal_results(ret[19].1, 0.679505));
            assert!(equal_results(ret[20].1, 0.181677));
            assert!(equal_results(ret[21].1, 0.151164));
            assert!(equal_results(ret[22].1, 0.045834));
            assert!(equal_results(ret[23].1, 0.495715));
            assert!(equal_results(ret[24].1, 0.534630));
            assert!(equal_results(ret[25].1, 0.173990));
            assert!(equal_results(ret[26].1, 0.808597));
            assert!(equal_results(ret[27].1, 0.431466));
            assert!(equal_results(ret[28].1, 0.021651));
            assert!(equal_results(ret[29].1, 0.309800));
            assert!(equal_results(ret[30].1, 0.869885));
            assert!(equal_results(ret[31].1, 0.839431));
            assert!(equal_results(ret[32].1, 0.601457));
            assert!(equal_results(ret[33].1, 0.484811));
            assert!(equal_results(ret[34].1, 0.837127));
            assert!(equal_results(ret[35].1, 0.636475));
            assert!(equal_results(ret[36].1, 0.178341));
            assert!(equal_results(ret[37].1, 0.076941));
            assert!(equal_results(ret[38].1, 0.123272));
            assert!(equal_results(ret[39].1, 0.224647));
            assert!(equal_results(ret[40].1, 0.793566));
            assert!(equal_results(ret[41].1, 0.006495));
            assert!(equal_results(ret[42].1, 0.081035));
            assert!(equal_results(ret[43].1, 0.496490));
            assert!(equal_results(ret[44].1, 0.166833));
            assert!(equal_results(ret[45].1, 0.049395));
            assert!(equal_results(ret[46].1, 0.760311));
            assert!(equal_results(ret[47].1, 0.807429));
            assert!(equal_results(ret[48].1, 0.368466));
            assert!(equal_results(ret[49].1, 0.219526));
            assert!(equal_results(ret[50].1, 0.392200));
            assert!(equal_results(ret[51].1, 0.348746));
            assert!(equal_results(ret[52].1, 0.911163));
            assert!(equal_results(ret[53].1, 0.174523));
            assert!(equal_results(ret[54].1, 0.868871));
            assert!(equal_results(ret[55].1, 0.792262));
            assert!(equal_results(ret[56].1, 0.338091));
            assert!(equal_results(ret[57].1, 0.448151));
            assert!(equal_results(ret[58].1, 0.883285));
            assert!(equal_results(ret[59].1, 0.969115));
            assert!(equal_results(ret[60].1, 0.311638));
            assert!(equal_results(ret[61].1, 0.504716));
            assert!(equal_results(ret[62].1, 0.776657));
            assert!(equal_results(ret[63].1, 0.326533));
            assert!(equal_results(ret[64].1, 0.041505));
            assert!(equal_results(ret[65].1, 0.463134));
            assert!(equal_results(ret[66].1, 0.262498));
            assert!(equal_results(ret[67].1, 0.108090));
            assert!(equal_results(ret[68].1, 0.756011));
            assert!(equal_results(ret[69].1, 0.673945));
            assert!(equal_results(ret[70].1, 0.948873));
            assert!(equal_results(ret[71].1, 0.830551));
            assert!(equal_results(ret[72].1, 0.958124));
            assert!(equal_results(ret[73].1, 0.541378));
            assert!(equal_results(ret[74].1, 0.496601));
            assert!(equal_results(ret[75].1, 0.867577));
            assert!(equal_results(ret[76].1, 0.896375));
            assert!(equal_results(ret[77].1, 0.773545));
            assert!(equal_results(ret[78].1, 0.679505));
            assert!(equal_results(ret[79].1, 0.610306));
            assert!(equal_results(ret[80].1, 0.097604));
            assert!(equal_results(ret[81].1, 0.964100));
            assert!(equal_results(ret[82].1, 0.941699));
            assert!(equal_results(ret[83].1, 0.540118));
            assert!(equal_results(ret[84].1, 0.651542));
            assert!(equal_results(ret[85].1, 0.018327));
            assert!(equal_results(ret[86].1, 0.918295));
            assert!(equal_results(ret[87].1, 0.932746));
            assert!(equal_results(ret[88].1, 0.574240));
            assert!(equal_results(ret[89].1, 0.013149));
            assert!(equal_results(ret[90].1, 0.726873));
            assert!(equal_results(ret[91].1, 0.510420));
            assert!(equal_results(ret[92].1, 0.654152));
            assert!(equal_results(ret[93].1, 0.012088));
            assert!(equal_results(ret[94].1, 0.284879));
            assert!(equal_results(ret[95].1, 0.297717));
            assert!(equal_results(ret[96].1, 0.402032));
            assert!(equal_results(ret[97].1, 0.731965));
            assert!(equal_results(ret[98].1, 0.176450));
            assert!(equal_results(ret[99].1, 0.049447));
            assert!(equal_results(ret[100].1, 0.965418));
            assert!(equal_results(ret[101].1, 0.850475));
            assert!(equal_results(ret[102].1, 0.008562));
            assert!(equal_results(ret[103].1, 0.062257));
            assert!(equal_results(ret[104].1, 0.076057));
            assert!(equal_results(ret[105].1, 0.464314));
            assert!(equal_results(ret[106].1, 0.095872));
            assert!(equal_results(ret[107].1, 0.896375));
            assert!(equal_results(ret[108].1, 0.684585));
            assert!(equal_results(ret[109].1, 0.616808));
            assert!(equal_results(ret[110].1, 0.050685));
            assert!(equal_results(ret[111].1, 0.821608));
            assert!(equal_results(ret[112].1, 0.462706));
            assert!(equal_results(ret[113].1, 0.157657));
            assert!(equal_results(ret[114].1, 0.046889));
            assert!(equal_results(ret[115].1, 0.825328));
            assert!(equal_results(ret[116].1, 0.143763));
            assert!(equal_results(ret[117].1, 0.738651));
            assert!(equal_results(ret[118].1, 0.755218));
            assert!(equal_results(ret[119].1, 0.343883));
            assert!(equal_results(ret[120].1, 0.713373));
            assert!(equal_results(ret[121].1, 0.169520));
            assert!(equal_results(ret[122].1, 0.954230));
            assert!(equal_results(ret[123].1, 0.692841));
            assert!(equal_results(ret[124].1, 0.424685));
            assert!(equal_results(ret[125].1, 0.664824));
            assert!(equal_results(ret[126].1, 0.119901));
            assert!(equal_results(ret[127].1, 0.457472));
            assert!(equal_results(ret[128].1, 0.159428));
            assert!(equal_results(ret[129].1, 0.683404));
            assert!(equal_results(ret[130].1, 0.531666));
            assert!(equal_results(ret[131].1, 0.824297));
            assert!(equal_results(ret[132].1, 0.108906));
            assert!(equal_results(ret[133].1, 0.725597));
            assert!(equal_results(ret[134].1, 0.271241));
            assert!(equal_results(ret[135].1, 0.714774));
            assert!(equal_results(ret[136].1, 0.783178));
            assert!(equal_results(ret[137].1, 0.498486));
            assert!(equal_results(ret[138].1, 0.924418));
            assert!(equal_results(ret[139].1, 0.575407));
            assert!(equal_results(ret[140].1, 0.494166));
            assert!(equal_results(ret[141].1, 0.967612));
            assert!(equal_results(ret[142].1, 0.458325));
            assert!(equal_results(ret[143].1, 0.927039));
            assert!(equal_results(ret[144].1, 0.105639));
            assert!(equal_results(ret[145].1, 0.017200));
            assert!(equal_results(ret[146].1, 0.390641));
            assert!(equal_results(ret[147].1, 0.541378));
        }

        #[test]
        fn test_overlapping() {
            use nistrs::overlapping_template::overlapping_template_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(
                overlapping_template_test(&res, 9).1,
                0.339426
            ))
        }

        #[test]
        fn test_universal() {
            use nistrs::universal::universal_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(universal_test(&res).1, 0.411079))
        }

        #[test]
        fn test_linear() {
            use nistrs::linear::linear_complexity_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(linear_complexity_test(&res, 500).1, 0.309412))
        }

        #[test]
        fn test_serial() {
            use nistrs::serial::serial_test;

            let res = load_sequnce().unwrap();

            let ret = serial_test(&res, 16);

            assert!(equal_results(ret[0].1, 0.760793));
        }

        #[test]
        fn test_approximate() {
            use nistrs::approximate::approximate_entropy_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(
                approximate_entropy_test(&res, 10).1,
                0.982885
            ));
        }

        #[test]
        fn test_cusum() {
            use nistrs::cusum::cumulative_sums_test;

            let res = load_sequnce().unwrap();

            let ret = cumulative_sums_test(&res);

            assert!(equal_results(ret[0].1, 0.451231));

            assert!(equal_results(ret[1].1, 0.550134));
        }

        #[test]
        #[should_panic]
        fn test_random_excursions() {
            use nistrs::random_excursions::random_excursions_test;

            let res = load_sequnce().unwrap();

            random_excursions_test(&res).unwrap();
        }

        #[test]
        #[should_panic]
        fn test_random_excursions_variant() {
            use nistrs::random_excursions_variant::random_excursions_variant_test;

            let res = load_sequnce().unwrap();

            random_excursions_variant_test(&res).unwrap();
        }
    }
}
