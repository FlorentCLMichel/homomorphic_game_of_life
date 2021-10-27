use homomorphic_game_of_life::*;
use std::sync::Mutex;

fn main() {
   
    // number of times the server key is duplicated
    let n_sk = 16;

    // read the dimensions and initial state
    let (dimensions, initial_state): ((usize, usize), Vec<bool>) 
                                       = read_csv("initial_state.csv").unwrap();

    // read the cofig file
    let config = Config::read("Config.csv").unwrap();

    // set-up the graphical interface
    let cb = ggez::ContextBuilder::new("Game of Life", "FM")
                 .window_setup(conf::WindowSetup::default().title("Conway's Game of Life"))
                 .window_mode(conf::WindowMode::default().dimensions(
                         (config.pixel_size*dimensions.1) as f32, 
                         (config.pixel_size*dimensions.0) as f32));
    let (ctx, event_loop) = cb.build().unwrap();
 
    // generate the keys
    let (client_key, server_key) = gen_keys();

    // encrypt false three times
    let zeros = (client_key.encrypt(false), client_key.encrypt(false), client_key.encrypt(false));

    // encrypt the initial configuration
    let initial_state: Vec<Ciphertext> = initial_state.into_iter().map(|x| client_key.encrypt(x)).collect();

    // build the board
    let board = Board::new(dimensions.1, initial_state);

    // create a vector of server keys for multithreading
    let mut server_keys = Vec::<Mutex<ServerKey>>::new(); 
    for _ in 0..n_sk {
        server_keys.push(Mutex::new(server_key.clone()));
    }
    
    // run the simulation
    let state = MainState::new(board, &config, server_keys, zeros, client_key).unwrap();
    event::run(ctx, event_loop, state)
}
