# Solution

I'm leaving this up in case anyone else runs into it, but it appears that ggez or one of it's dependencies is including the title bar in the window resolution. So when you request 800x600 you don't get a window with that much usable space. 

This means when you call `set_screen_coordinates` you get squashed since you "lose" the titlebar space, which is only in one axis.

Some variant of this appears to fix it for me:

```
canvas.set_screen_coordinates(graphics::Rect::new(
    15.5,
    15.5,
    ctx.gfx.window().inner_size().width as f32 / scale as f32,
    ctx.gfx.window().inner_size().height as f32 / scale as f32,
));
```

# Problem

When I set set_screen_coordinates to the same resolution as my window, I go from:

![Normal](normal.png)

to:

![Squashed](squashed.png)

# How to reproduce

- Have a mac it seems (does not reproduce on my Windows box)
- Sync repository
- Set cargo.toml to a local copy of ggez to a recent devel branch 41d9abe
- cargo run
- Hit F1 to flip between the two modes
