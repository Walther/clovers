# PLY models

These famous objects are from the [Large Geometric Models Archive at Georgia Tech](https://sites.cc.gatech.edu/projects/large_models/index.html).

| Model           | Faces     | Vertices |
| --------------- | --------- | -------- |
| Stanford Bunny  | 69,451    | 35,947   |
| Stanford Dragon | 871,414   | 437,645  |
| Happy Buddha    | 1,087,716 | 543,652  |

The files were converted from the ASCII ply format to the binary ply format using the `ply2binary` program, built from the sources available [here](https://sites.cc.gatech.edu/projects/large_models/ply.html). To quote from the README of that repository:

> These geometry filters have been developed on a Silicon Graphics
> workstation using the native C compiler. The code may very well run
> unmodified on other platforms but this has not yet been verified.

Indeed, the code assumes a different endianness from the running operating system. As a workaround, after running `./ply2binary < ~/example.ascii.ply > ~/example.binary.ply`, you need to edit the header in the binary file:

```diff
- format binary_big_endian 1.0
+ format binary_little_endian 1.0
```

After this change, the binary file can be read correctly by compliant parsers.
