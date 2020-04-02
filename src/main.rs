use std::process;
use std::collections::HashMap;
use std::char;
use std::fs::File;
use std::marker::Copy;
use std::ops::{Index, IndexMut};
use byteorder::{ByteOrder, LittleEndian};


/* 
 * Here are types of TeX
 */
struct Array<T:Default+Clone> {
    min: usize,
    max: usize,
    data: Vec<T>,
}

impl<T:Default+Clone> Array<T> {
    pub fn new(min:usize, max:usize) -> Array<T> {
        Array {
            min: min,
            max: max,
            data: vec![Default::default(); max - min + 1],
        }
    }
}

impl<T:Default+Clone> Index<usize> for Array<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        return &self.data[self.min + i];
    }
}

impl<T:Default+Clone> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        return &mut self.data[self.min + i];
    }
}

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

// 101
const unity: i32 = 0200000;
const two: i32 = 0400000;
type Scaled = i32;
type NonnegativeInteger = i32;
type SmallNumber = i32;


// 109
type GlueRatio = f32;

// 113
type QuarterWord = u8;  // 0..255
type HalfWord = u16;    // 0..65535
type TwoChoices = u8;   // 1..2
type FourChoices = u8;  // 1..4

#[derive(Debug, Clone, Copy)]
enum TwoHalvesLeftHalf {
    OneHalfWord(HalfWord),
    TwoQuarterWord(QuarterWord, QuarterWord),
}

#[derive(Debug, Clone, Copy)]
struct TwoHalves {
    data: [u8; 4],
}

impl TwoHalves {
    pub fn new() -> TwoHalves {
        TwoHalves {
            data: [0; 4]
        }
    }

    pub fn rh(&self) -> HalfWord {
        LittleEndian::read_u16(&self.data[2..4])
    }

    pub fn set_rh(&mut self, value: HalfWord) {
        LittleEndian::write_u16(&mut self.data[2..4], value);
    }

    pub fn lh(&self) -> HalfWord {
        LittleEndian::read_u16(&self.data[0..2])
    }
}

#[derive(Debug, Clone, Copy)]
struct FourQuarters {
	b0: QuarterWord,
	b1: QuarterWord,
	b2: QuarterWord,
	b3: QuarterWord,
}

#[derive(Debug, Clone, Copy)]
struct MemoryWord {
    data: [u8; 4],
}

impl MemoryWord {
    pub fn new() -> MemoryWord {
        Default::default()
    }

    pub fn new_i32(value: i32) -> MemoryWord {
        let mut ret: MemoryWord = Default::default();
        LittleEndian::write_i32(&mut ret.data, value);
        ret
    }

    pub fn int(&self) -> i32 {
        LittleEndian::read_i32(&self.data)
    }

    pub fn gr(&self) -> GlueRatio {
        LittleEndian::read_f32(&self.data)
    }

    pub fn hh_rh(&self) -> u16 {
        LittleEndian::read_u16(&self.data[2..4])
    }

    pub fn set_hh_rh(&mut self, value: u16) {
        LittleEndian::write_u16(&mut self.data[2..4], value);
    }

    pub fn hh_lh(&self) -> u16 {
        LittleEndian::read_u16(&self.data[0..2])
    }

    pub fn set_hh_lh(&mut self, value: u16) {
        LittleEndian::write_u16(&mut self.data[0..2], value);
    }

    pub fn qqqq(&self) -> FourQuarters {
        FourQuarters {
            b0: self.data[0],
            b1: self.data[1],
            b2: self.data[2],
            b3: self.data[3],
        }
    }
}

impl Default for MemoryWord {
    fn default() -> Self {
        MemoryWord {
            data: [0; 4]
        }
    }
}

type WordFile = File;


// 212
#[derive(Debug, Clone, Default, Copy)]
struct ListStateRecord {
    mode_field: i32,
    head_field: HalfWord,
    tail_field: HalfWord,
    pg_field: i32,
    ml_field: i32,
    aux_field: MemoryWord,
}

impl ListStateRecord {
    pub fn new() -> ListStateRecord {
        ListStateRecord {
            mode_field: 0,
            head_field: 0,
            tail_field: 0,
            pg_field: 0,
            ml_field: 0,
            aux_field: Default::default(),
        }
    }
}

// 222
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
    fn value(&self) -> HalfWord {
        match self {
            ActiveBase => 1,
            SingleBase => EquivRegion2Code::ActiveBase.value() + 256,
            NullCs => EquivRegion2Code::SingleBase.value() + 256,
            HashBase => EquivRegion2Code::NullCs.value() + 1,
            FrozenControlSequence => EquivRegion2Code::HashBase.value() + TexState::hash_size as HalfWord,
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
            FontIdBase => EquivRegion2Code::FrozenNullFont.value() - TexState::font_base as HalfWord,
            UndefinedControlSequence => EquivRegion2Code::FrozenNullFont.value() + 257,
            GlueBase => EquivRegion2Code::UndefinedControlSequence.value() + 1,
        }
    }
}

// Region 3 of eqtb
// 224
enum EquivRegion3Code
{
	LineSkipCode, // interline glue if baseline_skip is infeasible
	BaselineSkipCode, // desired glue between baselines
	ParSkipCode, // extra glue just above a paragraph
	AboveDisplaySkipCode, // extra glue just above displayed math
	BelowDisplaySkipCode, // extra glue just below deisplayed math
	AboveDisplayShortSkipCode, // glue above displayed math following short lines
	BelowDisplayShortSkipCode,  // glue below displayed math following short lines
	LeftSkipCode, // glue at left of justified lines
	RightSkipCode, // glue at right of justified lines
	TopSkipCode, // glue at top of main pages
	SplitTopSkipCode, // glue at top of split pages
	TabSkipCode, // glue between aligned entries
	SpaceSkipCode, // glue between words (if not zero_glue)
	XspaceSkipCode, // glue after sentences (if not zero glue)
	ParFillSkipCode, // glue on last line of paragraph
	ThinMuSkipCode, // thin space in math formula
	MedMuSkipCode, // medium space in math formula
	ThickMuSkipCode, // thick space in math formula
}


impl EquivRegion3Code {
    fn value(&self) -> HalfWord {
        match self {
            LineSkipCode => 0,
            BaselineSkipCode => 1,
            ParSkipCode => 2,
            AboveDisplaySkipCode => 3,
            BelowDisplaySkipCode => 4,
            AboveDisplayShortSkipCode => 5,
            BelowDisplayShortSkipCode => 6,
            LeftSkipCode => 7,
            RightSkipCode => 8,
            TopSkipCode => 9,
            SplitTopSkipCode => 10,
            TabSkipCode => 11,
            SpaceSkipCode => 12,
            XspaceSkipCode => 13,
            ParFillSkipCode => 14,
            ThinMuSkipCode => 15,
            MedMuSkipCode => 16,
            ThickMuSkipCode => 17,
        }
    }
}


const glue_pars: usize = 18; // total number of glue parameters
const skip_base: usize = glue_base + glue_pars; // table of 256 ``skip'' registers
const mu_skip_base: usize = skip_base + 256; // table of 256 ``muskip'' registers
const local_base: usize = mu_skip_base + 256; // beginning of region 4


// region 4 of eqtb
// 230
enum EquivRegion4Loc
{
	ParShapeLoc, // specifies paragraph shape
	OutputRoutineLoc, // points to token list for \output
	EveryParLoc, // points to token list for \everypar
	EveryMathLoc, // points to token list for \everymath
	EveryDisplayLoc, // points to token list for \everydisplay
	EveryHboxLoc, // points to token list for \everyhbox
	EveryVboxLoc, // points to token list for \everyvbox
	EveryJobLoc, // points to token list for \everyjob
	EveryCrLoc, // points to token list for \everycr
	ErrHelpLoc, // points to token list for \errhelp
	ToksBase, // table of 256 token list registers
	BoxBase, // table of 256 box registers
	CurFontLoc, // internal font number outside math mode
}


impl EquivRegion4Loc {
    fn value(&self) -> HalfWord {
        match self {
            ParShapeLoc => local_base as HalfWord,
            OutputRoutineLoc => local_base as HalfWord + 1,
            EveryParLoc => local_base as HalfWord + 2,
            EveryMathLoc => local_base as HalfWord + 3,
            EveryDisplayLoc => local_base as HalfWord + 4,
            EveryHboxLoc => local_base as HalfWord + 5,
            EveryVboxLoc => local_base as HalfWord + 6,
            EveryJobLoc => local_base as HalfWord + 7,
            EveryCrLoc => local_base as HalfWord + 8,
            ErrHelpLoc => local_base as HalfWord + 9,
            ToksBase => local_base as HalfWord + 10,
            BoxBase => toks_base as HalfWord + 256,
            CurFontLoc => box_base as HalfWord + 256,
        }
    }
}

const math_font_base: usize = EquivRegion4Loc::CurFontLoc.value() as usize + 1; // table of 48 math font numbers
const cat_code_base: usize = math_font_base + 48; // table of 256 command codes (the ``catcodes'')
const lc_code_base: usize = cat_code_base + 256; // table of 256 lowercase mappings
const uc_code_base: usize = lc_code_base + 256; // table of 256 uppercase mappings
const sf_code_base: usize = uc_code_base + 256; // table of 256 spacefactor mappings
const math_code_base: usize = sf_code_base + 256; // table of 256 math mode mappings
const int_base: usize = math_code_base + 256; // beginning of region 5

// 236
const int_pars: usize = 55; // total number of integer parameters
const count_base: usize = int_base + int_pars; // 256 user \count registers
const del_code_base: usize = count_base + 256; // 256 delimiter code mappings
const dimen_base: usize = del_code_base + 256; // beginning of region 6

const scaled_base: usize = dimen_base + dimen_pars; // table of 256 user-defined \dimen registers

const eqtb_size: usize = scaled_base + 255; // largest subscript of eqtb






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


type StrNumber = i32;

pub struct TexState
{
    ready_already: i32,
    // 20
    xord: [u8;256],
    xchr: [u8;256],

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

    // 73
    interaction: Interaction,

    // 109
    glue_ratio: f32,

    // 116
    mem: Array<MemoryWord>, // the big dynamic storage area
    lo_mem_max: HalfWord, // the largest location of variable-size memory in use
    hi_mem_min: HalfWord, // the smallest location of one-word memory in use

    // 117
    var_used: i32,
    dyn_used: i32, // how much memory is in use

    // 165
    // debug
    _free: [bool;TexState::mem_max],
    was_free: [bool;TexState::mem_max],
    was_mem_end: HalfWord,
    was_lo_max: HalfWord,
    was_hi_min: HalfWord,
    panicking: bool,

    // 213
    nest: [ListStateRecord; TexState::nest_size],
    nest_ptr: usize, // 0..nestsize
    max_nest_stack: usize, // 0..nestsize
    curlist:ListStateRecord,
    shown_mode: i32, // -203..203

    // 253
    eqtb: Array<MemoryWord>,
    xeq_level: Array<QuarterWord>,

    // 980
    page_tail: HalfWord,
    page_contents: u8, // 0..2
    page_max_depth: Scaled,
    best_page_break: HalfWord,
    least_page_cost: i32,
    best_size: Scaled,

    // 982
    page_so_far: Array<Scaled>, // 0..7
    last_glue: HalfWord,
    last_penalty: i32,
    last_kern: Scaled,
    insert_penalties: i32,
}

impl TexState
{
    // 2
    const banner: &'static str = "This is TeX, Version 3.1415926"; // printed when TEX starts

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
    const nest_size: usize = 40;
    const max_strings: i32 = 3000;
    const string_vacancies: i32 = 8000;
    const pool_size: i32 = 32000;
    const save_size: i32 = 600;
    const trie_size: i32 = 8000;
    const trie_op_size: i32 = 500;
    const dvi_buf_size: i32 = 800;
    const file_name_size: i32 = 40;
    const pool_name: &'static str = "TeXformats:TEX.POOL                     ";

    // 12 (compiler constants)
    const mem_bot: usize = 0;
    const mem_top: usize = 30000;
    const font_base: i32 = 0;
    const hash_size: i32 = 2100;
    const hash_prime: i32 = 1777;
    const hyph_size: i32 = 307;

    // 110
    const min_quarterword: QuarterWord = 0;
    const max_quarterword: QuarterWord = 255;
    const min_halfword: HalfWord = 0;
    const max_halfword: HalfWord = 65535;

    const format_default_length: i32 = 9;
    const format_area_length: i32 = 0;
    const format_ext_length: i32 = 4;

    // 980
    const inserts_only: i32 = 1;
    const box_there: i32 = 2;

    
    pub fn new() -> TexState
    {
        return TexState {
            ready_already: 0,
            // 20
            xord: [0;256],
            xchr: [0;256],

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

            // 109
            glue_ratio: 0.,

            // 116
            mem: Array::new(TexState::mem_min, TexState::mem_max), // the big dynamic storage area
            lo_mem_max: 0, // the largest location of variable-size memory in use
            hi_mem_min: 0, // the smallest location of one-word memory in use

            // 117
            var_used: 0,
            dyn_used: 0, // how much memory is in use
            
            // 73
            interaction: Interaction::BatchMode,

            // 165
            // debug
            _free: [false;TexState::mem_max],
            was_free: [false;TexState::mem_max],
            was_mem_end: 0,
            was_lo_max: 0,
            was_hi_min: 0,
            panicking: false,

            // 213
            nest: [ListStateRecord::new(); TexState::nest_size],
            nest_ptr: 0,
            max_nest_stack: 0,
            curlist: ListStateRecord::new(),
            shown_mode: 0,

            // 253
            eqtb: Array::new(EquivRegion2Code::ActiveBase.value() as usize, eqtb_size),
            xeq_level: Array::new(int_base, eqtb_size),

            // 980
            page_tail: 0,
            page_contents: 0, // 0..2
            page_max_depth: 0,
            best_page_break: 0,
            least_page_cost: 0,
            best_size: 0,

            // 982
            page_so_far: Array::new(0, 7), // 0..7
            last_glue: 0,
            last_penalty: 0,
            last_kern: 0,
            insert_penalties: 0,
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

    // 215
    state.nest_ptr = 0;
    state.max_nest_stack = 0;
    state.curlist.mode_field = 1;
    state.curlist.head_field = 29999;
    state.curlist.tail_field = 29999;
    state.curlist.aux_field = MemoryWord::new_i32(-65536000);
    state.curlist.ml_field = 0;
    state.curlist.pg_field = 0;
    state.shown_mode = 0;

    // 991
    state.page_contents = 0;
    state.page_tail = 29998;
    state.mem[29998].set_hh_rh(0);
    
    state.last_glue = 65535;
    state.last_penalty = 0;
    state.last_kern = 0;
    state.page_so_far[7] = 0;
    state.page_max_depth = 0;

    // 254
    for k in 5263..=6106 {
        state.xeq_level[k] = 1;
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
    let mut state = TexState::new();

    println!("{}", TexState::banner);

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
        if TokenList::CsTokenFlag.value() + EquivRegion2Code::UndefinedControlSequence.value() as i32 > TexState::max_halfword as i32
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
