# Combine #
Combine is a small Tetris-like math puzzle game written in Rust. It uses Bevy Engine and was originally submitted to [Bevy Jam #2](https://itch.io/jam/bevy-jam-2)

You can try WASM build of the game in [itch.io](https://rockyjam.itch.io/combine)

### Gameplay ###
The basic gameplay should be familiar to anyone who has ever played Tetris. Use Arrow Keys to move the dropping block around and try to land it next to same colored blocks to manipulate their numbers. The target is to *combine* numbers in way that they result to tens (0, 10, 20, 30, etc). Oh, and you can switch the color of the dropping block using shift key. (See full keymap below).

You will get points based on the value the block had before it breaks. As the maximum value of a block is 99 the highest score you can get from single block is 99.

![combine](https://user-images.githubusercontent.com/6039147/187083211-76b05111-973c-40b4-8e3a-aea5e25ab452.png)

### Key Map ###
- Left / Right: Move the block left/right
- Down: Speed up the dropping block
- Shift: Switch the color of the dropping block
- Q: Switch the color to Blue
- W: Switch the color to Red
- E: Switch the color to Yellow
- R: Switch the color to Green
- 1: Background music track #1
- 2: Background music track #2
- 3: Background music track #3
- M: Mute background music

### Source Code ###
The code is open source as requested in Bevy Jam rules. However, if you're here to learn about Bevy, keep in mind that this code is written in just a few days :)

### License ###
Check the LICENSE file

### Credits ###
Gamplay design and coding by Jussi Kallio

Music by Eric Matyas (www.soundimage.org)
