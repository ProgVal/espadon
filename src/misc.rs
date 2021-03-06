use nom_locate::LocatedSpan;

pub type StrSpan<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq)]
pub struct Location<'a> {
    pub start: StrSpan<'a>,
    pub end: StrSpan<'a>
}

macro_rules! position_result {
    ($start: expr, $end:expr, $ret:path {$($ret_attrs:tt)*}) => ({
        $ret {
            loc: Location {
                start: $start,
                end: $end,
            },
            $($ret_attrs)*
        }
    });
}

#[macro_export]
macro_rules! es_parse {
    ($i:expr, {$($rest:tt)*} => ($ret:path {$($ret_attrs:tt)*})) => ({
        do_parse!($i,
            start: position!() >>
            $($rest)* >>
            end: position!() >>
            (position_result!(start, end, $ret {$($ret_attrs)*})))
    });
}

/// Just like opt! except that it supports eof.
#[macro_export]
macro_rules! opt2 (
    ($i:expr, $submac:ident!( $($args:tt)* )) => ({
        use ::nom::InputLength;
        match ($i).input_len() {
            0 => ::nom::IResult::Done($i, ::std::option::Option::None),
            _ => opt!($i, $submac!($($args)*))
        }
    });
    ($i:expr, $f:expr) => (
        opt2!($i, call!($f));
    );
);

#[derive(Debug, PartialEq)]
pub struct Identifier<'a> {
    pub name: String,
    pub loc: Location<'a>
}

fn var_name_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$'
}

named!(pub identifier< StrSpan, Identifier >, es_parse!({
        return_error!(es_error!(InvalidVarName), peek!(none_of!("0123456789"))) >>
        id: take_while1_s!(var_name_char)
    } => (Identifier {
        name: id.to_string()
    })
));

