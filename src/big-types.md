# A note on type with owned buffers

When creating abstractions that requires some backing buffer, one can be inclined to use an owned
buffer inside the new type to avoid `lifetime` complications, however that can cause problems for
end users. For example, assume you are writing a library to interface with a display and you want to
use a frame buffer to allow for better manipulation of graphics, you might want to write something
like this:

```rust
{{#include ../ci/big-types/src/lib.rs:20:44}}
```

This seems nice, we are using type state to prevent the user from calling `draw` on a
non-initialized `Display`. But let's see what happens when an user tries to use this with
interrupts, a thing that is quite common in embedded software:

```rust
{{#include ../ci/big-types/examples/owned.rs:10:38}}
```

There is a nice dance to be able to put things on static context, but that is expected and is not
the problem that we are concerned here, there are crates to help you with that. There is another,
more subtle problem:

```rust
{{#include ../ci/big-types/examples/owned.rs:19}}
```

What is happening here is that we are creating and initializing a `Display` on the stack and, after
that, moving it to our `static` variable*. We are, at that point, using twice the RAM that we need
to store a `Display`, and since it is has a size of at least 1024 bytes that can really cause some
applications to become inviable due to stack overflows. Moreover, if the `interrupt_free` function
gets inlined we are wasting that space for the whole duration of our program, because `main` never
returns.****

> \* There is an optimization known as `NRVO` (named return value optimization), which tries to
> prevent this problem by creating the type in place. There is work being done to improve this
> optimization in the rust compiler, however, this optimization is not guaranteed to happen.

There are a couple of ways of addressing this problem, one could get rid of the type state and check
at runtime if the `Display` is already initialized before sending drawing commands, that way
`Display::new()` could be `const` (if there is a way to get an interface on a `const` context), but
even then, this solution does not seem satisfactory, we are adding some runtime overhead.

A better solution might be to borrow the frame buffer instead of owning it, for example:

```rust
{{#include ../ci/big-types/src/lib.rs:48:72}}
```

That way the user could then write something like this:

```rust
{{#include ../ci/big-types/examples/borrowed.rs:10:42}}
```

We are still paying double price on the `static` `Option`, but now its size is way smaller than
before. Unfortunately, it means that both the driver author and the user will need to pay more
attention to `lifetimes`. In several cases we can make our `new` method `const`, thus avoid both
problems.

## Bss and data

`Bss` and `data` are common sections of a firmware. In the usual case, all `static` variables will
end on one of these two sections. These sections are located in RAM and will be initialized by a
runtime (`cortex-m-rt` in our examples) before `main` is called.

RAM is volatile, because of that, the initial value of these variables must be stored in a non
volatile memory to be copied over to RAM during startup, causing it to consume program space. To
mitigate this, these variables are separated into two groups, zero and non-zero initialized. The
non-zero initialized variables will be placed into `data` and it will also consume program space,
while zero initialized ones will be placed into `bss`, however, we already know their initial
values: zero, which means that we do not need to also store their initial values on a non volatile
memory, our startup runtime just needs to know the start and end address of the `bss` area to fill
it with zeros before calling `main`.

Given that, even when having big types with a `const` "constructor", the library's authors should
strive to create abstractions that allow them to be zero initialized, otherwise, their users may
need to spare a considerable amount of flash space in addition to RAM space. In that case, it might
be better to use the borrow technique and initialize the buffer during driver creation instead of
having a driver type that can be created in `const` context.
