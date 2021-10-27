pub use concrete_boolean::{ gen_keys, server_key::ServerKey, ciphertext::Ciphertext,
                                      client_key::ClientKey };
use rayon::prelude::*;

mod graph;
pub use graph::*;

struct SendPtr (*const ServerKey);
unsafe impl Sync for SendPtr {}
unsafe impl Send for SendPtr {}

// add one encrypted bit `a` to the encrypted binary representation `b` of a 3-bit number, with 8
// identified with 0
fn add_1(server_key: &ServerKey, a: &Ciphertext, b: &(Ciphertext, Ciphertext, Ciphertext)) 
    -> (Ciphertext, Ciphertext, Ciphertext)
{
    let c1 = server_key.xor(a,&b.0);
    let r = server_key.and(a,&b.0);
    let c2 = server_key.xor(&r,&b.1);
    let r = server_key.and(&r,&b.1);
    let c3 = server_key.xor(&r,&b.2);
    (c1, c2, c3)
}

// sum the encrypted bits in `elements`, starting from an encrypted 3-bit representation of 0
fn sum(server_key: &ServerKey, elements: &Vec<&Ciphertext>, zeros: &(Ciphertext, Ciphertext, Ciphertext)) 
    -> (Ciphertext, Ciphertext, Ciphertext) 
{
    let mut result = add_1(server_key, elements[0], zeros);
    for i in 1..elements.len() {
        result = add_1(server_key, elements[i], &result);
    }
    result
}

// check if a cell will be alive after the update
fn is_alive(server_key: &ServerKey, cell: &Ciphertext, neighbours: &Vec<&Ciphertext>, 
            zeros: &(Ciphertext, Ciphertext, Ciphertext))
    -> Ciphertext 
{
    let sum_neighbours = sum(server_key, neighbours, zeros);
    let sum_is_2_or_3 = server_key.and(&sum_neighbours.1, &server_key.not(&sum_neighbours.2));
    let sum_is_3 = server_key.and(&sum_neighbours.0, &server_key.and(&sum_neighbours.1, 
                                                      &server_key.not(&sum_neighbours.2)));
    server_key.or(&sum_is_3, &server_key.and(cell, &sum_is_2_or_3))
}

/// a Game of Life board structure
///
/// # Fields
///
/// * `dimensions`: the dimensions of the board
/// * `states`: encrypted states of the cells
pub struct Board {
    dimensions: (usize, usize),
    pub states: Vec<Ciphertext>
}

impl Board {

    /// create a new board
    ///
    /// # Arguments
    ///
    /// * `n_cols`: number of columns 
    /// * `states`: encrypted states of the cells in the initial configuration
    ///
    /// # Example
    ///
    /// ```
    /// use homomorphic_game_of_life::*;
    ///
    /// // numbers of rows and columns in the board
    /// let (n_rows, n_cols): (usize, usize) = (6,6);
    ///
    /// // generate the client key
    /// let (client_key, _) = gen_keys();
    ///
    /// // initial configuration
    /// let states = vec![true, false, false, false, false, false,
    ///                   false, true, true, false, false, false,
    ///                   true, true, false, false, false, false,
    ///                   false, false, false, false, false, false,
    ///                   false, false, false, false, false, false,
    ///                   false, false, false, false, false, false];
    ///
    /// // encrypt the initial configuration
    /// let states: Vec<Ciphertext> = states.into_iter().map(|x| client_key.encrypt(x)).collect();
    ///
    /// // build the board
    /// let mut board = Board::new(n_cols, states);
    /// ```
    pub fn new(n_cols: usize, states: Vec<Ciphertext>) -> Board {
        let n_rows = states.len() / n_cols;
        Board { dimensions: (n_rows, n_cols), states }
    }

    /// update the board
    ///
    /// # Arguments
    ///
    /// * `server_key`: the server key
    /// * `zeros`: three encryption of `false`
    ///
    /// # Example
    ///
    /// ```
    /// use homomorphic_game_of_life::*;
    ///
    /// // numbers of rows and columns in the board
    /// let (n_rows, n_cols): (usize, usize) = (6,6);
    ///
    /// // generate the keys
    /// let (client_key, server_key) = gen_keys();
    ///
    /// // encrypt false three times
    /// let zeros = (client_key.encrypt(false), client_key.encrypt(false), client_key.encrypt(false));
    ///
    /// // initial configuration
    /// let states = vec![true, false, false, false, false, false,
    ///                   false, true, true, false, false, false,
    ///                   true, true, false, false, false, false,
    ///                   false, false, false, false, false, false,
    ///                   false, false, false, false, false, false,
    ///                   false, false, false, false, false, false];
    ///
    /// // encrypt the initial configuration
    /// let states: Vec<Ciphertext> = states.into_iter().map(|x| client_key.encrypt(x)).collect();
    ///
    /// // build the board
    /// let mut board = Board::new(n_cols, states);
    /// 
    /// // update the board
    /// board.update(&server_key, &zeros);
    ///
    /// // decrypt and show the board
    /// for i in 0..n_rows {
    ///     println!("");
    ///     for j in 0..n_rows {
    ///         if client_key.decrypt(&board.states[i*n_cols+j]) {
    ///             print!("█");
    ///         } else {
    ///             print!("░");
    ///         }
    ///     }
    /// }
    /// println!("");
    /// ```
    pub fn update(&mut self, server_keys: &Vec<std::sync::Mutex<ServerKey>>, zeros: &(Ciphertext, Ciphertext, Ciphertext)) 
    {
        
        let nx = self.dimensions.0;
        let ny = self.dimensions.1;
        
        let new_states = (0..nx*ny).into_par_iter().map( |k| {

            let i = k / ny;
            let j = k % ny;

            let im = if i == 0 { nx-1 } else { i-1 };
            let ip = if i == nx-1 { 0 } else { i+1 };
            let jm = if j == 0 { ny-1 } else { j-1 };
            let jp = if j == ny-1 { 0 } else { j+1 };

            // get the neighbours, with periodic boundary conditions
            let n1 = &self.states[im*ny+jm];
            let n2 = &self.states[im*ny+j];
            let n3 = &self.states[im*ny+jp];
            let n4 = &self.states[i*ny+jm];
            let n5 = &self.states[i*ny+jp];
            let n6 = &self.states[ip*ny+jm];
            let n7 = &self.states[ip*ny+j];
            let n8 = &self.states[ip*ny+jp];

            // see if the cell is alive of dead
            let mut k = i;
            let mut sk = server_keys[k % server_keys.len()].try_lock();
            while let Err(_) = sk {
                k += 1;
                sk = server_keys[k % server_keys.len()].try_lock();
            }
            is_alive(&sk.unwrap(), 
                     &self.states[i*ny+j], 
                     &vec![n1,n2,n3,n4,n5,n6,n7,n8], zeros)
            
        }).collect();

        // update the board
        self.states = new_states;
    }
}


// config structure
pub struct Config {
    pub wait_time_micros: u64,
    pub pixel_size: usize,
    pub dimensions: (usize, usize),
    pub col1: (f32,f32,f32),
    pub col2: (f32,f32,f32),
}

impl Config {
    pub fn read(fname: &str) -> Result<Config, Box<dyn std::error::Error>> {

        let err_message = "Missing argument in the config file";
    
        // load the content
        let content = std::fs::read_to_string(fname)?;

        // separate the nubers
        let mut content = content.split(' ');
    
        // read the content
        Ok(Config {
            wait_time_micros: content.next().ok_or(err_message)?.parse::<u64>()?,
            pixel_size: content.next().ok_or(err_message)?.parse::<usize>()?,
            dimensions : (
                content.next().ok_or(err_message)?.parse::<usize>()?,
                content.next().ok_or(err_message)?.parse::<usize>()?),
            col1 : (
                content.next().ok_or(err_message)?.parse::<f32>()?,
                content.next().ok_or(err_message)?.parse::<f32>()?,
                content.next().ok_or(err_message)?.parse::<f32>()?),
            col2 : (
                content.next().ok_or(err_message)?.parse::<f32>()?,
                content.next().ok_or(err_message)?.parse::<f32>()?,
                content.next().ok_or(err_message)?.parse::<f32>()?),
        })

    }
}


/// read a state file, space- and newline-separated
///
/// The file must contain only 0s and 1s and all rows need to have the same length.
pub fn read_csv(fname: &str) 
    -> Result<((usize, usize), Vec<bool>), Box<dyn std::error::Error>> 
{
    
    // load the content
    let content = std::fs::read_to_string(fname)?;

    // separate in rows
    let content = content.split('\n').collect::<Vec<&str>>();
    let n_rows = content.len() - 1;

    // number of columns
    let n_cols = content[0].split(' ').collect::<Vec<&str>>().len() - 1;

    // load the data
    let mut data = Vec::<bool>::new();
    for row in content {
        for el in row.split(' ') {
            if let Ok(x) = el.parse::<u8>() { data.push(x == 1) };
        }
    }
   
    Ok(((n_rows, n_cols), data))
}
