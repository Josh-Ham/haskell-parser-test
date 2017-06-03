#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data::Error;
// use Language::C::Data::Node;
// use Language::C::Data::Ident;
// use Language::C::Pretty;
// use Language::C::Syntax;
// use Language::C::Analysis::AstAnalysis;
// use tExpr;
// use Language::C::Analysis::DefTable;
// use TagFwdDecl;
// use Language::C::Analysis::Export;
// use Language::C::Analysis::SemError;
// use Language::C::Analysis::SemRep;
// use Language::C::Analysis::TravMonad;
// use Data::Foldable;
// use Data::Traversable;
// use Control::Monad;
// use liftM;
// use Data::List;
// use intersperse;
// use Data::Map;
// use Map;
// use Data::Map;
// use Text::PrettyPrint::HughesPJ;

#[derive(Debug, Eq, Ord, Read)]
pub enum StorageSpec {
    NoStorageSpec,
    AutoSpec,
    RegSpec,
    ThreadSpec,
    StaticSpec(bool),
    ExternSpec(bool)
}
pub use self::StorageSpec::*;

pub struct VarDeclInfo(VarName, bool, StorageSpec, Attributes, Type, NodeInfo);


#[derive(Eq, Ord)]
pub enum NumBaseType {
    NoBaseType,
    BaseChar,
    BaseInt,
    BaseFloat,
    BaseDouble
}
pub use self::NumBaseType::*;

#[derive(Eq, Ord)]
pub enum SignSpec {
    NoSignSpec,
    Signed,
    Unsigned
}
pub use self::SignSpec::*;

#[derive(Eq, Ord)]
pub enum SizeMod {
    NoSizeMod,
    ShortMod,
    LongMod,
    LongLongMod
}
pub use self::SizeMod::*;

pub struct NumTypeSpec{
    base: NumBaseType,
    signSpec: SignSpec,
    sizeMod: SizeMod,
    isComplex: bool
}
fn base(a: NumTypeSpec) -> NumBaseType { a.base }
fn signSpec(a: NumTypeSpec) -> SignSpec { a.signSpec }
fn sizeMod(a: NumTypeSpec) -> SizeMod { a.sizeMod }
fn isComplex(a: NumTypeSpec) -> bool { a.isComplex }

pub enum TypeSpecAnalysis {
    TSNone,
    TSVoid,
    TSBool,
    TSNum(NumTypeSpec),
    TSTypeDef(TypeDefRef),
    TSType(Type),
    TSNonBasic(CTypeSpec)
}
pub use self::TypeSpecAnalysis::*;

pub fn analyseTypeDecl<a>(CDecl(declspecs, declrs, node): CDecl) -> m<Type> {
    /* Expr::Error */ Error
}

pub fn analyseVarDecl<a>(handle_sue_def: bool, storage_specs: Vec<CStorageSpec>, decl_attrs: Vec<CAttr>, typequals: Vec<CTypeQual>, canonTySpecs: TypeSpecAnalysis, inline: bool, CDeclr(name_opt, derived_declrs, asmname_opt, declr_attrs, node): CDeclr, oldstyle_params: Vec<CDecl>, init_opt: Option<CInit>) -> m<VarDeclInfo> {
    /*do*/ {
        let storage_spec = canonicalStorageSpec(storage_specs);

        let typ = tType(handle_sue_def, node, typequals, canonTySpecs, derived_declrs, oldstyle_params);

        let attrs_q = mapM(tAttr, (__op_addadd(decl_attrs, declr_attrs)));

        let name = mkVarName(node, name_opt, asmname_opt);

        return(VarDeclInfo(name, inline, storage_spec, attrs_q, typ, node))
    }
}

pub fn analyseVarDecl_q<a>(handle_sue_def: bool, declspecs: Vec<CDeclSpec>, declr: CDeclr, oldstyle: Vec<CDecl>, init_opt: Option<CInit>) -> m<VarDeclInfo> {
    /*do*/ {
        let (storage_specs, attrs, type_quals, type_specs, inline) = partitionDeclSpecs(declspecs);

        let canonTySpecs = canonicalTypeSpec(type_specs);

        analyseVarDecl(handle_sue_def, storage_specs, attrs, type_quals, canonTySpecs, inline, declr, oldstyle, init_opt)
    }
}

pub fn canonicalStorageSpec<a>(storagespecs: Vec<CStorageSpec>) -> m<StorageSpec> {
    liftM(elideAuto)(foldrM(updStorage, NoStorageSpec, storagespecs))
}

pub fn canonicalTypeSpec<a>() -> m<TypeSpecAnalysis> {
    foldrM(go, TSNone)
}

pub fn computeParamStorage<a>(_0: NodeInfo, _1: StorageSpec) -> Either<BadSpecifierError, Storage> {
    match (_0, _1) {
        (_, NoStorageSpec) => {
            Right((Auto(false)))
        },
        (_, RegSpec) => {
            Right((Auto(true)))
        },
        (node, spec) => {
            Left(badSpecifierError(node)(__op_addadd("Bad storage specified for parameter: ".to_string(), show(spec))))
        },
    }
}

pub fn emptyDeclr<a>(node: NodeInfo) -> CDeclr {
    CDeclr(None, vec![], None, vec![], node)
}

pub fn emptyNumTypeSpec<a>() -> NumTypeSpec {
    NumTypeSpec {
        base: NoBaseType,
        signSpec: NoSignSpec,
        sizeMod: NoSizeMod,
        isComplex: false
    }
}

pub fn getOnlyDeclr<a>(_0: CDecl) -> m<CDeclr> {
    match (_0) {
        CDecl(_, [(Some(declr), _, _)], _) => {
            declr
        },
        CDecl(_, _, node) => {
            internalErr("getOnlyDeclr: declaration doesn\'t have a unique declarator".to_string())
        },
    }
}

pub fn hasThreadLocalSpec<a>(_0: StorageSpec) -> bool {
    match (_0) {
        ThreadSpec => {
            true
        },
        StaticSpec(b) => {
            b
        },
        ExternSpec(b) => {
            b
        },
        _ => {
            false
        },
    }
}

pub fn isTypeDef<a>(declspecs: Vec<CDeclSpec>) -> bool {
    not(null(/* Expr::Generator */ Generator))
}

pub fn mergeOldStyle<a>(_0: NodeInfo, _1: Vec<CDecl>, _2: Vec<CDerivedDeclr>) -> m<Vec<CDerivedDeclr>> {
    match (_0, _1, _2) {
        (_node, [], declrs) => {
            declrs
        },
        (node, oldstyle_params, [CFunDeclr(params, attrs, fdnode), dds]) => {
            match params {
                Left(list) => {
                    /*do*/ {
                        let oldstyle_params_q = liftM(concat)(mapM(splitCDecl, oldstyle_params));

                        let param_map = liftM(Map::fromList)(mapM(attachNameOfDecl, oldstyle_params_q));

                        let (newstyle_params, param_map_q) = foldrM(insertParamDecl, (vec![], param_map), list);

                        when((not(Map::null(param_map_q))))(astError(node)(__op_addadd("declarations for parameter(s) ".to_string(), __op_addadd(showParamMap(param_map_q), " but no such parameter".to_string()))));
                        return((__op_concat(CFunDeclr((Right((newstyle_params, false))), attrs, fdnode), dds)))
                    }
                },
                Right(_newstyle) => {
                    astError(node, "oldstyle parameter list, but newstyle function declaration".to_string())
                },
            }
        },
        (node, _, _) => {
            astError(node, "oldstyle parameter list, but not function type".to_string())
        },
    }
}

pub fn mergeTypeAttributes<a>(node_info: NodeInfo, quals: TypeQuals, attrs: Vec<Attr>, typ: Type) -> m<Type> {
    match typ {
        DirectType(ty_name, quals_q, attrs_q) => {
            merge(quals_q, attrs_q)(mkDirect(ty_name))
        },
        PtrType(ty, quals_q, attrs_q) => {
            merge(quals_q, attrs_q)(PtrType(ty))
        },
        ArrayType(ty, array_sz, quals_q, attrs_q) => {
            merge(quals_q, attrs_q)(ArrayType(ty, array_sz))
        },
        FunctionType(FunType(return_ty, params, inline), attrs_q) => {
            return(FunctionType((FunType(return_ty, params, inline)), (__op_addadd(attrs_q, attrs))))
        },
        TypeDefType(tdr, quals_q, attrs_q) => {
            merge(quals_q, attrs_q)(TypeDefType(tdr))
        },
    }
}

pub fn mkVarName<a>(_0: NodeInfo, _1: Option<Ident>, _2: Option<AsmName>) -> m<VarName> {
    match (_0, _1, _2) {
        (node, None, _) => {
            NoName
        },
        (node, Some(n), asm) => {
            return(VarName(n, asm))
        },
    }
}

pub fn nameOfDecl<a>(d: CDecl) -> m<Ident> {
    __op_bind(getOnlyDeclr(d), |declr| { match declr {
            CDeclr(Some(name), _, _, _, _) => {
                name
            },
            CDeclr(None, _, _, _, node) => {
                internalErr("nameOfDecl: abstract declarator".to_string())
            },
        } })
}

pub fn splitCDecl<a>(decl: CDecl, __OP__: m<Vec<CDecl>>) -> m<Vec<CDecl>> {
    match declrs {
        [] => {
            internalErr("splitCDecl applied to empty declaration".to_string())
        },
        [declr] => {
            vec![decl]
        },
        [d1, ds] => {
            {
                let declspecs_q = __map!(elideSUEDef, declspecs);

            return(__op_concat((CDecl(declspecs, vec![d1], node)), /* Expr::Generator */ Generator))            }
        },
    }
}

pub fn tArraySize<a>(_0: CArrSize) -> m<ArraySize> {
    match (_0) {
        CNoArrSize(false) => {
            (UnknownArraySize(false))
        },
        CNoArrSize(true) => {
            (UnknownArraySize(true))
        },
        CArrSize(__static, szexpr) => {
            liftM((ArraySize(__static)), (szexpr))
        },
    }
}

pub fn tAttr<a>(CAttr(name, cexpr, node): CAttr) -> m<Attr> {
    return(Attr(name, cexpr, node))
}

pub fn tCompType<a>(tag: SUERef, sue_ref: CompTyKind, member_decls: Vec<CDecl>, attrs: Attributes, node: NodeInfo) -> m<CompType> {
    ap((CompType(tag, sue_ref)), ap((concatMapM(tMemberDecls, member_decls)), ap((attrs), (node))))
}

pub fn tCompTypeDecl<a>(handle_def: bool, CStruct(tag, ident_opt, member_decls_opt, attrs, node_info): CStructUnion) -> m<CompTypeRef> {
    /*do*/ {
        let sue_ref = createSUERef(node_info, ident_opt);

        let tag_q = tTag(tag);

        let attrs_q = mapM(tAttr, attrs);

        let decl = CompTypeRef(sue_ref, tag_q, node_info);

        handleTagDecl((CompDecl(decl)));
        when(handle_def())(/*do*/ {
            maybeM(member_decls_opt)(|decls| { __op_bind(tCompType(sue_ref, tag_q, decls, attrs_q(), node_info), handleTagDef::CompDef()) })
        });
        decl
    }
}

pub fn tDirectType<a>(handle_sue_def: bool, node: NodeInfo, ty_quals: Vec<CTypeQual>, canonTySpec: TypeSpecAnalysis) -> m<Type> {
    /*do*/ {
        let (quals, attrs) = tTypeQuals(ty_quals);

        let baseType = |ty_name| {
            DirectType(ty_name, quals, attrs)
        };

        match canonTySpec {
            TSNone => {
                return(baseType((TyIntegral(TyInt))))
            },
            TSVoid => {
                return(baseType(TyVoid))
            },
            TSBool => {
                return(baseType((TyIntegral(TyBool))))
            },
            TSNum(tsnum) => {
                /*do*/ {
                    let numType = tNumType(tsnum);

                    baseType(match numType {
                        Left((floatType, iscomplex)) if iscomplex => { TyComplex(floatType) }
                        Left((floatType, iscomplex)) => { TyFloating(floatType) }
                        Right(intType) => {
                            TyIntegral(intType)
                        },
                    })
                }
            },
            TSTypeDef(tdr) => {
                return(TypeDefType(tdr, quals, attrs))
            },
            TSNonBasic(CSUType(su, _tnode)) => {
                liftM((baseType(TyComp)))(tCompTypeDecl(handle_sue_def, su))
            },
            TSNonBasic(CEnumType(__enum, _tnode)) => {
                liftM((baseType(TyEnum)))(tEnumTypeDecl(handle_sue_def, __enum))
            },
            TSType(t) => {
                mergeTypeAttributes(node, quals, attrs, t)
            },
            TSNonBasic(_) => {
                astError(node, "Unexpected typespec".to_string())
            },
        }
    }
}

pub fn tEnumType<a>(sue_ref: SUERef, enumerators: Vec<(Ident, Option<CExpr>)>, attrs: Attributes, node: NodeInfo) -> m<EnumType> {
    /*do*/ {
        mapM_(handleEnumeratorDef, enumerators_q);
        ty
    }
}

pub fn tEnumTypeDecl<a>(handle_def: bool, CEnum(ident_opt, enumerators_opt, attrs, node_info): CEnum) -> m<EnumTypeRef> {
    /* Expr::Error */ Error
}

pub fn tMemberDecls<a>(_0: CDecl) -> m<Vec<MemberDecl>> {
    match (_0) {
        CDecl(declspecs, [], node) => {
            /*do*/ {
                let (storage_specs, _attrs, typequals, typespecs, is_inline) = partitionDeclSpecs(declspecs);

                when(is_inline)(astError(node, "member declaration with inline specifier".to_string()));
                let canonTySpecs = canonicalTypeSpec(typespecs);

                let ty = tType(true, node, typequals, canonTySpecs, vec![], vec![]);

                match ty {
                    DirectType(TyComp(_), _, _) => {
                        return(vec![
                            MemberDecl((VarDecl(NoName, (DeclAttrs(false, NoStorage, vec![])), ty)), None, node),
                        ])
                    },
                    _ => {
                        astError(node, "anonymous member has a non-composite type".to_string())
                    },
                }
            }
        },
        CDecl(declspecs, declrs, node) => {
            mapM((uncurry(tMemberDecl)), (zip((__op_concat(true, repeat(false))), declrs)))
        },
    }
}

pub fn tNumType<a>(NumTypeSpec(basetype, sgn, sz, iscomplex): NumTypeSpec) -> m<Either<(FloatType, bool), IntType>> {
    match (basetype, sgn, sz) {
        (BaseChar, _, NoSizeMod) if Signed => { intType(TySChar) }
        (BaseChar, _, NoSizeMod) if Unsigned => { intType(TyUChar) }
        (BaseChar, _, NoSizeMod) => { intType(TyChar) }
        (intbase, _, NoSizeMod) if optBase(BaseInt, intbase) => { intType(match sgn {
            Unsigned => {
                TyUInt
            },
            _ => {
                TyInt
            },
        }) }
        (intbase, signed, sizemod) if optBase(BaseInt, intbase) && optSign(Signed, signed) => { intType(match sizemod {
            ShortMod => {
                TyShort
            },
            LongMod => {
                TyLong
            },
            LongLongMod => {
                TyLLong
            },
            _ => {
                internalErr("numTypeMapping: unexpected pattern matching error".to_string())
            },
        }) }
        (intbase, Unsigned, sizemod) if optBase(BaseInt, intbase) => { intType(match sizemod {
            ShortMod => {
                TyUShort
            },
            LongMod => {
                TyULong
            },
            LongLongMod => {
                TyULLong
            },
            _ => {
                internalErr("numTypeMapping: unexpected pattern matching error".to_string())
            },
        }) }
        (BaseFloat, NoSignSpec, NoSizeMod) => {
            floatType(TyFloat)
        },
        (BaseDouble, NoSignSpec, NoSizeMod) => {
            floatType(TyDouble)
        },
        (BaseDouble, NoSignSpec, LongMod) => {
            floatType(TyLDouble)
        },
        (_, _, _) => {
            __error!("Bad AST analysis".to_string())
        },
    }
}

pub fn tParamDecl<a>(CDecl(declspecs, declrs, node): CDecl) -> m<ParamDecl> {
    /*do*/ {
        let declr = getParamDeclr;

        let VarDeclInfo(name, is_inline, storage_spec, attrs, ty, declr_node) = analyseVarDecl_q(true, declspecs, declr, vec![], None);

        when(is_inline())(throwTravError((badSpecifierError(node, "parameter declaration with inline specifier".to_string()))));
        let storage = throwOnLeft(computeParamStorage(node, storage_spec));

        let paramDecl = mkParamDecl(name, storage, attrs, ty, declr_node);

        return(paramDecl)
    }
}

pub fn tTag<a>(_0: CStructTag) -> CompTyKind {
    match (_0) {
        CStructTag => {
            StructTag
        },
        CUnionTag => {
            UnionTag
        },
    }
}

pub fn tType<a>(handle_sue_def: bool, top_node: NodeInfo, typequals: Vec<CTypeQual>, canonTySpecs: TypeSpecAnalysis, derived_declrs: Vec<CDerivedDeclr>, oldstyle_params: Vec<CDecl>) -> m<Type> {
    __op_bind(mergeOldStyle(top_node, oldstyle_params, derived_declrs), buildType)
}

pub fn tTypeQuals<a>() -> m<(TypeQuals, Attributes)> {
    foldrM(go, (noTypeQuals, vec![]))
}

pub fn typeDefRef<a>(t_node: NodeInfo, name: Ident) -> m<TypeDefRef> {
    __op_bind(lookupTypeDef(name), |ty| { (TypeDefRef(name, (Some(ty)), t_node)) })
}



