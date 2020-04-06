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

// 2
const banner: &'static str = "This is TeX, Version 3.1415926"; // printed when TEX starts

// 36
const MemMax: usize = 30000;
const MemMin: usize = 0;
const BufSize: i32 = 500;
const ErrorLine: i32 = 72;
const HalfErrorLine: i32 = 42;
const MaxPrintLine: i32 = 79;
const StackSize: i32 = 200;
const MaxInOpen: i32 = 6;
const FontMax: i32 = 75;
const FontMemSize: i32 = 20000;
const ParamSize: i32 = 60;
const NestSize: usize = 40;
const MaxStrings: i32 = 3000;
const StringVacancies: i32 = 8000;
const PoolSize: i32 = 32000;
const SaveSize: i32 = 600;
const TrieSize: i32 = 8000;
const TrieOpSize: i32 = 500;
const DviBufSize: i32 = 800;
const FileNameSize: i32 = 40;
const PoolName: &'static str = "TeXformats:TEX.POOL                     ";


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

// 76


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

    pub fn set_lh(&mut self, value: HalfWord) {
        LittleEndian::write_u16(&mut self.data[0..2], value);
    }
}

impl Default for TwoHalves {
    fn default() -> Self {
        TwoHalves {
            data: [0; 4]
        }
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

    pub fn sc(&self) -> i32 {
        LittleEndian::read_i32(&self.data)
    }

    pub fn gr(&self) -> GlueRatio {
        LittleEndian::read_f32(&self.data)
    }

    pub fn hh_b0(&self) -> u8 {
        self.data[2]
    }

    pub fn hh_b1(&self) -> u8 {
        self.data[3]
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
impl TexState {
    fn link(&self, n:usize) -> HalfWord {
        self.mem[n].hh_rh()
    }

    fn info(&self, n:usize) -> HalfWord {
        self.mem[n].hh_lh()
    }

    fn set_link(&mut self, n:usize, value: HalfWord) {
        self.mem[n].set_hh_rh(value);
    }

    fn set_info(&mut self, n:usize, value: HalfWord) {
        self.mem[n].set_hh_lh(value);
    }
}

//133
impl TexState {
    // identifies what kind of node this is
    fn a_type(&self, n:usize) -> u8{
        self.mem[n].hh_b0()
    }

    // secondary identification in some cases
    fn subtype(&self, n:usize) -> u8{
        self.mem[n].hh_b1()
    }
}

// 135
const HlistNode: i32 = 0;
const BoxNodeSize: i32 = 7; // number of words to allocate for a box node
const WidthOffset: usize = 1; // position of width field in a box node
const DepthOffset: usize = 2; // position of depth field in a box node
const HeightOffset: usize = 3; // position of height field in a box node


impl TexState {
    // width of the box, in sp
    fn width(&self, n:usize) -> i32 {
        self.mem[n + WidthOffset].sc()
    }

    // depth of the box, in sp
    fn depth(&self, n:usize) -> i32 {
        self.mem[n + DepthOffset].sc()
    }

    // height of the box, in sp
    fn height(&self, n:usize) -> i32 {
        self.mem[n + HeightOffset].sc()
    }

    // repositioning distance, in sp
    fn shift_amount(&self, n:usize) -> i32 {
        self.mem[n + 4].sc()
    }

    // beginning of the list inside the box
    fn list_ptr(&self, n:usize) -> HalfWord {
        self.link(n + ListOffset)
    }

    // applicable order of infinity
    fn glue_order(&self, n:usize) -> u8 {
        self.subtype(n + ListOffset)
    }

    // stretching or shrinking
    fn glue_sign(&self, n:usize) -> u8 {
        self.a_type(n + ListOffset)
    }

    // a word of type glue_ratio for glue setting
    fn glue_set(&self, n:usize) -> GlueRatio {
        self.mem[n+GlueOffset].gr()
    }
}

const ListOffset: usize = 5; // position of list_ptr field in a box node
 
const Normal: i32 = 0; // the most common case when several cases are named
const Stretching: i32 = 1; // glue setting applies to the stretch components
const Shrinking: i32 = 2; // glue setting applies to the shrink components
const GlueOffset: usize = 6; // position of glue_set in a box node

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


// 269
enum GroupCode
{
	BottomLevel, // group code for the outside world
	SimpleGroup, // group code for local structure only
	HboxGroup, // code for `\hbox{...}'
	AdjustedHboxGroup, // code for `\hbox{...}' in vertical mode
	VboxGroup, // code for `\vbox{...}'
	VtopGroup, // code for `\vtop{...}'
	AlignGroup, // code for `\halign{...}', `\valign{...}'
	NoAlignGroup, // code for `\noalign{...}'
	OutputGroup, // code for output routine
	MathGroup, // code for, e.g., `^{...}'
	DiscGroup, // code for `\discretionary{...}{...}{...}'
	InsertGroup, // code for `\insert{...}', `\vadjust{...}'
	VcenterGroup, // code for `\vcenter{...}'
	MathChoiceGroup, // code for `\mathchoice{...}{...}{...}'
	SemiSimpleGroup, // code for `\begingroup...\endgroup'
	MathShiftGroup, // code for `$...$'
	MathLeftGroup, // code for `\left...\right'
}

const BottomLevel: QuarterWord = 0;
const SimpleGroup: QuarterWord = 1;
const HboxGroup: QuarterWord = 2;
const AdjustedHboxGroup: QuarterWord = 3;
const VboxGroup: QuarterWord = 4;
const VtopGroup: QuarterWord = 5;
const AlignGroup: QuarterWord = 6;
const NoAlignGroup: QuarterWord = 7;
const OutputGroup: QuarterWord = 8;
const MathGroup: QuarterWord = 9;
const DiscGroup: QuarterWord = 10;
const InsertGroup: QuarterWord = 11;
const VcenterGroup: QuarterWord = 12;
const MathChoiceGroup: QuarterWord = 13;
const SemiSimpleGroup: QuarterWord = 14;
const MathShiftGroup: QuarterWord = 15;
const MathLeftGroup: QuarterWord = 16;

const MaxGroupCode: QuarterWord = 16;


impl GroupCode {
    fn value(&self) -> QuarterWord {
        match self {
            GroupCode::BottomLevel => BottomLevel,
            GroupCode::SimpleGroup => SimpleGroup,
            GroupCode::HboxGroup => HboxGroup,
            GroupCode::AdjustedHboxGroup => AdjustedHboxGroup,
            GroupCode::VboxGroup => VboxGroup,
            GroupCode::VtopGroup => VtopGroup,
            GroupCode::AlignGroup => AlignGroup,
            GroupCode::NoAlignGroup => NoAlignGroup,
            GroupCode::OutputGroup => OutputGroup,
            GroupCode::MathGroup => MathGroup,
            GroupCode::DiscGroup => DiscGroup,
            GroupCode::InsertGroup => InsertGroup,
            GroupCode::VcenterGroup => VcenterGroup,
            GroupCode::MathChoiceGroup => MathChoiceGroup,
            GroupCode::SemiSimpleGroup => SemiSimpleGroup,
            GroupCode::MathShiftGroup => MathShiftGroup,
            GroupCode::MathLeftGroup => MathLeftGroup,
        }
    }

    fn from(value:QuarterWord) -> GroupCode {
        match value {
            BottomLevel =>  GroupCode::BottomLevel,
            SimpleGroup =>  GroupCode::SimpleGroup,
            HboxGroup   =>  GroupCode::HboxGroup,
            AdjustedHboxGroup   =>  GroupCode::AdjustedHboxGroup,
            VboxGroup   =>  GroupCode::VboxGroup,
            VtopGroup   =>  GroupCode::VtopGroup,
            AlignGroup  =>  GroupCode::AlignGroup,
            NoAlignGroup    =>  GroupCode::NoAlignGroup,
            OutputGroup =>  GroupCode::OutputGroup,
            MathGroup   =>  GroupCode::MathGroup,
            DiscGroup   =>  GroupCode::DiscGroup,
            InsertGroup =>  GroupCode::InsertGroup,
            VcenterGroup    =>  GroupCode::VcenterGroup,
            MathChoiceGroup =>  GroupCode::MathChoiceGroup,
            SemiSimpleGroup =>  GroupCode::SemiSimpleGroup,
            MathShiftGroup  =>  GroupCode::MathShiftGroup,
            MathLeftGroup   =>  GroupCode::MathLeftGroup,
            _ => panic!("GroupCode no found")
        }
    }
}

/**
 * Main TexState
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

// 256
impl TwoHalves {
    // link for coalesced lists
    fn next(&self) -> HalfWord {
        self.lh()
    }

    fn set_next(&mut self, value: HalfWord) {
        self.set_lh(value);
    }

    // string number for control sequence name
    fn text(&mut self) -> HalfWord {
        self.rh()
    }

    fn set_text(&mut self, value: HalfWord) {
        self.set_rh(value)
    }
}

// test if all positions are occupied
fn hash_is_full(hash_used: HalfWord, hash_base: HalfWord) -> bool {
    hash_used == hash_base
}

// a frozen font identifier's name
fn font_id_text(hash: &Array<TwoHalves>, n: HalfWord) -> HalfWord {
    hash[(FontIdBase + n) as usize].lh()
}

fn set_font_id_text(hash: &mut Array<TwoHalves>, n: HalfWord, value: HalfWord) {
    hash[(FontIdBase + n) as usize].set_lh(value);
}


// 289
const cs_token_flag: i32 = 0o7777; // amount added to the eqtb location in a token that stands for a control
                                 // sequence; is a multiple of 256, less 1
const left_brace_token: i32 = 0o400; // 2^8*left_brace
const left_brace_limit: i32 = 0o1000; // 2^8*(left_brace+1)
const right_brace_token: i32 = 0o1000; // 2^8*right_brace
const right_brace_limit: i32 = 0o1400; // 2^8*(right_brace+1)
const math_shift_token: i32 = 0o1400; // 2^8*math_shift
const tab_token: i32 = 0o2000; // 2^8*tab_mark
const out_param_token: i32 = 0o2400; // 2^8*out_param
const space_token: i32 = 0o5040; // 2^8*spacer + " "
const letter_token: i32 = 0o5400; // 2^8*letter
const other_token: i32 = 0o6000; // 2^8*other_char
const match_token: i32 = 0o6400; // 2^8*match
const end_match_token: i32 = 0o7000; // 2^8*end_match

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

// 382
const top_mark_code: usize = 0; // the mark in effect at the previous page break
const first_mark_code: usize = 1; // the first mark between top_mark and bot_mark
const bot_mark_code: usize = 2; // the mark in effect at the current page break
const split_first_mark_code: usize = 3; // the first mark found by \vsplit
const split_bot_mark_code: usize = 4; // the last mark found by \vsplit

macro_rules! top_mark {
    ($state:expr) => {
        $state.cur_mark[top_mark_code]
    };
}

macro_rules! first_mark {
    ($state:expr) => {
        $state.cur_mark[first_mark_code]
    };
}

macro_rules! bot_mark {
    ($state:expr) => {
        $state.cur_mark[bot_mark_code]
    };
}

macro_rules! split_first_mark {
    ($state:expr) => {
        $state.cur_mark[split_first_mark_code]
    };
}

macro_rules! split_bot_mark {
    ($state:expr) => {
        $state.cur_mark[split_bot_mark_code]
    };
}

// 410
const IntVal: i32 = 0; // integer values
const DimenVal: i32 = 1; // dimension values
const GlueVal: i32 = 2; // glue specifications
const MuVal: i32 = 3; // math glue specifications
const IdentVal: i32 = 4; // font identifier
const TokVal: i32 = 5; // token lists

type StrNumber = i32;

// 438
const octal_token: i32 = other_token + /*'*/39; // apostrophe, indicates an octal constant
const hex_token: i32 = other_token + /*"*/34; // double quote, indicates a hex constant
const alpha_token: i32 = other_token + /*`*/96; // reverse apostrophe, precedes alpha constants
const point_token: i32 = other_token + /*.*/46; // decimal point
const continental_point_token: i32 = other_token + /*,*/44; // decimal point, Eurostyle


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
    _free: [bool;MemMax],
    was_free: [bool;MemMax],
    was_mem_end: HalfWord,
    was_lo_max: HalfWord,
    was_hi_min: HalfWord,
    panicking: bool,

    // 213
    nest: [ListStateRecord; NestSize],
    nest_ptr: usize, // 0..nestsize
    max_nest_stack: usize, // 0..nestsize
    curlist:ListStateRecord,
    shown_mode: i32, // -203..203

    // 253
    eqtb: Array<MemoryWord>,
    xeq_level: Array<QuarterWord>,

    // 256
    hash: Array<TwoHalves>, // the hash table <hash_base, undefined_control_sequence - 1>
    hash_used: Pointer, // allocation pointer for hash
    no_new_control_sequence: bool, // are new identifiers legal?
    cs_count: i32, // total number of known identifiers

    // 271
    save_stack: Array<MemoryWord>, // Array<memory_word, 0, save_size>
    save_ptr: i32, // first unused entry on save_stack
    max_save_stack: i32, // maximum usage of save stack
    cur_level: QuarterWord, // current nesting level for groups
    cur_group: GroupCode, // current group type
    cur_boundary: i32, // where the current level begins

    // 286
    mag_set: i32, // if nonzero, this magnification should be used henceforth

    // 382
    cur_mark: Array<Pointer>, // token list for marks <top_mark_code, split_bot_mark_code>

    // 410
    cur_val: i32, // value returned by numeric scanners
    cur_val_level: i32, // int_val..tok_val, the ``level'' of this value

    // 438
    radix: SmallNumber, // scan_int sets this to 8, 10, 16, or zero

    // 447
    cur_order: GlueOrd, // order of infinity found by scan_dimen

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
            mem: Array::new(MemMin, MemMax), // the big dynamic storage area
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
            _free: [false;MemMax],
            was_free: [false;MemMax],
            was_mem_end: 0,
            was_lo_max: 0,
            was_hi_min: 0,
            panicking: false,

            // 213
            nest: [ListStateRecord::new(); NestSize],
            nest_ptr: 0,
            max_nest_stack: 0,
            curlist: ListStateRecord::new(),
            shown_mode: 0,

            // 253
            eqtb: Array::new(EquivRegion2Code::ActiveBase.value() as usize, EqtbSize as usize),
            xeq_level: Array::new(IntBase as usize, EqtbSize as usize),

            // 256
            hash: Array::new(HashBase as usize, UndefinedControlSequence as usize - 1),
            hash_used: 0,
            no_new_control_sequence: false,
            cs_count: 0,

            // 271
            save_stack: Array::new(0, SaveSize as usize),
            save_ptr: 0,
            max_save_stack: 0,
            cur_level: 0,
            cur_group: GroupCode::AlignGroup,
            cur_boundary: 0,

            
            // 286
            mag_set: 0, // if nonzero, this magnification should be used henceforth

            // 382
            cur_mark: Array::new(top_mark_code, split_bot_mark_code),

            // 410
            cur_val: 0,
            cur_val_level: 0,

            // 438
            radix: 0,

            // 447
            cur_order: 0,

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
        state.was_mem_end = MemMin as HalfWord;
        state.was_lo_max = MemMin as HalfWord;
        state.was_hi_min = MemMax as HalfWord;
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
    state.set_link(PageHead, Null);
    
    state.last_glue = MaxHalfWord;
    state.last_penalty = 0;
    state.last_kern = 0;
    state.page_so_far[7] = 0;
    state.page_max_depth = 0;

    // 254
    for k in IntBase..=EqtbSize {
        state.xeq_level[k as usize] = 1;
    }

    
	// 257
    state.no_new_control_sequence = true;
    state.hash[HashBase as usize].set_next(0);
    state.hash[HashBase as usize].set_next(0);
    state.hash[HashBase as usize].set_text(0);
    for k in HashBase + 1 ..= UndefinedControlSequence - 1 {
        state.hash[k as usize] = state.hash[HashBase as usize];
    }	

	// 272
    state.save_ptr = 0;
    state.cur_level = LevelOne as QuarterWord;
    state.cur_group = GroupCode::from(BottomLevel);
    state.cur_boundary = 0;
	state.max_save_stack = 0;

	// 287
	state.mag_set = 0;


	// 383
    top_mark!(state) = Null;
    first_mark!(state) = Null;
    bot_mark!(state) = Null;
    split_first_mark!(state) = Null;
	split_bot_mark!(state) = Null;

	// 439
    state.cur_val = 0;
    state.cur_val_level = IntVal;
    state.radix = 0;
    state.cur_order = Normal;

    // 481
    for k in 0..=16 {
        state.read_open[k] = closed;
    }	

	// 490
	state.cond_ptr = Null;
	state.if_limit = normal; state.cur_if = 0; state.if_line = 0;

	// 521
	strcpy(TEX_format_default.get_c_str(),"plain.fmt");

	// 551
	for k in FontBase..=FontMax {
        state.font_used[k] = false;
    }

	// 556
	null_character.b0 = min_quarterword; null_character.b1 = min_quarterword;
	null_character.b2 = min_quarterword; null_character.b3 = min_quarterword;


	// 593
	total_pages = 0; max_v = 0; max_h = 0; max_push = 0; last_bop = -1; doing_leaders = false;
	dead_cycles = 0; cur_s = -1;

	// 596
	half_buf = dvi_buf_size / 2; dvi_limit = dvi_buf_size; dvi_ptr = 0; dvi_offset = 0;
	dvi_gone = 0;


	// 606
	down_ptr = null; right_ptr = null;

	// 648
	adjust_tail = null; last_badness = 0;

	// 662
	pack_begin_line = 0;

	// 685
	empty_field.rh = empty; empty_field.lh = Null;
	null_delimiter.b0 = 0; null_delimiter.b1 = min_quarterword;
	null_delimiter.b2 = 0; null_delimiter.b3 = min_quarterword;

	// 771
	align_ptr = Null; cur_align = Null; cur_span = Null; cur_loop = Null;
	cur_head = Null; cur_tail = Null;

	// 928
	for z in 0..=HyphSize {
		state.hyph_word[z] = 0; state.hyph_list[z] = Null;
	}
	state.hyph_count = 0;

	// 990
	output_active = false;
	insert_penalties = 0;

	// 1033
	ligature_present = false;
	cancel_boundary = false;
	lft_hit = false;
	rt_hit = false;
	ins_disc = false;


	// 1267
	after_token = 0;

	// 1282
	long_help_seen = false;

	// 1300
	format_ident = 0;

	// 1343
	for k in 0..=17 {
        write_open[k] = false;
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

    println!("{}", banner);

    state.history = History::FatalErrorStop;

    // check consts
    if state.ready_already != 314159 {
		let mut bad = 0;
        if HalfErrorLine<30 || HalfErrorLine > ErrorLine - 15
        { bad = 1; }

        if MaxPrintLine < 60 { bad = 2; }
        if DviBufSize % 8 != 0 { bad = 3; }
        if MemBot + 1100 > MemTop { bad = 4; }
        if HashPrime > HashSize { bad = 5; }
        if MaxInOpen >= 128 { bad = 6; }
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

        if MemMin > MemBot || MemMax < MemTop { bad = 10; }
        if MinQuarterWord > 0 || MaxQuarterWord < 127 { bad = 11; }
        if MinHalfWord > 0 || MaxHalfWord < 32767 { bad = 12; }
        if (MinQuarterWord as HalfWord) < MinHalfWord 
            || MaxQuarterWord as HalfWord > MaxHalfWord { bad = 13; }
        if MemMin < MinHalfWord as usize
            || MemMax >= MaxHalfWord as usize
            || MemBot - MemMin > MaxHalfWord as usize + 1
            {
                bad = 14;
            }
        if FontBase < MinQuarterWord as i32
            || FontMax > MaxQuarterWord as i32 { bad = 15; }
        if FontMax > FontBase + 256 { bad = 16; }
        if SaveSize > MaxHalfWord as i32 { bad = 18; }
        if MaxQuarterWord - MinQuarterWord < 255 { bad = 19; }

        //290
        if TokenList::CsTokenFlag.value() + EquivRegion2Code::UndefinedControlSequence.value() as i32 > MaxHalfWord as i32
        {
            bad = 21;
        }

        // 522
        if TexState::format_default_length > FileNameSize { bad = 31; }

        // 1249
        if (2 * MaxHalfWord as usize) < MemTop - MemMin { bad = 41; }

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
