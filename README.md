# Find What is Behind the Gate

This is a RPG game, explore all you want.

Call this FWIBTG if you want.

## Bugs

Bugs are expected, especially in the pre-releases, if it is not in this files or other people already find it, please report it, this will massively help our game development

### Known bug

1. The parsing is not working as intended
	* I tried to fix it, no luck

## exe

I have included the exe file, double click it to run, but if you don't trust me or your CPU is not x86_64 Windows, make sure you have cargo downloaded (download from: [Rust Cargo Installation](https://doc.rust-lang.org/book/ch01-01-installation.html)) and run `build.bat`

## Customise Level

1. Start with `b` (block) or `e` (entity, for the future)
2. For `b`:
	1. Write the starting x coordinate, that number is the left of the `txt`
	2. Then, write the starting y coordinate, that number is the top of the `txt`
	3. After that, write the `txt`'s file name down, without `.txt`
	4. In that `txt` file, you write the level from the x and y coordinate you given in 1 and 2
		1. `d` is dirt
		2. `g` is grass
		3. `s` is stone
		4. `·`, `╵`, `╶`, `└`, `╷`, `│`, `┌`, `├`, `╴`, `┘`, `─`, `┴`, `┐`, `┤`, `┬`, `┼` are cliffs, where there's line means there's connection
		5. `a` or &#8203;` `&#8203; is air
3. Make sure to compile the code again

For some reason, it only works if there's only 1 block file in `data.txt`, if someone can help, it will be appreciated
