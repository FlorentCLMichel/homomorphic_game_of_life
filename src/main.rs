use homomorphic_game_of_life::*;

fn main() {

    // numbers of rows and columns in the board
    let (n_rows, n_cols): (usize, usize) = (6,6);

    // generate the keys
    let (client_key, server_key) = gen_keys();

    // encrypt false three times
    let zeros = (client_key.encrypt(false), client_key.encrypt(false), client_key.encrypt(false));

    // initial configuration
    let states = vec![true, false, false, false, false, false,
                      false, true, true, false, false, false,
                      true, true, false, false, false, false,
                      false, false, false, false, false, false,
                      false, false, false, false, false, false,
                      false, false, false, false, false, false];

    // encrypt the initial configuration
    let states: Vec<Ciphertext> = states.into_iter().map(|x| client_key.encrypt(x)).collect();

    // build the board
    let mut board = Board::new(n_cols, states);
    
    loop {

        // show the board
        for i in 0..n_rows {
            println!("");
            for j in 0..n_rows {
                if client_key.decrypt(&board.states[i*n_cols+j]) {
                    print!("█");
                } else {
                    print!("░");
                }
            }
        }
        println!("");
        
        // increase the time step
        board.evolve(&server_key, &zeros);
    }
}
