use std::fs::File;
use std::io::prelude::*;

pub enum History
{
    Spotless=0,
    WarningIssued=1,
    ErrorMessageIssued=2,
    FatalErrorStop=3,
}

enum EquivRegion2Code
{
	ActiveBase,
	SingleBase,
	NullCs,
	HashBase,
	FrozenControlSequence,
	FrozenProtection,
	FrozenCr,
	FrozenEndGroup,
	FrozenRight,
	FrozenFi,
	FrozenEndTemplate,
	FrozenEndv,
	FrozenRelax,
	EndWrite,
	FrozenDontExpand,
	FrozenNullFont,
	FontIdBase,
	UndefinedControlSequence,
	GlueBase,
}

impl EquivRegion2Code {
    fn value(&self) -> i16 {
        match self {
            ActiveBase => 1,
            SingleBase => EquivRegion2Code::ActiveBase.value() + 256,
            NullCs => EquivRegion2Code::SingleBase.value() + 256,
            HashBase => EquivRegion2Code::NullCs.value() + 1,
            FrozenControlSequence => EquivRegion2Code::HashBase.value() + TexState::hash_size as i16,
            FrozenProtection => EquivRegion2Code::FrozenControlSequence.value(),
            FrozenCr => EquivRegion2Code::FrozenControlSequence.value() + 1,
            FrozenEndGroup => EquivRegion2Code::FrozenControlSequence.value() + 2,
            FrozenRight => EquivRegion2Code::FrozenControlSequence.value() + 3,
            FrozenFi => EquivRegion2Code::FrozenControlSequence.value() + 4,
            FrozenEndTemplate => EquivRegion2Code::FrozenControlSequence.value() + 5,
            FrozenEndv => EquivRegion2Code::FrozenControlSequence.value() + 6,
            FrozenRelax => EquivRegion2Code::FrozenControlSequence.value() + 7,
            EndWrite => EquivRegion2Code::FrozenControlSequence.value() + 8,
            FrozenDontExpand => EquivRegion2Code::FrozenControlSequence.value() + 9,
            FrozenNullFont => EquivRegion2Code::FrozenControlSequence.value() + 10,
            FontIdBase => EquivRegion2Code::FrozenNullFont.value() - TexState::font_base as i16,
            UndefinedControlSequence => EquivRegion2Code::FrozenNullFont.value() + 257,
            GlueBase => EquivRegion2Code::UndefinedControlSequence.value() + 1,
        }
    }
}

pub enum TokenList
{
    CsTokenFlag,
    LeftBraceToken,
    LeftBraceLimit,
    RightBraceToken,
    RightBraceLimit,
    MathShiftToken,
    TabToken,
    OutParamToken,
    SpaceToken,
    LetterToken,
    OtherToken,
    MatchToken,
    EndMatchToken,
}

impl TokenList {
    fn value(&self) -> i32 {
        match self {
            TokenList::CsTokenFlag => 0o7777,
            TokenList::LeftBraceToken =>  0o400,
            TokenList::LeftBraceLimit => 0o1000,
            TokenList::RightBraceToken => 0o1000,
            TokenList::RightBraceLimit => 0o1400,
            TokenList::MathShiftToken => 0o1400,
            TokenList::TabToken => 0o2000,
            TokenList::OutParamToken => 0o2400,
            TokenList::SpaceToken => 0o5040,
            TokenList::LetterToken => 0o5400,
            TokenList::OtherToken => 0o6000,
            TokenList::MatchToken => 0o6400,
            TokenList::EndMatchToken => 0o7000,
        }
    }
}

pub struct TexState
{
    ready_already: i32,
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

    const mem_bot: i32 = 0;
    const mem_top: i32 = 30000;
    const font_base: i32 = 0;
    const hash_size: i32 = 2100;
    const hash_prime: i32 = 1777;
    const hyph_size: i32 = 307;

    const min_quarterword: i32 = 0;
    const max_quarterword: i32 = 255;
    const min_halfword: i32 = 0;
    const max_halfword: i32 = 65535;

    const format_default_length: i32 = 9;
    const format_area_length: i32 = 0;
    const format_ext_length: i32 = 4;
    
    pub fn new() -> TexState
    {
        return TexState {
            ready_already: 0,
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


fn wterm(text:&str)
{
    print!("{}", text);
}

fn wterm_ln(text:&str)
{
    println!("{}", text);
}

fn wterm_cr(text:&str)
{
    println!("{}", text);
}
/*
@d wlog(#)==write(log_file,#)
@d wlog_ln(#)==write_ln(log_file,#)
@d wlog_cr==write_ln(log_file)
*/



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

    let state = TexState::new();

    let history = History::FatalErrorStop;

    // check consts
    if state.ready_already != 314159 {
		let mut bad = 0;
        if TexState::half_error_line<30 || TexState::half_error_line > TexState::error_line - 15
        { bad = 1; }

        if TexState::max_print_line < 60 { bad = 2; }
        if TexState::dvi_buf_size % 8 != 0 { bad = 3; }
        if TexState::mem_bot + 1100 > TexState::mem_top { bad = 4; }
        if TexState::hash_prime > TexState::hash_size { bad = 5; }
        if TexState::max_in_open >= 128 { bad = 6; }
        if TexState::mem_top < 256 + 11 { bad = 7; }

        #[cfg(init)]
        {
            if TexState::mem_min != TexState::mem_bot
            || TexState::mem_max != TexState::mem_top
            {
                bad = 10;
            }
        }
        

        if TexState::mem_min > TexState::mem_bot || TexState::mem_max < TexState::mem_top { bad = 10; }
        if TexState::min_quarterword > 0 || TexState::max_quarterword < 127 { bad = 11; }
        if TexState::min_halfword > 0 || TexState::max_halfword < 32767 { bad = 12; }
        if TexState::min_quarterword < TexState::min_halfword || TexState::max_quarterword > TexState::max_halfword { bad = 13; }
        if TexState::mem_min < TexState::min_halfword || TexState::mem_max >= TexState::max_halfword ||
            TexState::mem_bot - TexState::mem_min > TexState::max_halfword + 1
            {
                bad = 14;
            }
        if TexState::font_base < TexState::min_quarterword || TexState::font_max > TexState::max_quarterword { bad = 15; }
        if TexState::font_max > TexState::font_base + 256 { bad = 16; }
        if TexState::save_size > TexState::max_halfword { bad = 18; }
        if TexState::max_quarterword - TexState::min_quarterword < 255 { bad = 19; }

        //290
        if TokenList::CsTokenFlag.value() + EquivRegion2Code::UndefinedControlSequence.value() as i32 > TexState::max_halfword
        {
            bad = 21;
        }

        // 522
        if TexState::format_default_length > TexState::file_name_size { bad = 31; }

        // 1249
        if 2 * TexState::max_halfword < TexState::mem_top - TexState::mem_min { bad = 41; }

        if bad > 0 {
            wterm_ln(&format!("Ouch---my internal constants have been clobbered!\n---case {}\n", bad));
        }
        
        /* init */
        initialize(&state);
	}

    
}
