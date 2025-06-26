# Challenge Overview

In the burgeoning era of quantum computing, traditional cryptographic methods are under threat. 
Visionary scientists have developed new 'post-quantum' cryptosystems, one of which is based on 
the Learning With Errors (LWE) problem. LWE's security relies on the presumed difficulty of solving
certain mathematical problems involving lattices.  -- Google Gemini

## Learning With Errors (LWE)

Learning with Errors (LWE) is a foundational problem in lattice-based cryptography. SUSTech students
should alreadly take the linear algebra course, so we won't bother repeating the basics of linear algebra here.

Integer Lattice is a group of basis vectors spanning a discrete grid in n-dimensional space. All of the vectors
in the lattice are expressed as integer linear combinations of the basis vectors. However, if we constuct a vector
using

$$
\mathbf{v} = \mathbf{A} \cdot \mathbf{x} + \mathbf{e}
$$

where A is a matrix (lattice), x is a vector of secret values, and e is a small error vector. It's hard to find x 
given v, A, and e, because all you can do is to guess or go through every combination of x, which is computationally 
infeasible for large dimensions.

One may argue that: well, we could use guassian elimination to solve the linear system, but the error vector e
accumulates the error in the system, making it impossible to find the exact solution. The error vector is small,
but it is a disaster for the system.

That gives security to the LWE problem.

## Fully Homomorphic Encryption (FHE)

> Acknowledgements: pictures are all from [tfhe-rs](https://docs.zama.ai/tfhe-rs) and its blog post.

When applying LWE to cryptography, we can either put our secret in the vector x, and we can do an addition on the ciphertext
because suppose

$$
    b_1 = A \cdot x_1 + e_1
$$

$$
    b_2 = A \cdot x_2 + e_2
$$

if we add them together, we get

$$
    b_1 + b_2 = A \cdot (x_1 + x_2) + (e_1 + e_2) = A \cdot (x_1 + x_2) + e^{\prime}
$$

but it's weird that we have e as the secret key and we have to add the secret keys to recover the original secret.
How can we know what the operations done on the ciphertexts? So it's not applicable. The most common way is to use the
vector x as the secret key and put the plaintext in the error vector e

$$
    b = A \cdot x + e + m * \Delta
$$

so that we have (A, b) as the public key and x as the secret key. The real implementation will be more complicated, for example

![](https://cdn.prod.website-files.com/622ef9de9152c97467eac748/6734ce8356f7618c99eca317_626beffc06ae641cb1dc6078_GLWE_encryption.png)

It's `tfhe-rs`'s choice of GLWE, and for decryption, it's straightforward

![](https://cdn.prod.website-files.com/622ef9de9152c97467eac748/6734ce8356f7618c99eca314_626bf01306ae64175cdc611a_GLWE_decryption.png)

The ciphertext may look like

![](https://cdn.prod.website-files.com/622ef9de9152c97467eac748/6734ce8356f7618c99eca2f9_626bf08f2e754fa2b3537f09_GLWE_encoding_coefficient.png)

adding them up is like

![](https://cdn.prod.website-files.com/622ef9de9152c97467eac748/62754a57b2e64bf8df5c8b47_LWE_encode_msb_ADD.png)

in math, we have

$$
    b_1 + b_2 = A \cdot x + (e_1 + e_2) + (m_1 + m_2) * \Delta
$$

the error is trivial as long as it's less than the delta. The additive property of the ciphertext is called homomorphic addition.
If we extend the computational property beyond addition, like with multiplication, comparison and etc, it becomes fully homomorphic
encryption (FHE).

With that, we can confidentially handle the computation task to the untrusted server, and the server can compute the result without 
knowing the plaintext. Isn't it cool?

## Playing it with a Game of Life

In this challenge, we will implement a simple Game of Life using LWE. The Game of Life is a cellular automaton devised by the British
mathematician John Horton Conway in 1970. It consists of a grid of cells that can be either alive or dead, and the state of each cell 
changes based on the states of its neighbors.

However, the FHE is more expensive than expected due to the encryption/decryption/homomorphic operations, so we need to optimize the
Game of Life to make it efficient. Thankfully, the game of life is naturally parallelizable.

## Cracking LWE (Bonus)

After finishing _Fhe Game of Life_, we can now try to crack the LWE problem. It's not that hard as we scale down the parameters a little
bit. The point of this challenge is to understand how to accelerate a problem that you probably don't know a bit. When we say optimize it,
we are saying
1. you need to understand the problem and research about the algorithms that can solve it
2. you need to implement the algorithm in a way that is efficient and scalable, and in Rust
3. you need to optimize the implementation using parallel, SIMD, and other techniques

To crack the LWE problem, we will find the secret vector x given the public key (A, b).

An intuitive way to crack LWE is to reduce it to a Closest Vector Problem (CVP). CVP defines a problem of finding the closest vector
to some vector in lattice. It's formulated as follows: given a vector w that is not in the lattice, find a vector v in the lattice such
that the distance between w and v is minimized. The distance is usually measured by the Euclidean norm.

Normally, when the dimension is small, Babai's nearest plane algorithm can solve the problem
1. Find a basis that is sufficiently orthogonal with LLL (Lenstra–Lenstra–Lovász lattice basis reduction)
2. let b = t (t is the encrypted vector)
3. for j from n to 1, run `b = b - B[j] * c[j]` where `c = round(b * hat(B[j] / (hat(B[j]) * hat(B[j])))` 
    (hat(B[j]) means the gram schmidt of B[j], and B[j] is the j-th basis vector after LLL reduction)
4. return t - b

However, under a module `q`, we need to do some tricks to make it work. Since

$$
    b = A \cdot x + e \mod q
$$

we have

$$
    A x + q I_m k = b - e
$$

so that to find x, we can construct a new basis with

$$
    [A | q I_m] \begin{bmatrix} x \ k \end{bmatrix} = b - e = b^{\prime}
$$

we don't need to care about the k at the solution. The final step is to do guassian elimination on the 

$$
    A x = b^{\prime} \mod q
$$

which is trivial.