# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Bevy Migration - [v0.3.8](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.3.8) - 2023-08-18

[![v0.3.8](https://img.shields.io/badge/v0.2.0alpha-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.3.8)](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.3.8)
[![**Full Commits History**](https://img.shields.io/badge/GitHubLog-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/fabinistere/fabien-et-la-trahison-de-olf/commits/v0.3.8)](https://github.com/fabinistere/fabien-et-la-trahison-de-olf/commits/v0.3.8)

- [Migration Guide Bevy 0.10 -> 0.11](https://bevyengine.org/learn/migration-guides/0.10-0.11/)
- *not needed* [Changelog Bevy Rapier 0.21 -> 0.22](https://github.com/dimforge/bevy_rapier/blob/master/CHANGELOG.md#0220-10-july-2023)

### Added

- [![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/fabinistere/fabien-et-la-trahison-de-olf#license)

### [Bevy 0.11](https://bevyengine.org/learn/migration-guides/0.10-0.11/) Migration

- ECS
  - `in_set(OnUpdate(*))` -> `run_if(in_state(*))`
  - Add the `#[derive(Event)]` macro for events.
  - Allow tuples and single plugins in `add_plugins`, deprecate `add_plugin`
  - [Schedule-First: the new and improved `add_systems`](https://bevyengine.org/learn/migration-guides/0.10-0.11/#schedule-first-the-new-and-improved-add-systems)
- UI
  - Flatten UI Style properties that use Size + remove Size
    - The `size`, `min_size`, `max_size`, and `gap` properties have been replaced by the `width`, `height`, `min_width`, `min_height`, `max_width`, `max_height`, `row_gap`, and `column_gap` properties. Use the new properties instead.
  - [Remove `Val::Undefinded`](https://bevyengine.org/learn/migration-guides/0.10-0.11/#remove-val-undefined)
    - `Val::Undefined` has been removed. Bevy UI’s behaviour with default values should remain the same.
    The default values of `UiRect`’s fields have been changed to `Val::Px(0.)`.
    `Style`’s position field has been removed. Its `left`, `right`, `top` and `bottom` fields have been added to `Style` directly.
    For the `size`, `margin`, `border`, and `padding` fields of `Style`, `Val::Undefined` should be replaced with `Val::Px(0.)`.
    For the `min_size`, `max_size`, `left`, `right`, `top` and `bottom` fields of `Style`, `Val::Undefined` should be replaced with `Val::Auto`
  - [Rename keys like `LAlt` to `AltLeft`](https://bevyengine.org/learn/migration-guides/0.10-0.11/#rename-keys-like-lalt-to-altleft)
  - [Delay asset hot reloading](https://bevyengine.org/learn/migration-guides/0.10-0.11/#delay-asset-hot-reloading)
  - [`Interaction::Clicked` replaced by `Interaction::Pressed`](https://bevyengine.org/learn/migration-guides/0.10-0.11/#rename-interaction-clicked-interaction-pressed)
- Dependencies
  - bevy_rapier_2d `0.22`
  - bevy_tweening `0.8`

### [Bevy 0.10](https://bevyengine.org/learn/migration-guides/0.9-0.10/) Migration

- Dependencies
  - bevy_rapier2d [0.21](https://github.com/dimforge/bevy_rapier/blob/master/CHANGELOG.md#0210--07-march-2023)
    - feature `debug-render` change to `debug-render-2d`
  - Remove bevy-web-resizer dependency: [Note: this functionality is now built into Bevy and this crate will no longer be maintained.](https://github.com/frewsxcv/bevy-web-resizer#readme)
- ECS
  - [Migrate engine to Schedule v3 (stageless)](https://bevyengine.org/learn/migration-guides/0.9-0.10/#migrate-engine-to-schedule-v3-stageless)
  - [System sets (Bevy 0.9)](https://bevyengine.org/learn/migration-guides/0.9-0.10/#system-sets-bevy-0-9)
  - [States](https://bevyengine.org/learn/migration-guides/0.9-0.10/#states)
- UI
  - [Windows as Entities](https://bevyengine.org/learn/migration-guides/0.9-0.10/#windows-as-entities)
  - [Remove VerticalAlign from TextAlignment](https://bevyengine.org/learn/migration-guides/0.9-0.10/#remove-verticalalign-from-textalignment)
  - [Remove the `GlobalTransform::translation_mut` method](https://bevyengine.org/learn/migration-guides/0.9-0.10/#remove-the-globaltransform-translation-mut-method)

### [Bevy 0.9](https://bevyengine.org/learn/migration-guides/0.8-0.9/) Migration

- ECS
  - [Make `Resource` trait opt-in, requiring `#[derive(Resource)]` V2](https://bevyengine.org/learn/migration-guides/0.8-0.9/#make-resource-trait-opt-in-requiring-derive-resource-v2)
  - [Spawn now takes a Bundle](https://bevyengine.org/learn/migration-guides/0.8-0.9/#spawn-now-takes-a-bundle)
  - [Accept Bundles for insert and remove. Deprecate `insert`/`remove_bundle`](https://bevyengine.org/learn/migration-guides/0.8-0.9/#accept-bundles-for-insert-and-remove-deprecate-insert-remove-bundle)
  - [Replace the `bool` argument of `Timer` with `TimerMode`](https://bevyengine.org/learn/migration-guides/0.8-0.9/#replace-the-bool-argument-of-timer-with-timermode)
  - [Add global time scaling](https://bevyengine.org/learn/migration-guides/0.8-0.9/#add-global-time-scaling)
- UI
  - TODO: [Change UI coordinate system to have origin at top left corner](https://bevyengine.org/learn/migration-guides/0.8-0.9/#change-ui-coordinate-system-to-have-origin-at-top-left-corner)
  - [Rename `UiColor` to `BackgroundColor`](https://bevyengine.org/learn/migration-guides/0.8-0.9/#rename-uicolor-to-backgroundcolor)
  - [Make the default background color of `NodeBundle` transparent](https://bevyengine.org/learn/migration-guides/0.8-0.9/#make-the-default-background-color-of-nodebundle-transparent)
    - remove useless field (completed by `..NodeBundle::default()`)
  - [Merge TextureAtlas::from_grid_with_padding into TextureAtlas::from_grid through option arguments](https://bevyengine.org/learn/migration-guides/0.8-0.9/#merge-textureatlas-from-grid-with-padding-into-textureatlas-from-grid-through-option-arguments)
- Dependency
  - bevy_tweening 0.6
    - [Removed the `tweening_type` parameter from the signature of `Tween<T>::new()`; use `with_repeat_count()` and `with_repeat_strategy()` instead.](https://github.com/djeedai/bevy_tweening/blob/main/CHANGELOG.md#changed-2)

## Curtains Update - [v0.2.0-alpha](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.2.0-alpha) - 2022-05-29

[![v0.2.0-alpha](https://img.shields.io/badge/v0.2.0alpha-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.2.0-alpha)](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/releases/tag/v0.2.0-alpha)
[![**Full Commits History**](https://img.shields.io/badge/GitHubLog-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/fabinistere/fabien-et-la-trahison-de-olf/commits/v0.2.0-alpha)](https://github.com/fabinistere/fabien-et-la-trahison-de-olf/commits/v0.2.0-alpha)

### Preview

![Physics](https://github.com/Fabinistere/fabien-et-la-trahison-de-olf/assets/73140258/89c2279a-9a56-4708-8812-220a8ea0645e)

### Feature

- Play with curtains

### Added

- Spawn in the Temple
