pub mod RNG{
    use rand::{rngs::StdRng, Rng, SeedableRng};

    pub trait RandomNumberGenerator {
        fn next_random(&mut self)->u8;
    } 

    pub struct DefaultRandomNumberGenerator{
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

pub mod roulette_simulator{

    use crate::RNG::DefaultRandomNumberGenerator;
    use super::RNG::RandomNumberGenerator;
    pub struct Roulette {
        rng: DefaultRandomNumberGenerator,
        pending_bets: Vec<BetInformation>
    }
    
    impl Roulette{
        
        pub fn from_entropy()-> Roulette{
            Roulette{
                rng:DefaultRandomNumberGenerator::default(),
                pending_bets: Vec::new()
            }
        }
        pub fn from_rng(rng: &'a mut dyn RandomNumberGenerator)-> Roulette{
            Roulette{
                rng,
                pending_bets: Vec::new()
            }
        }
        
        pub fn simulate(&mut self) -> (usize, SpinInformation){
            let random_number = self.rng.next_random();
            let spin_information = SpinInformation::from(random_number);

            let mut total_win = 0;
            for bet in self.pending_bets.drain(0..){
                total_win+= Self::check_winnings_of_bet(&spin_information,bet);
            }
            return (total_win,spin_information);
        }
        fn check_winnings_of_bet(spin_information: &SpinInformation, bet_information: BetInformation) -> usize{
            //Num
            match bet_information.number{
                Some(number_bet) =>{
                    if number_bet as u8 == spin_information.num  {
                        return bet_information.bet_amount*36;
                    }else{
                        return 0;
                    }
                }
                _ => {}
            } 
            
            match bet_information.color{
                Some(color_bet) =>{
                    if color_bet == spin_information.color {
                        return bet_information.bet_amount*2;
                    }else{
                        return 0;
                    }
                }
                _ => {}
            } 

            match bet_information.dozen{
                Some(dozen_bet) =>{
                    if dozen_bet == spin_information.dozen{
                        return bet_information.bet_amount*3;
                    }else{
                        return 0;
                    }
                }
                _ => {}
            } 
            match bet_information.column{
                Some(column_bet) =>{
                    if column_bet == spin_information.column {
                        return bet_information.bet_amount*3;
                    }else{
                        return 0;
                    }
                }
                _ => {}
            } 
            match bet_information.parity{
                Some(parity_bet) =>{
                    if parity_bet == spin_information.parity {
                        return bet_information.bet_amount*2;
                    }else{
                        return 0;
                    }
                }
                _ => {}
            } 
            match bet_information.split{
                Some(split_bet) =>{
                    if split_bet.check(spin_information.num){
                        return bet_information.bet_amount*18;
                    }else{
                        return 0;
                    }
                }
                _ => {}
            } 
            match bet_information.street{
                Some(street_bet) =>{
                    if street_bet == spin_information.street {
                        return bet_information.bet_amount*12;
                    }else{
                        return 0;
                    }
                }
                _ => {}
            } 
            match bet_information.line{
                Some(line_bet) =>{
                    if line_bet == spin_information.line {
                        return bet_information.bet_amount*6;
                    }else{
                        return 0;
                    }
                }
                _ => {}
            } 
            match bet_information.corner{
                Some(corner_bet) =>{
                    if corner_bet.check(spin_information.num) {
                        return bet_information.bet_amount*9;
                    }else{
                        return 0;
                    }
                }
                _ => {}
            } 
            match bet_information.half_numbers{
                Some(half_numbers_bet) =>{
                    if half_numbers_bet == spin_information.half_number {
                        return bet_information.bet_amount*2;
                    }else{
                        return 0;
                    }
                }
                _ => {}
            } 
            return 0;
        }
        pub fn bet_one_number(&mut self,bet_amount:usize,number:Nums){
            let bet = BetInformation{
                bet_amount,
                number : Some(number),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }

        pub fn bet_two_numbers(&mut self,bet_amount:usize,split:Splits){
            let bet = BetInformation{
                bet_amount,
                split: Some(split),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }
        pub fn bet_three_numbers(&mut self,bet_amount:usize,street:Streets){
            let bet = BetInformation{
                bet_amount,
                street : Some(street),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }
        pub fn bet_four_numbers(&mut self,bet_amount:usize,corner:Corners){
            let bet = BetInformation{
                bet_amount,
                corner: Some(corner),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }
        pub fn bet_six_numbers(&mut self,bet_amount:usize,line:Lines){
            let bet = BetInformation{
                bet_amount,
                line : Some(line),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }
        pub fn bet_twelve_numbers(&mut self,bet_amount:usize,dozen:Dozens){
            let bet = BetInformation{
                bet_amount,
                dozen: Some(dozen),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }
        pub fn bet_column(&mut self,bet_amount:usize, column: Columns){
            let bet = BetInformation{
                bet_amount,
                column: Some(column),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }
        pub fn bet_half_numbers(&mut self,bet_amount:usize, half_numbers: HalfNumbers){
            let bet = BetInformation{
                bet_amount,
                half_numbers: Some(half_numbers),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }

        pub fn bet_color(&mut self,bet_amount:usize, color:Colors){
            let bet = BetInformation{
                bet_amount,
                color:Some(color),
                ..Default::default()
            };
            self.pending_bets.push(bet);
        }
        pub fn bet_parity(&mut self,bet_amount:usize, parity: Parity){
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

    #[derive(Debug)]
    pub struct SpinInformation{
        pub num:u8,
        color: Colors,
        dozen: Dozens,
        column: Columns,
        parity: Parity,
        street: Streets,
        line: Lines,
        half_number: HalfNumbers,

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
                let half_number = match num {
                    0 => HalfNumbers::Zero,
                    1..=18 => HalfNumbers::From1To18,
                    19..=36 => HalfNumbers::From19To36,
                    _ => panic!("This should be not happening")
                };

                return SpinInformation{num, color, dozen, column, parity, street, line,half_number};
            }else{
                panic!("This number is wrong");
            }
        }
    }


    #[derive(PartialEq, PartialOrd)]
    pub enum Nums {
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
    
    #[derive(PartialEq,Debug)]
    pub enum Colors{
        Red,
        Black,
        Green
    }
    #[derive(PartialEq,Debug)]
    pub enum Dozens{
        Zero,
        First,
        Second,
        Third
    }
    #[derive(PartialEq,Debug)]
    pub enum Columns{
        Zero,
        First,
        Second,
        Third
    }
    #[derive(PartialEq,Debug)]
    pub enum Parity{
        Zero,
        Odd,
        Even
    }
    #[derive(PartialEq,Debug)]
    pub enum Splits{
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

    impl Splits{
        fn check(&self,num:u8)-> bool{
            match num {
                0 => false,
                1 => *self == Splits::Split1_2 || *self == Splits::Split1_4,
                2 => *self == Splits::Split1_2 || *self == Splits::Split2_5,
                3 => *self == Splits::Split2_3 || *self == Splits::Split3_6,
                4 => *self == Splits::Split1_4 || *self == Splits::Split4_7,
                5 => *self == Splits::Split2_5 || *self == Splits::Split5_8,
                6 => *self == Splits::Split3_6 || *self == Splits::Split6_9,
                7 => *self == Splits::Split4_7 || *self == Splits::Split7_10,
                8 => *self == Splits::Split5_8 || *self == Splits::Split8_11,
                9 => *self == Splits::Split6_9 || *self == Splits::Split9_12,
                10 => *self == Splits::Split7_10 || *self == Splits::Split10_13,
                11 => *self == Splits::Split8_11 || *self == Splits::Split11_14,
                12 => *self == Splits::Split9_12 || *self == Splits::Split12_15,
                13 => *self == Splits::Split10_13 || *self == Splits::Split13_16,
                14 => *self == Splits::Split11_14 || *self == Splits::Split14_17,
                15 => *self == Splits::Split12_15 || *self == Splits::Split15_18,
                16 => *self == Splits::Split13_16 || *self == Splits::Split16_19,
                17 => *self == Splits::Split14_17 || *self == Splits::Split17_20,
                18 => *self == Splits::Split15_18 || *self == Splits::Split18_21,
                19 => *self == Splits::Split16_19 || *self == Splits::Split19_22,
                20 => *self == Splits::Split17_20 || *self == Splits::Split20_23,
                21 => *self == Splits::Split18_21 || *self == Splits::Split21_24,
                22 => *self == Splits::Split19_22 || *self == Splits::Split22_25,
                23 => *self == Splits::Split20_23 || *self == Splits::Split23_26,
                24 => *self == Splits::Split21_24 || *self == Splits::Split24_27,
                25 => *self == Splits::Split22_25 || *self == Splits::Split25_28,
                26 => *self == Splits::Split23_26 || *self == Splits::Split26_29,
                27 => *self == Splits::Split24_27 || *self == Splits::Split27_30,
                28 => *self == Splits::Split25_28 || *self == Splits::Split28_31,
                29 => *self == Splits::Split26_29 || *self == Splits::Split29_32,
                30 => *self == Splits::Split27_30 || *self == Splits::Split30_33,
                31 => *self == Splits::Split28_31 || *self == Splits::Split31_34,
                32 => *self == Splits::Split29_32 || *self == Splits::Split32_35,
                33 => *self == Splits::Split30_33 || *self == Splits::Split33_36,
                34 => *self == Splits::Split31_34 || *self == Splits::Split34_35,
                35 => *self == Splits::Split32_35 || *self == Splits::Split35_36,
                36 => *self == Splits::Split33_36 || *self == Splits::Split35_36,
                _ => false
            }
        }
    }
    #[derive(PartialEq,Debug)]
    pub enum Streets{
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
    #[derive(PartialEq,Debug)]
    pub enum Lines {
        Zero,
        Line1_6,
        Line7_12,
        Line13_18,
        Line19_24,
        Line25_30,
        Line31_36

    }
    #[derive(PartialEq,Debug)]
    pub enum HalfNumbers {
        Zero,
        From1To18,
        From19To36
    }
    #[derive(PartialEq,Debug)]
    pub enum Corners {
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
    impl Corners {
        fn check(&self, num: u8) -> bool {
            match num {
                1 => *self == Corners::Corner1_2_4_5,
                2 => *self == Corners::Corner1_2_4_5 || *self == Corners::Corner2_3_5_6,
                3 => *self == Corners::Corner2_3_5_6,
                4 => *self == Corners::Corner1_2_4_5 || *self == Corners::Corner4_5_7_8,
                5 => *self == Corners::Corner1_2_4_5 || *self == Corners::Corner2_3_5_6 || *self == Corners::Corner4_5_7_8,
                6 => *self == Corners::Corner2_3_5_6 || *self == Corners::Corner5_6_8_9,
                7 => *self == Corners::Corner4_5_7_8,
                8 => *self == Corners::Corner4_5_7_8 || *self == Corners::Corner5_6_8_9,
                9 => *self == Corners::Corner5_6_8_9,
                10 => *self == Corners::Corner7_8_10_11,
                11 => *self == Corners::Corner7_8_10_11 || *self == Corners::Corner8_9_11_12 || *self == Corners::Corner10_11_13_14,
                12 => *self == Corners::Corner8_9_11_12 || *self == Corners::Corner11_12_14_15,
                13 => *self == Corners::Corner10_11_13_14,
                14 => *self == Corners::Corner10_11_13_14 || *self == Corners::Corner11_12_14_15 || *self == Corners::Corner13_14_16_17,
                15 => *self == Corners::Corner11_12_14_15 || *self == Corners::Corner14_15_17_18,
                16 => *self == Corners::Corner13_14_16_17,
                17 => *self == Corners::Corner13_14_16_17 || *self == Corners::Corner14_15_17_18 || *self == Corners::Corner16_17_19_20,
                18 => *self == Corners::Corner14_15_17_18 || *self == Corners::Corner16_17_19_20,
                19 => *self == Corners::Corner16_17_19_20,
                20 => *self == Corners::Corner16_17_19_20 || *self == Corners::Corner19_20_22_23,
                21 => *self == Corners::Corner17_18_20_21,
                22 => *self == Corners::Corner19_20_22_23,
                23 => *self == Corners::Corner19_20_22_23 || *self == Corners::Corner20_21_23_24 || *self == Corners::Corner22_23_25_26,
                24 => *self == Corners::Corner20_21_23_24 || *self == Corners::Corner23_24_26_27,
                25 => *self == Corners::Corner22_23_25_26,
                26 => *self == Corners::Corner22_23_25_26 || *self == Corners::Corner23_24_26_27 || *self == Corners::Corner25_26_28_29,
                27 => *self == Corners::Corner23_24_26_27 || *self == Corners::Corner26_27_29_30,
                28 => *self == Corners::Corner25_26_28_29,
                29 => *self == Corners::Corner26_27_29_30 || *self == Corners::Corner28_29_31_32 || *self == Corners::Corner29_30_32_33,
                30 => *self == Corners::Corner26_27_29_30 || *self == Corners::Corner29_30_32_33,
                31 => *self == Corners::Corner28_29_31_32 || *self == Corners::Corner31_32_34_35,
                32 => *self == Corners::Corner29_30_32_33 || *self == Corners::Corner31_32_34_35 || *self == Corners::Corner32_33_35_36,
                33 => *self == Corners::Corner29_30_32_33 || *self == Corners::Corner32_33_35_36,
                34 => *self == Corners::Corner31_32_34_35,
                35 => *self == Corners::Corner31_32_34_35 || *self == Corners::Corner32_33_35_36,
                36 => *self == Corners::Corner32_33_35_36,
                _ => false,
            }
        }
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
            let mut test_rng = TestRandomNumberGenerator::default();
            let mut roulette_sim = Roulette::from_rng(&mut test_rng);
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
            let mut test_rng = TestRandomNumberGenerator::default();
            let mut roulette_sim = Roulette::from_rng(&mut test_rng);
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
            let mut test_rng = TestRandomNumberGenerator::default();
            let mut roulette_sim = Roulette::from_rng(&mut test_rng);
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
            let mut test_rng = TestRandomNumberGenerator::default();
            let mut roulette_sim = Roulette::from_rng(&mut test_rng);
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
            let mut test_rng = TestRandomNumberGenerator::default();
            let mut roulette_sim = Roulette::from_rng(&mut test_rng);
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
            let mut test_rng = TestRandomNumberGenerator::default();
            let mut roulette_sim = Roulette::from_rng(&mut test_rng);
            roulette_sim.bet_twelve_numbers(1,Dozens::First);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(0,returned_earnings);

            roulette_sim.bet_twelve_numbers(1,Dozens::First);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(3,returned_earnings);
        }


        #[test]
        fn test_payout_bet_columns(){
            //Define a roulette for testing
            //We pass a custom RNG for testing, if no RNG is provided it falls back to rand
            let mut test_rng = TestRandomNumberGenerator::default();
            let mut roulette_sim = Roulette::from_rng(&mut test_rng);
            roulette_sim.bet_column(1,Columns::First);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(0,returned_earnings);

            roulette_sim.bet_column(1,Columns::First);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(3,returned_earnings);
        }


        #[test]
        fn test_payout_bet_up_to_18(){
            //Define a roulette for testing
            //We pass a custom RNG for testing, if no RNG is provided it falls back to rand
            let mut test_rng = TestRandomNumberGenerator::default();
            let mut roulette_sim = Roulette::from_rng(&mut test_rng);
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
            let mut test_rng = TestRandomNumberGenerator::default();
            let mut roulette_sim = Roulette::from_rng(&mut test_rng);
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
            let mut test_rng = TestRandomNumberGenerator::default();
            let mut roulette_sim = Roulette::from_rng(&mut test_rng);
            roulette_sim.bet_parity(1,Parity::Odd);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(0,returned_earnings);

            roulette_sim.bet_parity(1,Parity::Odd);
            let (returned_earnings,spin_information) = roulette_sim.simulate();

            assert_eq!(2,returned_earnings);
        }
    }
}