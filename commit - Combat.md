# Commits and various todos

## spawn `CombatPuppet` in the `FightingScene`

- Create designed ingame zone to fight (depending the location)
  - create 18 place as tactical position
  - spawn additional fighters where needed
  - move all of fighters to designated tactical position
  - add all HUD-like components to fighters (Clickable, etc)
- Or stick with all HUD
  - spawn 18 combat puppets which will change visibility/appearance to match the fighter at the current tactical position

## Use `OnExit(CombatState)` exclusive systems instead of `combat::phases::phase_transition`

If a certain case of transition requested wasn't "satisfied", just handle these 3 cases locally before switching `CombatState`

## Current tasks

- Use `CombatWallStage` as a `Resource` not a `State`. (we do not use it as a run_if and need it instantly not a frame late)
