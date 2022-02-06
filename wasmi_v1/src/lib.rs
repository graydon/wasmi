//! The `wasmi` virtual machine definitions.
//!
//! These closely mirror the WebAssembly specification definitions.
//! The overall structure is heavily inspired by the `wasmtime` virtual
//! machine architecture.

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

#[macro_use]
mod foreach_tuple;

mod arena;
mod engine;
mod error;
mod external;
mod func;
mod func_type;
mod global;
mod instance;
mod linker;
mod memory;
mod module;
mod module2;
mod store;
mod table;

/// Definitions from the `wasmi_core` crate.
#[doc(inline)]
pub use wasmi_core as core;

/// Defines some errors that may occur upon interaction with `wasmi`.
pub mod errors {
    pub use super::{
        func::FuncError,
        global::GlobalError,
        linker::LinkerError,
        memory::MemoryError,
        module::{InstantiationError, TranslationError},
        module2::InstantiationError as InstantiationError2,
        table::TableError,
    };
}

use self::{
    arena::{GuardedEntity, Index},
    engine::{DropKeep, FuncBody, LabelIdx},
    func::{FuncEntity, FuncEntityInternal, FuncIdx},
    global::{GlobalEntity, GlobalIdx},
    instance::{InstanceEntity, InstanceEntityBuilder, InstanceIdx},
    memory::{MemoryEntity, MemoryIdx},
    store::Stored,
    table::{TableEntity, TableIdx},
};
pub use self::{
    engine::{Config, Engine},
    error::Error,
    external::Extern,
    func::{Caller, Func, TypedFunc, WasmParams, WasmResults},
    func_type::FuncType,
    global::{Global, GlobalType, Mutability},
    instance::{ExportsIter, Instance},
    linker::Linker,
    memory::{Memory, MemoryType},
    module::{InstancePre, Module},
    module2::{InstancePre as InstancePre2, Module as Module2, ModuleError, Read},
    store::{AsContext, AsContextMut, Store, StoreContext, StoreContextMut},
    table::{Table, TableType},
};
