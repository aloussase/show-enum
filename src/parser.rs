use core::panic;
use std::fmt::Display;

use arraystring::{typenum::U50, ArrayString};

type EnumTag = ArrayString<U50>;
type EnumAlias = ArrayString<U50>;
type EnumVariant = ArrayString<U50>;

#[derive(Debug)]
pub struct CEnum {
    tag: EnumTag,
    alias: EnumAlias,
    variants: [EnumVariant; 50],
    nvariants: u32,
}

impl Display for CEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = if self.alias.is_empty() {
            self.tag.as_str()
        } else {
            self.alias.as_str()
        };

        f.write_fmt(format_args!(
            "const char *show_{}({} self) {{\n",
            name.to_lowercase(),
            name
        ))?;

        f.write_str("    switch (self) {\n")?;

        for variant in &self.variants[..self.nvariants as usize] {
            let name = variant.as_str();
            f.write_fmt(format_args!(
                "        case {}: return \"{}\";\n",
                name, name
            ))?;
        }

        f.write_str("        default: return \"\";\n")?;
        f.write_str("    }\n}")
    }
}

impl Default for CEnum {
    fn default() -> Self {
        Self {
            tag: Default::default(),
            alias: Default::default(),
            variants: [Default::default(); 50],
            nvariants: 0,
        }
    }
}

impl CEnum {
    pub fn parse(source: &str) -> Self {
        let mut enum_: CEnum = Default::default();
        let mut next: u32;
        (enum_.tag, next) = CEnum::parse_tag(source, 0);
        next = CEnum::parse_fields(source, next, &mut enum_);
        enum_.alias = CEnum::parse_alias(source, next);
        enum_
    }

    fn expect(source: &str, start: u32, expected: &str) -> u32 {
        let slice = &source[start as usize..];
        if !slice.starts_with(expected) {
            panic!("expected {}, but got {}", expected, slice);
        }
        start + expected.len() as u32
    }

    fn optional(source: &str, start: u32, expected: &str) -> u32 {
        let slice = &source[start as usize..];
        if slice.starts_with(expected) {
            start + expected.len() as u32
        } else {
            start
        }
    }

    fn skip_ws(source: &str, start: u32) -> u32 {
        source[start as usize..]
            .find(|c: char| !c.is_whitespace())
            .map(|n| start + n as u32)
            .unwrap_or(start)
    }

    fn parse_tag(source: &str, start: u32) -> (EnumTag, u32) {
        let mut next = CEnum::skip_ws(source, start);

        next = CEnum::optional(source, next, "typedef");
        next = CEnum::skip_ws(source, next);

        next = CEnum::expect(source, next, "enum");
        next = CEnum::skip_ws(source, next);

        let bytes = source.as_bytes();
        let start = next as usize;

        while (next as usize) < bytes.len()
            && !bytes[next as usize].is_ascii_whitespace()
            && bytes[next as usize] != b'{'
        {
            next += 1;
        }

        if next as usize - start > 0 {
            (
                EnumTag::try_from_str(&source[start..next as usize]).expect("failed to read alias"),
                next,
            )
        } else {
            (Default::default(), next)
        }
    }

    fn parse_fields(source: &str, start: u32, enum_: &mut CEnum) -> u32 {
        let mut next = CEnum::skip_ws(source, start);
        next = CEnum::expect(source, next, "{");
        next = CEnum::skip_ws(source, next);

        let bytes = source.as_bytes();
        let mut ix = 0;

        while (next as usize) < bytes.len()
            && bytes[next as usize] != b'}'
            && !bytes[next as usize].is_ascii_whitespace()
        {
            next = CEnum::skip_ws(source, next);
            let start = next as usize;

            while (next as usize) < bytes.len()
                && !bytes[next as usize].is_ascii_whitespace()
                && bytes[next as usize] != b','
            {
                next += 1;
            }

            let name = &source[start..next as usize];
            enum_.nvariants += 1;
            enum_.variants[ix] =
                EnumVariant::try_from_str(name).expect("failed to read enum variant");

            next = CEnum::skip_ws(source, next);

            next += if bytes[next as usize] == b',' {
                1
            } else {
                break;
            };

            next = CEnum::skip_ws(source, next);

            ix += 1;
        }

        next
    }

    pub fn parse_alias(source: &str, start: u32) -> EnumAlias {
        let mut next = CEnum::skip_ws(source, start);

        next = CEnum::expect(source, next, "}");
        next = CEnum::skip_ws(source, next);

        let start = next as usize;
        let bytes = source.as_bytes();

        while (next as usize) < bytes.len()
            && bytes[next as usize] != b';'
            && !bytes[next as usize].is_ascii_whitespace()
        {
            next += 1;
        }

        let alias = &source[start..next as usize];

        EnumAlias::try_from_str(alias).expect("failed to parse enum alias")
    }
}
