# Compile time experiments


Baseline compile with an empty main is `0.3s`.

```
extern crate lib;
fn main() {
    lib::test("somepath");
}
```

Incremental build in 2.38s

```
extern crate lib;
fn main() {
    lib::test1("somepath");
}
```

Incremental build in 0.31s

```
extern crate lib;
fn main() {
    lib::test2("somepath");
}
```

Incremental build in 0.31s
