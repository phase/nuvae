use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_uint;
use crate::ir::{FloatTy, IrType, IrTypeIndex, Module};
use llvm_sys::*;
use llvm_sys::core::*;
use llvm_sys::error::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;
use crate::ast::Type;
use crate::Compiler;

pub struct LLVMBackend<'compiler> {
    compiler: &'compiler Compiler,
    module: &'compiler Module,
    context: LLVMContextRef,
    builder: LLVMBuilderRef,
    llvm_module: LLVMModuleRef,
    type_cache: HashMap<IrTypeIndex, LLVMTypeRef>,
}

impl<'c> LLVMBackend<'c> {
    pub fn new(compiler: &'c Compiler, module: &'c Module) -> Self {
        unsafe {
            LLVMInitializeX86TargetInfo();
            LLVMInitializeX86Target();
            LLVMInitializeX86TargetMC();
            LLVMInitializeX86AsmPrinter();
            let context = LLVMContextCreate();
            let module_name = CString::new(module.name.clone()).unwrap();
            let llvm_module: LLVMModuleRef = LLVMModuleCreateWithNameInContext(module_name.as_ptr(), context);
            let builder: LLVMBuilderRef = LLVMCreateBuilderInContext(context);

            /*
            let target_triple = LLVMGetDefaultTargetTriple();
            let mut target = ptr::null_mut();
            let mut error = ptr::null_mut();
            LLVMGetTargetFromTriple(target_triple, target, error);
            LLVMDisposeErrorMessage(*error);
            let target_machine = LLVMCreateTargetMachine(*target, target_triple, cstr(""), cstr(""), LLVMCodeGenOptLevel::LLVMCodeGenLevelNone, LLVMRelocMode::LLVMRelocDefault, LLVMCodeModel::LLVMCodeModelDefault);
            LLVMDisposeMessage(target_triple);
            */

            Self {
                compiler,
                module,
                context,
                builder,
                llvm_module,
                type_cache: HashMap::new(),
            }
        }
    }

    unsafe fn convert_type(&mut self, type_index: IrTypeIndex) -> LLVMTypeRef {
        if let Some(typ) = self.type_cache.get(&type_index) {
            return *typ;
        }
        let typ = self.module.typ(type_index);
        let llvm_type: LLVMTypeRef = match typ {
            IrType::Bool => LLVMIntTypeInContext(self.context, 1),
            IrType::Int(i) => LLVMIntTypeInContext(self.context, i.bits()),
            IrType::UInt(u) => LLVMIntTypeInContext(self.context, u.bits()),
            IrType::Float(u) => {
                match u {
                    FloatTy::F16 => LLVMHalfTypeInContext(self.context),
                    FloatTy::F32 => LLVMFloatTypeInContext(self.context),
                    FloatTy::F64 => LLVMDoubleTypeInContext(self.context),
                    FloatTy::F128 => LLVMFP128TypeInContext(self.context),
                }
            }
            IrType::Function(f, ret) => {
                let mut args = Vec::with_capacity(f.len());
                for arg in f.iter() {
                    args.push(self.convert_type(arg.clone()));
                }
                let result_type = self.convert_type(ret.clone());
                LLVMFunctionType(result_type, args.as_mut_ptr(), args.len() as c_uint, 0)
            }
            t => panic!("couldn't convert type {:?}", t)
        };

        self.type_cache.insert(type_index, llvm_type);
        llvm_type
    }
}
