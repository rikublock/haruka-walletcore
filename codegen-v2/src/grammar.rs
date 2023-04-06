use super::reader::{Reader, ReaderBranch, ReaderPending, ReaderStaged};
use super::{Error, Result};

pub trait ParseTree {
    type Derivation;

    fn derive(reader: Reader<'_>) -> Result<DerivationResult<'_, Self::Derivation>>;
}

#[derive(Debug, Clone)]
pub struct DerivationResult<'a, T> {
    pub derived: T,
    pub branch: ReaderBranch<'a>,
}

pub enum EitherOr<T, D> {
    Either(T),
    Or(D),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GType {
    Bool,
    Char,
    Int,
    Struct(GStruct),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GStruct;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GEof;

pub type GSeparator = Continuum<GSeparatorItem>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GSeparatorItem {
    Space,
    Newline,
    Tab,
}

pub enum GParamItem {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GParamItemWithMarker {
    pub ty: GType,
    pub marker: GMarker,
    pub name: GParamName,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GParamItemWithoutMarker {
    pub ty: GType,
    pub name: GParamName,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GParamName(String);

impl From<String> for GParamName {
    fn from(string: String) -> Self {
        GParamName(string)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GMarker(String);

impl From<String> for GMarker {
    fn from(string: String) -> Self {
        GMarker(string)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Continuum<T> {
    Thing(T),
    Next(ContinuumNext<T>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContinuumNext<T> {
    thing: T,
    next: Box<Continuum<T>>,
}

impl<T: ParseTree, D: ParseTree> ParseTree for EitherOr<T, D> {
    type Derivation = EitherOr<T::Derivation, D::Derivation>;

    fn derive<'a>(reader: Reader<'_>) -> Result<DerivationResult<'_, Self::Derivation>> {
        let (pending, checked_out) = reader.checkout();
        if let Ok(res) = T::derive(checked_out) {
            return Ok(DerivationResult {
                derived: EitherOr::Either(res.derived),
                branch: res.branch,
            });
        }

        let reader = pending.discard();

        if let Ok(res) = D::derive(reader) {
            return Ok(DerivationResult {
                derived: EitherOr::Or(res.derived),
                branch: res.branch,
            });
        }

        Err(Error::Todo)
    }
}

impl ParseTree for GEof {
    type Derivation = Self;

    fn derive<'a>(reader: Reader<'_>) -> Result<DerivationResult<'_, Self::Derivation>> {
        let (slice, handle) = reader.read_amt(1)?;

        if slice.is_some() {
            return Err(Error::Todo);
        }

        Ok(DerivationResult {
            derived: GEof,
            branch: handle.commit().into_branch(),
        })
    }
}

impl ParseTree for GType {
    type Derivation = Self;

    fn derive<'a>(mut reader: Reader<'_>) -> Result<DerivationResult<'_, Self::Derivation>> {
        // Read data until the separator is reached, then return the sub-slice
        // leading up to it.
        let (slice, handle) = reader.read_until::<EitherOr<GSeparator, GEof>>()?;
        dbg!(&slice);

        let derived = match slice.as_str() {
            "bool" => GType::Bool,
            "char" => GType::Char,
            "int" => GType::Int,
            _ => {
                // Rollback reader, retry with the next derivation attempt.
                let reader = handle.reset();

                if let Ok(res) = GStruct::derive(reader) {
                    return Ok(DerivationResult {
                        derived: GType::Struct(res.derived),
                        branch: res.branch,
                    });
                }

                return Err(Error::Todo);
            }
        };

        Ok(DerivationResult {
            derived,
            branch: handle.commit().into_branch(),
        })
    }
}

impl ParseTree for GStruct {
    type Derivation = Self;

    fn derive<'a>(_driver: Reader<'_>) -> Result<DerivationResult<'_, Self::Derivation>> {
        todo!()
    }
}

impl ParseTree for GSeparatorItem {
    type Derivation = Self;

    fn derive<'a>(reader: Reader<'_>) -> Result<DerivationResult<'_, Self::Derivation>> {
        let (slice, handle) = reader.read_amt(1)?;
        dbg!(&slice, &handle);
        let slice = match slice {
            Some(string) => string,
            None => {
                return Err(Error::Todo);
            }
        };

        let derived = match slice.as_str() {
            " " => GSeparatorItem::Space,
            "\n" => GSeparatorItem::Newline,
            "\t" => GSeparatorItem::Tab,
            _ => return Err(Error::Todo),
        };

        Ok(DerivationResult {
            derived,
            branch: handle.commit().into_branch(),
        })
    }
}

impl ParseTree for GParamItemWithMarker {
    type Derivation = Self;

    fn derive<'a>(mut reader: Reader<'_>) -> Result<DerivationResult<'_, Self::Derivation>> {
        // Derive parameter type.
        let (pending, checked_out) = reader.checkout();
        let ty_res = GType::derive(checked_out)?;
        reader = pending.merge(ty_res.branch);

        // Ignore leading separators.
        let (pending, checked_out) = reader.checkout();
        if let Ok(res) = EitherOr::<GSeparator, GEof>::derive(checked_out) {
            reader = pending.merge(res.branch);
        } else {
            reader = pending.discard();
        }

        // Derive marker, ignore leading separators.
        let (pending, checked_out) = reader.checkout();
        let marker_res = GMarker::derive(checked_out)?;
        reader = pending.merge(marker_res.branch);

        // Ignore leading separators.
        let (pending, checked_out) = reader.checkout();
        if let Ok(res) = EitherOr::<GSeparator, GEof>::derive(checked_out) {
            reader = pending.merge(res.branch);
        } else {
            reader = pending.discard();
        }

        // Derive parameter name.
        let (pending, checked_out) = reader.checkout();
        let name_res = GParamName::derive(checked_out)?;

        // Everything derived successfully, return.
        Ok(DerivationResult {
            derived: GParamItemWithMarker {
                ty: ty_res.derived,
                marker: marker_res.derived,
                name: name_res.derived,
            },
            branch: pending.merge(name_res.branch).into_branch(),
        })
    }
}

impl ParseTree for GParamItemWithoutMarker {
    type Derivation = Self;

    fn derive<'a>(mut reader: Reader<'_>) -> Result<DerivationResult<'_, Self::Derivation>> {
        // Derive parameter type.
        let (pending, checked_out) = reader.checkout();
        let ty_res = GType::derive(checked_out)?;
        reader = pending.merge(ty_res.branch);

        // Ignore leading separators.
        let (pending, checked_out) = reader.checkout();
        if let Ok(res) = EitherOr::<GSeparator, GEof>::derive(checked_out) {
            reader = pending.merge(res.branch);
        } else {
            reader = pending.discard();
        }

        // Derive parameter name.
        let name_res = GParamName::derive(reader)?;

        // Everything derived successfully, return.
        Ok(DerivationResult {
            derived: GParamItemWithoutMarker {
                ty: ty_res.derived,
                name: name_res.derived,
            },
            branch: name_res.branch,
        })
    }
}

impl ParseTree for GParamName {
    type Derivation = Self;

    fn derive<'a>(reader: Reader<'_>) -> Result<DerivationResult<'_, Self::Derivation>> {
        let (string, handle) = reader.read_until::<EitherOr<GSeparator, GEof>>()?;

        Ok(DerivationResult {
            derived: GParamName(string),
            branch: handle.commit().into_branch(),
        })
    }
}

impl ParseTree for GMarker {
    type Derivation = Self;

    fn derive<'a>(reader: Reader<'_>) -> Result<DerivationResult<'_, Self::Derivation>> {
        let (string, handle) = reader.read_until::<GSeparator>()?;

        Ok(DerivationResult {
            derived: GMarker(string),
            branch: handle.commit().into_branch(),
        })
    }
}

impl<T> Continuum<T> {
    fn add(self, new_thing: T) -> Self {
        match self {
            Continuum::Thing(thing) => Continuum::Next(ContinuumNext {
                thing,
                next: Box::new(Continuum::Thing(new_thing)),
            }),
            Continuum::Next(next) => Continuum::Next(ContinuumNext {
                thing: next.thing,
                next: Box::new(next.next.add(new_thing)),
            }),
        }
    }
}

impl<T: ParseTree<Derivation = T> + std::fmt::Debug> ParseTree for Continuum<T> {
    type Derivation = Continuum<T>;

    fn derive<'a>(mut reader: Reader<'_>) -> Result<DerivationResult<'_, Self::Derivation>> {
        let mut sep_items: Option<Continuum<T::Derivation>> = None;
        loop {
            dbg!(&reader);
            let (pending, checked_out) = reader.checkout();
            if let Ok(res) = T::derive(checked_out) {
                dbg!(&res.branch);
                reader = pending.merge(res.branch);
                dbg!(&reader);

                if let Some(sep) = sep_items {
                    sep_items = Some(sep.add(res.derived));
                } else {
                    sep_items = Some(Continuum::Thing(res.derived));
                }
            } else {
                dbg!(&sep_items);
                if let Some(items) = sep_items {
                    return Ok(DerivationResult {
                        derived: items,
                        branch: pending.discard().into_branch(),
                    });
                } else {
                    return Err(Error::Todo);
                }
            }
        }
    }
}