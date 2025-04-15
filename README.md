# Instructions

Set up the rust toolchain installer, and cargo. See the following link for more instructions.
https://www.rust-lang.org/tools/install

To build the target for release, enter the command.

```console
cargo build --release
```

This will write the executable to target/release/ia_challenge. We can then run it from `ia_challenge/` with
```console
./target/release/ia_challenge
```

After that, please input a positive integer for the facility count and RET, or just RET for a default value.

Coordinates should be inputted with the following format in the range of [-10, 10] for x and y, followed by a RET

ex:
```console
-5,10
```

To end the program, just enter RET.

# Assumptions

Looks like there are no guidelines on how many facilities to generate, I'll add it as user input. I can reuse some code
this way, and I will design it to be relatively easy to swap out for an environment variable or compile-time
constant.

The example prompt doesn't really indicate the facility locations either, so it would be good to tell the user that
information somehow.

Facility ID's should be unique.

I should match the formatting given in the example.

Upon revisiting the problem statement, it states that 

    "Each central fill facility has different medications (A, B, C) as Inventory"

    ...returns a list of the three closest central fill facilities, along with the 
    cheapest medication price for each central fill

This indicates to me that each Central Fill Facility should have a price for all 3 medications. I had assumed prior that
each one could only hold one medication. Modified the code for this behavior.

If the input is out of bounds, the program should prompt the user for another value.

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

## Edge Cases

> What if the user inputs a coordinate out of bounds? What about other types of inputs?

The program will display a relevant error message, and exit. If the input is out of bounds, the program should prompt
the user for another value.

> Double check the bounds of the program, [-10, 10] should work.

> What if there are multiple facilities of the same distance?

Currently, the behavior depends on the `HashMap` keys ordering (in this case the index). This is randomly ordered. If
ordering is important/deterministic, we can use a `BTreeMap`, which will give ordering to the keys.

> What if a Facility contains 2 equal medication costs?

This behavior depends on how `PartialOrd` is implemented for `f64`, since comparing floats is weird. I am not sure on the 
exact behavior, but I would imagine it defaults to `self`, which would be the first medication type by order.

> What if our random seeding algorithm creates a collision?

We can either increase the runtime, by re-reunning and checking for collisions, or choose a different generation method.
I chose a generation method with no collisions, which is getting a list of all the indices, and randomly shuffling them.
For this problem, I think this approach works best, given the space constraints.

# Improvements

> Provide a brief summary of how you might change your program if you needed to support multiple central fills at the
> same location?

I structured my code modularity and maintainability, so adding this feature wouldn't be too bad. Instead of storing
a single `Facility` in the `HashMap`, we can instead store a `Vec<Facility>`, containing any overlapping facilities.
Then, when we pop elements off of the `PriorityQueue`, we can just keep a running counter.

> Provide a brief summary of how you would change your program if you were working with a much larger world size?

If the world size was significantly larger, there are some optimizations we can do such as implementing a k-d tree, 
or adding locality sensitive hashing. However, these approaches are more difficult to maintain.

Past a certain point depending on the system memory, we can keep the setup, but I would use something like `PostgreSQL` 
and leverage the spatial query  abilities to solve for the nearest neighbors. In this case, we can remove the `knn()` 
function, and replace it with the database connection, and run a query to find knn.

Assuming the same problem statement, the DB would look something like this 

```postgresql
CREATE TABLE medication_type (
  id serial PRIMARY KEY,
  name varchar
);

CREATE TABLE distance (
  id serial PRIMARY KEY,
  loc point
);

CREATE TABLE facilities (
  id serial PRIMARY KEY references distance,
  inventory numeric(10,5)[],
  price numeric(10, 5),
  medication_type serial references medication_type(id)
);
```

Then, you can create a spatial index on the `distance` table, and select knn. There is some more normalization possible
with facilities, but I am leaving it to preserve the relative logic in the current implementation.