#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Data::Typeable;
// use Language::C::Analysis::SemRep;
// use Language::C::Data::Error;
// use Language::C::Data::Node;

#[derive(Debug, Error)]
pub struct InvalidASTError(ErrorInfo);


#[derive(Debug, Error)]
pub struct BadSpecifierError(ErrorInfo);


#[derive(Debug)]
pub struct RedefError(ErrorLevel, RedefInfo);


pub struct RedefInfo(String, RedefKind, NodeInfo, NodeInfo);


pub enum RedefKind {
    DuplicateDef,
    DiffKindRedecl,
    ShadowedDef,
    DisagreeLinkage,
    NoLinkageOld
}
pub use self::RedefKind::*;

#[derive(Debug)]
pub struct TypeMismatch(String, (NodeInfo, Type), (NodeInfo, Type));


pub fn badSpecifierError<a>(node_info: NodeInfo, msg: String) -> BadSpecifierError {
    BadSpecifierError((mkErrorInfo(LevelError, msg, node_info)))
}

pub fn invalidAST<a>(node_info: NodeInfo, msg: String) -> InvalidASTError {
    InvalidAST((mkErrorInfo(LevelError, msg, node_info)))
}

pub fn prevDeclMsg<a>(old_node: NodeInfo) -> Vec<String> {
    vec!["The previous declaration was here: ".to_string(), show((posOfNode(old_node)))]
}

pub fn redefErrLabel<a>(RedefInfo(ident, _, _, _): RedefInfo) -> String {
    __op_addadd(ident, " redefined".to_string())
}

pub fn redefErrReason<a>(_0: RedefInfo) -> String {
    match (_0) {
        RedefInfo(ident, DuplicateDef, _, _) => {
            __op_addadd("duplicate definition of ".to_string(), ident)
        },
        RedefInfo(ident, ShadowedDef, _, _) => {
            __op_addadd("this declaration of ".to_string(), __op_addadd(ident, " shadows a previous one".to_string()))
        },
        RedefInfo(ident, DiffKindRedecl, _, _) => {
            __op_addadd(ident, " previously declared as a different kind of symbol".to_string())
        },
        RedefInfo(ident, DisagreeLinkage, _, _) => {
            __op_addadd(ident, " previously declared with different linkage".to_string())
        },
        RedefInfo(ident, NoLinkageOld, _, _) => {
            __op_addadd(ident, " previously declared without linkage".to_string())
        },
    }
}

pub fn redefErrorInfo<a>(lvl: ErrorLevel, info: RedefInfo, __OP__: ErrorInfo) -> ErrorInfo {
    ErrorInfo(lvl, (posOfNode(node)), (__op_addadd(vec![redefErrReason(info)], prevDeclMsg(old_node))))
}

pub fn redefinition<a>(lvl: ErrorLevel, ctx: String, kind: RedefKind, new: NodeInfo, old: NodeInfo) -> RedefError {
    RedefError(lvl, (RedefInfo(ctx, kind, new, old)))
}

pub fn typeMismatch<a>() -> TypeMismatch {
    TypeMismatch
}

pub fn typeMismatchInfo<a>(TypeMismatch(reason, (node1, _ty2), _t2): TypeMismatch) -> ErrorInfo {
    ErrorInfo(LevelError, (posOfNode(node1)), vec![reason])
}



