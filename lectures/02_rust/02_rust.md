---
marp: true
theme: gaia
---

<!-- _class: lead -->

# Rust (!)

COLL 110, Spring 2023: Class 02

Clayton Ramsey

---

## Null pointer exception

```c
#include <stdlib.h>

int *russian_roulette() {
    if (rand() % 6 != 0) {
        // you live
        return malloc(sizeof int);
    }
    // oopsie poopsie!
    return NULL;
}

int main() {
    // hmm. today I will assume all pointers are valid ♥( ˆ⌣ ˆԅ)
    int *array = russian_roulette();
    *array = 25;
    return 0;
}
```

---

## Access out of bounds

```c
#include <stdio.h>

int main() {
    char my_str[5] = {'h', 'e', 'l', 'l', 'o'};
    printf("%s\n", (char *) &my_str);
}
```

---

## Use-after-free

```c
#include <stdlib.h>
#include <stdio.h>

void do_stuff(int *x, int n) {
    x[0] = n;
    free(x);
}

int main() {
    int *x = malloc(3 * sizeof(int));
    // you can look at my pointer, just for a little bit (ㅅ´ ˘ `)♡
    do_stuff(x, 1);
    printf("%d\n", x[0]);
    return 0;
}
```

---

## Data race

```c
#include <threads.h>

int a = 0;

int do_thread_thing(void* ignore) {
    a = 1;
    return 0;
}

int main() {
    // i love my value of a (っ◕‿◕)っ

    thrd_t id;
    thrd_create(&id , do_thread_thing, NULL);

    // read time
    int b = a;

    thrd_join(id, NULL);

    // where is my value of a (o⌓o)
    printf("%d\n", b);
}
```

---

<!-- _class: lead -->

# memory safety time

---

## Safety guarantees

```rust
fn main() {
    // we are assured that this is a valid Vec<u8> because null doesn't exist in Rust
    let array: Vec<u8> = russian_roulette();

    // we have to handle the case where the vector is empty
    match array.get(0) {
        Some(n) => println!("we got a number ٩( ᐛ )و {n}"),
        None => println!("the russians have been thwarted (⌐■_■)"),
    };

    // free the array and its memory. goodbye! ( ◕ᴗ◕)っ✂
    drop(array);

    // `array` is now inaccessible because we dropped it.
    // this is a compile error:
    // println!("{:?}", array.get(1));
}
```

---

## Ownership

```rust
fn main() {
    let x: Vec<u8> = vec![1, 2, 3];

    // this operation is a move.
    // To prevent multiple places from accessing the internals of `x` concurrently, `x` has now
    // been rendered inaccessible.
    let y = x;

    // Creating a reference does not move its target.
    let ref_y = &y;

    println!("`y` is {y:?}, and `ref_y` points to {ref_y:?}");
}
```

---

## Sum and product types

```rust
#[derive(Debug)]
struct Horse {
    num_legs: u8,
    name: String,
}

/// All creatures are either horses or not horses.
enum Creature {
    ItsAHorse(Horse),
    SomethingElse,
}

fn what_is_it(creature: &Creature) {
    match creature {
        Creature::ItsAHorse(h) => println!("It's a horse named {} with {} legs!", h.name, h.num_legs),
        Creature::SomethingElse => println!("I don't know what that is.")
    }
}

fn main() {
    let johnny = Horse {
        num_legs: 4,
        name: "johnny".to_string(),
    };

    let creature1 = Creature::ItsAHorse(johnny);
    let creature2 = Creature::SomethingElse;

    what_is_it(&creature1);
    what_is_it(&creature2);
}
```

---

## Mutation

```rust
fn main() {
    // All things are immutable by default and must be opted into with the `mut` keyword.
    let mut vector = vec![1969, 2001, 2022];

    let ref_mut = &mut vector; // This reference allows mutation of what it points to.

    ref_mut.push(2023);

    // A mutable reference cannot exist at the same time as any other reference.
    // This is OK because `ref_mut` no longer exists by the start of this line.
    let immutable_ref = &vector;
    println!("immutable_ref points to {:?}", immutable_ref);
}
```
