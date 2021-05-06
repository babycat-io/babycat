use pyo3::prelude::FromPyObject;

#[derive(FromPyObject)]
struct Foo();

#[derive(FromPyObject)]
struct Foo2 {}

#[derive(FromPyObject)]
enum EmptyEnum {}

#[derive(FromPyObject)]
enum EnumWithEmptyTupleVar {
    EmptyTuple(),
    Valid(String),
}

#[derive(FromPyObject)]
enum EnumWithEmptyStructVar {
    EmptyStruct {},
    Valid(String),
}

#[derive(FromPyObject)]
#[pyo3(transparent)]
struct EmptyTransparentTup();

#[derive(FromPyObject)]
#[pyo3(transparent)]
struct EmptyTransparentStruct {}

#[derive(FromPyObject)]
enum EnumWithTransparentEmptyTupleVar {
    #[pyo3(transparent)]
    EmptyTuple(),
    Valid(String),
}

#[derive(FromPyObject)]
enum EnumWithTransparentEmptyStructVar {
    #[pyo3(transparent)]
    EmptyStruct {},
    Valid(String),
}

#[derive(FromPyObject)]
#[pyo3(transparent)]
struct TransparentTupTooManyFields(String, String);

#[derive(FromPyObject)]
#[pyo3(transparent)]
struct TransparentStructTooManyFields {
    foo: String,
    bar: String,
}

#[derive(FromPyObject)]
enum EnumWithTransparentTupleTooMany {
    #[pyo3(transparent)]
    EmptyTuple(String, String),
    Valid(String),
}

#[derive(FromPyObject)]
enum EnumWithTransparentStructTooMany {
    #[pyo3(transparent)]
    EmptyStruct {
        foo: String,
        bar: String,
    },
    Valid(String),
}

#[derive(FromPyObject)]
struct UnknownAttribute {
    #[pyo3(attr)]
    a: String,
}

#[derive(FromPyObject)]
struct InvalidAttributeArg {
    #[pyo3(attribute(1))]
    a: String,
}

#[derive(FromPyObject)]
struct TooManyAttributeArgs {
    #[pyo3(attribute("a", "b"))]
    a: String,
}

#[derive(FromPyObject)]
struct EmptyAttributeArg {
    #[pyo3(attribute(""))]
    a: String,
}

#[derive(FromPyObject)]
struct NoAttributeArg {
    #[pyo3(attribute())]
    a: String,
}

#[derive(FromPyObject)]
struct TooManyitemArgs {
    #[pyo3(item("a", "b"))]
    a: String,
}

#[derive(FromPyObject)]
struct NoItemArg {
    #[pyo3(item())]
    a: String,
}

#[derive(FromPyObject)]
struct ItemAndAttribute {
    #[pyo3(item, attribute)]
    a: String,
}

#[derive(FromPyObject)]
#[pyo3(unknown = "should not work")]
struct UnknownContainerAttr {
    a: String,
}

#[derive(FromPyObject)]
#[pyo3(annotation = "should not work")]
struct AnnotationOnStruct {
    a: String,
}

#[derive(FromPyObject)]
enum InvalidAnnotatedEnum {
    #[pyo3(annotation = 1)]
    Foo(String),
}

#[derive(FromPyObject)]
enum TooManyLifetimes<'a, 'b> {
    Foo(&'a str),
    Bar(&'b str),
}

#[derive(FromPyObject)]
union Union {
    a: usize,
}

#[derive(FromPyObject)]
enum UnitEnum {
    Unit,
}

fn main() {}
