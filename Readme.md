# bool_enum

A simple Rust macro for creating semantic wrappers over a boolean value.

A way to create the enum of two possible states.

The bool_enums implement From<bool> traits and are clonable.

```Rust
// This private item:
bool_enum!(
    #[some_metadata]
    MyEnum: MyFalse=0, MyTrue=1
)

// Is equivalent to
#[some_metadata]
#derive(Clone)
enum MyEnum {MyFalse=0, MyTrue=1}
impl core::convert::From<bool> for MyEnum {
    ...
}

// Also public item definitions are possible:
bool_enum!{
    /// Documentation
    pub MyPublicEnum: MyFalse=0, MyTrue=1
}
```
