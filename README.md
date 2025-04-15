# Instructions

## Assumptions

Looks like there are no guidelines on how many facilities to generate, I'll add it as user input. I can reuse some code
this way, and I will design it to be relatively easy to swap out for an environment variable or compile-time
constant.

The example prompt doesn't really indicate the facility locations either, so it would be good to tell the user that
information somehow.

Facility ID's probably should be unique.

# Initial Thoughts

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



> Provide a brief summary of how you would change your program if you were working with a much larger world size? 