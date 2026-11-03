use std::sync::atomic::{AtomicUsize, Ordering};

use strum_macros::EnumString;

use super::{Assembler, AssemblyResult, Error};
use crate::{Argument, Endian, InstructionSet, Register, mask};

#[derive(Debug, Clone, Copy, EnumString)]
#[strum(serialize_all = "lowercase")]
enum Opcode {
    Nop,
    Byte,
    Load,
}

/// Minimal instruction set with one-byte instructions.
struct Test<const BIG_ENDIAN: bool>;

impl<const BIG_ENDIAN: bool> InstructionSet for Test<BIG_ENDIAN> {
    type Opcode = Opcode;
    type Register = Register<4>;

    const ENDIAN: Endian = if BIG_ENDIAN {
        Endian::Big
    } else {
        Endian::Little
    };

    fn encode(
        _: u64,
        opcode: Opcode,
        arguments: &[Argument<Self::Register>],
    ) -> Result<Vec<u8>, String> {
        match (opcode, arguments) {
            (Opcode::Nop, []) => Ok(vec![0]),
            (Opcode::Byte, &[Argument::Imm(x)]) => Ok(vec![mask(x, 8)?]),
            // encodes the base register (0xff for none) and the offset
            (Opcode::Load, &[Argument::Mem { base, offset }]) => {
                let base = base.map_or(0xff, |reg| u8::try_from(reg.0).unwrap());
                Ok(vec![base, mask(offset, 8)?])
            }
            _ => Err("invalid instruction".to_owned()),
        }
    }
}

fn assemble_with<const BIG_ENDIAN: bool>(source: &str) -> Result<AssemblyResult, Error> {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let path = std::env::temp_dir().join(format!(
        "assemulator-test-{}-{}.asm",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::write(&path, source).unwrap();
    let result = Assembler::<Test<BIG_ENDIAN>>::assemble(&path);
    std::fs::remove_file(&path).unwrap();
    result
}

fn data(source: &str) -> Vec<u8> {
    assemble_with::<true>(source).unwrap().data
}

fn program(source: &str) -> Vec<u8> {
    assemble_with::<true>(source).unwrap().program
}

fn fails(source: &str) -> bool {
    assemble_with::<true>(source).is_err()
}

#[test]
fn data_directives_use_processor_byte_order() {
    let source = ".u8 1, 255\n.i8 -1\n.u16 0x1234\n.i16 -2\n";
    assert_eq!(data(source), [1, 255, 255, 0x12, 0x34, 0xff, 0xfe]);
    let little = assemble_with::<false>(".u16 0x1234\n").unwrap().data;
    assert_eq!(little, [0x34, 0x12]);
}

#[test]
fn data_directives_check_ranges() {
    assert_eq!(
        data(".i8 -128, 127\n.u16 65535\n"),
        [0x80, 0x7f, 0xff, 0xff]
    );
    assert!(fails(".u8 256\n"));
    assert!(fails(".u8 -1\n"));
    assert!(fails(".i8 128\n"));
    assert!(fails(".i16 40000\n"));
}

#[test]
fn data_directives_accept_strings() {
    assert_eq!(data(".u8 \"hi\\n\", 0\n"), b"hi\n\0");
    assert_eq!(data(".u16 \"A\"\n"), [0, b'A']);
    assert_eq!(data(".strz \"ab\", \"c\"\n"), b"ab\0c\0");
    assert!(fails(".strz 5\n"));
}

#[test]
fn expressions_accept_characters_and_labels() {
    assert_eq!(data(".u8 'A' + 1, '\\n'\n"), [b'B', b'\n']);
    assert_eq!(data("n: .set 3\nn_max: .set 40\n.u8 n_max + n\n"), [43]);
    assert_eq!(data("a: .set b + 1\nb: .set 2\n.u8 a\n"), [3]);
}

#[test]
fn if_else_picks_one_branch() {
    let source = "x: .set 1
.if x
.u8 1
.else
.u8 2
.endif
.if x == 0
.u8 3
.else
.u8 4
.endif
";
    assert_eq!(data(source), [1, 4]);
}

#[test]
fn if_blocks_nest() {
    let source = ".if 0
.if 1
.u8 1
.else
.u8 2
.endif
.else
.if 0
.u8 3
.else
.u8 4
.endif
.endif
";
    assert_eq!(data(source), [4]);
}

#[test]
fn unbalanced_conditionals_are_errors() {
    assert!(fails(".endif\n"));
    assert!(fails(".else\n"));
    assert!(fails(".if 1\n.u8 1\n"));
    assert!(fails(".if 0\n.u8 1\n"));
}

#[test]
fn layout_change_between_passes_is_an_error() {
    // `later` is unknown in the first pass, so the condition is false there
    // and true in the final pass, which moves `later`.
    assert!(fails(".if later == 0\nnop\n.endif\nlater: nop\n"));
    // forward references are fine as long as the layout doesn't change
    assert_eq!(
        program("nop\n.if later == 5\nnop\n.endif\nlater: nop\n"),
        [0, 0]
    );
}

#[test]
fn memory_operands_split_base_and_offset() {
    let source = "arr: .u8 0, 0, 0
load [5]
load [r1]
load [r1 + 2]
load [2 + r1]
load [arr + r2 + 1]
load [r3 - 1]
";
    assert_eq!(program(source), [0xff, 5, 1, 0, 1, 2, 1, 2, 2, 1, 3, 0xff]);
}

#[test]
fn memory_operands_only_add_one_register() {
    assert!(fails("load [r1 + r2]\n"));
    assert!(fails("load [r1 * 2]\n"));
    assert!(fails("load [4 - r1]\n"));
    assert!(fails(".u8 [4]\n"));
}

#[test]
fn macros_substitute_whole_arguments() {
    let source = "emit: .macro x, xy
.u8 $xy, $x
.endm
emit 1, 2
";
    assert_eq!(data(source), [2, 1]);
    assert!(fails("emit: .macro x\n.u8 $x\n.endm\nemit 1, 2\n"));
}

fn error(source: &str) -> String {
    assemble_with::<true>(source).err().unwrap().to_string()
}

#[test]
fn errors_name_the_file_and_line() {
    let message = error("nop\nbyte missing\n");
    assert!(message.contains(".asm:2:6"), "{message}");
    assert!(message.contains("undefined label"), "{message}");
}

#[test]
fn macro_errors_point_at_the_definition_and_the_call() {
    let message = error("emit: .macro x\n    .u8 $x\n.endm\nemit missing\n");
    assert!(message.contains(".asm:2:9"), "{message}");
    assert!(message.contains(".u8 missing"), "{message}");
    assert!(message.contains(".asm:4:1"), "{message}");
    assert!(message.contains("in expansion of this macro"), "{message}");
}

#[test]
fn unclosed_blocks_point_at_their_start() {
    assert!(error("nop\n.if 1\nnop\n").contains(".asm:2:1"));
    assert!(error("m: .macro\nnop\n").contains("missing .endm"));
}

#[test]
fn zero_has_no_size_limit() {
    assert_eq!(data(".zero 300\n").len(), 300);
}

#[test]
fn namespaced_includes_see_outer_symbols() {
    let lib = format!("assemulator-test-lib-{}.asm", std::process::id());
    let lib_path = std::env::temp_dir().join(&lib);
    // `limit` comes from the including file; `double` and `value` are namespaced
    let lib_source = "double: .macro x\n.u8 $x, $x\n.endm\nvalue: .set limit+1\ndouble value\n";
    std::fs::write(&lib_path, lib_source).unwrap();
    let source = format!("limit: .set 4\nlib: .include \"{lib}\"\n.u8 lib::value\n");
    let result = assemble_with::<true>(&source);
    std::fs::remove_file(&lib_path).unwrap();
    assert_eq!(result.unwrap().data, [5, 5, 5]);
}
