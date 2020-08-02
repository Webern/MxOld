use crate::error::{Error, Result};
use crate::model::builtin::Builtin::Number;
use crate::xsd::constants::*;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::io::BufRead;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Traits

pub trait PrefixedParse {
    type ParsedThing;

    /// Given a parseable thing, `parseable` return the parsed representation. e.g.
    /// `parse_name("something")` might return a `Something` variant.
    fn parse<S: AsRef<str>>(parseable: S) -> Result<Self::ParsedThing>;

    /// Given a string `s`, of the format "xs:something", and the expected prefix, e.g. in this case
    /// "xs", then parse the string and return the `Something` representation. Returns an error if
    /// the string's prefix does not match `prefix`. e.g. `parse("xsd:something", "foo")` returns an
    /// error because "xsd" does not equal "foo".
    fn parse_prefixed<S1, S2>(s: S1, prefix: S2) -> Result<Self::ParsedThing>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let (ns, val) = split_raw_str(s.as_ref());
        if ns != prefix.as_ref() {
            return raise!("wrong namespace prefix. expected '{}', got '{}'", prefix.as_ref(), ns);
        }
        Self::parse(val)
    }
}

pub trait PrefixedString: Display {
    fn name(&self) -> &str;

    fn as_str(&self, prefix: &str) -> Cow<'_, str> {
        let s = self.name();
        if prefix.is_empty() {
            Cow::Borrowed(s)
        } else {
            Cow::Owned(format!("{}:{}", prefix, s))
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// BaseType

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BaseType {
    Primitive(Primitive),
    Other(String),
}

impl PrefixedParse for BaseType {
    type ParsedThing = BaseType;

    fn parse<S: AsRef<str>>(parseable: S) -> Result<Self::ParsedThing> {
        Ok(BaseType::Primitive(Primitive::parse(parseable)?))
    }

    fn parse_prefixed<S1, S2>(s: S1, prefix: S2) -> Result<Self::ParsedThing>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        match Primitive::parse_prefixed(&s, &prefix) {
            Ok(primitive) => Ok(BaseType::Primitive(primitive)),
            Err(_) => Ok(BaseType::Other(s.as_ref().to_owned())),
        }
    }
}

impl PrefixedString for BaseType {
    fn name(&self) -> &str {
        match self {
            BaseType::Primitive(p) => p.name(),
            BaseType::Other(o) => o.as_str(),
        }
    }
}

impl Display for BaseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Primitive

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Primitive {
    Numeric(Numeric),
    Character(Character),
    DateTime(DateTime),
}

impl PrefixedParse for Primitive {
    type ParsedThing = Primitive;

    fn parse<S: AsRef<str>>(parseable: S) -> Result<Self::ParsedThing> {
        if let Ok(x) = Numeric::parse(&parseable) {
            return Ok(Self::Numeric(x));
        }
        if let Ok(x) = Character::parse(&parseable) {
            return Ok(Self::Character(x));
        }
        if let Ok(x) = DateTime::parse(&parseable) {
            return Ok(Self::DateTime(x));
        }
        raise!("'{}' could not be parsed as a primitive type", parseable.as_ref())
    }
}

impl PrefixedString for Primitive {
    fn name(&self) -> &str {
        match self {
            Primitive::Numeric(x) => x.name(),
            Primitive::Character(x) => x.name(),
            Primitive::DateTime(x) => x.name(),
        }
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Numeric Types

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Numeric {
    Byte,
    Decimal,
    Int,
    Integer,
    Long,
    NegativeInteger,
    NonNegativeInteger,
    NonPositiveInteger,
    PositiveInteger,
    Short,
    UnsignedLong,
    UnsignedInt,
    UnsignedShort,
    UnsignedByte,
}

impl Default for Numeric {
    fn default() -> Self {
        Numeric::Int
    }
}

impl PrefixedParse for Numeric {
    type ParsedThing = Numeric;

    fn parse<S: AsRef<str>>(parseable: S) -> Result<Self::ParsedThing> {
        match parseable.as_ref() {
            BYTE => Ok(Numeric::Byte),
            DECIMAL => Ok(Numeric::Decimal),
            INT => Ok(Numeric::Int),
            INTEGER => Ok(Numeric::Integer),
            LONG => Ok(Numeric::Long),
            NEGATIVE_INTEGER => Ok(Numeric::NegativeInteger),
            NON_NEGATIVE_INTEGER => Ok(Numeric::NonNegativeInteger),
            NON_POSITIVE_INTEGER => Ok(Numeric::NonPositiveInteger),
            POSITIVE_INTEGER => Ok(Numeric::PositiveInteger),
            SHORT => Ok(Numeric::Short),
            UNSIGNED_LONG => Ok(Numeric::UnsignedLong),
            UNSIGNED_INT => Ok(Numeric::UnsignedInt),
            UNSIGNED_SHORT => Ok(Numeric::UnsignedShort),
            UNSIGNED_BYTE => Ok(Numeric::UnsignedByte),
            _ => raise!("unknown numeric type: '{}'", parseable.as_ref()),
        }
    }
}

impl PrefixedString for Numeric {
    fn name(&self) -> &str {
        let s = match self {
            Numeric::Byte => BYTE,
            Numeric::Decimal => DECIMAL,
            Numeric::Int => INT,
            Numeric::Integer => INTEGER,
            Numeric::Long => LONG,
            Numeric::NegativeInteger => NEGATIVE_INTEGER,
            Numeric::NonNegativeInteger => NON_NEGATIVE_INTEGER,
            Numeric::NonPositiveInteger => NON_POSITIVE_INTEGER,
            Numeric::PositiveInteger => POSITIVE_INTEGER,
            Numeric::Short => SHORT,
            Numeric::UnsignedLong => UNSIGNED_LONG,
            Numeric::UnsignedInt => UNSIGNED_INT,
            Numeric::UnsignedShort => UNSIGNED_SHORT,
            Numeric::UnsignedByte => UNSIGNED_BYTE,
        };
        s
    }
}

impl Display for Numeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// String Types - named Character for disambiguation

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Character {
    ID,
    IDREF,
    Language,
    Name,
    NmToken,
    NormalizedString,
    String,
    Token,
}

impl Default for Character {
    fn default() -> Self {
        Character::String
    }
}

impl PrefixedParse for Character {
    type ParsedThing = Character;

    fn parse<S: AsRef<str>>(parseable: S) -> Result<Self::ParsedThing> {
        match parseable.as_ref() {
            ID => Ok(Character::ID),
            IDREF => Ok(Character::IDREF),
            LANGUAGE => Ok(Character::Language),
            NAME => Ok(Character::Name),
            NMTOKEN => Ok(Character::NmToken),
            NORMALIZED_STRING => Ok(Character::NormalizedString),
            STRING => Ok(Character::String),
            TOKEN => Ok(Character::Token),
            _ => raise!("unknown string type: '{}'", parseable.as_ref()),
        }
    }
}

impl PrefixedString for Character {
    fn name(&self) -> &str {
        let s = match self {
            Character::ID => ID,
            Character::IDREF => IDREF,
            Character::Language => LANGUAGE,
            Character::Name => NAME,
            Character::NmToken => NMTOKEN,
            Character::NormalizedString => NORMALIZED_STRING,
            Character::String => STRING,
            Character::Token => TOKEN,
        };
        s
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// DateTime Types

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DateTime {
    Date,
    DateTime,
    Duration,
    GDay,
    GMonth,
    GMonthDay,
    GYear,
    GYearMonth,
    Time,
}

impl PrefixedParse for DateTime {
    type ParsedThing = DateTime;

    fn parse<S: AsRef<str>>(parseable: S) -> Result<Self::ParsedThing> {
        match parseable.as_ref() {
            DATE => Ok(DateTime::Date),
            DATETIME => Ok(DateTime::DateTime),
            DURATION => Ok(DateTime::Duration),
            G_DAY => Ok(DateTime::GDay),
            G_MONTH => Ok(DateTime::GMonth),
            G_MONTH_DAY => Ok(DateTime::GMonthDay),
            G_YEAR => Ok(DateTime::GYear),
            G_YEAR_MONTH => Ok(DateTime::GYearMonth),
            TIME => Ok(DateTime::Time),
            _ => raise!("unknown datetime type: '{}'", parseable.as_ref()),
        }
    }
}

impl PrefixedString for DateTime {
    fn name(&self) -> &str {
        let s = match self {
            DateTime::Date => DATE,
            DateTime::DateTime => DATETIME,
            DateTime::Duration => DURATION,
            DateTime::GDay => G_DAY,
            DateTime::GMonth => G_MONTH,
            DateTime::GMonthDay => G_MONTH_DAY,
            DateTime::GYear => G_YEAR,
            DateTime::GYearMonth => G_YEAR_MONTH,
            DateTime::Time => TIME,
        };
        s
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers

fn split_raw_str(raw_str: &str) -> (&str, &str) {
    let mut split = raw_str.split(":");
    let mut ns: &str = split.next().unwrap_or("");
    let mut val: &str = split.next().unwrap_or("");
    if val.is_empty() && !ns.is_empty() {
        // if there no val but there is an ns, then it means that ns was actually the val.
        // e.g. "blah" -> ns="blah" val="", so we swap to ns="" val="blah"
        val = ns;
        ns = "";
    }
    (ns, val)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Tests

#[test]
fn split_no_ns() {
    let input = "bloop";
    let got = split_raw_str(input);
    let want = ("", "bloop");
    assert_eq!(got, want);
}

#[test]
fn split_with_ns() {
    let input = "x:bloop";
    let got = split_raw_str(input);
    let want = ("x", "bloop");
    assert_eq!(got, want);
}

#[test]
fn numeric_parse_prefixed() {
    let input = "xs:decimal";
    let got = Numeric::parse_prefixed(input, "xs").unwrap();
    let want = Numeric::Decimal;
    assert_eq!(got, want);
}

#[test]
fn numeric_parse_bad_prefix() {
    let input = "foo:decimal";
    let result = Numeric::parse_prefixed(input, "bar");
    assert!(result.is_err());
}

#[test]
fn numeric_parse_bad_value() {
    let input = "xs:decimate";
    let result = Numeric::parse_prefixed(input, "xs");
    assert!(result.is_err());
}

#[test]
fn parse_primitive_xs_datetime() {
    let input = "xs:dateTime";
    let got = Primitive::parse_prefixed(input, "xs").unwrap();
    let want = Primitive::DateTime(DateTime::DateTime);
    assert_eq!(got, want);
}

#[test]
fn parse_primitive_xs_string() {
    let input = "xs:string";
    let got = Primitive::parse_prefixed(input, "xs").unwrap();
    let want = Primitive::Character(Character::String);
    assert_eq!(got, want);
}

#[test]
fn parse_primitive_xs_byte() {
    let input = "floop:byte";
    let got = Primitive::parse_prefixed(input, "floop").unwrap();
    let want = Primitive::Numeric(Numeric::Byte);
    assert_eq!(got, want);
}

#[test]
fn parse_base_type() {
    struct TestCase {
        prefix: &'static str,
        input: &'static str,
        want: BaseType,
    }
    let test_cases = vec![
        TestCase {
            prefix: "foo",
            input: "foo:bar",
            want: BaseType::Other("foo:bar".to_owned()),
        },
        TestCase {
            prefix: "xs",
            input: "xs:string",
            want: BaseType::Primitive(Primitive::Character(Character::String)),
        },
        TestCase {
            prefix: "xs",
            input: "bloop:blerp",
            want: BaseType::Other("bloop:blerp".to_owned()),
        },
    ];
    for test_case in &test_cases {
        let got = BaseType::parse_prefixed(test_case.input, test_case.prefix).unwrap();
        assert_eq!(&got, &test_case.want);
    }
}

#[test]
fn display_numeric() {
    let numeric = Numeric::Byte;
    let got = format!("{}", numeric);
    let want = "byte";
    assert_eq!(got.as_str(), want);
}

#[test]
fn display_base_type() {
    let base_type = BaseType::Primitive(Primitive::Numeric(Numeric::Byte));
    let got = format!("{}", base_type);
    let want = "byte";
    assert_eq!(got.as_str(), want);
}
