use crate::error::Result;
use crate::generate::cpp::writer::Writer;
use crate::model::enumeration::{Enumeration, OtherField};
use crate::model::Model;
use crate::utils::string_stuff::{camel_case, linestart, pascal_case, sep, write_documentation, Altered, Symbol};
use indexmap::set::IndexSet;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

pub fn write_tabs<W: Write>(w: &mut W, num: u32) -> std::io::Result<()> {
    for _ in 0..num {
        write!(w, "    ")?;
    }
    Ok(())
}

macro_rules! w {
    ($w:expr, $tabs:expr, $($arg:expr),+) => {{
        crate::generate::cpp::write_enums::write_tabs($w, $tabs)?;
        write!($w, $($arg),+)
    }};
}

macro_rules! l {
    ($w:expr, $tabs:expr, $($arg:expr),+) => {{
        crate::generate::cpp::write_enums::write_tabs($w, $tabs)?;
        writeln!($w, $($arg),+)
    }};
}

impl Writer {
    pub(crate) fn write_enums(&self, enumerations: &mut [&Enumeration]) -> Result<()> {
        enumerations.sort_by(|&a, &b| {
            let a = a.name.pascal();
            let b = b.name.pascal();
            a.cmp(b)
        });
        let mut hwrite = wrap!(self.open_enums_h())?;
        let mut cwrite = wrap!(self.open_enums_cpp())?;
        for (i, &enumer) in enumerations.iter().enumerate() {
            let is_last = i == enumerations.len() - 1;
            wrap!(self.write_enum_h(enumer, &mut hwrite))?;
            if !is_last {
                wrap!(writeln!(hwrite, ""))?;
            }
            wrap!(self.write_enum_cpp(enumer, &mut cwrite))?;
            if !is_last {
                wrap!(writeln!(cwrite, ""))?;
            }
        }
        wrap!(self.close_enums_h(&mut hwrite))?;
        wrap!(self.close_enums_cpp(&mut cwrite))?;
        Ok(())
    }

    // fn collect_enums(&self) -> Result<Vec<&Enumeration>> {
    //     let mut result = Vec::new();
    //     for model in &self.models {
    //         if let Model::Enumeration(enumer) = model {
    //             result.push(enumer);
    //         }
    //     }
    //     result.sort_by(|a, b| a.name.camel().cmp(b.name.camel()));
    //     Ok(result)
    // }

    fn open_enums_h(&self) -> std::io::Result<impl Write> {
        let _igore_error = std::fs::remove_file(&self.paths.enums_h);
        let mut f = OpenOptions::new().write(true).create(true).open(&self.paths.enums_h)?;
        l!(&mut f, 0, "// MusicXML Class Library")?;
        l!(&mut f, 0, "// Copyright (c) by Matthew James Briggs")?;
        l!(&mut f, 0, "// Distributed under the MIT License")?;
        l!(&mut f, 0, "")?;
        l!(&mut f, 0, "#pragma once")?;
        l!(&mut f, 0, "")?;
        l!(&mut f, 0, "#include \"mx/core/EnumsBuiltin.h\"")?;
        l!(&mut f, 0, "")?;
        l!(&mut f, 0, "namespace mx")?;
        l!(&mut f, 0, "{{")?;
        l!(&mut f, 1, "namespace core")?;
        l!(&mut f, 1, "{{")?;
        Ok(f)
    }

    fn open_enums_cpp(&self) -> std::io::Result<impl Write> {
        let _igore_error = std::fs::remove_file(&self.paths.enums_cpp);
        let mut f = OpenOptions::new().write(true).create(true).open(&self.paths.enums_cpp)?;
        l!(&mut f, 0, "// MusicXML Class Library")?;
        l!(&mut f, 0, "// Copyright (c) by Matthew James Briggs")?;
        l!(&mut f, 0, "// Distributed under the MIT License")?;
        l!(&mut f, 0, "")?;
        l!(&mut f, 0, "#include \"mx/core/Enums.h\"")?;
        l!(&mut f, 0, "")?;
        l!(&mut f, 0, "#include <ostream>")?;
        l!(&mut f, 0, "#include <string>")?;
        l!(&mut f, 0, "")?;
        l!(&mut f, 0, "namespace mx")?;
        l!(&mut f, 0, "{{")?;
        l!(&mut f, 1, "namespace core")?;
        l!(&mut f, 1, "{{")?;
        Ok(f)
    }

    fn close_enums_h<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        l!(w, 1, "}}")?;
        l!(w, 0, "}}")?;
        Ok(())
    }

    fn close_enums_cpp<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        l!(w, 1, "}}")?;
        l!(w, 0, "}}")?;
        Ok(())
    }

    fn write_enum_h<W: Write>(&self, enumer: &Enumeration, w: &mut W) -> std::io::Result<()> {
        let n = enumer.name.pascal();
        l!(w, 2, "{}", sep(n, 2))?;
        linestart(w, 2, false)?;
        writeln!(w)?;
        write_documentation(w, enumer.documentation.as_str(), 2)?;
        writeln!(w)?;
        linestart(w, 2, false)?;
        writeln!(w)?;
        l!(w, 2, "enum class {}", n)?;
        l!(w, 2, "{{")?;
        for (i, m) in enumer.members.iter().enumerate() {
            let is_last = i == enumer.members.len() - 1;
            w!(w, 3, "{} = {}", m.camel(), i)?;
            if is_last {
                if let Some(other_field) = &enumer.other_field {
                    w!(w, 0, ",\n")?;
                    w!(w, 3, "{} = {}", other_field.name.camel(), i + 1)?;
                    w!(w, 0, "\n")?;
                } else {
                    w!(w, 0, "\n")?;
                }
            } else {
                w!(w, 0, ",\n")?;
            }
        }
        l!(w, 2, "}};")?;
        l!(w, 0, "")?;
        l!(w, 2, "{} parse{}( const std::string& value );", n, n)?;
        l!(w, 2, "std::string toString( const {} value );", n)?;
        l!(w, 2, "std::ostream& toStream( std::ostream& os, const {} value );", n)?;
        l!(w, 2, "std::ostream& operator<<( std::ostream& os, const {} value );", n)?;

        if let Some(other_field) = &enumer.other_field {
            let cn = other_field.wrapper_class_name.pascal();
            let en = enumer.name.pascal();
            l!(w, 0, "")?;
            l!(w, 2, "class {}", cn)?;
            l!(w, 2, "{{")?;
            l!(w, 2, "public:")?;
            l!(w, 3, "explicit {}( const {} value );", cn, en)?;
            l!(w, 3, "explicit {}( const std::string& value );", cn)?;
            l!(w, 3, "{}();", cn)?;
            l!(w, 3, "{} getValue() const;", en)?;
            l!(w, 3, "std::string getValueString() const;")?;
            l!(w, 3, "void setValue( const {} value );", en)?;
            l!(w, 3, "void setValue( const std::string& value );")?;
            l!(w, 2, "private:")?;
            l!(w, 3, "{} myEnum;", en)?;
            l!(w, 3, "std::string myCustomValue;")?;
            l!(w, 2, "}};")?;
            l!(w, 0, "")?;
            l!(w, 2, "{} parse{}( const std::string& value );", cn, cn)?;
            l!(w, 2, "std::string toString( const {}& value );", cn)?;
            l!(w, 2, "std::ostream& toStream( std::ostream& os, const {}& value );", cn)?;
            l!(w, 2, "std::ostream& operator<<( std::ostream& os, const {}& value );", cn)?;
        }

        Ok(())
    }

    fn write_enum_cpp<W: Write>(&self, enumer: &Enumeration, w: &mut W) -> Result<()> {
        if let Some(other_field) = &enumer.other_field {
            wrap!(self.write_enum_cpp_other_enum(enumer, other_field, w))
        } else {
            wrap!(self.write_enum_cpp_normal_enum(enumer, w))
        }
    }

    fn write_enum_cpp_normal_enum<W: Write>(&self, enumer: &Enumeration, w: &mut W) -> std::io::Result<()> {
        let pc = enumer.name.pascal();
        l!(w, 2, "{}", sep(pc, 2))?;
        l!(w, 0, "")?;
        l!(w, 2, "{} parse{}( const std::string& value )", pc, pc)?;
        l!(w, 2, "{{")?;
        for (i, member) in enumer.members.iter().enumerate() {
            let o = member.original();
            let n = member.camel();
            if i == 0 {
                l!(w, 3, "if ( value == \"{}\" ) {{ return {}::{}; }}", o, pc, n)?;
            } else {
                l!(w, 3, "else if ( value == \"{}\" ) {{ return {}::{}; }}", o, pc, n)?;
            }
        }
        l!(w, 3, "return {}::{};", pc, enumer.members.get(0).unwrap().camel())?;
        l!(w, 2, "}}")?;
        l!(w, 0, "")?;
        l!(w, 2, "std::string toString( const {} value )", pc)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "switch ( value )")?;
        l!(w, 3, "{{")?;
        for member in &enumer.members {
            let o = member.original();
            let n = member.camel();
            l!(w, 4, "case {}::{}: {{ return \"{}\"; }}", pc, n, o)?;
        }
        l!(w, 4, "default: break;")?;
        l!(w, 3, "}}")?;
        l!(w, 3, "return \"{}\";", enumer.members.get(0).unwrap().original())?;
        l!(w, 2, "}}")?;
        l!(w, 0, "")?;
        l!(w, 2, "std::ostream& toStream( std::ostream& os, const {} value )", pc)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "return os << toString( value );")?;
        l!(w, 2, "}}")?;
        l!(w, 0, "")?;
        l!(w, 2, "std::ostream& operator<<( std::ostream& os, const {} value )", pc)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "return toStream( os, value );")?;
        l!(w, 2, "}}")?;
        Ok(())
    }

    fn write_enum_cpp_other_enum<W: Write>(&self, enumer: &Enumeration, other_field: &OtherField, w: &mut W) -> std::io::Result<()> {
        let pc = enumer.name.pascal();
        let other = other_field;
        let of_orig = other.name.original();
        let of_pasc = other.name.camel();
        let cn = other.wrapper_class_name.pascal();
        l!(w, 2, "{}", sep(pc, 2))?;
        l!(w, 0, "")?;
        l!(w, 2, "{} parse{}( const std::string& value, bool& success )", pc, pc)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "success = true;")?;
        for (i, member) in enumer.members.iter().enumerate() {
            let o = member.original();
            let n = member.camel();
            if i == 0 {
                l!(w, 3, "if ( value == \"{}\" ) {{ return {}::{}; }}", o, pc, n)?;
            } else {
                l!(w, 3, "else if ( value == \"{}\" ) {{ return {}::{}; }}", o, pc, n)?;
            }
        }
        l!(w, 3, "else if ( value == \"{}\" ) {{ return {}::{}; }}", of_orig, pc, of_pasc)?;
        l!(w, 3, "success = false;")?;
        l!(w, 3, "return {}::{};", pc, of_pasc)?;
        l!(w, 2, "}}")?;
        l!(w, 0, "")?;
        l!(w, 2, "{} parse{}( const std::string& value )", pc, pc)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "bool success = true;")?;
        l!(w, 3, "return parse{}( value, success );", pc)?;
        l!(w, 2, "}}")?;
        l!(w, 0, "")?;
        l!(w, 2, "std::string toString( const {} value )", pc)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "switch ( value )")?;
        l!(w, 3, "{{")?;
        for member in &enumer.members {
            let o = member.original();
            let n = member.camel();
            l!(w, 4, "case {}::{}: {{ return \"{}\"; }}", pc, n, o)?;
        }
        l!(w, 4, "case {}::{}: {{ return \"{}\"; }}", pc, of_pasc, of_orig)?;
        l!(w, 4, "default: break;")?;
        l!(w, 3, "}}")?;
        l!(w, 3, "return \"default\";")?;
        l!(w, 2, "}}")?;
        l!(w, 0, "")?;
        l!(w, 2, "std::ostream& toStream( std::ostream& os, const {} value )", pc)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "return os << toString( value );")?;
        l!(w, 2, "}}")?;
        l!(w, 0, "")?;
        l!(w, 2, "std::ostream& operator<<( std::ostream& os, const {} value )", pc)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "return toStream( os, value );")?;
        l!(w, 2, "}}")?;
        l!(w, 0, "")?;
        l!(w, 2, "{}::{}( const {} value )", cn, cn, pc)?;
        l!(w, 2, ":myEnum( value )")?;
        l!(w, 2, ",myCustomValue( \"\" )")?;
        l!(w, 2, "{{")?;
        l!(w, 3, "setValue( value );")?;
        l!(w, 2, "}}")?;
        l!(w, 0, "")?;
        l!(w, 2, "{}::{}( const std::string& value )", cn, cn)?;
        l!(w, 2, ":myEnum( {}::{} )", pc, of_pasc)?;
        l!(w, 2, ",myCustomValue( value )")?;
        l!(w, 2, "{{")?;
        l!(w, 3, "setValue( value );")?;
        l!(w, 2, "}}")?;
        l!(w, 0, "")?;
        l!(w, 2, "{}::{}()", cn, cn)?;
        l!(w, 2, ":myEnum( {}::{} )", pc, other.default_value.camel())?;
        l!(w, 2, ",myCustomValue( \"\" )")?;
        l!(w, 2, "{{")?;
        l!(w, 3, "setValue( {}::{} );", pc, other.default_value.camel())?;
        l!(w, 2, "}}")?;
        l!(w, 0, "")?;
        l!(w, 2, "{} {}::getValue() const", pc, cn)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "return myEnum;")?;
        l!(w, 2, "}}")?;
        l!(w, 0, "")?;
        l!(w, 2, "std::string {}::getValueString() const", cn)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "if ( myEnum != {}::{} )", pc, of_pasc)?;
        l!(w, 3, "{{")?;
        l!(w, 4, "return toString( myEnum );")?;
        l!(w, 3, "}}")?;
        l!(w, 0, "")?;
        l!(w, 3, "else")?;
        l!(w, 3, "{{")?;
        l!(w, 4, "return myCustomValue;")?;
        l!(w, 3, "}}")?;
        l!(w, 2, "}}")?;
        l!(w, 0, "")?;
        l!(w, 2, "void {}::setValue( const {} value )", cn, pc)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "myEnum = value;")?;
        l!(w, 2, "}}")?;
        l!(w, 0, "")?;
        l!(w, 2, "void {}::setValue( const std::string& value )", cn)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "bool found = false;")?;
        l!(w, 3, "{} temp = parse{}( value, found );", pc, pc)?;
        l!(w, 3, "if ( found )")?;
        l!(w, 3, "{{")?;
        l!(w, 4, "myEnum = temp;")?;
        l!(w, 3, "}}")?;
        l!(w, 3, "else")?;
        l!(w, 3, "{{")?;
        l!(w, 4, "setValue( {}::{} );", pc, of_pasc)?;
        l!(w, 4, "myCustomValue = value;")?;
        l!(w, 3, "}}")?;
        l!(w, 2, "}}")?;
        l!(w, 0, "")?;
        l!(w, 2, "{} parse{}( const std::string& value )", cn, cn)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "return {}( value );", cn)?;
        l!(w, 2, "}}")?;
        l!(w, 2, "")?;
        l!(w, 2, "std::string toString( const {}& value )", cn)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "return value.getValueString();")?;
        l!(w, 2, "}}")?;
        l!(w, 2, "")?;
        l!(w, 2, "std::ostream& toStream( std::ostream& os, const {}& value )", cn)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "return os << toString( value );")?;
        l!(w, 2, "}}")?;
        l!(w, 2, "")?;
        l!(w, 2, "std::ostream& operator<<( std::ostream& os, const {}& value )", cn)?;
        l!(w, 2, "{{")?;
        l!(w, 3, "return toStream( os, value );")?;
        l!(w, 2, "}}")?;
        Ok(())
    }
}
