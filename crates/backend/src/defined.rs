use ast;
use datatype::TypeKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportedTypeKind {
    /// The definition of an imported type.
    Definition,
    /// A reference to an imported type.
    Reference,
}

/// Iterate over definitions of and references to imported types in the AST.
pub trait ImportedTypes {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind, ImportedTypeKind);
}

/// Iterate over definitions of imported types in the AST.
pub trait ImportedTypeDefinitions {
    fn imported_type_definitions<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind);
}

impl<T> ImportedTypeDefinitions for T
where
    T: ImportedTypes,
{
    fn imported_type_definitions<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind),
    {
        self.imported_types(&mut |ty, kind| {
            if let ImportedTypeKind::Definition = kind {
                f(ty);
            }
        });
    }
}

/// Iterate over references to imported types in the AST.
pub trait TypeKinderences {
    fn imported_type_references<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind);
}

impl<T> TypeKinderences for T
where
    T: ImportedTypes,
{
    fn imported_type_references<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind),
    {
        self.imported_types(&mut |ty, kind| {
            if let ImportedTypeKind::Reference = kind {
                f(ty);
            }
        });
    }
}

impl ImportedTypes for ast::Program {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind, ImportedTypeKind),
    {
        self.imports.imported_types(f);
        self.type_aliases.imported_types(f);
        self.consts.imported_types(f);
    }
}

impl<T> ImportedTypes for Vec<T>
where
    T: ImportedTypes,
{
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind, ImportedTypeKind),
    {
        for x in self {
            x.imported_types(f);
        }
    }
}

impl ImportedTypes for ast::Import {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind, ImportedTypeKind),
    {
        self.kind.imported_types(f)
    }
}

impl ImportedTypes for ast::ImportKind {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind, ImportedTypeKind),
    {
        match self {
            ast::ImportKind::Static(s) => s.imported_types(f),
            ast::ImportKind::Function(fun) => fun.imported_types(f),
            ast::ImportKind::Type(ty) => ty.imported_types(f),
            ast::ImportKind::Enum(enm) => enm.imported_types(f),
        }
    }
}

impl ImportedTypes for ast::ImportStatic {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind, ImportedTypeKind),
    {
        f(&self.ty, ImportedTypeKind::Reference)
    }
}

impl ImportedTypes for ast::ImportFunction {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind, ImportedTypeKind),
    {
        self.function.imported_types(f);
        self.kind.imported_types(f);
    }
}

impl ImportedTypes for ast::ImportFunctionKind {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind, ImportedTypeKind),
    {
        match self {
            ast::ImportFunctionKind::Method { ty, .. } => f(ty, ImportedTypeKind::Reference),
            ast::ImportFunctionKind::Normal => {}
        }
    }
}

impl ImportedTypes for ast::Function {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind, ImportedTypeKind),
    {
        self.arguments.imported_types(f);
        if let Some(ref r) = self.ret {
            f(r, ImportedTypeKind::Reference)
        }
    }
}

impl ImportedTypes for ast::ArgCaptured {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind, ImportedTypeKind),
    {
        f(&self.ty, ImportedTypeKind::Reference)
    }
}

impl ImportedTypes for ast::ImportType {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind, ImportedTypeKind),
    {
        f(&TypeKind::Interface(self.name.to_string()), ImportedTypeKind::Definition);
    }
}

impl ImportedTypes for ast::ImportEnum {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind, ImportedTypeKind),
    {
        f(&TypeKind::Enum(self.name.to_string()), ImportedTypeKind::Definition);
    }
}

impl ImportedTypes for ast::TypeAlias {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind, ImportedTypeKind),
    {
        f(&self.src, ImportedTypeKind::Reference);
    }
}

impl ImportedTypes for ast::Const {
    fn imported_types<F>(&self, f: &mut F)
    where
        F: FnMut(&TypeKind, ImportedTypeKind),
    {
        f(&self.ty, ImportedTypeKind::Reference)
    }
}

/// Remove any methods, statics, &c, that reference types that are *not*
/// defined.
pub trait RemoveUndefinedImports {
    fn remove_undefined_imports<F>(&mut self, is_defined: &F)
    where
        F: Fn(&TypeKind) -> bool;
}

impl RemoveUndefinedImports for ast::Program {
    fn remove_undefined_imports<F>(&mut self, is_defined: &F)
    where
        F: Fn(&TypeKind) -> bool,
    {
        self.imports.remove_undefined_imports(is_defined);
        self.type_aliases.remove_undefined_imports(is_defined);
        self.consts.remove_undefined_imports(is_defined);
    }
}

impl<T> RemoveUndefinedImports for Vec<T>
where
    T: TypeKinderences,
{
    fn remove_undefined_imports<F>(&mut self, is_defined: &F)
    where
        F: Fn(&TypeKind) -> bool,
    {
        self.retain(|x| {
            let mut all_defined = true;
            x.imported_type_references(&mut |id| {
                if all_defined {
                    if !is_defined(id) {
                        info!("removing due to {:?} not being defined", id);
                        all_defined = false;
                    }
                }
            });
            all_defined
        });
    }
}
