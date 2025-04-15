# Instructions

Set up the rust toolchain installer, and cargo.
https://www.rust-lang.org/tools/install

```console
cargo build --release
```

This will write the executable to target/release/ia_challenge. We can then run it with
```console
./target/release/ia_challenge
```

## Assumptions

Looks like there are no guidelines on how many facilities to generate, I'll add it as user input. I can reuse some code
this way, and I will design it to be relatively easy to swap out for an environment variable or compile-time
constant.

The example prompt doesn't really indicate the facility locations either, so it would be good to tell the user that
information somehow.

Facility ID's probably should be unique.

While I should manually test the code, writing unit tests and such seems a bit overkill for a demo.

# Initial Thoughts

I am preferring Rust here to make the packaging and linking easier.

I can initialize the world size as a compile-time constant, but it looks like it might be better to fit as an environment
variable.

I need some sort of user input, a full TUI is probably too much. There is some security concerns about accepting strings
as user input, but I'm not sure on how Rust specifically interacts with it. With C/C++, it would be more of a concern.

KNN Algorithm - straightforward approach, but it would run in some order of  `O(nk)`, where `n` is the number of 
facilities, and `k` is the number of neighbors, with the steps looking something like:

    1. Compute all the distances from a chosen node - O(n)
    2. Loop over the distances, and select the minimum unselected distance O(n * k)

However, our selection space is only [-10,10], which means at most O(400 * 3) computations, which can be easily
parallelized. For a smaller selection like this, it might be faster to do it directly. 
So, this will be the approach I take.

# Improvements

> Provide a brief summary of how you might change your program if you needed to support multiple central fills at the
> same location?

I structured my code modularity and maintainability, so adding this feature wouldn't be too bad. Instead of storing
a single `Facility` in the `HashMap`, we can instead store a `Vec<Facility>`, containing any overlapping facilities.
Then, when we pop elements off of the `PriorityQueue`, we can just keep a running counter.

> Provide a brief summary of how you would change your program if you were working with a much larger world size?

If the world size was significantly larger, there are some optimizations we can do such as implementing a k-d tree, 
or adding locality sensitive hashing. However, these approaches are more difficult to maintain.

Past a certain point, we can keep the setup, but I would use something like `PostgreSQL` and leverage the spatial query
abilities to solve for the nearest neighbors. In this case, we can remove the `knn()` function, and replace it with the
database connection.