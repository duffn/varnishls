use std::collections::BTreeMap;
use std::mem::discriminant;

use crate::document::Definition;

#[derive(Debug, Clone)]
pub enum Type {
    Obj(Obj),
    Func(Func),
    Backend,
    // Director,
    String,
    Number,
    Duration,
    Bool,
    Acl,
    Sub,
    Probe,
    // UnresolvedNew, // hack
}

impl Type {
    pub fn is_same_type_as(&self, other: &Self) -> bool {
        return discriminant(self) == discriminant(other);
    }

    pub fn can_this_cast_into(&self, other: &Self) -> bool {
        match self {
            Type::String | Type::Number | Type::Duration => match other {
                Type::String | Type::Number | Type::Duration => {
                    return true;
                }
                _ => {}
            },
            _ => {}
        }
        return discriminant(self) == discriminant(other);
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Obj(_obj) => write!(f, "STRUCT"),
            Type::Func(func) => write!(
                f,
                "{}{}{}",
                match func.ret_type {
                    Some(ref return_str) => format!("{} ", return_str),
                    _ => "".to_string(),
                },
                func.name,
                func.signature.clone().unwrap_or("()".to_string())
            ),
            Type::Backend => write!(f, "BACKEND"),
            Type::String => write!(f, "STRING"),
            Type::Number => write!(f, "NUMBER"),
            Type::Duration => write!(f, "DURATION"),
            Type::Bool => write!(f, "BOOL"),
            Type::Acl => write!(f, "ACL"),
            Type::Sub => write!(f, "SUBROUTINE"),
            Type::Probe => write!(f, "PROBE"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Obj {
    pub name: String,
    pub properties: BTreeMap<String, Type>,
    pub read_only: bool,
    pub definition: Option<Definition>,
}

#[derive(Debug, Default, Clone)]
pub struct Func {
    pub name: String,
    pub definition: Option<Definition>,
    pub signature: Option<String>,   // arguments
    pub ret_type: Option<String>,    // TEMP
    pub r#return: Option<Box<Type>>, // TEMP
}

const DEFAULT_REQUEST_HEADERS: &'static [&str] = &[
    "host",
    "origin",
    "cookie",
    "user-agent",
    "referer",
    "if-none-match",
    "if-modified-since",
    "accept",
    "authorization",
];

const DEFAULT_RESPONSE_HEADERS: &'static [&str] = &[
    "vary",
    "origin",
    "server",
    "age",
    "expires",
    "etag",
    "last-modified",
    "content-type",
    "cache-control",
    "surrogate-control",
    "location",
    "set-cookie",
];

// https://github.com/varnishcache/varnish-cache/blob/a3bc025c2df28e4a76e10c2c41217c9864e9963b/lib/libvcc/vcc_backend.c#L121-L130
pub const PROBE_FIELDS: &'static [&str] = &[
    "url",
    "request",
    "expected_response",
    "timeout",
    "interval",
    "window",
    "threshold",
    "initial",
];

// https://github.com/varnishcache/varnish-cache/blob/a3bc025c2df28e4a76e10c2c41217c9864e9963b/lib/libvcc/vcc_backend.c#L311-L322
pub const BACKEND_FIELDS: &'static [&str] = &[
    "host",
    "port",
    "path",
    "host_header",
    "connect_timeout",
    "first_byte_timeout",
    "between_bytes_timeout",
    "probe",
    "max_connections",
    "proxy_header",
];

pub fn get_varnish_builtins() -> Type {
    let req: Type = Type::Obj(Obj {
        name: "req".to_string(),
        read_only: true,
        properties: BTreeMap::from([
            (
                "http".to_string(),
                Type::Obj(Obj {
                    name: "req.http".to_string(),
                    read_only: false,
                    properties: BTreeMap::from_iter(
                        DEFAULT_REQUEST_HEADERS
                            .iter()
                            .map(|header| (header.to_string(), Type::String)),
                    ),
                    ..Obj::default()
                }),
            ),
            ("url".to_string(), Type::String),
            ("method".to_string(), Type::String),
            ("hash".to_string(), Type::String),
            ("proto".to_string(), Type::String),
            ("backend_hint".to_string(), Type::Backend),
            ("restarts".to_string(), Type::Number),
            ("ttl".to_string(), Type::Duration),
            ("grace".to_string(), Type::Duration),
            ("is_hitmiss".to_string(), Type::Bool),
            ("is_hitpass".to_string(), Type::Bool),
        ]),
        ..Obj::default()
    });

    let bereq: Type = Type::Obj(Obj {
        name: "bereq".to_string(),
        read_only: true,
        properties: BTreeMap::from([
            (
                "http".to_string(),
                Type::Obj(Obj {
                    name: "bereq.http".to_string(),
                    read_only: false,
                    properties: BTreeMap::from_iter(
                        DEFAULT_REQUEST_HEADERS
                            .iter()
                            .map(|header| (header.to_string(), Type::String)),
                    ),
                    ..Obj::default()
                }),
            ),
            ("url".to_string(), Type::String),
            ("method".to_string(), Type::String),
            ("xid".to_string(), Type::String),
            ("retries".to_string(), Type::Number),
            ("hash".to_string(), Type::String),
            ("proto".to_string(), Type::String),
            ("backend".to_string(), Type::Backend),
            ("uncacheable".to_string(), Type::Bool),
            ("is_bgfetch".to_string(), Type::Bool),
        ]),
        ..Obj::default()
    });

    let resp: Type = Type::Obj(Obj {
        name: "resp".to_string(),
        read_only: true,
        properties: BTreeMap::from([
            (
                "http".to_string(),
                Type::Obj(Obj {
                    name: "req.http".to_string(),
                    read_only: false,
                    properties: BTreeMap::from_iter(
                        DEFAULT_RESPONSE_HEADERS
                            .iter()
                            .map(|header| (header.to_string(), Type::String)),
                    ),
                    ..Obj::default()
                }),
            ),
            ("status".to_string(), Type::Number),
            ("reason".to_string(), Type::String),
            ("backend".to_string(), Type::Backend),
            ("is_streaming".to_string(), Type::Bool),
        ]),
        ..Obj::default()
    });

    let beresp: Type = Type::Obj(Obj {
        name: "beresp".to_string(),
        read_only: true,
        properties: BTreeMap::from([
            (
                "http".to_string(),
                Type::Obj(Obj {
                    name: "req.http".to_string(),
                    read_only: false,
                    properties: BTreeMap::from_iter(
                        DEFAULT_RESPONSE_HEADERS
                            .iter()
                            .map(|header| (header.to_string(), Type::String)),
                    ),
                    ..Obj::default()
                }),
            ),
            ("status".to_string(), Type::Number),
            ("reason".to_string(), Type::String),
            ("backend".to_string(), Type::Backend),
            ("backend.name".to_string(), Type::String),
            ("backend.ip".to_string(), Type::String),
            ("uncacheable".to_string(), Type::Bool),
            ("age".to_string(), Type::Duration),
            ("grace".to_string(), Type::Duration),
            ("keep".to_string(), Type::Duration),
            // ("storage".to_string(), Type::String),
            ("storage_hint".to_string(), Type::String),
        ]),
        ..Obj::default()
    });

    let obj: Type = Type::Obj(Obj {
        name: "obj".to_string(),
        read_only: true,
        properties: BTreeMap::from([
            ("ttl".to_string(), Type::Duration),
            ("grace".to_string(), Type::Duration),
            ("keep".to_string(), Type::Duration),
            ("age".to_string(), Type::Duration),
            ("hits".to_string(), Type::Number),
            ("uncacheable".to_string(), Type::Bool),
        ]),
        ..Obj::default()
    });

    let client: Type = Type::Obj(Obj {
        name: "client".to_string(),
        read_only: true,
        properties: BTreeMap::from([
            ("ip".to_string(), Type::String),
            ("identity".to_string(), Type::String),
        ]),
        ..Obj::default()
    });

    let server: Type = Type::Obj(Obj {
        name: "client".to_string(),
        read_only: true,
        properties: BTreeMap::from([
            ("ip".to_string(), Type::String),
            ("hostname".to_string(), Type::String),
            ("identity".to_string(), Type::String),
        ]),
        ..Obj::default()
    });

    let regsub = Type::Func(Func {
        name: "regsub".to_string(),
        signature: Some("(STRING str, STRING regex, STRING sub)".to_string()),
        r#return: Some(Box::new(Type::String)),
        ..Func::default()
    });

    let regsuball = Type::Func(Func {
        name: "regsuball".to_string(),
        signature: Some("(STRING str, STRING regex, STRING sub)".to_string()),
        r#return: Some(Box::new(Type::String)),
        ..Func::default()
    });

    let synthetic = Type::Func(Func {
        name: "synthetic".to_string(),
        signature: Some("(STRING str)".to_string()),
        ..Func::default()
    });

    let global_scope: Type = Type::Obj(Obj {
        name: "GLOBAL".to_string(),
        read_only: true,
        properties: BTreeMap::from([
            ("req".to_string(), req),
            ("bereq".to_string(), bereq),
            ("resp".to_string(), resp),
            ("beresp".to_string(), beresp),
            ("obj".to_string(), obj),
            ("client".to_string(), client),
            ("server".to_string(), server),
            ("regsub".to_string(), regsub),
            ("regsuball".to_string(), regsuball),
            ("synthetic".to_string(), synthetic),
        ]),
        ..Obj::default()
    });

    return global_scope;
}

/*
 * Check if provided `scope` contains provided type (`type_to_compare`). can_this_turn_into means
 * checking whether anything in `scope` can turn (cast) into `type_to_compare`.
 */
pub fn scope_contains(scope: &Type, type_to_compare: &Type, can_this_turn_into: bool) -> bool {
    if can_this_turn_into {
        if scope.can_this_cast_into(type_to_compare) {
            return true;
        }
    } else {
        if scope.is_same_type_as(type_to_compare) {
            return true;
        }
    }

    match scope {
        Type::Obj(obj) => obj
            .properties
            .values()
            .any(|prop| scope_contains(prop, type_to_compare, can_this_turn_into)),
        Type::Func(func) => match func.r#return {
            Some(ref ret_type) => scope_contains(ret_type, type_to_compare, can_this_turn_into),
            _ => false,
        },
        _ => false,
    }
}