mod RNG{
    use rand::{rngs::StdRng, Rng, SeedableRng};

    pub trait RandomNumberGenerator {
        fn next_random(&mut self)->u8;
    } 

    struct DefaultRandomNumberGenerator{
        RNG: StdRng
    }

    impl RandomNumberGenerator for DefaultRandomNumberGenerator {
        fn next_random(&mut self)->u8 {
            self.RNG.gen_range(0..37)
        }
    }

    impl Default for DefaultRandomNumberGenerator {
        fn default() -> DefaultRandomNumberGenerator {
            DefaultRandomNumberGenerator{
                RNG: StdRng::from_entropy()
            }
        }
    }
}

mod roulette_simulator{
    use super::RNG::RandomNumberGenerator;
    struct Roulette<'a> {
        rng: &'a mut dyn RandomNumberGenerator,
        pending_bets: Vec<BetInformation>
    }
    

    impl <'a>Roulette<'a>{
        fn from(rng: &'a mut dyn RandomNumberGenerator)-> Roulette{
            Roulette{
                rng,
                pending_bets: Vec::new()
            }
        }

        fn simulate(&mut self) -> (usize, SpinInformation){
            let random_number = self.rng.next_random();
            let spin_information = SpinInformation::from(random_number);

            let mut total_win = 0;
            for bet in self.pending_bets.drain(0..){

            }
            return (total_win,spin_information);
        }
        fn bet_one_number(&mut self,bet_amount:usize,number:Nums){
            let bet = BetInformation{
                bet_amount,
                number : Some(number),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }

        fn bet_two_numbers(&mut self,bet_amount:usize,split:Splits){
            let bet = BetInformation{
                bet_amount,
                split: Some(split),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }
        fn bet_three_numbers(&mut self,bet_amount:usize,street:Streets){
            let bet = BetInformation{
                bet_amount,
                street : Some(street),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }
        fn bet_four_numbers(&mut self,bet_amount:usize,corner:Corners){
            let bet = BetInformation{
                bet_amount,
                corner: Some(corner),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }
        fn bet_six_numbers(&mut self,bet_amount:usize,line:Lines){
            let bet = BetInformation{
                bet_amount,
                line : Some(line),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }
        fn bet_twelve_numbers(&mut self,bet_amount:usize,dozen:Dozens){
            let bet = BetInformation{
                bet_amount,
                 dozen: Some(dozen),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }
        fn bet_column(&mut self,bet_amount:usize, column: Columns){
            let bet = BetInformation{
                bet_amount,
                column: Some(column),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }
        fn bet_half_numbers(&mut self,bet_amount:usize, half_numbers: HalfNumbers){
            let bet = BetInformation{
                bet_amount,
                half_numbers: Some(half_numbers),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }

        fn bet_color(&mut self,bet_amount:usize, color:Colors){
            let bet = BetInformation{
                bet_amount,
                color:Some(color),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }
        fn bet_parity(&mut self,bet_amount:usize, parity: Parity){
            let bet = BetInformation{
                bet_amount,
                parity: Some(parity),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }

    }


    struct BetInformation{
        bet_amount: usize,
        number: Option<Nums>,
        color: Option<Colors>,
        dozen: Option<Dozens>,
        column: Option<Columns>,
        parity: Option<Parity>,
        street: Option<Streets>,
        line: Option<Lines>,
        split: Option<Splits>,
        corner:Option<Corners>,
        half_numbers : Option<HalfNumbers>
    }

    impl Default for BetInformation{
        fn default() -> Self {
            BetInformation{
                bet_amount:0,
                number: None,
                color: None,
                dozen: None,
                column: None,
                parity: None,
                split: None,
                street: None,
                line: None,
                corner: None,
                half_numbers: None,
                
            }
        }
    }

    struct SpinInformation{
        num:u8,
        color: Colors,
        dozen: Dozens,
        column: Columns,
        parity: Parity,
        street: Streets,
        line: Lines
    }

    impl SpinInformation {
        fn from(num:u8) ->  SpinInformation{
            if num <=36{
                let color = match num {
                    0 => Colors::Green,
                    1 | 3 | 5 | 7 | 9 | 12  | 14  | 16 | 18 | 19 | 21 | 23 | 25 | 27 | 30 | 32 | 34 | 36 => Colors::Red,
                    2 | 4 | 6 | 8 | 10 | 11 | 13 | 15 | 17 | 20 | 22 | 24 | 26 | 28 | 29 | 31 | 33 | 35 => Colors::Black,
                    _ => panic!("This should be not happening")
                };
                let dozen =match num {
                    1..=12 => Dozens::First,
                    13..=24 => Dozens::Second,
                    25..=36 => Dozens::Third,
                    0 => Dozens::Zero,
                    _ => panic!("This should be not happening")
                };
                let column;
                if num==0{
                    column=Columns::Zero;
                }else if num%3==1 {
                    column=Columns::First;
                }else if num%3==2 {
                    column=Columns::Second;
                }else {
                    column=Columns::Third;
                }

                let parity;
                if num==0{
                    parity=Parity::Zero
                } else if num%2==0 {
                    parity=Parity::Even;
                }else {
                    parity=Parity::Odd;
                }

                let street = match num {
                    0 => Streets::Zero,
                    1..=3 => Streets::Street1_2_3,
                    4..=6 => Streets::Street4_5_6,
                    7..=9 => Streets::Street7_8_9,
                    10..=12 => Streets::Street10_11_12,
                    13..=15 => Streets::Street13_14_15,
                    16..=18 => Streets::Street16_17_18,
                    19..=21 => Streets::Street19_20_21,
                    22..=24 => Streets::Street22_23_24,
                    25..=27 => Streets::Street25_26_27,
                    28..=30 => Streets::Street28_29_30,
                    31..=33 => Streets::Street31_32_33,
                    34..=36=>Streets::Street34_35_36,
                    _ => panic!("This should be not happening")
                };

                let line = match num {
                    0 => Lines::Zero,
                    1..=6 =>    Lines::Line1_6  ,
                    7..=12=>    Lines::Line7_12 ,
                    13..=18 =>  Lines::Line13_18,
                    19..=24 =>  Lines::Line19_24,
                    25..=30 =>  Lines::Line25_30,
                    31..=36 =>  Lines::Line31_36,
                    _ => panic!("This should be not happening")
                };
                return SpinInformation{ num, color, dozen, column, parity, street, line};
            }else{
                panic!("This number is wrong");
            }
        }
    }
    enum Nums {
        Only0,
        Only1,
        Only2,
        Only3,
        Only4,
        Only5,
        Only6,
        Only7,
        Only8,
        Only9,
        Only10,
        Only11,
        Only12,
        Only13,
        Only14,
        Only15,
        Only16,
        Only17,
        Only18,
        Only19,
        Only20,
        Only21,
        Only22,
        Only23,
        Only24,
        Only25,
        Only26,
        Only27,
        Only28,
        Only29,
        Only30,
        Only31,
        Only32,
        Only33,
        Only34,
        Only35,
        Only36,
    }
    enum Colors{
        Red,
        Black,
        Green
    }
    enum Dozens{
        Zero,
        First,
        Second,
        Third
    }
    enum Columns{
        Zero,
        First,
        Second,
        Third
    }
    enum Parity{
        Zero,
        Odd,
        Even
    }
    enum Splits{
        // Splits between adjacent numbers
        Split1_2,
        Split2_3,
        Split4_5,
        Split5_6,
        Split7_8,
        Split8_9,
        Split10_11,
        Split11_12,
        Split13_14,
        Split14_15,
        Split16_17,
        Split17_18,
        Split19_20,
        Split20_21,
        Split22_23,
        Split23_24,
        Split25_26,
        Split26_27,
        Split28_29,
        Split29_30,
        Split31_32,
        Split32_33,
        Split34_35,
        Split35_36,
            
        // Splits between numbers separated by one number
        Split1_4,
        Split2_5,
        Split3_6,
        Split4_7,
        Split5_8,
        Split6_9,
        Split7_10,
        Split8_11,
        Split9_12,
        Split10_13,
        Split11_14,
        Split12_15,
        Split13_16,
        Split14_17,
        Split15_18,
        Split16_19,
        Split17_20,
        Split18_21,
        Split19_22,
        Split20_23,
        Split21_24,
        Split22_25,
        Split23_26,
        Split24_27,
        Split25_28,
        Split26_29,
        Split27_30,
        Split28_31,
        Split29_32,
        Split30_33,
        Split31_34,
        Split32_35,
        Split33_36,
    }
    enum Streets{
        Zero,
        Street1_2_3,
        Street4_5_6,
        Street7_8_9,
        Street10_11_12,
        Street13_14_15,
        Street16_17_18,
        Street19_20_21,
        Street22_23_24,
        Street25_26_27,
        Street28_29_30,
        Street31_32_33,
        Street34_35_36,
    }
    enum Lines {
        Zero,
        Line1_6,
        Line7_12,
        Line13_18,
        Line19_24,
        Line25_30,
        Line31_36

    }
    enum HalfNumbers {
        From1To18,
        From19To36
    }
    enum Corners {
        Corner1_2_4_5,
        Corner2_3_5_6,
        Corner4_5_7_8,
        Corner5_6_8_9,
        Corner7_8_10_11,
        Corner8_9_11_12,
        Corner10_11_13_14,
        Corner11_12_14_15,
        Corner13_14_16_17,
        Corner14_15_17_18,
        Corner16_17_19_20,
        Corner17_18_20_21,
        Corner19_20_22_23,
        Corner20_21_23_24,
        Corner22_23_25_26,
        Corner23_24_26_27,
        Corner25_26_28_29,
        Corner26_27_29_30,
        Corner28_29_31_32,
        Corner29_30_32_33,
        Corner31_32_34_35,
        Corner32_33_35_36,
    }

    #[cfg(test)]
    mod tests{
        use super::*;

        struct TestRandomNumberGenerator{
            current_num:u8,
        }
        impl Default for TestRandomNumberGenerator {
            fn default() -> TestRandomNumberGenerator {
                TestRandomNumberGenerator{
                    current_num:0
                }
            }
        }

        impl RandomNumberGenerator for TestRandomNumberGenerator {
            fn next_random(&mut self) -> u8{
                let current_val = self.current_num;
                self.current_num=(self.current_num+1)%37;
                return current_val;
            }
        }

        
        #[test]
        fn test_payout_bet_num(){
            //Define a roulette for testing
            //We pass a custom RNG for testing, if no RNG is provided it falls back to rand
            let mut roulette_sim = Roulette::from(&TestRandomNumberGenerator::default());
            roulette_sim.bet_one_number(1,Nums::Only1);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(0,returned_earnings);

            roulette_sim.bet_one_number(1,Nums::Only1);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(36,returned_earnings);
        }

        #[test]
        fn test_payout_bet_two_nums(){
            //Define a roulette for testing
            //We pass a custom RNG for testing, if no RNG is provided it falls back to rand
            let mut roulette_sim = Roulette::from(&TestRandomNumberGenerator::default());
            roulette_sim.bet_two_numbers(1,Splits::Split1_2);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(0,returned_earnings);

            roulette_sim.bet_two_numbers(1,Splits::Split1_2);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(18,returned_earnings);
        }

        #[test]
        fn test_payout_bet_three_nums(){
            //Define a roulette for testing
            //We pass a custom RNG for testing, if no RNG is provided it falls back to rand
            let mut roulette_sim = Roulette::from(&TestRandomNumberGenerator::default());
            roulette_sim.bet_three_numbers(1,Streets::Street1_2_3);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(0,returned_earnings);

            roulette_sim.bet_three_numbers(1,Streets::Street1_2_3);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(12,returned_earnings);
        }


        #[test]
        fn test_payout_bet_four_nums(){
            //Define a roulette for testing
            //We pass a custom RNG for testing, if no RNG is provided it falls back to rand
            let mut roulette_sim = Roulette::from(&TestRandomNumberGenerator::default());
            roulette_sim.bet_four_numbers(1,Corners::Corner1_2_4_5);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(0,returned_earnings);

            roulette_sim.bet_four_numbers(1,Corners::Corner1_2_4_5);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(9,returned_earnings);
        }



        #[test]
        fn test_payout_bet_six_nums(){
            //Define a roulette for testing
            //We pass a custom RNG for testing, if no RNG is provided it falls back to rand
            let mut roulette_sim = Roulette::from(&TestRandomNumberGenerator::default());
            roulette_sim.bet_six_numbers(1,Lines::Line1_6);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(0,returned_earnings);

            roulette_sim.bet_six_numbers(1,Lines::Line1_6);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(6,returned_earnings);
        }



        #[test]
        fn test_payout_bet_twelve_nums(){
            //Define a roulette for testing
            //We pass a custom RNG for testing, if no RNG is provided it falls back to rand
            let mut roulette_sim = Roulette::from(&TestRandomNumberGenerator::default());
            roulette_sim.bet_twelve_numbers(1,Dozens::First);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(0,returned_earnings);

            roulette_sim.bet_twelve_numbers(1,Dozens::First);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(2,returned_earnings);
        }


        #[test]
        fn test_payout_bet_columns(){
            //Define a roulette for testing
            //We pass a custom RNG for testing, if no RNG is provided it falls back to rand
            let mut roulette_sim = Roulette::from(&TestRandomNumberGenerator::default());
            roulette_sim.bet_column(1,Columns::First);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(0,returned_earnings);

            roulette_sim.bet_column(1,Columns::First);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(2,returned_earnings);
        }


        #[test]
        fn test_payout_bet_up_to_18(){
            //Define a roulette for testing
            //We pass a custom RNG for testing, if no RNG is provided it falls back to rand
            let mut roulette_sim = Roulette::from(&TestRandomNumberGenerator::default());
            roulette_sim.bet_half_numbers(1,HalfNumbers::From1To18);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(0,returned_earnings);

            roulette_sim.bet_half_numbers(1,HalfNumbers::From1To18);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(2,returned_earnings);
        }


        #[test]
        fn test_payout_bet_color(){
            //Define a roulette for testing
            //We pass a custom RNG for testing, if no RNG is provided it falls back to rand
            let mut roulette_sim = Roulette::from(&TestRandomNumberGenerator::default());
            roulette_sim.bet_color(1,Colors::Red);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(0,returned_earnings);

            roulette_sim.bet_color(1,Colors::Red);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(2,returned_earnings);
        }

        #[test]
        fn test_payout_bet_even(){
            //Define a roulette for testing
            //We pass a custom RNG for testing, if no RNG is provided it falls back to rand
            let mut roulette_sim = Roulette::from(&TestRandomNumberGenerator::default());
            roulette_sim.bet_parity(1,Parity::Odd);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(0,returned_earnings);

            roulette_sim.bet_parity(1,Parity::Odd);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(2,returned_earnings);
        }


        #[test]
        fn test_seed_reproducibility(){
            let seed = 1;
            let mut roulette_sim = Roulette::from_seed(seed);
            let (_,spin_information) = roulette_sim.simulate();
            assert_eq!(1,spin_information.num);

            let seed = 1;
            let mut roulette_sim = Roulette::from_seed(seed);
            let (_,spin_information) = roulette_sim.simulate();
            assert_eq!(1,spin_information.num);

            let seed = 1;
            let mut roulette_sim = Roulette::from_seed(seed);
            let (_,spin_information) = roulette_sim.simulate();
            assert_eq!(1,spin_information.num);

            let seed = 2;
            let mut roulette_sim = Roulette::from_seed(seed);
            let (_,spin_information) = roulette_sim.simulate();
            assert_ne!(1,spin_information.num);
        }
    }

        
}