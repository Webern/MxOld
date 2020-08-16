use crate::error::Result;
use crate::generate::cpp::cpp_template::{render_core_cpp, render_core_h};
use crate::generate::cpp::writer::Writer;
use crate::generate::template::{
    render, CORE_H, DECIMAL_BUILTINS_CPP, DECIMAL_BUILTINS_H, DECIMAL_TYPE_CPP, DECIMAL_TYPE_H,
    INTEGER_BUILTINS_CPP, INTEGER_BUILTINS_H, INTEGER_TYPE_CPP, INTEGER_TYPE_H, NO_DATA,
};
use crate::model::scalar::{Bound, NumericData, Range};
use crate::model::scalar::{ScalarNumeric, ScalarString};
use crate::model::symbol::Symbol;
use crate::model::Def;
use crate::utils::string_stuff::{documentation, sep, write_documentation};
use crate::xsd::primitives::BaseType;
use crate::xsd::primitives::Numeric;
use crate::xsd::primitives::Primitive;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Deref;

pub fn write_tabs<W: Write>(w: &mut W, num: u32) -> std::io::Result<()> {
    for _ in 0..num {
        write!(w, "    ")?;
    }
    Ok(())
}

macro_rules! w {
    ($w:expr, $tabs:expr, $($arg:expr),+) => {{
        crate::generate::cpp::write_numerics::write_tabs($w, $tabs)?;
        write!($w, $($arg),+)
    }};
}

macro_rules! l {
    ($w:expr, $tabs:expr, $($arg:expr),+) => {{
        crate::generate::cpp::write_numerics::write_tabs($w, $tabs)?;
        writeln!($w, $($arg),+)
    }};
}

impl Writer {
    pub(crate) fn write_integers(&self, mut numerics: Vec<NumericData<i64>>) -> Result<()> {
        // yuckily add these because i'm not dealing with XSD built-ins right now, but I used
        // PositiveInteger is referenced by, e.g. PositiveIntegerOrEmpty
        let positive_integer = NumericData {
            name: Symbol::new("positiveInteger"),
            base_type: Numeric::PositiveInteger,
            documentation: String::from("The built-in primitive xs:positiveInteger"),
            range: Range {
                min: Some(Bound::Inclusive(1 as i64)),
                max: None,
            },
        };
        let non_negative_integer = NumericData {
            name: Symbol::new("nonNegativeInteger"),
            base_type: Numeric::NonNegativeInteger,
            documentation: String::from("The built-in primitive xs:nonNegativeInteger"),
            range: Range {
                min: Some(Bound::Inclusive(0 as i64)),
                max: None,
            },
        };
        // i used an 'int range' for Color to clamp the range of the bytes from 0-255. to replace
        // that i'll create a 'Byte' numeric
        let byte = NumericData {
            name: Symbol::new("byte"),
            base_type: Numeric::NonNegativeInteger,
            documentation: String::from(
                "This is not part of MusicXML. It represents a clamped byte.",
            ),
            range: Range {
                min: Some(Bound::Inclusive(0 as i64)),
                max: Some(Bound::Inclusive(255 as i64)),
            },
        };
        numerics.push(byte);
        numerics.push(positive_integer);
        numerics.push(non_negative_integer);
        numerics.sort_by(|a, b| {
            let a = a.name.pascal();
            let b = b.name.pascal();
            a.cmp(b)
        });
        self.write_integers_h(numerics.as_slice())?;
        self.write_integers_cpp(&numerics)?;
        Ok(())
    }

    fn write_integers_h(&self, numerics: &[NumericData<i64>]) -> Result<()> {
        let mut contents = String::new();
        let mut nothing = HashMap::<String, String>::new();

        let builtins = render(INTEGER_BUILTINS_H, &nothing)?;
        contents.push_str(&builtins);
        for numeric in numerics {
            let mut data = HashMap::new();
            data.insert("classname", numeric.name.pascal().to_owned());
            data.insert("documentation", documentation(document_int(numeric), 2)?);
            let rendered_type = render(INTEGER_TYPE_H, &data)?;
            contents.push('\n');
            contents.push('\n');
            contents.push_str(&rendered_type);
        }
        let file_contents =
            render_core_h(contents, None, Some(&mut ["iostream", "string", "limits"]))?;
        wrap!(std::fs::write(&self.paths.integers_h, file_contents))?;
        Ok(())
    }

    fn write_integers_cpp(&self, numerics: &[NumericData<i64>]) -> Result<()> {
        let mut contents = render(INTEGER_BUILTINS_CPP, NO_DATA.deref())?;
        for (i, numeric) in numerics.iter().enumerate() {
            let (min, max) = min_max_ints(numeric);
            let classname = numeric.name.pascal();
            let mut data = HashMap::new();
            data.insert("min_val", min);
            data.insert("max_val", max);
            data.insert("classname", classname.to_owned());
            let mut rendered = String::from("\n\n");
            rendered.push_str(render(INTEGER_TYPE_CPP, &data)?.as_str());
            if i < numerics.len() - 1 {}
            contents.push_str(&rendered);
        }
        let file_contents = render_core_cpp(
            contents,
            Some("mx/core/Integers.h"),
            None,
            Some(&mut ["sstream"]),
        )?;
        wrap!(std::fs::write(&self.paths.integers_cpp, file_contents))?;
        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////
    pub(crate) fn write_decimals(&self, mut numerics: Vec<NumericData<f64>>) -> Result<()> {
        // TODO - this needs to be properly generated
        let positive_divisions_value = NumericData {
            name: Symbol::new("positive-divisions-value"),
            base_type: Numeric::Decimal,
            documentation: String::from("TODO - this needs to be properly generated"),
            range: Range {
                min: Some(Bound::Exclusive(0 as f64)),
                max: None,
            },
        };
        numerics.push(positive_divisions_value);
        numerics.sort_by(|a, b| {
            let a = a.name.pascal();
            let b = b.name.pascal();
            a.cmp(b)
        });
        self.write_decimals_h(&numerics)?;
        self.write_decimals_cpp(&numerics)?;
        Ok(())
    }

    fn write_decimals_h(&self, numerics: &[NumericData<f64>]) -> Result<()> {
        let mut contents = String::new();
        let mut nothing = HashMap::<String, String>::new();

        let builtins = render(DECIMAL_BUILTINS_H, &nothing)?;
        contents.push_str(&builtins);
        for numeric in numerics {
            let mut data = HashMap::new();
            data.insert("classname", numeric.name.pascal().to_owned());
            data.insert(
                "documentation",
                documentation(document_decimal(numeric), 2)?,
            );
            let rendered_type = render(DECIMAL_TYPE_H, &data)?;
            contents.push('\n');
            contents.push('\n');
            contents.push_str(&rendered_type);
        }
        let file_contents = render_core_h(
            contents,
            None,
            Some(&mut ["iostream", "string", "functional"]),
        )?;
        wrap!(std::fs::write(&self.paths.decimals_h, file_contents))?;
        Ok(())
    }

    fn write_decimals_cpp(&self, numerics: &[NumericData<f64>]) -> Result<()> {
        let mut contents = render(DECIMAL_BUILTINS_CPP, NO_DATA.deref())?;
        for (i, numeric) in numerics.iter().enumerate() {
            let (minexpr, maxexpr) = minmax_expr_decimal(numeric);
            let defaultval = defaultval(numeric);
            let classname = numeric.name.pascal();
            let mut data = HashMap::new();
            data.insert("classname", classname.to_owned());
            data.insert("minexpr", minexpr);
            data.insert("maxexpr", maxexpr);
            data.insert("defaultval", defaultval);
            let mut rendered = String::from("\n\n");
            rendered.push_str(render(DECIMAL_TYPE_CPP, &data)?.as_str());
            if i < numerics.len() - 1 {}
            contents.push_str(&rendered);
        }
        let file_contents = render_core_cpp(
            contents,
            Some("mx/core/Decimals.h"),
            Some(&mut ["mx/core/UnusedParameter.h"]),
            Some(&mut ["sstream", "cmath"]),
        )?;
        wrap!(std::fs::write(&self.paths.decimals_cpp, file_contents))?;
        Ok(())
    }
}

fn min_max_ints(numeric: &NumericData<i64>) -> (String, String) {
    let (min, max) = maybe_min_max_ints(numeric);
    (
        min.unwrap_or_else(|| String::from("IntMin")),
        max.unwrap_or_else(|| String::from("IntMax")),
    )
}

fn maybe_min_max_ints(numeric: &NumericData<i64>) -> (Option<String>, Option<String>) {
    let min = numeric.range.min.as_ref().and_then(|bound| match bound {
        Bound::Inclusive(i) => Some(format!("{}", *i)),
        Bound::Exclusive(e) => Some(format!("{}", *e + 1)),
    });
    let max = numeric.range.max.as_ref().and_then(|bound| match bound {
        Bound::Inclusive(i) => Some(format!("{}", *i)),
        Bound::Exclusive(e) => Some(format!("{}", *e - 1)),
    });
    (min, max)
}

fn describe_range_int(numeric: &NumericData<i64>) -> String {
    let (min, max) = maybe_min_max_ints(numeric);
    format!(
        "Range: min={}, max={}",
        min.unwrap_or_else(|| String::from("None")),
        max.unwrap_or_else(|| String::from("None"))
    )
}

fn describe_range_decimal(minmax: (String, String)) -> String {
    // let (min, max) = maybe_min_max_ints(numeric);
    format!("Range: min={}, max={}", minmax.0, minmax.1)
}

fn minmax_expr_decimal(numeric: &NumericData<f64>) -> (String, String) {
    let min = match numeric.range.min.as_ref() {
        None => String::from("MX_NOOP"),
        Some(bound) => match bound {
            Bound::Inclusive(f) => format!("MXMININ( {} )", f),
            Bound::Exclusive(f) => format!("MXMINEX( {} )", f),
        },
    };
    let max = match numeric.range.max.as_ref() {
        None => String::from("MX_NOOP"),
        Some(bound) => match bound {
            Bound::Inclusive(f) => format!("MXMAXIN( {} )", f),
            Bound::Exclusive(f) => format!("MXMAXEX( {} )", f),
        },
    };
    (min, max)
}

fn defaultval(numeric: &NumericData<f64>) -> String {
    match numeric.range.min.as_ref() {
        None => String::from("0.0"),
        Some(bound) => match bound {
            Bound::Inclusive(f) => {
                if *f > 0.0 {
                    format!("{}", f)
                } else {
                    String::from("0.0")
                }
            }
            Bound::Exclusive(f) => {
                if *f >= 0.0 {
                    format!("{}", f + 1.0)
                } else {
                    String::from("0.0")
                }
            }
        },
    }
}

fn document_int(numeric: &NumericData<i64>) -> String {
    if numeric.documentation.is_empty() {
        describe_range_int(numeric)
    } else {
        format!(
            "{}\n\n{}",
            numeric.documentation,
            describe_range_int(numeric)
        )
    }
}

fn document_decimal(numeric: &NumericData<f64>) -> String {
    let range = describe_range_decimal(decimal_range_doc_strings(numeric));
    if numeric.documentation.is_empty() {
        range
    } else {
        format!("{}\n\n{}", numeric.documentation, range)
    }
}

fn decimal_range_doc_strings(numeric: &NumericData<f64>) -> (String, String) {
    let min = match numeric.range.min.as_ref() {
        None => String::from("None"),
        Some(bound) => match bound {
            Bound::Inclusive(f) => format!("Inclusive({})", f),
            Bound::Exclusive(f) => format!("Exclusive({})", f),
        },
    };
    let max = match numeric.range.max.as_ref() {
        None => String::from("None"),
        Some(bound) => match bound {
            Bound::Inclusive(f) => format!("Inclusive({})", f),
            Bound::Exclusive(f) => format!("Exclusive({})", f),
        },
    };
    (min, max)
}
