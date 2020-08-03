## Known bugs

### Breaking

* On initialization:
  ```
  thread 'dummy_game' panicked at 'assertion failed: attribute_pool.is_empty()', src/gamestate/active.rs:47:9
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  ```
* FIXED? Defender assist (on winning team) lost token of attacker-color

### Non-breaking

* 'targeted/initial' displayed _below_ player
* "Adding defenders/attackers to [attack]" is confusing, b/c "[attack]"
  formatting starts with "Attackers:"
* Assists are listed as "combatants", after primary combatant has already been
  listed
