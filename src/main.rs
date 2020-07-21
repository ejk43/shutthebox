pub mod game;

fn play_until_victory() {
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

fn accumulate_stats(total: i64) {
    let mut stats = game::Statistics::new();
    for _ in 0..total {
        let game = game::simulate_game();
        stats.save_game(&game);
        // println!("One Game: {:?}", game);
    }
    println!("Stats: {:?}", stats);
    for pct in (1..100).step_by(5) {
        println!(
            "{}'th percentile of data is {}",
            pct,
            stats
                .games_between_win
                .value_at_quantile((pct as f64) / 100.0)
        );
    }
    let mut sum = 0;
    for bounds in (0..2450).step_by(50).zip((50..2500).step_by(50)) {
        let count = stats
            .games_between_win
            .count_between(bounds.0, bounds.1 - 1);
        println!(
            "Amount between {} and {}: {}",
            bounds.0,
            bounds.1 - 1,
            count
        );
        sum += count;
    }
    println!("total: {}", sum);
}

fn main() {
    // play_until_victory();
    accumulate_stats(200_000);
}
