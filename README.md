# My Rust battlefield

* Operators and symbols [[RPL]](https://doc.rust-lang.org/book/appendix-02-operators.html)
* _Type coercion_ - automatic type casting. [[REF]](https://doc.rust-lang.org/reference/type-coercions.html).
* _Deref coercion_ - automatic dereference. [[RPL]](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/deref-coercions.html).
* _Orphan rule_ (aka _Coherence_) - Implementation allowed in the same crate. [[RPL]](file:///home/ps/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/book/ch10-02-traits.html#implementing-a-trait-on-a-type).

## Variable declaration

**let** [**ref**] [**mut**] _VAR_ [**:**_T_] [**=** _RHS_] **;**

where:

_T_ - specifies _RHS_ value type. If not specified _T_ is implied from _RHS_.<br>

_T_: [**&mut**] [**&**] _T_<br>
_T_: _non_ref_type_

**ref**, **mut** defines _VAR_ as a reference and/or make it mutable.

```rust
// i type: i32
let i = 0;
// ri type: &i32
let ref ri = i;
// rri type: &&i32
let ref rri = ri;
// mi type: mutable i32
let mut mi = i;
// rmi type: &mut i32
let ref mut rmi = mi;
```

Note, the most generic format of the variable(s) declaration is:

**let** _PATTERN_ = _RHS_ **;**

That means pattern matching may occur for the **let** statement, e.g.:

```rust
let mut mi = 0;
let ref mut rmi = mi;

// pattern matching, i type: i32
let &mut i = rmi;
```

## `Fn`, `FnMut`, `FnOnce`

`Fn`: possible to call multiple times for multiple copies,<br>
`FnMut`: possible to call multiple times for a single copy,<br>
`FnOnce`: possible to call once for a single copy.<br>

If an object may be called multiple times for multiple copies, it may be also
called multiple times for a single copy. Similarly, if an object may be called
multiple times for a single copy, it may be also called once for that copy.

For that reason `Fn` is subtype of `FnMut` which is subtype of `FnOnce`.

## Lifetimes

* Notations
  * `'b: 'a`: Generic lifetime 'b must outlive lifetime `'a` (`'b` is a subtype
    of `'a`). See _Lifetime bounds_ [[REF]](https://doc.rust-lang.org/reference/trait-bounds.html#lifetime-bounds).
  * `T: 'a`: Generic type T must outlive lifetime `'a` (all references in T
    must outlive `'a`). See _Implied bounds_ [[REF]](https://doc.rust-lang.org/reference/trait-bounds.html#implied-bounds).

* _Lifetime elision_ - automatic lifetimes marking. [[REF]](https://doc.rust-lang.org/reference/lifetime-elision.html).

* `struct S` defined as: `struct S<'a, 'b, ...> { ... }` means: lifetimes `'a`,
  `'b`, ..., of objects they refer to must outlive struct `S` object containing
  the references.

  ```rust
  struct S<'a>(Option<&'a S<'a>>);

  let s1 = S::<'_>(None);
  {
      // s2 references to s1 which outlives s2
      let s2 = S::<'_>(Some(&s1));
  }
  ```

* While matching declared lifetimes with the actual object (for structs or function
  calls) they refer to, a compiler tries to find proper lifetimes fulfilling the
  declaration, lifetime bounds etc.

  Some example [here](src/lifetime.rs) illustrating lifetime bounds for legacy and
  newer lifetime notation.

  The process of the substitution of actual lifetimes (e.g. in the process of
  function call) with their identifiers is expressed by _Higher-ranked trait bounds_
  (_HRTB_) [[NOM]](https://doc.rust-lang.org/nomicon/hrtb.html),
  [[REF]](https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds),
  where "for all lifetimes" term is used to name the substitution while defining
  a type (function, structs, traits) containing the lifetimes.

* _Subtyping and Variance_ [[NOM]](https://doc.rust-lang.org/nomicon/subtyping.html#subtyping), [[REF]](https://doc.rust-lang.org/reference/subtyping.html)

  While passing reference to T in a function: `fn func<'a>(t: &'a mut T)`,
  passing parameter must not outlive the function argument, which basically means
  `T` returned by `func()` must not contain references with shorter lifetime that
  object passed to the function to avoid dangling references.

  Good example illustrating the rule has been provided in [NOM](https://doc.rust-lang.org/nomicon/subtyping.html#variance).

  ```rust
  fn assign<T>(input: &mut T, val: T) {
      *input = val;
  }

  fn main() {
      let mut hello: &'static str = "hello";
      {
          let world = String::from("world");
          assign(&mut hello, &world);
      }
      println!("{hello}"); // use after free!
  }
  ```

  Actual parameter to `assign()` has type `&'a mut &'static str`, while the
  function argument type is `&'a mut &'a str`, where `'a` denotes `world`
  lifetime. `'static` is a subtype of `'a` (longer lifetime), therefore after
  returning from `assign()` `str` may be a dangling reference.

  NOTE: The above excerpt would compile fine in case mutability had been removed.
