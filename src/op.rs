use std::fmt;

use smallvec::SmallVec;
use Atom;
use Uuid;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Terminator {
    // Raw ops are stand-alone within a frame.
    Raw,
    // Query and header ops, as well as reduced ops following them, create a chunk in a frame.
    Query,
    Header,
    // Reduced ops belong to the query/header op before them.
    Reduced,
}

impl fmt::Debug for Terminator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Terminator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Terminator::Raw => f.write_str(";"),
            Terminator::Reduced => f.write_str(","),
            Terminator::Header => f.write_str("!"),
            Terminator::Query => f.write_str("?"),
        }
    }
}

impl Default for Terminator {
    fn default() -> Terminator {
        Terminator::Reduced
    }
}

impl Terminator {
    pub fn from_string(inp: &str) -> Result<Terminator, &'static str> {
        match &inp {
            &";" => Ok(Terminator::Raw),
            &"?" => Ok(Terminator::Query),
            &"!" => Ok(Terminator::Header),
            &"," => Ok(Terminator::Reduced),
            _ => Err("invalid terminator"),
        }
    }

    pub fn from_char(inp: char) -> Result<Terminator, &'static str> {
        Terminator::from_string(&inp.to_string())
    }
}

/// An Op (operation) in RON describes part of the initial state of an object, or a specific
/// action on an object, or some other part related to the Swarm protocol (such as a query or
/// handshake).
///
/// Every op consists of four UUIDs (type, object, event and re), a (possibly empty) sequence of
/// atoms, and a terminator.
#[derive(PartialEq)]
pub struct Op {
    pub ty: Uuid,
    pub object: Uuid,
    pub event: Uuid,
    pub location: Uuid,
    pub atoms: SmallVec<[Atom; 3]>,
    pub term: Terminator,
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "*{}#{}@{}:{}", self.ty, self.object, self.event,
               self.location)?;

        match self.atoms.len() {
            0 => write!(f, "{}", self.term),
            1 => write!(f, "{}{}", self.atoms[0], self.term),
            _ => {
                write!(f, "{}", self.atoms[0])?;
                for atm in self.atoms[1..].iter() {
                    write!(f, ", {}", atm)?;
                }
                write!(f, "{}", self.term)
            }
        }
    }
}

