use std::fs::File;
use std::io::prelude::*;


pub struct TexState
{
    /* variables */
    mem_bot: i32,
    mem_top: i32,
    font_base: i32,
    hash_size: i32,
    hash_prime: i32,
    hyph_size: i32,
}

impl TexState
{
    const mem_max: i32 = 30000;
    const mem_min: i32 = 0;
    const buf_size: i32 = 500;
    const error_line: i32 = 72;
    const half_error_line: i32 = 42;
    const max_print_line: i32 = 79;
    const stack_size: i32 = 200;
    const max_in_open: i32 = 6;
    const font_max: i32 = 75;
    const font_mem_size: i32 = 20000;
    const param_size: i32 = 60;
    const nest_size: i32 = 40;
    const max_strings: i32 = 3000;
    const string_vacancies: i32 = 8000;
    const pool_size: i32 = 32000;
    const save_size: i32 = 600;
    const trie_size: i32 = 8000;
    const trie_op_size: i32 = 500;
    const dvi_buf_size: i32 = 800;
    const file_name_size: i32 = 40;
    const pool_name: &'static str = "TeXformats:TEX.POOL                     ";
    
    pub fn new() -> TexState
    {
        return TexState {
            /* variables */
            mem_bot: 0,
            mem_top: 30000,
            font_base: 0,
            hash_size: 2100,
            hash_prime: 1777,
            hyph_size: 307,
        }
    }
}



fn initialize(state: &TexState) {
    /* local variables */
    
    // Initialize table entries
    #[cfg(not(feature = "release"))]
    {
        /*
        for k in state.mem_bot + 1 ..  state.lo_mem_stat_max
        {

        }
        */
    }
}

fn main() {
    let banner = "This is TeX, Version 3.1415926"; // printed when TEX starts
    println!("{}", banner);

    /* label */
    let start_of_tex = 1; // go here when TEX’s variables are initialized
    let end_of_tex = 9998; // go here to close files and terminate gracefully
    let end = 9999; // this label marks the ending of the program

    let state = TexState::new();
    /* types */

    /* global variables */

    /* init */
    initialize(&state);
}
