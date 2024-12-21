# re-name

`re-name` is a command-line tool for (bulk) file renaming.

## Motivation

There are two standard unix tools for renaming multiple files (confusingly both called rename): [util-linux rename](https://manpages.debian.org/bookworm/util-linux/rename.ul.1.en.html) and the different flavors of [perl rename](https://manpages.debian.org/bookworm/rename/rename.1.en.html). They both follow the general syntax 
```
rename <EXPRESSION> <REPLACEMENT> <FILES>
```
i.e. the files to be renamed are selected by both `<FILES>` and  `<EXPRESSION>`.

This is different from the syntax to rename single files with `mv`:
```
mv <SOURCE> <DESTINATION>
```

`re-name` aims to do file renaming in a `mv` like fashion where `SOURCE` and `DESTINATION` can be file patterns.

For example, consider the task of changing a bunch of files file extensions. For util-linux `rename` and perl `rename` one would use
```
rename .h .hpp *.h
```
and
```
rename 's/\.h$/\.hpp/' *.h
```
respectively while the same task is done in `re-name` as following:
```
re-name '(.*)\.h' '$1.hpp'
```

## Usage

`re-name` uses [regular expressions](https://docs.rs/regex/latest/regex/#syntax) to select the files or folders to be renamed. The `SOURCE` argument must match the *full* filename of the file to be renamed. Capture groups(`(...)`) can be used to extract parts of a filename which can then be used in the destination pattern.

The `DESTINATION` pattern makes use of the [replacement syntax](https://docs.rs/regex/latest/regex/bytes/struct.Regex.html#replacement-string-syntax) such that all occurrences of `$ref`, are replaced by the corresponding capture groups. Capture groups can be addressed by index (`$1`, `$2`, `$3` ...) or by name (`$group_name`). In case of ambiguity the name or number should be enclosed in braces, e.g., `${1}`.

Per default, `re-name` only renames files in the current folders and does not descend into subdirectories. This is the case to avoid accidentally matching path separators with a regex expression (e.g. `(.*)\.h` matches both `some_file.h` as well as `some_folder/another_file.h` which can lead to unexpected results. Matching subdirectories can be enabled with the `-r`/`--match-subdirs` flag but should be used with care.

In doubt it is recommended to use the `-p`/`--preview` flag to see what would be renamed and check if it everything worked out as intended. 

## Installation
### Using cargo:
```
cargo install re-name
```

## License

re-name was inspired by the command line tools [sd](https://github.com/chmln/sd) and [fd](https://github.com/sharkdp/fd/) and reuses some of their code. Both are licensed under the MIT License and come with the following copyright notices:

* sd: Copyright (c) 2018 Gregory
* fd: Copyright (c) 2017-present The fd developers

re-name itself is licensed under the [MIT License](LICENSE) as well.