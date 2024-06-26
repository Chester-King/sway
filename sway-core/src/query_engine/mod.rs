use parking_lot::RwLock;
use std::{collections::HashMap, path::PathBuf, sync::Arc, time::SystemTime};
use sway_error::error::CompileError;
use sway_error::warning::CompileWarning;
use sway_types::IdentUnique;

use crate::decl_engine::{DeclId, DeclRef};
use crate::language::ty::{TyFunctionDecl, TyFunctionSig};
use crate::{Engines, Programs};

pub type ModulePath = Arc<PathBuf>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ModuleCacheKey {
    pub path: Arc<PathBuf>,
    pub include_tests: bool,
}

impl ModuleCacheKey {
    pub fn new(path: Arc<PathBuf>, include_tests: bool) -> Self {
        Self {
            path,
            include_tests,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ModuleCacheEntry {
    pub path: ModulePath,
    pub modified_time: Option<SystemTime>,
    pub hash: u64,
    pub dependencies: Vec<ModulePath>,
    pub include_tests: bool,
    pub version: Option<u64>,
}

pub type ModuleCacheMap = HashMap<ModuleCacheKey, ModuleCacheEntry>;

#[derive(Clone, Debug)]
pub struct ProgramsCacheEntry {
    pub path: Arc<PathBuf>,
    pub programs: Programs,
    pub handler_data: (Vec<CompileError>, Vec<CompileWarning>),
}

pub type ProgramsCacheMap = HashMap<Arc<PathBuf>, ProgramsCacheEntry>;

#[derive(Clone, Debug)]
pub struct FunctionCacheEntry {
    pub fn_decl: DeclRef<DeclId<TyFunctionDecl>>,
}

pub type FunctionsCacheMap = HashMap<(IdentUnique, String), FunctionCacheEntry>;

#[derive(Debug, Default, Clone)]
pub struct QueryEngine {
    // We want the below types wrapped in Arcs to optimize cloning from LSP.
    parse_module_cache: Arc<RwLock<ModuleCacheMap>>,
    programs_cache: Arc<RwLock<ProgramsCacheMap>>,
    function_cache: Arc<RwLock<FunctionsCacheMap>>,
}

impl QueryEngine {
    pub fn get_parse_module_cache_entry(&self, path: &ModuleCacheKey) -> Option<ModuleCacheEntry> {
        let cache = self.parse_module_cache.read();
        cache.get(path).cloned()
    }

    pub fn insert_parse_module_cache_entry(&self, entry: ModuleCacheEntry) {
        let path = entry.path.clone();
        let include_tests = entry.include_tests;
        let key = ModuleCacheKey::new(path, include_tests);
        let mut cache = self.parse_module_cache.write();
        cache.insert(key, entry);
    }

    pub fn get_programs_cache_entry(&self, path: &Arc<PathBuf>) -> Option<ProgramsCacheEntry> {
        let cache = self.programs_cache.read();
        cache.get(path).cloned()
    }

    pub fn insert_programs_cache_entry(&self, entry: ProgramsCacheEntry) {
        let mut cache = self.programs_cache.write();
        cache.insert(entry.path.clone(), entry);
    }

    pub fn get_function(
        &self,
        engines: &Engines,
        ident: IdentUnique,
        sig: TyFunctionSig,
    ) -> Option<DeclRef<DeclId<TyFunctionDecl>>> {
        let cache = self.function_cache.read();
        cache
            .get(&(ident, sig.get_type_str(engines)))
            .map(|s| s.fn_decl.clone())
    }

    pub fn insert_function(
        &self,
        engines: &Engines,
        ident: IdentUnique,
        sig: TyFunctionSig,
        fn_decl: DeclRef<DeclId<TyFunctionDecl>>,
    ) {
        let mut cache = self.function_cache.write();
        cache.insert(
            (ident, sig.get_type_str(engines)),
            FunctionCacheEntry { fn_decl },
        );
    }
}
