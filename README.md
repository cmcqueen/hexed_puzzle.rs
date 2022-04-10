# Hexed Vintage Puzzle Solver

My family has a vintage puzzle from the 70s called "Hexed". It consists of 12 plastic pieces, with the goal of fitting them into a 6×10 unit grid. Each piece is a different shape, which is made up of 5 unit squares in some grid pattern (so I guess the game should be called "Pented" instead, but that's not such a catchy name). Each piece can be rotated by various 90° angles. Many of them can be flipped over to get a mirror-opposite shape.

According to the box, there are over 2,000 solutions.

I have tried solving the puzzle, and found it tauntingly tricky. Some of the pieces are such annoyingly awkward shapes. It's reasonable enough to fit most of them together into the available space. But getting those last 2 or 3 pieces to fit perfectly is a triumph. As far as I can tell, algorithmically, it comes down to a brute-force solution search. But a little logic might speed it up—eg, any "holes" in the solution grid must be a multiple of 5 unit squares, since every piece is made of some arrangement of 5 unit squares.

So I thought I'd try to write a solver. Normally Python is my go-to language for small projects, but for a solver like this, I want speed. I've been learning Rust in the last year or two, so Rust seems to be an excellent choice for a "Hexed" solver.
