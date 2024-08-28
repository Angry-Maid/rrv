# rrv
`rrv` (Rusted Replay Viewer) - a `rust`-ed version of the [GTFO Replay Viewer](https://github.com/randomuserhi/GTFOReplay) by [randomuserhi](https://github.com/randomuserhi)

## Features
As this project is pretty much a rewrite from ground up for the three.js/electron version of the viewer it will pretty much start off from most basic and barebones features(for now, todo list has all functionality from current replay viewer for parity sake, some of the things may become unimplemented or change their implementation/behaviour).
- [ ] Parsing replay file
  - [ ] Building map
  - [ ] Building static objects(mostly terminals, resource lockers/boxes, bulkhead hubs, doors and security doors, padlocks and hacklocks) placed on map
  - [ ] Building gatherables(both consumables and throwables) on map
- [ ] Loading models
  - [ ] Player
  - [ ] Enemies
  - [ ] Tools
  - [ ] Melee
  - [ ] Weapons
  - [ ] (?) Bulkhead Hub
  - [ ] (?) Terminal
- [ ] Timeline controlls and playback
  - [ ] Scrubbing timeline back and forth
  - [ ] Playback controlls
  - [ ] Live preview
- [ ] Additional viewer sidebar options
  - [ ] Settings
    - [ ] Timescale
    - [ ] Render Distance
    - [ ] Follow Player
    - [ ] Relative Rotation
    - [ ] Dimension Switcher
    - [ ] Transparent Resource Containers
    - [ ] Show Enemy Info (may migrate to show info for hovered entity)
    - [ ] Colour Enemies Based on Aggro
    - [ ] Show Fog Repelle Radius
  - [ ] Stats
    - [ ] Global statistics
    - [ ] Per-player statistics
    - [ ] "Medals" in statictics Tab-Menu
  - [ ] Item Finder (gatherables)
    - [ ] Filter by dimension
    - [ ] Search by name
    - [ ] Include Unknown Items
    - [ ] Navigate to item by clicking on it
  - [ ] Replay Information
        Should include basic info about replay, so far for now it's whether or not replay was done by host
  - [ ] App Info
    - [ ] Version and Changelog
    - [ ] (?) Compatability
- [ ] Loading custom profiles
