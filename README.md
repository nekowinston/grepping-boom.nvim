# grepping-boom.nvim

*stupid neovim plugin that booms after you're done with grepping*

### Requirements:

You need to have `make` available and Rust/`cargo` installed. This should
theoretically also work on Windows, but you'll have to build the plugin for
yourself, I'm not sure how to handle the build hook for it yet.


### Installation:

With [Lazy.nvim](https://github.com/folke/lazy.nvim):

```lua
-- lua
{
  "nekowinston/grepping-boom.nvim",
  build = "make", -- check the requirements above!
  opts = {},
}
```

### Usage

If you need to change the max volume of the booms, you can use
`:BoomVolume <volume>` where `<volume>` is a value between `0.0` and `1.0`.
Going above 1.0 is possible but not recommended (the author will not be held
responsible for plugin usage related hearing loss).

When you had enough of booming after grepping, you can face the consequences of
your actions via `:TwitchBan`, which will unload the plugin.

### Credit / blame

Feel free to blame the existence of this plugin on EMiNY from the Richcord.
