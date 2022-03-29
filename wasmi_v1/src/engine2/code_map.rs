//! Datastructure to efficiently store function bodies and their instructions.

use super::ExecInstruction;
use crate::arena::Index;
use alloc::vec::Vec;

/// A reference to a Wasm function body stored in the [`CodeMap`].
#[derive(Debug, Copy, Clone)]
pub struct FuncBody {
    /// The offset within the [`CodeMap`] to the first instruction.
    inst: FirstInstr,
    /// The number of instructions of the [`FuncBody`].
    len: u32,
}

impl FuncBody {
    /// Creates a new [`FuncBody`].
    pub fn new(inst: FirstInstr, len: u32) -> Self {
        Self { inst, len }
    }

    /// Returns the index to the first instruction stored in the [`CodeMap`].
    ///
    /// # Note
    ///
    /// Since instruction of the same function in the [`CodeMap`] are stored
    /// consecutively the only other information required to form the entire
    /// function body is the amount of instructions of the function which is
    /// given by [`FuncBody::len`].
    ///
    /// [`FuncBody::len`]: #method.len
    pub(super) fn inst(self) -> FirstInstr {
        self.inst
    }

    /// Returns the number of instruction of the [`FuncBody`].
    pub(super) fn len(self) -> u32 {
        self.len
    }
}

/// A reference to the first [`Instruction`] of a [`FuncBody`] stored within a [`CodeMap`].
#[derive(Debug, Copy, Clone)]
pub struct FirstInstr(u32);

impl Index for FirstInstr {
    fn into_usize(self) -> usize {
        self.0 as usize
    }

    fn from_usize(value: usize) -> Self {
        assert!(value <= u32::MAX as usize);
        Self(value as u32)
    }
}

/// Datastructure to efficiently store Wasm function bodies.
#[derive(Debug, Default)]
pub struct CodeMap {
    /// The instructions of all allocated function bodies.
    ///
    /// By storing all `wasmi` bytecode instructions in a single
    /// allocation we avoid an indirection when calling a function
    /// compared to a solution that stores instructions of different
    /// function bodies in different allocations.
    ///
    /// Also this improves efficiency of deallocating the [`CodeMap`]
    /// and generally improves data locality.
    insts: Vec<ExecInstruction>,
}

impl CodeMap {
    /// Returns the next [`FuncBody`] index.
    fn next_index(&self) -> FirstInstr {
        FirstInstr::from_usize(self.insts.len())
    }

    /// Allocates a new function body to the [`CodeMap`].
    ///
    /// Returns a reference to the allocated function body that can
    /// be used with [`CodeMap::resolve`] in order to resolve its
    /// instructions.
    pub fn alloc<I>(&mut self, insts: I) -> FuncBody
    where
        I: IntoIterator<Item = ExecInstruction>,
    {
        let inst = self.next_index();
        let insts = insts.into_iter();
        let len_before = self.insts.len();
        self.insts.extend(insts);
        let len_after = self.insts.len();
        let len_insts = (len_after - len_before).try_into().unwrap_or_else(|error| {
            panic!(
                "tried to allocate function with too many instructions ({}): {}",
                len_before, error
            )
        });
        FuncBody::new(inst, len_insts)
    }

    /// Resolves the instruction of the function body.
    ///
    /// # Panics
    ///
    /// If the given `func_body` is invalid for this [`CodeMap`].
    pub fn resolve(&self, func_body: FuncBody) -> ResolvedFuncBody {
        let first_inst = func_body.inst().into_usize();
        let len_insts = func_body.len() as usize;
        let insts = &self.insts[first_inst..(first_inst + len_insts)];
        ResolvedFuncBody { insts }
    }
}

/// A resolved Wasm function body that is stored in a [`CodeMap`].
///
/// Allows to immutably access the `wasmi` instructions of a Wasm
/// function stored in the [`CodeMap`].
///
/// # Dev. Note
///
/// This does not include the [`Instruction::FuncBodyStart`] and
/// [`Instruction::FuncBodyEnd`] instructions surrounding the instructions
/// of a function body in the [`CodeMap`].
#[derive(Debug, Copy, Clone)]
pub struct ResolvedFuncBody<'a> {
    insts: &'a [ExecInstruction],
}

impl ResolvedFuncBody<'_> {
    /// Returns the instruction at the given index.
    ///
    /// # Panics
    ///
    /// If there is no instruction at the given index.
    #[cfg(test)]
    pub fn get(&self, index: usize) -> Option<&ExecInstruction> {
        self.insts.get(index)
    }
}