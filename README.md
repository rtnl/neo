# neo

neo is a modal shell program with an emphasis on consistency, versatility and transparency.
this project is a solution for a (finally) unified experience for virtually all platforms.

## features

- execution
  - command execution
    - [x] run program
    - [x] args
    - [ ] envs
- io interface
  - channels
    - [ ] channel registering
    - [ ] push into channel
    - [ ] pull from channel
- runtime
  - objects
    - [ ] number
    - [ ] string
    - [ ] list
    - [ ] table
    - [ ] record
    - [ ] request
    - [ ] response
    - [ ] function
    - [ ] monad
    - [ ] schema
  - state
    - [ ] global state
    - [ ] local state
  - parsing
    - [ ] string literals
    - [ ] system command integration
    - [ ] string templates
  - serialize
    - [ ] json
    - [ ] ion
  - suggestion
    - [ ] autocomplete commands
    - [ ] autocomplete paths
  - ui
    - components
      - [ ] text
      - [ ] block
      - [ ] menu
      - [ ] bar
      - [ ] image
    - [ ] panes
    - [ ] keybinds
  - commands
    - [ ] exit
    - [ ] print
    - [ ] env-get
    - [ ] env-set
    - [ ] env-export
    - [ ] env-import
    - [ ] file-read
    - [ ] file-copy
    - [ ] file-move
    - [ ] file-remove
    - [ ] file-write
    - [ ] find

## author's note

as of now, I'm thinking of leading this into something like a blend of tmux with native object support inspired from
powershell and fancy data display like nushell. I'll revise the feature set continuously as I work on the
implementation.

[![Star History Chart](https://app.repohistory.com/api/svg?repo=rtnl/neo&type=Date&background=0D1117&color=b562f8)](https://app.repohistory.com/star-history)
