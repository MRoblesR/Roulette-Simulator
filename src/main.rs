use  roulette_simulator::roulette_simulator::{Roulette, SpinInformation};
use roulette_simulator::RNG::DefaultRandomNumberGenerator;
use roulette_simulator::roulette_simulator::{Columns,Dozens,Corners,Nums,Colors,Streets,Splits};
use std::io::Split;
use std::{thread, time::Duration};
use colour::{green_ln,red_ln};
fn main() {
    let mut current_turn=0;
    let mut wins=0;
    while current_turn<10000 {
        println!("Now running turn {}",current_turn);
        println!("┬");
        println!("├─Starting with 1000€");
        if strategy(){
            wins+=1;
        }
        current_turn+=1;
    }
    println!("Has ganado un total de {} rondas",wins)     
}

fn print_logs(wallet_money:usize, winning:usize,spin_information: SpinInformation){
    let mut str = String::new();
    str+="├";
    for _ in 1..wallet_money/25{
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
}


fn strategy() ->bool{
    let mut r = Roulette::default();

    let mut wallet_money=405; // The money you start with, the lib only handles the spin, you must manage the wallets
    let objective = 600;// The amount you aim to earn

    while wallet_money>400 {
        wallet_money-=400;

        //bet whatever you want
        r.bet_twelve_numbers(100, Dozens::First);
        r.bet_twelve_numbers(100, Dozens::Second);

        r.bet_column(100, Columns::First);
        r.bet_column(100, Columns::Second);
        
        //Uncomment this line to make it run slower
        //thread::sleep(Duration::from_millis(25)); 

        //Check the results
        let (winning, spin_information) = r.simulate();
        
        //Update the wallet
        wallet_money+=winning;

        //print_logs(wallet_money, winning, spin_information);

        //Check if the objetive has been reached
        if wallet_money>objective {
            return true;
        }

    }
    //Return false in case you lost all the money
    return false;
}
