use crate::{AliasId, BlockId, DeclId, Span};

use indexmap::IndexMap;

// TODO: Move the import pattern matching logic here from use/hide commands and
// parse_use/parse_hide

/// Collection of definitions that can be exported from a module
#[derive(Debug, Clone)]
pub struct Overlay {
    pub decls: IndexMap<Vec<u8>, DeclId>,
    pub aliases: IndexMap<Vec<u8>, AliasId>,
    pub env_vars: IndexMap<Vec<u8>, BlockId>,
    pub span: Option<Span>,
}

impl Overlay {
    pub fn new() -> Self {
        Overlay {
            decls: IndexMap::new(),
            aliases: IndexMap::new(),
            env_vars: IndexMap::new(),
            span: None,
        }
    }

    pub fn from_span(span: Span) -> Self {
        Overlay {
            decls: IndexMap::new(),
            aliases: IndexMap::new(),
            env_vars: IndexMap::new(),
            span: Some(span),
        }
    }

    pub fn add_decl(&mut self, name: &[u8], decl_id: DeclId) -> Option<DeclId> {
        self.decls.insert(name.to_vec(), decl_id)
    }

    pub fn add_alias(&mut self, name: &[u8], alias_id: AliasId) -> Option<AliasId> {
        self.aliases.insert(name.to_vec(), alias_id)
    }

    pub fn add_env_var(&mut self, name: &[u8], block_id: BlockId) -> Option<BlockId> {
        self.env_vars.insert(name.to_vec(), block_id)
    }

    pub fn extend(&mut self, other: &Overlay) {
        self.decls.extend(other.decls.clone());
        self.env_vars.extend(other.env_vars.clone());
    }

    pub fn is_empty(&self) -> bool {
        self.decls.is_empty() && self.env_vars.is_empty()
    }

    pub fn get_decl_id(&self, name: &[u8]) -> Option<DeclId> {
        self.decls.get(name).copied()
    }

    pub fn get_alias_id(&self, name: &[u8]) -> Option<AliasId> {
        self.aliases.get(name).copied()
    }

    pub fn has_decl(&self, name: &[u8]) -> bool {
        self.decls.contains_key(name)
    }

    pub fn has_alias(&self, name: &[u8]) -> bool {
        self.aliases.contains_key(name)
    }

    pub fn decl_name_with_head(&self, name: &[u8], head: &[u8]) -> Option<Vec<u8>> {
        if self.has_decl(name) {
            let mut new_name = head.to_vec();
            new_name.push(b' ');
            new_name.extend(name);
            Some(new_name)
        } else {
            None
        }
    }

    pub fn alias_name_with_head(&self, name: &[u8], head: &[u8]) -> Option<Vec<u8>> {
        if self.has_alias(name) {
            let mut new_name = head.to_vec();
            new_name.push(b' ');
            new_name.extend(name);
            Some(new_name)
        } else {
            None
        }
    }

    pub fn decls_with_head(&self, head: &[u8]) -> Vec<(Vec<u8>, DeclId)> {
        self.decls
            .iter()
            .map(|(name, id)| {
                let mut new_name = head.to_vec();
                new_name.push(b' ');
                new_name.extend(name);
                (new_name, *id)
            })
            .collect()
    }

    pub fn decl_names_with_head(&self, head: &[u8]) -> Vec<Vec<u8>> {
        self.decls
            .keys()
            .map(|name| {
                let mut new_name = head.to_vec();
                new_name.push(b' ');
                new_name.extend(name);
                new_name
            })
            .collect()
    }

    pub fn aliases_with_head(&self, head: &[u8]) -> Vec<(Vec<u8>, AliasId)> {
        self.aliases
            .iter()
            .map(|(name, id)| {
                let mut new_name = head.to_vec();
                new_name.push(b' ');
                new_name.extend(name);
                (new_name, *id)
            })
            .collect()
    }

    pub fn alias_names_with_head(&self, head: &[u8]) -> Vec<Vec<u8>> {
        self.aliases
            .keys()
            .map(|name| {
                let mut new_name = head.to_vec();
                new_name.push(b' ');
                new_name.extend(name);
                new_name
            })
            .collect()
    }

    pub fn decls(&self) -> Vec<(Vec<u8>, DeclId)> {
        self.decls
            .iter()
            .map(|(name, id)| (name.clone(), *id))
            .collect()
    }

    pub fn decl_names(&self) -> Vec<Vec<u8>> {
        self.decls.keys().cloned().collect()
    }

    pub fn alias_names(&self) -> Vec<Vec<u8>> {
        self.aliases.keys().cloned().collect()
    }

    pub fn aliases(&self) -> Vec<(Vec<u8>, AliasId)> {
        self.aliases
            .iter()
            .map(|(name, id)| (name.clone(), *id))
            .collect()
    }

    pub fn get_env_var_id(&self, name: &[u8]) -> Option<BlockId> {
        self.env_vars.get(name).copied()
    }

    pub fn has_env_var(&self, name: &[u8]) -> bool {
        self.env_vars.contains_key(name)
    }

    pub fn env_var_with_head(&self, name: &[u8], head: &[u8]) -> Option<(Vec<u8>, BlockId)> {
        if let Some(id) = self.get_env_var_id(name) {
            let mut new_name = head.to_vec();
            new_name.push(b' ');
            new_name.extend(name);
            Some((new_name, id))
        } else {
            None
        }
    }

    pub fn env_vars_with_head(&self, head: &[u8]) -> Vec<(Vec<u8>, BlockId)> {
        self.env_vars
            .iter()
            .map(|(name, id)| {
                let mut new_name = head.to_vec();
                new_name.push(b' ');
                new_name.extend(name);
                (new_name, *id)
            })
            .collect()
    }

    pub fn env_vars(&self) -> Vec<(Vec<u8>, BlockId)> {
        self.env_vars
            .iter()
            .map(|(name, id)| (name.clone(), *id))
            .collect()
    }
}

impl Default for Overlay {
    fn default() -> Self {
        Self::new()
    }
}
