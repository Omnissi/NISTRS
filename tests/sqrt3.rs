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
        let path = Path::new("./tests/files/data.sqrt3");
        if !path.exists() {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Test sequece not found",
            ));
        }

        let mut data = String::new();
        File::open(path)?.read_to_string(&mut data)?;

        Ok(BitsData::from_text(data))
    }
    mod sqrt3 {
        use crate::test::{equal_results, load_sequnce};

        #[test]
        fn test_freq() {
            use nistrs::freq::frequency_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(frequency_test(&res).1, 0.546820));
        }

        #[test]
        fn test_block_freq() {
            use nistrs::block_freq::block_frequency_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(
                block_frequency_test(&res, 128).unwrap().1,
                0.473925
            ));
        }

        #[test]
        fn test_runs() {
            use nistrs::runs::runs_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(runs_test(&res).1, 0.229863));
        }

        #[test]
        fn test_longest_ones_runs() {
            use nistrs::longest_run_of_ones::longest_run_of_ones_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(
                longest_run_of_ones_test(&res).unwrap().1,
                0.446726
            ));
        }

        #[test]
        fn test_rank() {
            use nistrs::rank::rank_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(rank_test(&res).unwrap().1, 0.348786));
        }

        #[test]
        fn test_fft() {
            use nistrs::fft::fft_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(fft_test(&res).1, 0.509824));
        }

        #[test]
        fn test_non_overlapping() {
            use nistrs::non_overlapping_template::non_overlapping_template_test;

            let res = load_sequnce().unwrap();

            let ret = non_overlapping_template_test(&res, 9).unwrap();

            assert!(equal_results(ret[0].1, 0.424530));
            assert!(equal_results(ret[1].1, 0.887225));
            assert!(equal_results(ret[2].1, 0.328113));
            assert!(equal_results(ret[3].1, 0.992024));
            assert!(equal_results(ret[4].1, 0.854501));
            assert!(equal_results(ret[5].1, 0.273225));
            assert!(equal_results(ret[6].1, 0.389844));
            assert!(equal_results(ret[7].1, 0.805707));
            assert!(equal_results(ret[8].1, 0.238178));
            assert!(equal_results(ret[9].1, 0.981848));
            assert!(equal_results(ret[10].1, 0.986376));
            assert!(equal_results(ret[11].1, 0.688552));
            assert!(equal_results(ret[12].1, 0.513210));
            assert!(equal_results(ret[13].1, 0.226998));
            assert!(equal_results(ret[14].1, 0.753423));
            assert!(equal_results(ret[15].1, 0.033345));
            assert!(equal_results(ret[16].1, 0.194836));
            assert!(equal_results(ret[17].1, 0.253272));
            assert!(equal_results(ret[18].1, 0.399046));
            assert!(equal_results(ret[19].1, 0.183303));
            assert!(equal_results(ret[20].1, 0.891072));
            assert!(equal_results(ret[21].1, 0.394578));
            assert!(equal_results(ret[22].1, 0.822593));
            assert!(equal_results(ret[23].1, 0.946055));
            assert!(equal_results(ret[24].1, 0.676465));
            assert!(equal_results(ret[25].1, 0.208579));
            assert!(equal_results(ret[26].1, 0.549645));
            assert!(equal_results(ret[27].1, 0.961691));
            assert!(equal_results(ret[28].1, 0.389116));
            assert!(equal_results(ret[29].1, 0.362742));
            assert!(equal_results(ret[30].1, 0.684970));
            assert!(equal_results(ret[31].1, 0.199787));
            assert!(equal_results(ret[32].1, 0.723818));
            assert!(equal_results(ret[33].1, 0.824072));
            assert!(equal_results(ret[34].1, 0.703254));
            assert!(equal_results(ret[35].1, 0.888162));
            assert!(equal_results(ret[36].1, 0.321256));
            assert!(equal_results(ret[37].1, 0.122400));
            assert!(equal_results(ret[38].1, 0.924783));
            assert!(equal_results(ret[39].1, 0.064828));
            assert!(equal_results(ret[40].1, 0.506507));
            assert!(equal_results(ret[41].1, 0.377596));
            assert!(equal_results(ret[42].1, 0.713390));
            assert!(equal_results(ret[43].1, 0.740004));
            assert!(equal_results(ret[44].1, 0.497652));
            assert!(equal_results(ret[45].1, 0.256037));
            assert!(equal_results(ret[46].1, 0.466285));
            assert!(equal_results(ret[47].1, 0.202168));
            assert!(equal_results(ret[48].1, 0.227588));
            assert!(equal_results(ret[49].1, 0.131734));
            assert!(equal_results(ret[50].1, 0.158560));
            assert!(equal_results(ret[51].1, 0.109698));
            assert!(equal_results(ret[52].1, 0.574424));
            assert!(equal_results(ret[53].1, 0.460508));
            assert!(equal_results(ret[54].1, 0.436346));
            assert!(equal_results(ret[55].1, 0.816963));
            assert!(equal_results(ret[56].1, 0.417371));
            assert!(equal_results(ret[57].1, 0.781368));
            assert!(equal_results(ret[58].1, 0.957793));
            assert!(equal_results(ret[59].1, 0.747372));
            assert!(equal_results(ret[60].1, 0.402440));
            assert!(equal_results(ret[61].1, 0.098035));
            assert!(equal_results(ret[62].1, 0.767256));
            assert!(equal_results(ret[63].1, 0.485520));
            assert!(equal_results(ret[64].1, 0.666936));
            assert!(equal_results(ret[65].1, 0.354678));
            assert!(equal_results(ret[66].1, 0.596152));
            assert!(equal_results(ret[67].1, 0.134610));
            assert!(equal_results(ret[68].1, 0.680591));
            assert!(equal_results(ret[69].1, 0.939800));
            assert!(equal_results(ret[70].1, 0.901293));
            assert!(equal_results(ret[71].1, 0.079350));
            assert!(equal_results(ret[72].1, 0.176848));
            assert!(equal_results(ret[73].1, 0.071205));
            assert!(equal_results(ret[74].1, 0.424530));
            assert!(equal_results(ret[75].1, 0.247669));
            assert!(equal_results(ret[76].1, 0.273731));
            assert!(equal_results(ret[77].1, 0.788742));
            assert!(equal_results(ret[78].1, 0.598447));
            assert!(equal_results(ret[79].1, 0.004249));
            assert!(equal_results(ret[80].1, 0.322169));
            assert!(equal_results(ret[81].1, 0.570237));
            assert!(equal_results(ret[82].1, 0.308038));
            assert!(equal_results(ret[83].1, 0.957103));
            assert!(equal_results(ret[84].1, 0.930850));
            assert!(equal_results(ret[85].1, 0.198524));
            assert!(equal_results(ret[86].1, 0.041746));
            assert!(equal_results(ret[87].1, 0.555491));
            assert!(equal_results(ret[88].1, 0.839062));
            assert!(equal_results(ret[89].1, 0.137494));
            assert!(equal_results(ret[90].1, 0.421857));
            assert!(equal_results(ret[91].1, 0.849809));
            assert!(equal_results(ret[92].1, 0.135209));
            assert!(equal_results(ret[93].1, 0.768058));
            assert!(equal_results(ret[94].1, 0.004231));
            assert!(equal_results(ret[95].1, 0.605397));
            assert!(equal_results(ret[96].1, 0.161513));
            assert!(equal_results(ret[97].1, 0.028669));
            assert!(equal_results(ret[98].1, 0.785612));
            assert!(equal_results(ret[99].1, 0.319326));
            assert!(equal_results(ret[100].1, 0.917332));
            assert!(equal_results(ret[101].1, 0.289763));
            assert!(equal_results(ret[102].1, 0.402166));
            assert!(equal_results(ret[103].1, 0.977245));
            assert!(equal_results(ret[104].1, 0.547568));
            assert!(equal_results(ret[105].1, 0.666999));
            assert!(equal_results(ret[106].1, 0.658943));
            assert!(equal_results(ret[107].1, 0.310540));
            assert!(equal_results(ret[108].1, 0.616125));
            assert!(equal_results(ret[109].1, 0.914996));
            assert!(equal_results(ret[110].1, 0.875309));
            assert!(equal_results(ret[111].1, 0.272204));
            assert!(equal_results(ret[112].1, 0.188305));
            assert!(equal_results(ret[113].1, 0.146763));
            assert!(equal_results(ret[114].1, 0.055417));
            assert!(equal_results(ret[115].1, 0.256215));
            assert!(equal_results(ret[116].1, 0.969386));
            assert!(equal_results(ret[117].1, 0.009763));
            assert!(equal_results(ret[118].1, 0.079618));
            assert!(equal_results(ret[119].1, 0.270477));
            assert!(equal_results(ret[120].1, 0.315155));
            assert!(equal_results(ret[121].1, 0.313453));
            assert!(equal_results(ret[122].1, 0.906179));
            assert!(equal_results(ret[123].1, 0.827663));
            assert!(equal_results(ret[124].1, 0.449617));
            assert!(equal_results(ret[125].1, 0.507883));
            assert!(equal_results(ret[126].1, 0.218126));
            assert!(equal_results(ret[127].1, 0.766346));
            assert!(equal_results(ret[128].1, 0.919958));
            assert!(equal_results(ret[129].1, 0.126258));
            assert!(equal_results(ret[130].1, 0.096635));
            assert!(equal_results(ret[131].1, 0.499688));
            assert!(equal_results(ret[132].1, 0.181426));
            assert!(equal_results(ret[133].1, 0.001628));
            assert!(equal_results(ret[134].1, 0.519370));
            assert!(equal_results(ret[135].1, 0.203758));
            assert!(equal_results(ret[136].1, 0.007882));
            assert!(equal_results(ret[137].1, 0.947168));
            assert!(equal_results(ret[138].1, 0.759773));
            assert!(equal_results(ret[139].1, 0.424877));
            assert!(equal_results(ret[140].1, 0.101836));
            assert!(equal_results(ret[141].1, 0.149678));
            assert!(equal_results(ret[142].1, 0.267217));
            assert!(equal_results(ret[143].1, 0.754730));
            assert!(equal_results(ret[144].1, 0.168805));
            assert!(equal_results(ret[145].1, 0.600663));
            assert!(equal_results(ret[146].1, 0.248307));
            assert!(equal_results(ret[147].1, 0.071205));
        }

        #[test]
        fn test_overlapping() {
            use nistrs::overlapping_template::overlapping_template_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(
                overlapping_template_test(&res, 9).1,
                0.070981
            ))
        }

        #[test]
        fn test_universal() {
            use nistrs::universal::universal_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(universal_test(&res).1, 0.150578))
        }

        #[test]
        fn test_linear() {
            use nistrs::linear::linear_complexity_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(linear_complexity_test(&res, 500).1, 0.341994))
        }

        #[test]
        fn test_serial() {
            use nistrs::serial::serial_test;

            let res = load_sequnce().unwrap();

            let ret = serial_test(&res, 16);

            assert!(equal_results(ret[0].1, 0.180826));
            assert!(equal_results(ret[1].1, 0.230061));
        }

        #[test]
        fn test_approximate() {
            use nistrs::approximate::approximate_entropy_test;

            let res = load_sequnce().unwrap();

            assert!(equal_results(
                approximate_entropy_test(&res, 10).1,
                0.154929
            ));
        }

        #[test]
        fn test_cusum() {
            use nistrs::cusum::cumulative_sums_test;

            let res = load_sequnce().unwrap();

            let ret = cumulative_sums_test(&res);

            assert!(equal_results(ret[0].1, 0.918218));

            assert!(equal_results(ret[1].1, 0.605167));
        }

        #[test]
        fn test_random_excursions() {
            use nistrs::random_excursions::random_excursions_test;

            let res = load_sequnce().unwrap();

            let ret = random_excursions_test(&res);

            assert!(equal_results(ret.unwrap()[4].1, 0.783283));
        }

        #[test]
        fn test_random_excursions_variant() {
            use nistrs::random_excursions_variant::random_excursions_variant_test;

            let res = load_sequnce().unwrap();

            let ret = random_excursions_variant_test(&res);

            assert!(equal_results(ret.unwrap()[8].1, 0.155066));
        }
    }
}
