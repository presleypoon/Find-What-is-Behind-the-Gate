# Find What is Behind the Gate

This is a RPG game, explore all you want

## Bugs

Bugs are expected, especially in the pre-releases, if it is not in this files or other people already find it, please report it, this will massively help our game development

## exe

I have included the exe file, double click it to run, but if you don't trust me or your CPU is not x86_64 Windows, make sure you have cargo downloaded and run `build.bat`

## Customise Level

1. There's some built in levels in `assets\level`
2. you have to include it in `data.txt`
3. the format per level is `x y file_name_relative_to_data.txt`
4. If you need to make a comment, start line with `//`
5. Inside the file specified, the format is the following
    1. `s` is stone, non walk-through (in the future)
    2. `g` is grass, walk-through
    3. `d` is grass, walk-through
    4. `a` or &#8203;` `&#8203; is air, walk-through

For example:

```plain text
0 0 level-1_0.txt
-1 0 level0_0.txt
```
