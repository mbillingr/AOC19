use crate::intcode2::{Computer, Op, Operand, MEMORY_SIZE};
use crate::intcode_decompile::FixOp;
use cranelift::codegen::{ir, Context};
use cranelift::prelude as cl;
use cranelift::prelude::{FunctionBuilderContext, InstBuilder, IntCC, Uimm64};
use cranelift_module::{default_libcall_names, DataId, FuncId, Linkage, Module};
use cranelift_simplejit::{SimpleJITBackend, SimpleJITBuilder};
use std::collections::HashMap;

pub type IntcodeProgram = fn(vm: &mut Computer, memory: *mut i64);

pub struct Compiler<'c> {
    module: &'c mut Module<SimpleJITBackend>,
    builder: cl::FunctionBuilder<'c>,

    dyneval: ir::FuncRef,

    vm: cl::Value,
    memory: cl::Value,
}

impl<'c> Compiler<'c> {
    pub fn new(ctx: &'c mut CompilerContext) -> Self {
        let mut builder =
            cl::FunctionBuilder::new(&mut ctx.module_context.func, &mut ctx.function_context);

        let entry_ebb = builder.create_ebb();
        let err_ebb = builder.create_ebb();
        let code_ebb = builder.create_ebb();

        // define entry block
        builder.append_ebb_params_for_function_params(entry_ebb);
        builder.switch_to_block(entry_ebb);
        let vm = builder.ebb_params(entry_ebb)[0];
        let memory = builder.ebb_params(entry_ebb)[1];

        let dyneval = ctx
            .module
            .declare_func_in_func(ctx.dyneval, &mut builder.func);

        Compiler {
            module: &mut ctx.module,

            memory,
            vm,

            dyneval,

            builder,
        }
    }

    /// finish compilation
    pub fn finalize(mut self) {
        //println!("{}", self.builder.display(None));
        self.builder.seal_all_blocks();
        self.builder.finalize();
    }

    pub fn build_blocks(&mut self, blocks: &HashMap<usize, Vec<FixOp>>) {
        let ebbs: HashMap<_, _> = blocks
            .keys()
            .map(|k| (k, self.builder.create_ebb()))
            .collect();

        // there must always be a 0-block
        self.builder.ins().jump(ebbs[&0], &[]);

        for k in blocks.keys() {
            self.builder.switch_to_block(ebbs[k]);
            for op in &blocks[k] {
                match op {
                    FixOp::Dynamic(ofs) => {
                        let ofs = self.builder.ins().iconst(cl::types::I64, *ofs as i64);
                        self.builder.ins().call(self.dyneval, &[self.vm, ofs]);
                    }
                    FixOp::Halt => {
                        self.builder.ins().return_(&[]);
                    }
                    FixOp::Set(a, c) => {
                        let a = self.get_operand(a);
                        self.set_operand(c, a);
                    }
                    FixOp::Add(a, b, c) => {
                        let a = self.get_operand(a);
                        let b = self.get_operand(b);
                        let r = self.builder.ins().iadd(a, b);
                        self.set_operand(c, r);
                    }
                    FixOp::Mul(a, b, c) => {
                        let a = self.get_operand(a);
                        let b = self.get_operand(b);
                        let r = self.builder.ins().imul(a, b);
                        self.set_operand(c, r);
                    }
                    FixOp::Equ(a, b, c) => {
                        let a = self.get_operand(a);
                        let b = self.get_operand(b);
                        let r = self.builder.ins().icmp(IntCC::Equal, a, b);
                        self.set_operand(c, r);
                    }
                    FixOp::Ltn(a, b, c) => {
                        let a = self.get_operand(a);
                        let b = self.get_operand(b);
                        let r = self.builder.ins().icmp(IntCC::SignedLessThan, a, b);
                        self.set_operand(c, r);
                    }
                    _ => unimplemented!("{:?}", op),
                }
            }
        }
    }

    fn get_operand(&mut self, o: &Operand<i64>) -> cl::Value {
        match o {
            Operand::Imm(i) => self.builder.ins().iconst(cl::types::I64, *i),
            Operand::Pos(p) => self.builder.ins().load(
                ir::types::I64,
                cl::MemFlags::new(),
                self.memory,
                (*p * std::mem::size_of::<i64>()) as i32,
            ),
            _ => unimplemented!("{:?}", o),
        }
    }

    fn set_operand(&mut self, o: &Operand<i64>, val: cl::Value) {
        match o {
            Operand::Imm(i) => panic!("write to immediate"),
            Operand::Pos(p) => {
                self.builder.ins().store(
                    cl::MemFlags::new(),
                    val,
                    self.memory,
                    (*p * std::mem::size_of::<i64>()) as i32,
                );
            }
            _ => unimplemented!("{:?}", o),
        }
    }
}

unsafe fn dyneval(vm: &mut Computer, offset: usize) {
    match vm.peek_at(offset) {
        None => println!(" *** invalid dynamic operation *** "),
        Some((op, _)) => {
            if let Some(r) = vm.apply(op) {
                println!("Unsupported dynamic return value: {:?}", r);
            }
        }
    }
}

pub struct CompilerContext {
    module: Module<SimpleJITBackend>,
    module_context: Context,
    function_context: FunctionBuilderContext,

    dyneval: FuncId,
}

impl CompilerContext {
    pub fn new() -> Self {
        let mut module = {
            let mut jit_builder = SimpleJITBuilder::new(default_libcall_names());
            jit_builder.symbol("dyneval", dyneval as *const u8);
            /*for (name, func) in dictionary {
                jit_builder.symbol(name.as_str(), *func as *const u8);
            }*/
            //runtime::init_symbols(&mut jit_builder);
            Module::new(jit_builder)
        };
        let module_context = module.make_context();
        let function_context = cl::FunctionBuilderContext::new();

        let mut dyneval_signature = module.make_signature();
        dyneval_signature
            .params
            .push(cl::AbiParam::new(cl::types::I64));
        dyneval_signature
            .params
            .push(cl::AbiParam::new(cl::types::I64));

        let dyneval = module
            .declare_function("dyneval", Linkage::Import, &dyneval_signature)
            .unwrap();

        CompilerContext {
            dyneval,

            module,
            module_context,
            function_context,
        }
    }

    pub fn compile_program(&mut self, blocks: &HashMap<usize, Vec<FixOp>>) -> IntcodeProgram {
        let signature = Self::intcode_program_signature(&mut self.module);

        let func = self
            .module
            .declare_function("function", Linkage::Export, &signature)
            .unwrap();

        self.module_context.func.signature = signature;

        let mut compiler = Compiler::new(self);
        compiler.build_blocks(blocks);
        compiler.finalize();

        self.module
            .define_function(func, &mut self.module_context)
            .unwrap();
        self.module.clear_context(&mut self.module_context);

        self.module.finalize_definitions();
        let raw_code = self.module.get_finalized_function(func);
        // converting a raw pointer to the compiled function to a typed Rust function pointer
        // is inherently unsafe because there is no way for the compiler to verify the function
        // signature, or to determine what the function does is actually safe.
        let code_ptr = unsafe { std::mem::transmute(raw_code) };
        code_ptr
    }

    pub fn intcode_program_signature(module: &mut Module<SimpleJITBackend>) -> cl::Signature {
        let mut sig = module.make_signature();
        sig.params.push(cl::AbiParam::new(cl::types::I64));
        sig.params.push(cl::AbiParam::new(cl::types::I64));
        sig
    }
}
