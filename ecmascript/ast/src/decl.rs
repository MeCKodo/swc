use crate::{
    class::Class,
    expr::Expr,
    function::Function,
    ident::Ident,
    pat::Pat,
    typescript::{TsEnumDecl, TsInterfaceDecl, TsModuleDecl, TsTypeAliasDecl},
};
#[cfg(feature = "fold")]
use swc_common::Fold;
use swc_common::{ast_node, Span};

#[ast_node]
pub enum Decl {
    Class(ClassDecl),
    Fn(FnDecl),
    Var(VarDecl),
    TsInterface(TsInterfaceDecl),
    TsTypeAlias(TsTypeAliasDecl),
    TsEnum(TsEnumDecl),
    TsModule(TsModuleDecl),
}

#[ast_node]
pub struct FnDecl {
    pub ident: Ident,
    pub declare: bool,
    #[span]
    pub function: Function,
}

#[ast_node]
pub struct ClassDecl {
    pub ident: Ident,
    pub declare: bool,
    #[span]
    pub class: Class,
}

#[ast_node]
pub struct VarDecl {
    pub span: Span,
    pub kind: VarDeclKind,
    pub declare: bool,
    pub decls: Vec<VarDeclarator>,
}

#[derive(StringEnum, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "fold", derive(Fold))]
pub enum VarDeclKind {
    /// `var`
    Var,
    /// `let`
    Let,
    /// `const`
    Const,
}

#[ast_node]
pub struct VarDeclarator {
    pub span: Span,

    pub name: Pat,
    /// Initialization expresion.
    pub init: Option<(Box<Expr>)>,

    /// Typescript onpy
    pub definite: bool,
}
