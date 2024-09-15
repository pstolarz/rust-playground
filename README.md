# My Rust battlefield

## Variable declaration

**let** [*destruct*] [**ref**] [**mut**] `VAR` [**:** *T*] [**=** `RHS`] **;**

where:

*T* - specifies RHS value type. If not specified *T* is implied from RHS value.<br>
&nbsp;&nbsp;&nbsp;&nbsp;*T*: [**&mut**] [**&**] *T*<br>
&nbsp;&nbsp;&nbsp;&nbsp;*T*: *non_ref_type*

**ref**, **mut** makes the defined `VAR` as a reference and/or mutable.

```
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

*destruct* allows destructurize RHS value while defining `VAR`.

```
let mut mi = 0;
let ref mut rmi = mi;
// i type: i32
let &mut i = rmi;
```
