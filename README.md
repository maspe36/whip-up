# whip-up
```whip-up``` is a build system with strong typing. ```.whip``` files are effectively arbitrary rust code*.

* = With the exception of the Compiler and Target struct. Which are part of the internals of ```whip-up```.

## Example

**.whip file**
```
let gpp = Compiler {
    path: PathBuf::from("/usr/bin/g++"),
};

let example_program = Target {
    name: "example_program",
    compiler: gpp,
    files: vec![
        PathBuf::from("main.cpp"),
        PathBuf::from("foo/bar.hpp")
    ]
};

example_program.build()

```

**Output g++ command which gets run**
```
/usr/bin/g++ -o example_project main.cpp foo/bar.hpp
```

## Usage
Say we have the following folder structure and a .whip file with the same contents as the example above.
```
projects/
|
└─── examples/
     |
     └─── foo/
     |    └─── bar.hpp
     |
     |    example.whip
     └─── main.cpp
```

Running ```whip-up examples/``` from the ```projects``` folder will result in a binary being built in the examples folder

```
projects/
|
└─── examples/
     |
     └─── foo/
     |    └─── bar.hpp
     |
     |    example.whip
     |    example_program
     └─── main.cpp
```

## About
I started working on this project during the COVID-19 pandemic. I was bored inside and decided to do something productive.

Currently this project is incredibly bare-bones and rather hacky. It should go without saying, but ```whip-up``` is not recommended for use in production.