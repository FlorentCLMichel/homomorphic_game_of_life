pub use concrete_boolean::{ gen_keys, server_key::ServerKey, ciphertext::Ciphertext };

/// add one encrypted bit `a` to the encrypted binary representation `b` of a 3-bit number, with 8
/// identified with 0
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

/// sum the encrypted bits in `elements`, starting from an encrypted 3-bit representation of 0
fn sum(server_key: &ServerKey, elements: &Vec<&Ciphertext>, zeros: &(Ciphertext, Ciphertext, Ciphertext)) 
    -> (Ciphertext, Ciphertext, Ciphertext) 
{
    let mut result = add_1(server_key, elements[0], zeros);
    for i in 1..elements.len() {
        result = add_1(server_key, elements[i], &result);
    }
    result
}

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

pub struct Board {
    dimensions: (usize, usize),
    pub states: Vec<Ciphertext>
}

impl Board {

    pub fn new(n_cols: usize, states: Vec<Ciphertext>) -> Board {
        let n_rows = states.len() / n_cols;
        Board { dimensions: (n_rows, n_cols), states }
    }

    pub fn evolve(&mut self, server_key: &ServerKey, zeros: &(Ciphertext, Ciphertext, Ciphertext)) {
        
        let mut new_states = Vec::<Ciphertext>::new();

        let nx = self.dimensions.0;
        let ny = self.dimensions.1;
        for i in 0..nx {
            let im = if i == 0 { nx-1 } else { i-1 };
            let ip = if i == nx-1 { 0 } else { i+1 };
            for j in 0..ny {
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
                new_states.push(is_alive(server_key, &self.states[i*ny+j], 
                                         &vec![n1,n2,n3,n4,n5,n6,n7,n8], zeros));

            }
        }

        // update the board
        self.states = new_states;
    }
}
