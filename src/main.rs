use std::process;
use std::collections::HashMap;
use std::char;
use std::fs::File;
use std::marker::Copy;
use std::ops::{Index, IndexMut};
use byteorder::{ByteOrder, LittleEndian};
use num_traits::{PrimInt, Signed};
use std::borrow::BorrowMut;


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
        return &self.data[i - self.min];
    }
}

impl<T:Default+Clone> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        return &mut self.data[i - self.min];
    }
}

// 12 (compiler constants)
const MemBot: usize = 0;
const MemTop: usize = 30000;
const FontBase: i32 = 0;
const HashSize: i32 = 2100;
const HashPrime: i32 = 1777;
const HyphSize: i32 = 307;

// 16
fn incr<T:PrimInt + BorrowMut<T>>(mut n: T) {
    n = n + num_traits::one(); // increase a variable by unity
}

fn decr<T:PrimInt + BorrowMut<T>>(mut n: T) {
    n = n - num_traits::one(); // decrease a variable by unity
}

// change the sign of a variable ; NOTE: If we're paranoid we could check for overflow here
fn negate<T:Signed + BorrowMut<T>>(mut s: T) {
    s = -s;
}


const Empty: i32 = 0; // symbolic name for a null constant


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

// 110
const MinQuarterWord: QuarterWord = 0;
const MaxQuarterWord: QuarterWord = 255;
const MinHalfWord: HalfWord = 0;
const MaxHalfWord: HalfWord = 65535;

// 113
type sc = i32; // |scaled| data is equivalent to |integer|

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

// 115
type Pointer = HalfWord;
const Null: HalfWord = MinHalfWord; // the null pointer

// 118
impl MemoryWord {
    fn get_link(&self) -> HalfWord {
        self.hh_rh()
    }

    fn get_info(&self) -> HalfWord {
        self.hh_lh()
    }

    fn set_link(&mut self, value: HalfWord) {
        self.set_hh_rh(value);
    }

    fn set_info(&mut self, value: HalfWord) {
        self.set_hh_lh(value);
    }
}

// glue 150
const GlueSpecSize: usize = 4; // number of words to allocate for a glue specification
/*
#define glue_ref_count(n) link(n) // reference count of a glue specification
#define stretch(n) (mem[n + 2].sc) // the stretchability of this glob of glue
#define shrink(n) (mem[n + 3].sc) // the shrinkability of this glob of glue
#define stretch_order type // order of infinity for stretching
#define shrink_order subtype // order of infinity for shrinking
*/
const fil: i32 = 1; // first-order infinity
const fill: i32 = 2; // second-order infinity
const filll: i32 = 3; // third-order infinity

type GlueOrd = i32; // normal .. filll, infinity to the 0, 1, 2, 3 power


// 162
const ZeroGlue: usize = MemBot; // specification for 0pt plus 0pt minus 0pt
const FilGlue: usize = ZeroGlue + GlueSpecSize; // 0pt plus 1fil minus 0pt
const FillGlue: usize = FilGlue + GlueSpecSize; // 0pt plus 1fill minus 0pt
const SsGlue: usize = FillGlue + GlueSpecSize; // 0pt plus 1fil minus 1fil
const FilNegGlue: usize = SsGlue + GlueSpecSize; // 0pt plus -1fil minus 0pt
const LoMemStatMax: usize = FilNegGlue + GlueSpecSize - 1; // largest statically allocated word in the variable-size mem

const PageInsHead: usize = MemTop; // list of insertion data for current page
const ContribHead: usize = MemTop - 1; // vlist items not yet on current page
const PageHead: usize = MemTop - 2; // vlist for current page
const TempHead: usize = MemTop - 3; // head of a temporary list of some kind
const HoldHead: usize = MemTop - 4; // head of a temporary list of another kind
const AdjustHead: usize = MemTop - 5; // head of adjustment list returned by hpack
const Active: usize = MemTop - 7; // head of active list in line_break, needs two words
const AlignHead: usize = MemTop - 8; // head of preamble list for alignments
const EndSpan: usize = MemTop - 9; // tail of spanned-width lists
const OmitTemplate: usize = MemTop - 10; // a constant token list
const NullList: usize = MemTop - 11; // permanently empty list
const LigTrick: usize = MemTop - 12; // a ligature masquerading as a char_node
const Garbage: usize = MemTop - 12; // used for scrap information
const BackupHead: usize = MemTop - 13; // head of token list built by scan_keyword
const HiMemStatMin: usize = MemTop - 13; // smallest statically allocated word in the one-word mem
const HiMemStatUsage: usize = 14; // the number of one-word nodes always present


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

// 221
/*
#define eq_level_field(s) s.hh.b1
#define eq_type_field(s) s.hh.b0
#define equiv_field(s) s.hh.rh
#define eq_level(s) eq_level_field(eqtb[s]) // level of definition
#define eq_type(s) eq_type_field(eqtb[s]) // command code for equivalent
#define equiv(s) equiv_field(eqtb[s]) // equivalent value
*/
const LevelZero: i32 = MinQuarterWord as i32; // level for undefined quantities
const LevelOne: i32 = LevelZero + 1; // outermost level for defined quantities


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

const ActiveBase: HalfWord = 1;
const SingleBase: HalfWord = ActiveBase + 256;
const NullCs: HalfWord = SingleBase + 256;
const HashBase: HalfWord = NullCs + 1;
const FrozenControlSequence: HalfWord = HashBase + HashSize as HalfWord;
const FrozenProtection: HalfWord = FrozenControlSequence;
const FrozenCr: HalfWord = FrozenControlSequence + 1;
const FrozenEndGroup: HalfWord = FrozenControlSequence + 2;
const FrozenRight: HalfWord = FrozenControlSequence + 3;
const FrozenFi: HalfWord = FrozenControlSequence + 4;
const FrozenEndTemplate: HalfWord = FrozenControlSequence + 5;
const FrozenEndv: HalfWord = FrozenControlSequence + 6;
const FrozenRelax: HalfWord = FrozenControlSequence + 7;
const EndWrite: HalfWord = FrozenControlSequence + 8;
const FrozenDontExpand: HalfWord = FrozenControlSequence + 9;
const FrozenNullFont: HalfWord = FrozenControlSequence + 10;
const FontIdBase: HalfWord = FrozenNullFont - FontBase as HalfWord;
const UndefinedControlSequence: HalfWord = FrozenNullFont + 257;
const GlueBase: HalfWord = UndefinedControlSequence + 1;

impl EquivRegion2Code {
    fn value(&self) -> HalfWord {
        match self {
            EquivRegion2Code::ActiveBase => ActiveBase,
            EquivRegion2Code::SingleBase => SingleBase,
            EquivRegion2Code::NullCs => NullCs,
            EquivRegion2Code::HashBase => HashBase,
            EquivRegion2Code::FrozenControlSequence => FrozenControlSequence,
            EquivRegion2Code::FrozenProtection => FrozenProtection,
            EquivRegion2Code::FrozenCr => FrozenCr,
            EquivRegion2Code::FrozenEndGroup => FrozenEndGroup,
            EquivRegion2Code::FrozenRight => FrozenRight,
            EquivRegion2Code::FrozenFi => FrozenFi,
            EquivRegion2Code::FrozenEndTemplate => FrozenEndTemplate,
            EquivRegion2Code::FrozenEndv => FrozenEndv,
            EquivRegion2Code::FrozenRelax => FrozenRelax,
            EquivRegion2Code::EndWrite => EndWrite,
            EquivRegion2Code::FrozenDontExpand => FrozenDontExpand,
            EquivRegion2Code::FrozenNullFont => FrozenNullFont,
            EquivRegion2Code::FontIdBase => FontIdBase,
            EquivRegion2Code::UndefinedControlSequence => UndefinedControlSequence,
            EquivRegion2Code::GlueBase => GlueBase,
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


const GluePars: HalfWord = 18; // total number of glue parameters
const SkipBase: HalfWord = GlueBase + GluePars; // table of 256 ``skip'' registers
const MuSkipBase: HalfWord = SkipBase + 256; // table of 256 ``muskip'' registers
const LocalBase: HalfWord = MuSkipBase + 256; // beginning of region 4


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

const ParShapeLoc: HalfWord = LocalBase;
const OutputRoutineLoc: HalfWord = LocalBase + 1;
const EveryParLoc: HalfWord = LocalBase + 2;
const EveryMathLoc: HalfWord = LocalBase + 3;
const EveryDisplayLoc: HalfWord = LocalBase + 4;
const EveryHboxLoc: HalfWord = LocalBase + 5;
const EveryVboxLoc: HalfWord = LocalBase + 6;
const EveryJobLoc: HalfWord = LocalBase + 7;
const EveryCrLoc: HalfWord = LocalBase + 8;
const ErrHelpLoc: HalfWord = LocalBase + 9;
const ToksBase: HalfWord = LocalBase + 10;
const BoxBase: HalfWord = ToksBase + 256;
const CurFontLoc: HalfWord = ToksBase + 256;


impl EquivRegion4Loc {
    fn value(&self) -> HalfWord {
        match self {
            EquivRegion4Loc::ParShapeLoc => ParShapeLoc,
            EquivRegion4Loc::OutputRoutineLoc => OutputRoutineLoc,
            EquivRegion4Loc::EveryParLoc => EveryParLoc,
            EquivRegion4Loc::EveryMathLoc => EveryMathLoc,
            EquivRegion4Loc::EveryDisplayLoc => EveryDisplayLoc,
            EquivRegion4Loc::EveryHboxLoc => EveryHboxLoc,
            EquivRegion4Loc::EveryVboxLoc => EveryVboxLoc,
            EquivRegion4Loc::EveryJobLoc => EveryJobLoc,
            EquivRegion4Loc::EveryCrLoc => EveryCrLoc,
            EquivRegion4Loc::ErrHelpLoc => ErrHelpLoc,
            EquivRegion4Loc::ToksBase => ToksBase,
            EquivRegion4Loc::BoxBase => BoxBase,
            EquivRegion4Loc::CurFontLoc => CurFontLoc,
        }
    }
}

const MathFontBase: HalfWord = CurFontLoc + 1; // table of 48 math font numbers
const CatCodeBase: HalfWord = MathFontBase + 48; // table of 256 command codes (the ``catcodes'')
const LcCodeBase: HalfWord = CatCodeBase + 256; // table of 256 lowercase mappings
const UcCodeBase: HalfWord = LcCodeBase + 256; // table of 256 uppercase mappings
const SfCodeBase: HalfWord = UcCodeBase + 256; // table of 256 spacefactor mappings
const MathCodeBase: HalfWord = SfCodeBase + 256; // table of 256 math mode mappings
const IntBase: HalfWord = MathCodeBase + 256; // beginning of region 5

// 236
const IntPars: HalfWord = 55; // total number of integer parameters
const CountBase: HalfWord = IntBase + IntPars; // 256 user \count registers
const DelCodeBase: HalfWord = CountBase + 256; // 256 delimiter code mappings
const DimenBase: HalfWord = DelCodeBase + 256; // beginning of region 6

const ScaledBase: HalfWord = DimenBase + DimenPars; // table of 256 user-defined \dimen registers

const EqtbSize: HalfWord = ScaledBase + 255; // largest subscript of eqtb

// 247
enum EquivRegion6Code
{
	ParIndentCode,
    MathSurroundCode,
    LineSkipLimitCode,
    HsizeCode,
    VsizeCode,
    MaxDepthCode,
    SplitMaxDepthCode,
    BoxMaxDepthCode,
    HfuzzCode,
    VfuzzCode,
    DelimiterShortfallCode,
    NullDelimiterSpaceCode,
    ScriptSpaceCode,
    PreDisplaySizeCode,
    DisplayWidthCode,
    DisplayIndentCode,
    OverfullRuleCode,
    HangIndentCode,
    HOffsetCode,
    VOffsetCode,
    EmergencyStretchCode,
}

const ParIndentCode: HalfWord = 0; // indentation of paragraphs
const MathSurroundCode: HalfWord = 1; // space around math in text
const LineSkipLimitCode: HalfWord = 2; // threshold for line_skip instead of baseline_skip
const HsizeCode: HalfWord = 3; // line width in horizontal mode
const VsizeCode: HalfWord = 4; // page height in vertical mode
const MaxDepthCode: HalfWord = 5; // maximum depth of boxes on main pages
const SplitMaxDepthCode: HalfWord = 6; // maximum depth of boxes on split pages
const BoxMaxDepthCode: HalfWord = 7; // maximum depth of explicit vboxes
const HfuzzCode: HalfWord = 8; // tolerance for overfull hbox messages
const VfuzzCode: HalfWord = 9; // tolerance for overfull vbox messages
const DelimiterShortfallCode: HalfWord = 10; // maximum amount uncovered by variable delimiters
const NullDelimiterSpaceCode: HalfWord = 11; // blank space in null delimiters
const ScriptSpaceCode: HalfWord = 12; // extra space after subscript or superscript
const PreDisplaySizeCode: HalfWord = 13; // length of text preceding a display
const DisplayWidthCode: HalfWord = 14; // length of line for displayed equation
const DisplayIndentCode: HalfWord = 15; // indentation of line for dispalyed equation
const OverfullRuleCode: HalfWord = 16; // width of rule that identifies overfull hboxes
const HangIndentCode: HalfWord = 17; // amount of hanging indentation
const HOffsetCode: HalfWord = 18; // amount of horizontal offset when shipping pages out
const VOffsetCode: HalfWord = 19; // amount of vertical offset when shipping pages out
const EmergencyStretchCode: HalfWord = 20; // reduces badness on final pass of line-breaking

const DimenPars: HalfWord = 21; // total number of dimension parameters


impl EquivRegion6Code {
    fn value(&self) -> HalfWord {
        match self {
            EquivRegion6Code::ParIndentCode => ParIndentCode,
            EquivRegion6Code::MathSurroundCode => MathSurroundCode,
            EquivRegion6Code::LineSkipLimitCode => LineSkipLimitCode,
            EquivRegion6Code::HsizeCode => HsizeCode,
            EquivRegion6Code::VsizeCode => VsizeCode,
            EquivRegion6Code::MaxDepthCode => MaxDepthCode,
            EquivRegion6Code::SplitMaxDepthCode => SplitMaxDepthCode,
            EquivRegion6Code::BoxMaxDepthCode => BoxMaxDepthCode,
            EquivRegion6Code::HfuzzCode => HfuzzCode,
            EquivRegion6Code::VfuzzCode => VfuzzCode,
            EquivRegion6Code::DelimiterShortfallCode => DelimiterShortfallCode,
            EquivRegion6Code::NullDelimiterSpaceCode => NullDelimiterSpaceCode,
            EquivRegion6Code::ScriptSpaceCode => ScriptSpaceCode,
            EquivRegion6Code::PreDisplaySizeCode => PreDisplaySizeCode,
            EquivRegion6Code::DisplayWidthCode => DisplayWidthCode,
            EquivRegion6Code::DisplayIndentCode => DisplayIndentCode,
            EquivRegion6Code::OverfullRuleCode => OverfullRuleCode,
            EquivRegion6Code::HangIndentCode => HangIndentCode,
            EquivRegion6Code::HOffsetCode => HOffsetCode,
            EquivRegion6Code::VOffsetCode => VOffsetCode,
            EquivRegion6Code::EmergencyStretchCode => EmergencyStretchCode,
        }
    }
}

/*
#define dimen(s) (eqtb[scaled_base+s].sc)
#define dimen_par(s) (eqtb[dimen_base+s].sc)
#define par_indent dimen_par(par_indent_code)
#define math_surround dimen_par(math_surround_code)
#define line_skip_limit dimen_par(line_skip_limit_code)
#define hsize dimen_par(hsize_code)
#define vsize dimen_par(vsize_code)
#define max_depth dimen_par(max_depth_code)
#define split_max_depth dimen_par(split_max_depth_code)
#define box_max_depth dimen_par(box_max_depth_code)
#define hfuzz dimen_par(hfuzz_code)
#define vfuzz dimen_par(vfuzz_code)
#define delimiter_shortfall dimen_par(delimiter_shortfall_code)
#define null_delimiter_space dimen_par(null_delimiter_space_code)
#define script_space dimen_par(script_space_code)
#define pre_display_size dimen_par(pre_display_size_code)
#define display_width dimen_par(display_width_code)
#define display_indent dimen_par(display_indent_code)
#define overfull_rule dimen_par(overfull_rule_code)
#define hang_indent dimen_par(hang_indent_code)
#define h_offset dimen_par(h_offset_code)
#define v_offset dimen_par(v_offset_code)
#define emergency_stretch dimen_par(emergency_stretch_code)
*/

impl TexState {
    fn dimen(&self, s: HalfWord) -> i32{
        self.eqtb[(ScaledBase + s)as usize].int()
    }

    fn dimen_par(&self, s: HalfWord) -> i32{
        self.eqtb[(DimenBase + s)as usize].int()
    }

    fn par_indent(&self) -> i32{
        self.dimen_par(ParIndentCode)
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

    // 118
    avail: Pointer, // head of the list of available one-word nodes
    mem_end: Pointer, // the last one-word node used in mem

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

            // 118
            avail: 0,
            mem_end: 0,
            
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
            eqtb: Array::new(EquivRegion2Code::ActiveBase.value() as usize, EqtbSize as usize),
            xeq_level: Array::new(IntBase as usize, EqtbSize as usize),

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
    state.page_contents = Empty as u8;
    state.page_tail = PageHead as HalfWord;
    state.mem[PageHead].set_link(Null);
    
    state.last_glue = MaxHalfWord;
    state.last_penalty = 0;
    state.last_kern = 0;
    state.page_so_far[7] = 0;
    state.page_max_depth = 0;

    // 254
    for k in IntBase..=EqtbSize {
        state.xeq_level[k as usize] = 1;
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
        if MemBot + 1100 > MemTop { bad = 4; }
        if HashPrime > HashSize { bad = 5; }
        if TexState::max_in_open >= 128 { bad = 6; }
        if MemTop < 256 + 11 { bad = 7; }

        // 111
        #[cfg(init)]
        {
            if TexState::mem_min != TexState::mem_bot
            || TexState::mem_max != TexState::mem_top
            {
                bad = 10;
            }
        }

        if TexState::mem_min > MemBot || TexState::mem_max < MemTop { bad = 10; }
        if MinQuarterWord > 0 || MaxQuarterWord < 127 { bad = 11; }
        if MinHalfWord > 0 || MaxHalfWord < 32767 { bad = 12; }
        if (MinQuarterWord as HalfWord) < MinHalfWord 
            || MaxQuarterWord as HalfWord > MaxHalfWord { bad = 13; }
        if TexState::mem_min < MinHalfWord as usize
            || TexState::mem_max >= MaxHalfWord as usize
            || MemBot - TexState::mem_min > MaxHalfWord as usize + 1
            {
                bad = 14;
            }
        if FontBase < MinQuarterWord as i32
            || TexState::font_max > MaxQuarterWord as i32 { bad = 15; }
        if TexState::font_max > FontBase + 256 { bad = 16; }
        if TexState::save_size > MaxHalfWord as i32 { bad = 18; }
        if MaxQuarterWord - MinQuarterWord < 255 { bad = 19; }

        //290
        if TokenList::CsTokenFlag.value() + EquivRegion2Code::UndefinedControlSequence.value() as i32 > MaxHalfWord as i32
        {
            bad = 21;
        }

        // 522
        if TexState::format_default_length > TexState::file_name_size { bad = 31; }

        // 1249
        if (2 * MaxHalfWord as usize) < MemTop - TexState::mem_min { bad = 41; }

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
