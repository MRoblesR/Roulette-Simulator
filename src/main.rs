use  roulette_simulator::roulette_simulator::{Roulette, SpinInformation};
use roulette_simulator::RNG::DefaultRandomNumberGenerator;
use roulette_simulator::roulette_simulator::{Columns,Dozens,Corners,Nums};
use std::{thread, time::Duration};
use colour::{green_ln,red_ln};
fn main() {

    prueba();
        
}


fn prueba(){
    let mut current_turn=0;
    loop {
        println!("Now running turn {}",current_turn);
        println!("┬");
        println!("├─Starting with 1000€");
        if huerfanos_attempt(){
            break;
        }
        current_turn+=1;
    }
    print!("Te ha llevado un total de {} turnos", current_turn);
}
fn huerfanos_attempt() ->bool{
    let mut rng = DefaultRandomNumberGenerator::default();
    let mut r = Roulette::from_rng(&mut rng);

    let mut wallet_money=1000;
    while wallet_money>10 {
        wallet_money-=8;
        r.bet_one_number(1, Nums::Only1);
        r.bet_one_number(1, Nums::Only6);
        r.bet_one_number(1, Nums::Only9);
        r.bet_one_number(1, Nums::Only14);
        r.bet_one_number(1, Nums::Only17);
        r.bet_one_number(1, Nums::Only20);
        r.bet_one_number(1, Nums::Only31);
        r.bet_one_number(1, Nums::Only34);

        thread::sleep(Duration::from_millis(100)); 
        let (winning, spin_information) = r.simulate();
        
        wallet_money+=winning;

        
        let mut str = String::new();
        str+="├";
        for _ in 1..wallet_money/100{
            str+="─";
        }
        str+=wallet_money.to_string().as_str();
        str+="€";
        if winning>0{
            str+=", you earned ";
            str+=winning.to_string().as_str();
            green_ln!("{}",str);
        }else{
            red_ln!("{}",str);
        }
        

        if (wallet_money>2000){
            return true;
        }

    }
    return false;
}

fn MGattempt() ->bool{
    let mut rng = DefaultRandomNumberGenerator::default();
    let mut r = Roulette::from_rng(&mut rng);

    let mut wallet_money=1000;
    let mut multiplier=1;
    while wallet_money>8*multiplier {
        wallet_money-=8*multiplier;
        r.bet_twelve_numbers(3*multiplier, Dozens::First);
        r.bet_twelve_numbers(3*multiplier, Dozens::Second);

        r.bet_four_numbers(1*multiplier, Corners::Corner26_27_29_30);
        r.bet_four_numbers(1*multiplier, Corners::Corner31_32_34_35);

        //thread::sleep(Duration::from_millis(1000)); 
        let (winning, spin_information) = r.simulate();
        
        wallet_money+=winning;
        if winning>0{
            //println!("├─{}€, you earned {}",wallet_money,winning);
            multiplier=1;
        }else{
            //println!("├─{}€ - {}",wallet_money,spin_information.num);
            multiplier*=4;
        }

        if wallet_money>2000 {
            return true;
        }

    }
    return false;
}

fn attempt() ->bool{
    let mut rng = DefaultRandomNumberGenerator::default();
    let mut r = Roulette::from_rng(&mut rng);

    let mut wallet_money=1000;
    while wallet_money>10 {
        wallet_money-=8;
        r.bet_twelve_numbers(3, Dozens::First);
        r.bet_twelve_numbers(3, Dozens::Second);

        r.bet_four_numbers(1, Corners::Corner26_27_29_30);
        r.bet_four_numbers(1, Corners::Corner31_32_34_35);

        //thread::sleep(Duration::from_millis(100)); 
        let (winning, spin_information) = r.simulate();
        
        wallet_money+=winning;
        if winning>0{
            //println!("├─{}€, you earned {}",wallet_money,winning);
        }else{
            //println!("├─{}€ - {}",wallet_money,spin_information.num);
        }

        if (wallet_money>2000){
            return true;
        }

    }
    return false;
}