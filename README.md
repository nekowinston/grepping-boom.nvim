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
  config = function()
    -- yes this is ugly, I'll probably fix it sometime so we can just use
    -- `config = true`
    require("grepping-boom")
  end
}
```
