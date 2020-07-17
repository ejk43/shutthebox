pub mod game;

fn main() {
    let mut won = false;
    let mut niter = 0;
    while !won {
        let game = game::simulate_game();
        won = game.victory();
        niter += 1;
        println!(
            "{} Try number {}",
            match won {
                true => "WON!!",
                false => "Lost :(",
            },
            niter
        );
        println!("Rolls: {:?}", game.get_rolls());
        println!("Shut: {:?}", game.get_numbers());
    }
}
