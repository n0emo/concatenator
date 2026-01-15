# Concatenator

quickly concatenate many files into one

# Examples

```console
concatenator -d .
concatenator -d . -e gitignore -e rs -e toml -p '\n%{comment} File: %{path}\n\n'
concatenator -d chapters -d others -e md -o out.txt
```
