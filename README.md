<div align="center">

  <h1><code>sweetch-bot</code></h1>

  <h3>
    <strong>Daemon which sends desktop notifications about games on sale</strong>
  </h3>

  <p>
    <img src="https://img.shields.io/github/workflow/status/devzbysiu/sweetch-bot/ci?style=for-the-badge" alt="CI status badge" />
    <a href="https://codecov.io/gh/devzbysiu/sweetch-bot">
      <img src="https://img.shields.io/codecov/c/github/devzbysiu/sweetch-bot?style=for-the-badge&token=f2339b3de9e44be0a902458a669c1160" alt="Code coverage"/>
    </a>
  </p>

  <h3>
    <a href="#about">About</a>
    <span> | </span>
    <a href="#installation">Installation</a>
    <span> | </span>
    <a href="#configuration">Configuration</a>
    <span> | </span>
    <a href="#license">License</a>
    <span> | </span>
    <a href="#contribution">Contribution</a>
  </h3>

  <sub><h4>Built with 🦀</h4></sub>
</div>

# <p id="about">About</p>

This app works as a daemon and periodically fetches the data about switch games then shows desktop
notification if any of the predefined games is on sale or have price below set limit.

It's rather for my own use but if you find it useful you can do things with it.

# <p id="installation">Installation</p>

Currently only Linux is supported.
- go to [releases](https://github.com/devzbysiu/je/releases) page
- download the latest `sweetch-bot` archive
- extract it
- run `sweetch-bot`

# <p id="configuration">Configuration</p>

### Location
`sweetch-bot` expects that configuration is in OS' configuration path.
| Platform | Value                                                  | Example                                                   |
| -------  | ------------------------------------------------------ | --------------------------------------------------------- |
| Linux    | `$XDG_CONFIG_HOME` or `$HOME`/.config/sweetch-bot.toml | /home/alice/.config/sweetch-bot.toml                      |

### Example configuration
Below you can see all options which can be configured.
```toml
debug = true                      # Enables debug level in logs.

[schedule]
run_at = ["7:00 pm", "8:00 am"]   # Run games check at those hours.

# List of watched games below

[[watched_game]]
title = "Minecraft Dungeons"      # By default, if this game is on sale, it will be
                                  # included in notification. You can use price 
                                  # criterion - see below.

[[watched_game]]
title = "DOOM"
acceptable_price = 7.00           # If this game has price <= acceptable_price, then
                                  # it will be included in the notification.

[[watched_game]]
title = "Alien: Isolation"
```

# <p id="license">License</p>

This project is licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

# <p id="contribution">Contribution</p>


Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
