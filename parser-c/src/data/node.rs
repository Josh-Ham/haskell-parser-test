// Original file: "Node.hs"
// File auto-generated using Corollary.

#[macro_use] use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data::Position;
// use Language::C::Data::Name;
// use Name;
// use Data::Generics;

#[derive(Clone, Debug, Eq, Ord)]
pub enum NodeInfo {
    OnlyPos(Position, PosLength),
    NodeInfo(Position, PosLength, Name)
}
pub use self::NodeInfo::*;

pub fn eqByName<a>(obj1: a, obj2: a) -> bool {
    ((nodeInfo(obj1)) == (nodeInfo(obj2)))
}

pub fn fileOfNode() -> Option<FilePath> {
    fmap(posFile, justIf(isSourcePos, posOfNode(nodeInfo)))
}

pub fn getLastTokenPos(_0: NodeInfo) -> PosLength {
    match (_0) {
        _0 => {
            lastTok
        },
        _0 => {
            lastTok
        },
    }
}

pub fn internalNode() -> NodeInfo {
    undefNode
}

pub fn lengthOfNode(ni: NodeInfo) -> Option<isize> {
    len
}

pub fn mkNodeInfo(pos: Position, name: Name) -> NodeInfo {
    NodeInfo(pos, (nopos, -(1)), name)
}

pub fn mkNodeInfo_q(pos: Position, lasttok: PosLength, name: Name) -> NodeInfo {
    NodeInfo(pos, lasttok, name)
}

pub fn mkNodeInfoOnlyPos(pos: Position) -> NodeInfo {
    OnlyPos(pos, (nopos, -(1)))
}

pub fn mkNodeInfoPosLen() -> NodeInfo {
    OnlyPos
}

pub fn nameOfNode(_0: NodeInfo) -> Option<Name> {
    match (_0) {
        _0 => {
            None
        },
        _0 => {
            None
        },
    }
}

pub fn posOfNode(ni: NodeInfo) -> Position {
    match ni {
        OnlyPos(pos, _) => {
            pos
        },
        NodeInfo(pos, _, _) => {
            pos
        },
    }
}

pub fn undefNode() -> NodeInfo {
    OnlyPos(nopos, (nopos, -(1)))
}



