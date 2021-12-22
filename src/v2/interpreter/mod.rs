//! The `wasmi` interpreter.

#![allow(dead_code)] // TODO: remove

pub mod call_stack;
pub mod code_map;
pub mod inst_builder;
pub mod isa;
pub mod value_stack;

#[allow(unused_imports)]
use self::{
    call_stack::{CallStack, CallStackError, FunctionFrame},
    code_map::{CodeMap, FuncBody, ResolvedFuncBody},
    inst_builder::{InstructionIdx, InstructionsBuilder},
    isa::{DropKeep, Instruction, Target},
    value_stack::{FromStackEntry, StackEntry, ValueStack},
};
use super::Func;
use alloc::sync::Arc;
use spin::mutex::Mutex;

/// The outcome of a `wasmi` instruction execution.
///
/// # Note
///
/// This signals to the `wasmi` interpreter what to do after the
/// instruction has been successfully executed.
pub enum ExecutionOutcome {
    /// Continue with next instruction.
    Continue,
    /// Branch to an instruction at the given position.
    Branch(Target),
    /// Execute function call.
    ExecuteCall(Func),
    /// Return from current function block.
    Return(DropKeep),
}

/// The `wasmi` interpreter.
///
/// # Note
///
/// This structure is intentionally cheap to copy.
/// Most of its API has a `&self` receiver, so can be shared easily.
#[derive(Debug)]
pub struct Interpreter {
    inner: Arc<Mutex<InterpreterInner>>,
}

impl Interpreter {
    /// Allocates the instructions of a Wasm function body to the [`Interpreter`].
    ///
    /// Returns a [`FuncBody`] reference to the allocated function body.
    pub(super) fn alloc_func_body<I>(&self, insts: I) -> FuncBody
    where
        I: IntoIterator<Item = Instruction>,
        I::IntoIter: ExactSizeIterator,
    {
        self.inner.lock().alloc_func_body(insts)
    }
}

/// The internal state of the `wasmi` interpreter.
#[derive(Debug)]
pub struct InterpreterInner {
    value_stack: ValueStack,
    call_stack: CallStack,
    code_map: CodeMap,
}

impl InterpreterInner {
    /// Allocates the instructions of a Wasm function body to the [`Interpreter`].
    ///
    /// Returns a [`FuncBody`] reference to the allocated function body.
    pub fn alloc_func_body<I>(&mut self, insts: I) -> FuncBody
    where
        I: IntoIterator<Item = Instruction>,
        I::IntoIter: ExactSizeIterator,
    {
        self.code_map.alloc(insts)
    }
}
