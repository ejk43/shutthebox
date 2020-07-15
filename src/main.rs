pub mod game;

fn main() {
    let mut won = false;
    let mut niter = 0;
    while !won {
        let mut game = game::ShutTheBox::init(12);
        println!("Hello, world!");
        println!("My Game: {:?}", game);

        let mut dice = game::Dice::new();
        let mut valid = true;
        while valid && !game.victory() {
            dice.roll();
            valid = game.play_value(dice.result());
            println!("Roll: {:?} = {}", dice.values, dice.result());
            println!("Result? {} / Update: {:?}", valid, game);
        }
        won = game.victory();
        niter += 1;
        println!("GAME OVER! Try number {}", niter);
    }
}
