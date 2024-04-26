# Rules for Advent of Code:

1. Chain should grow vertically. If it is growing horizontally (like this line), split the closure into an abstraction.

1. Functions shouldn't be classified as long or short, but as form of abstraction.
  > Meaningful Abstraction or Unnecassary Wrappers. Avoid latter.

1. Abstraction should be meaningful. WET over DRY.
  > Do not hack some function just because you can.

1. Meaningful abstraction at the cost of time commitment?
  > Do it manually.

1. No to loops. I find those hard to read.

1. Prefer Point-free calls where possible.

1. Minimal error handling. AOC has puzzle with well defined input.
  > No need to overengineer X to prevent Y when Y does not exist.

