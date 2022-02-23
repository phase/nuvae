use crate::compiler::Compiler;
use crate::ir::IrNode;

mod ast;
mod parser;
mod diagnostic;
mod ir;
mod backend;
mod compiler;

fn main() {
    let test_file = "test.nuv";
    let test_source = "\

    interface CoolInterface {
        fun publicFunc(x: Int32, y: Int32);
    }

    public struct CoolApi {
        public let CONSTANT = 7;
    }

    struct X {
        let x = 2;

        fun test(): Int32 {
            return 7;
        }
    }

    fun testRefinement(a: (v: Int32 where v >= 0 and v <= 10 + 7), b): Int32 {
        let x = 1;
        let y = 0;
        if x < a {
            y = 7;
        } else if x >= 500 {
            y = 5;
        } else if x >= a and x < b {
            y = 9;
        } else {
            y = b;
        }
        return y;
    }

    fun testRow(x: {field1: Int32, field2: Int32}) {
        let y = test((x.field1), x.field2);
    }

    type Nat32 = (v: Int32 where v >= 0);
    struct Box {
        let x: Int32;
    }
    type PosBox = (b: Box where b.x >= 0);
    type PosBox2 = (b: {x: Int32} where b.x >= 0);
    type PosBox3 = Box where it.x >= 0;
    unique type Meters = Int32;

    public fun refTest(arena: ArenaAllocator): X {
        return new X in arena;
    }

    public fun buildX2(arena: &mut ArenaAllocator): X {
        return new X in arena;
    }

    public fun buildX3(arena: &Allocator): X {
        return new X in arena;
    }

    public fun derefX(refX: &X): X {
        return x.*;
    }

    public fun derefX2(refX: &?X): X {
        return x.*.?;
    }

    public fun derefX2(refX: &?&?X): X {
        let xRefCopy = refX.&.*.&.*;
        return x.*.?.*.?;
    }

    fun add(x, y) {
        return x + y;
    }

    enum Node {
        Point(x: Int32, y: Int32, next: &Node),
        Nil
    }

    fun max(x, y) {
        if x > y {
            return x;
        } else {
            return y;
        }
    }

    fun max2(x: Int32, y: Int32): (ret: Int32 where x <= v or y <= v) {
        if x > y { return x; }
        else { return y; }
    }

    fun sum(k) {
        if k < 0 {
            return 0;
        } else {
            let s = sum(k - 1);
            return s + k;
        }
    }

    fun sum2(k: Int32): (ret: Int32 where 0 <= ret and k <= ret) {
        if k < 0 {
            return 0;
        } else {
            let s = sum(k - 1);
            return s + k;
        }
    }

    fun loop(n, i, c, f) {
        if i < n {
            return loop(n, i + 1, f(i, c), f);
        } else {
            return c;
        }
    }

    fun foldn(n, b, f) {
        return loop(n, 0, b, f);
    }

    public fun foldn2[A](n: Int32, b: A, f: (Int32 where 0 <= it or it < n, A) -> A): A {
        return loop(n, 0, b, f);
    }

    ".to_string();

    let mut compiler = Compiler::new();
    compiler.parse_module(ast::Path::of("test"), test_file.to_string(), test_source);
    for (_index, module) in compiler.modules.iter() {
        for (_node_index, node) in module.module_arena.node_arena.iter() {
            println!("{:?}", node);
        }
        for (_block_index, block) in module.module_arena.block_arena.iter() {
            println!("{:?}", block);
        }
        for (_ins_index, ins) in module.module_arena.instruction_arena.iter() {
            println!("{:?}", ins);
        }
        for (_ins_index, typ) in module.module_arena.type_arena.iter() {
            println!("{:?}", typ);
        }
    }

    println!("parse complete!")
}
