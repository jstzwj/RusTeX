use std::process;
use std::collections::HashMap;
use std::char;


// 73
pub enum Interaction
{
    BatchMode,
    NonstopMode,
    ScrollMode,
    ErrorStopMode,
}

impl Interaction {
    fn value(&self) -> i32 {
        match self {
            Interaction::BatchMode => 0,
            Interaction::NonstopMode => 1,
            Interaction::ScrollMode => 2,
            Interaction::ErrorStopMode => 3,
        }
    }
}

pub enum History
{
    Spotless,
    WarningIssued,
    ErrorMessageIssued,
    FatalErrorStop,
}


impl History {
    fn value(&self) -> i32 {
        match self {
            History::Spotless => 0,
            History::WarningIssued => 1,
            History::ErrorMessageIssued => 2,
            History::FatalErrorStop => 3,
        }
    }
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

// 113
type QuarterWord = u8;
type HalfWord = u16;
type TwoChoices = u8;
type FourChoices = u8;


type StrNumber = i32;

pub struct TexState
{
    ready_already: i32,

    // 76
    deletions_allowed: bool,
    set_box_allowed: bool,
    history: History,
    error_count: i32,

    // 79
    helpline:[StrNumber;6],
    help_ptr: u8, // 0..6
    use_err_help: bool,

    // 96
    interrupt: i32,
    OK_to_interrupt: bool,

    mem_bot: i32,
    mem_top: i32,
    font_base: i32,
    hash_size: i32,
    hash_prime: i32,
    hyph_size: i32,

    xord: [u8;256],
    xchr: [u8;256],

    // 73
    interaction: Interaction,

    // 165
    // debug
    _free: [bool;TexState::mem_max],
    was_free: [bool;TexState::mem_max],
    was_mem_end: HalfWord,
    was_lo_max: HalfWord,
    was_hi_min: HalfWord,
    panicking: bool,
    
}

impl TexState
{
    const mem_max: usize = 30000;
    const mem_min: usize = 0;
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

    const mem_bot: usize = 0;
    const mem_top: usize = 30000;
    const font_base: i32 = 0;
    const hash_size: i32 = 2100;
    const hash_prime: i32 = 1777;
    const hyph_size: i32 = 307;

    const min_quarterword: QuarterWord = 0;
    const max_quarterword: QuarterWord = 255;
    const min_halfword: HalfWord = 0;
    const max_halfword: HalfWord = 65535;

    const format_default_length: i32 = 9;
    const format_area_length: i32 = 0;
    const format_ext_length: i32 = 4;
    
    pub fn new() -> TexState
    {
        return TexState {
            ready_already: 0,
            // 76
            deletions_allowed: false,
            set_box_allowed: false,
            history: History::FatalErrorStop,
            error_count: 0,

            // 79
            helpline:[0;6],
            help_ptr: 0,
            use_err_help: false,

            // 96
            interrupt: 0,
            OK_to_interrupt: false,

            mem_bot: 0,
            mem_top: 30000,
            font_base: 0,
            hash_size: 2100,
            hash_prime: 1777,
            hyph_size: 307,

            xord: [0;256],
            xchr: [0;256],

            interaction: Interaction::BatchMode,

            // 165
            // debug
            _free: [false;TexState::mem_max],
            was_free: [false;TexState::mem_max],
            was_mem_end: 0,
            was_lo_max: 0,
            was_hi_min: 0,
            panicking: false,
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



fn initialize(state: &mut TexState) {
    
    // 21
    state.xchr[32] = ' ' as u8;
    state.xchr[33] = '!' as u8;   state.xchr[34] = '"' as u8;   state.xchr[35] = '#' as u8;   state.xchr[36] = '$' as u8;  state.xchr[37] = '%' as u8;
    state.xchr[38] = '&' as u8;   state.xchr[39] = '\'' as u8;  state.xchr[40] = '(' as u8;   state.xchr[41] = ')' as u8;  state.xchr[42] = '*' as u8;
    state.xchr[43] = '+' as u8;   state.xchr[44] = ',' as u8;   state.xchr[45] = '-' as u8;   state.xchr[46] = '.' as u8;  state.xchr[47] = '/' as u8;
    state.xchr[48] = '0' as u8;   state.xchr[49] = '1' as u8;   state.xchr[50] = '2' as u8;   state.xchr[51] = '3' as u8;  state.xchr[52] = '4' as u8;
    state.xchr[53] = '5' as u8;   state.xchr[54] = '6' as u8;   state.xchr[55] = '7' as u8;   state.xchr[56] = '8' as u8;  state.xchr[57] = '9' as u8;
    state.xchr[58] = ':' as u8;   state.xchr[59] = ';' as u8;   state.xchr[60] = '<' as u8;   state.xchr[61] = '=' as u8;  state.xchr[62] = '>' as u8;
    state.xchr[63] = '?' as u8;   state.xchr[64] = '@' as u8;   state.xchr[65] = 'A' as u8;   state.xchr[66] = 'B' as u8;  state.xchr[67] = 'C' as u8;
    state.xchr[68] = 'D' as u8;   state.xchr[69] = 'E' as u8;   state.xchr[70] = 'F' as u8;   state.xchr[71] = 'G' as u8;  state.xchr[72] = 'H' as u8;
    state.xchr[73] = 'I' as u8;   state.xchr[74] = 'J' as u8;   state.xchr[75] = 'K' as u8;   state.xchr[76] = 'L' as u8;  state.xchr[77] = 'M' as u8;
    state.xchr[78] = 'N' as u8;   state.xchr[79] = 'O' as u8;   state.xchr[80] = 'P' as u8;   state.xchr[81] = 'Q' as u8;  state.xchr[82] = 'R' as u8;
    state.xchr[83] = 'S' as u8;   state.xchr[84] = 'T' as u8;   state.xchr[85] = 'U' as u8;   state.xchr[86] = 'V' as u8;  state.xchr[87] = 'W' as u8;
    state.xchr[88] = 'X' as u8;   state.xchr[89] = 'Y' as u8;   state.xchr[90] = 'Z' as u8;   state.xchr[91] = '[' as u8;  state.xchr[92] = '\\' as u8;
    state.xchr[93] = ']' as u8;   state.xchr[94] = '^' as u8;   state.xchr[95] = '_' as u8;   state.xchr[96] = '`' as u8;  state.xchr[97] = 'a' as u8;
    state.xchr[98] = 'b' as u8;   state.xchr[99] = 'c' as u8;   state.xchr[100] = 'd' as u8;  state.xchr[101] = 'e' as u8;
    state.xchr[102] = 'f' as u8;  state.xchr[103] = 'g' as u8;  state.xchr[104] = 'h' as u8;  state.xchr[105] = 'i' as u8;
    state.xchr[106] = 'j' as u8;  state.xchr[107] = 'k' as u8;  state.xchr[108] = 'l' as u8;  state.xchr[109] = 'm' as u8;
    state.xchr[110] = 'n' as u8;  state.xchr[111] = 'o' as u8;  state.xchr[112] = 'p' as u8;  state.xchr[113] = 'q' as u8;
    state.xchr[114] = 'r' as u8;  state.xchr[115] = 's' as u8;  state.xchr[116] = 't' as u8;  state.xchr[117] = 'u' as u8;
    state.xchr[118] = 'v' as u8;  state.xchr[119] = 'w' as u8;  state.xchr[120] = 'x' as u8;  state.xchr[121] = 'y' as u8;
    state.xchr[122] = 'z' as u8;  state.xchr[123] = '{' as u8;  state.xchr[124] = '|' as u8;  state.xchr[125] = '}' as u8;
    state.xchr[126] = '~' as u8;
    // 23
    for i in 0..=31 { state.xchr[i] = ' ' as u8; }
    for i in 127..=255 { state.xchr[i] = ' ' as u8; }

    // 24
    for i in 0..=255 { state.xord[i] = 127; }
    for i in 128..=255 { state.xord[state.xchr[i] as usize] = i as u8; }
    for i in 0..=126 { state.xord[state.xchr[i] as usize] = i as u8; }
    // 74
    state.interaction = Interaction::ErrorStopMode;
    // 77
    state.deletions_allowed = true;
    state.set_box_allowed = true;
    state.error_count = 0;
    // 80
	state.help_ptr = 0;
	state.use_err_help = false;

	// 97
	state.interrupt = 0;
	state.OK_to_interrupt = true;

    // 166
    #[cfg(not(feature = "debug"))]
    {
        state.was_mem_end = TexState::mem_min as HalfWord;
        state.was_lo_max = TexState::mem_min as HalfWord;
        state.was_hi_min = TexState::mem_max as HalfWord;
        state.panicking = false;
    }
    
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

fn do_final_end(state:&mut TexState) -> i32
{
	state.ready_already = 0;

    match state.history.value() <= History::WarningIssued.value()
    {
        true => process::exit(0),
        false => process::exit(1),
    }
}


fn main() {
    let banner = "This is TeX, Version 3.1415926"; // printed when TEX starts
    println!("{}", banner);

    let mut state = TexState::new();

    state.history = History::FatalErrorStop;

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

        // 111
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
        if (TexState::min_quarterword as HalfWord) < TexState::min_halfword 
            || TexState::max_quarterword as HalfWord > TexState::max_halfword { bad = 13; }
        if TexState::mem_min < TexState::min_halfword as usize
            || TexState::mem_max >= TexState::max_halfword as usize
            || TexState::mem_bot - TexState::mem_min > TexState::max_halfword as usize + 1
            {
                bad = 14;
            }
        if TexState::font_base < TexState::min_quarterword as i32
            || TexState::font_max > TexState::max_quarterword as i32 { bad = 15; }
        if TexState::font_max > TexState::font_base + 256 { bad = 16; }
        if TexState::save_size > TexState::max_halfword as i32 { bad = 18; }
        if TexState::max_quarterword - TexState::min_quarterword < 255 { bad = 19; }

        //290
        if TokenList::CsTokenFlag.value() + EquivRegion2Code::UndefinedControlSequence.value() as i32 > TexState::max_halfword
        {
            bad = 21;
        }

        // 522
        if TexState::format_default_length > TexState::file_name_size { bad = 31; }

        // 1249
        if (2 * TexState::max_halfword as usize) < TexState::mem_top - TexState::mem_min { bad = 41; }

        if bad > 0 {
            wterm_ln(&format!("Ouch---my internal constants have been clobbered!\n---case {}\n", bad));
            do_final_end(&mut state);
        }
        
        /* init */
        initialize(&mut state);

        #[cfg(init)]
        {
        }

        state.ready_already = 314159;
	}

    
}
