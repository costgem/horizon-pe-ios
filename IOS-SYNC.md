# Horizon Pocket Edition (iOS): the sync ledger

`main` of shopifygav222/horizon-pocket-edition-ios is Starling with an iOS layer: the iOS app,
Horizon Pocket Edition ("Horizon PE" under its icon), is an iOS port of Horizon based on Starling, Horizon's Android
port. The older Horizon
Android iOS port is kept under `archive/horizon-android-ios`, and `main`'s history holds it as a parent; this app was
developed on `starling-ios` until it moved to `main`.

- **Branches and releases:** `main` is Horizon PE (release tags `v1.0.0`, ...). `shared-worlds` is `main` plus
  shared worlds and voice chat (release tags `v1.0.0-shared-worlds`, ...). `shared-worlds-mods` is `shared-worlds`
  plus fifteen native mods in its `native-mods/`, their touch buttons and controller binds, and its own defaults
  (release tags `v1.0.0-shared-worlds-mods`, ...; `docs/shared-worlds-mods.md`): its .ipa is built with those
  fifteen ids in `ios.yml`'s `mods`. A fix goes to the lowest branch it belongs to and is merged upwards
  (`main` into `shared-worlds` into `shared-worlds-mods`). The README is the same file on all three, and names
  each edition's latest release.
- **Releasing:** set `MARKETING_VERSION` (`packaging/ios/project.yml`) on both branches, run `ios.yml` with
  `deliver` on each, and publish each .ipa as a release of this repository: tag `v<version>` from `main`,
  `v<version>-shared-worlds` from `shared-worlds`, `v<version>-shared-worlds-mods` from `shared-worlds-mods`
  (built with its mods), none a draft or a pre-release, with the .ipa as an asset.
  The apps find them there (`horizon_client::ios_releases`, `game/update_offer.rs`): each edition is offered
  only its own tags, and only a version above its own. `ios_releases::EDITION` is the one line that differs
  between the branches. Then the README's links (the same file on every branch) name the new releases.

- **Base:** starling-client/starling `starling` @ `0e5bb58a44` (tag `v0.9.0` and its README commit). Remote:
  `starling`. The sync before it was from `ac874422ed` (`v0.8.0`). Starling 0.8.0 took Horizon Android's control
  system, account profiles with Ely.by, skins, catalog and Diagnostics itself, so this port carries none of its own:
  the work on the 0.2.3 base (a hand port of those) is in history under `archive/starling-ios-0.2.3`. 0.9.0 split
  `game/` into area folders whose modules `build.rs` discovers (`horizon-game-discovery`): a module's platforms are
  its first line (`// game-module-cfg: ...`), and a platform's start installs the engine's platform (seal, mod
  instrumentation, render points) before `entry::prepare` and `entry::start` (horizon-ios-game's `app.rs`, as
  Starling's `android_lib.rs`).
- **Workflows:** only `ios.yml` and `ios-guard.yml`. Starling's own (its APK and release builds, its build service,
  its registry, schedules and pull-request bots) need Starling's secrets and machines, so they are not in this tree:
  a sync that changes one of them is a "deleted here" conflict, kept deleted. So are its `CODEOWNERS`, pull request
  template and native-build issue form; `.github/ISSUE_TEMPLATE/bug_report.yml` is this app's.
- **Name:** the app is Horizon Pocket Edition. `APP_NAME` (render crate, "Horizon PE" on iOS, "Starling" elsewhere)
  is the name the game says; the setup's titles (`game/setup.rs`), the Files-app folder hints (`options_ui.rs`,
  `pack_screen.rs`) and the iOS-only text follow it, and on iOS the language files' "Starling" reads "Horizon PE"
  (`Language::load`), the credits excepted. The app is `HorizonPE.app`, bundle identifier `dev.horizon.pocketedition`, with the
  executable, Xcode project, target and scheme all `HorizonPE` (`packaging/ios/project.yml`). Builds before the
  rename were `Starling.app`, `dev.horizon.starling`: another app to iOS, whose worlds move over in Files.

## Repository layout

Starling's source (Horizon with Starling's Android port) with the iOS layer on top:

| Path | What it is |
| --- | --- |
| `crates/horizon-ios-game` | The app's program: its folders, the 16 MB stack the game runs on, the event loop |
| `crates/horizon-gamecontroller` | Every UIKit, GameController, Keychain, CoreMotion and Photos call (the game crates forbid unsafe code) |
| `crates/horizon-client/src/ios.rs` | The game's iOS hooks: files, Keychain sealing, the keyboard, haptics, gyroscope, imports |
| `crates/horizon-client/src/game/native_builds_ios.rs` | Native mods built on GitHub from the Mods page |
| `crates/horizon-ios-prepare` | Lays the game files out on the phone without Java |
| `packaging/ios` | The Xcode project (XcodeGen), `Info.plist`, icon, and the native-mod staging script |
| `tools/ios_prepare.py`, `tools/ios_native_mods.py` | The game tables made in CI; native mods fetched and checked |
| `third_party/winit` | Starling's winit with the iOS patches (scenes, keyboard and mouse, opened files) |
| Everything else | Starling and Horizon, as they are |

## Rules

1. iOS code lives in files of its own: `crates/horizon-client/src/ios.rs` and `ios/`, `crates/horizon-ios-game`, and
   `packaging/ios`. Starling's files change only where the game has to call the iOS layer, and every such change is a
   row in the engine-edit ledger below.
2. Starling's `crates/horizon-client/src/lib.rs` keeps `#![cfg_attr(not(target_os = "android"), forbid(unsafe_code))]`
   byte for byte (`packaging/android/native_build.sh` looks for that exact line). Every `unsafe` call into UIKit,
   Foundation, Security or CoreMotion is in `crates/horizon-gamecontroller`, behind safe functions.
3. A behaviour both phones share is gated `any(target_os = "android", target_os = "ios")`; what only Android's JNI
   can do stays `target_os = "android"` and gets an iOS counterpart where the game needs one.
4. The app follows Starling's phone build: where Starling's Android code is not JNI, iOS shares it; nothing is
   added that Starling's Android build does not have.

## Sync procedure

1. At a phase boundary, never in the middle of one: `git fetch starling` and merge `starling/starling` on a
   `sync/<date>` branch.
2. Resolve each conflict with the engine-edit ledger: every row says what the change is and why.
3. Walk the Android gates the merge added (`git diff <old base> <new base> | grep target_os`), and classify each one in
   the gate ledger.
4. `ios.yml` and `ios-guard.yml` green, then merge into `main` and move **Base** above.
5. When Starling takes one of the features below itself, switch to Starling's and mark the row "upstream"; ours goes
   once Starling's passes the same checks.

## What iOS shares with Starling's Android build, and what it cannot

| Starling 0.8.0 | iOS |
|---|---|
| Control system (MCPE, GoyControl, editor, hub, virtual mouse, keypad, gyroscope) | shared (`game/mobile_controls.rs`); its keyboard and haptics are `mobile_controls/mobile_platform_ios.rs` |
| Accounts (Microsoft, Ely.by, offline profiles), skins per profile | shared; secrets sealed with a Keychain key (`ios.rs`) |
| Catalog (mods, resource packs, shader packs) | shared; native mods are not offered (`starling-catalog`) |
| Diagnostics (Options > Diagnostics, performance recordings, ZIP) | shared; the ZIP is shared through Files' sheet; no exit history or recovery window (Android's `ApplicationExitInfo` and activity) |
| Setup: sign-in, download, prepare | shared; preparing lays out the tables CI made (`horizon-ios-prepare`), since iOS runs no Java |
| Bring worlds over from Horizon Android | shared; the folder is picked in Files |
| Game folder in the Files app | the app's Documents (UIFileSharingEnabled) |
| Native mods (`.hnmod`) compiled on the build server, the library swapped in at the next start | compiled into the app itself: `ios.yml` dispatched with `mods` fetches them (`tools/ios_native_mods.py`), applies them as `native_build.sh` does (`packaging/ios/native_stage.sh`, same horizon-mixins pin and adapt step), builds the .ipa (the run's artifact, never a release); `native-mods.json` beside the program names them. In the game, the catalog lists native mods and the Mods page's top (`game/native_builds_ios.rs`, drawn as Android's card) starts that build with the player's GitHub token, follows it, downloads the artifact with the same token and opens the share sheet (`horizon_gamecontroller::share`) for SideStore, AltStore, LiveContainer or any signer. A build that ends shows a toast (`ServerPackState::notice`) and a title-screen line along the top, clear of the touch pad's buttons (`draw_build_badge`, `build_badge_at` in game.rs's title paths; no-ops elsewhere), which opens the Mods page. The catalog's Installed view (`mod_catalog.rs`) lists and removes everything in a shelf's folders, on Android too. iOS runs no code it did not ship with, so there is no swap |
| Folder offer, in-app APK update, architecture APKs, Adreno/Mali/PowerVR paths | Android's own |

## Engine-edit ledger

Every change to a file Starling has, in the order they were made.

| File | Change | Why |
|---|---|---|
| `horizon-client-render/src/android_controls/layout.rs` | Toggle Sprint and Inventory Walk Controls on by default; `defaults_rev` turns them on once in files saved before | Asked for on iOS; shared with Android |
| `horizon-client-render/src/android_controls.rs` | Over an inventory, the MCPE Sneak button draws a Shift arrow (`shift_glyph`), filled while Shift is down | It is Shift there (quick move) |
| `horizon-client/src/game/mobile_controls.rs` | Sneak latches Shift and Inventory closes over an inventory without the walk controls too; LMB and RMB no longer clear the touch state (Shift, walking); Shift ends with the inventory | Shift stayed on for taps but LMB and RMB dropped it |
| `horizon-client-render/src/gfx/oit.rs` | Improved Transparency asks for `FLOAT32_BLENDABLE` only when `Rgba32Float` itself blends on the adapter | Metal reports the feature without blending the format: the depth-bounds pipeline panicked at world load (M2 iPad, Kelly-Inc/horizon-ios#25) |
| `horizon-client/src/game/native_hooks.rs`, `game.rs` | `native_camera_pose_now` (the frame's camera as native mods see it; handed back unchanged, as this base has no camera-posing hook) and its call where the frame's view is made | The registry's Atlas mod reads the camera at its return; upstream Horizon f319701e8c has it |
| `horizon-client/src/game/native_client_services.rs` (new), `game.rs` | `native_client_profiles`, `native_gui_nine_slice`, `native_send_chat`, as upstream Horizon f319701e8c has them | Host functions Atlas calls; upstream's own file name, so a sync meets them |
| `packaging/android/native_mod_adapt.py` | `render.pixels.get()` becomes `&render.pixels[..]` (CHANGED) | Atlas reads the block textures as newer Horizon keeps them |
| `horizon-client-render/src/dynamic_light.rs` | `select_iter` (`select` over any iterator, appended) | dynamiclights hands its sources from a drain, as newer Horizon has it |
| `horizon-client-core/src/text.rs` | `Text::option_name_value` ("Caption: value" via `options.generic_value`) | dynamiclights' settings screen |
| `horizon-client-render/src/widgets.rs`, `ui.rs` | `widgets::Action` and `ui::action_button` (the plain button here; newer Horizon adds a controller glyph) | dynamiclights' Done button |
| `horizon-client/src/game/native_client_services.rs` | `native_set_mod_key`, `native_client_action_bar`, `native_apply_screen_blur` (false: Options and pause already blur) | dynamiclights' key binding, toggle message and screen |
| `horizon-client/src/mobs/armor.rs` (new), `mobs.rs` | `mobs::armor::values`: armour and toughness as `hurt_after_armor` sums them | Jade's health-and-armour line |
| `packaging/android/native_mod_adapt.py` | Rewrites: burning projectiles from `arrow.on_fire()` (no fire time here), `mob_hit()` as a pair, `key_pressed` with one key, a pet's body armour from `extra.head` | dynamiclights and Jade, pinned to newer Horizon |
| `horizon-client/src/game/setup.rs`, `game/setup/account_form.rs` | Text typed with no key code (iOS's on-screen keyboard) goes into the first-run account form (`Form::type_text`) | Typing an offline name on first launch |
| `horizon-gamecontroller/src/lib.rs`, `Cargo.toml` (block2, log) | Background time asked for or given back from another thread is done on the main thread (GCD), and has an expiration handler that gives it back | iOS no longer ends the game for background time never returned (a Microsoft sign-in in Safari) |
| `horizon-client/src/ios.rs`, `game.rs`, `game/no_resources.rs` | The window prefers the home bar hidden and defers the bottom and top edges' system gestures (`ios::window_attributes`) | The home bar fades, and a thumb at the edge doesn't open Type to Siri or the Control Center |
| `horizon-client-render/src/android_controls/layout.rs` | `virtual_mouse_enabled` defaults to on on iOS, and `DEFAULTS_REV` 2 turns it on once in older iOS settings | The virtual mouse is on by default on iOS (Android unchanged) |
| `horizon-client/src/game.rs` | A character that comes with no key code passes the open screen's character hooks (`screen_text_typed`) before the focused text box | Every character of the iPhone keyboard comes that way, and a native mod's own screen (Waymap's waypoint editor) is typed in through those hooks only |
| `horizon-client-input/src/window.rs`, `horizon-client-render/src/gfx.rs`, `horizon-client/src/game/options.rs` | iOS paces the frame cap from the display (`display_paced_wait`, counted from `Renderer::acquired_at`) instead of the limiter's own clock | An iPhone presents only on its refreshes; the limiter beside them locked the frame rate at half (30) until something moved one of the two |
| `horizon-client/src/android_control_ui.rs` | The virtual mouse's LMB and RMB over a menu are 64 x 30 (were 48 x 20), their names centred, press a little past what is drawn (`mouse_button_touch_rects`), sit at the right edge from the screen's middle down (were in the bottom corner), and a grip over them drags the pair anywhere (`mouse_buttons_pos` in the touch settings; `layout.rs`, `game/mobile_controls.rs`) | Asked for on iOS: they were easy to miss with a thumb; shared with Android |
| `horizon-gamecontroller/src/lib.rs`, `horizon-ios-game/src/app.rs` | `request_high_frame_rate`: one display link whose preferred frame rate range is the screen's maximum, asked for at the first start | A ProMotion iPhone shows an app at 60 Hz unless something in it asks for more; the Info.plist key only allows it. As razlua/horizon-ios (Ferrite, MIT) does |
| `horizon-client-render/src/graphics.rs` | Phones start with asynchronous entity culling and every block entity cache on (`phone_savings`), turned on once in a graphics file saved before (`.horizon-phone-savings`) | Less work for each frame; as razlua/horizon-ios does. Its leaf mode Hidden and its particle culling are newer upstream Horizon's and not in this base |
| `horizon-client-render/src/extras.rs` | `wheel_cycles_settings` is off by default on iOS, and an older iOS extras file has it turned off once (`phoneDefaults`) | Scrolling the settings with a finger doesn't change them |
| `horizon-client-render/src/android_controls.rs` | On a screen nearer square than a phone's (an iPad), the factory top row spreads from its middle until its buttons stand apart | The top row's buttons overlapped on an iPad |
| `horizon-client/src/game/update_offer.rs`, `game.rs`, `game/touch.rs`, `ios_releases.rs` (new), `game/mod_catalog.rs`, `game/native_builds_ios.rs` | Android's update prompt is iOS's too (`cfg` android or ios): it asks this repository's releases for the app's own edition (`ios_releases::EDITION`), and installs through the share sheet (`horizon_gamecontroller::share`); an app from a fork or with native mods is only told (`official_build`) | The game says when an update of its edition is out, and installs it |
| `horizon-client/src/game/starling/{mobile_controls/mod.rs,mod_catalog.rs,update_offer.rs}`, `runtime/no_resources.rs` | First line `game-module-cfg` widened to `any(android, ios)` (no_resources: `not(any(...))`) | 0.9.0's module discovery: the modules the phones share, as `game.rs`'s `cfg`s said before |
| `horizon-client/src/game/runtime/resource_loader.rs` | `run_loading` hands the loading app to `ios::take_game` on iOS (`run_until_quit` elsewhere); the setup runs on iOS too | UIKit's event loop already runs; the setup asks for the game files as on Android |
| `horizon-client/src/game/runtime/{window_handler.rs,loading_screen.rs}` | `ios::window_attributes` on the game's and the loading screen's windows | The status bar and home bar hidden over the game, the edges' system gestures deferred |
| `horizon-client/src/game/interface/hud_room.rs`, `horizon-client-render/src/graphics_ui.rs` | The HUD, button and menu safe zones on iOS too; Video Settings' Display page on iOS has only those three groups | Starling 0.9.0's safe zones keep the HUD and buttons off the sensor housing and rounded corners |
| `horizon-mod-package/src/{mod_list.rs,selection/context.rs}` | iOS's `NOT_BUILT`, its tip and its native game buttons; `data_dir_with` reads iOS as macOS | `mod_list` moved out of horizon-client-render in 0.9.0 |
| `horizon-client-render/src/gfx/{arena.rs,pack_shadow.rs}`, `horizon-hdr`, `gfx/{surface_setup.rs,ray_query.rs}` | iOS pages cap at 32 MiB (`page_ceiling`), shadow maps at 4096 (`shadow_side_limit`); the Vulkan-only calls are `not(target_vendor = "apple")` | 0.9.0 has its own Android caps; iOS joins them. iOS has no Vulkan |

## Gate ledger

Every `target_os = "android"` site in Starling's module tree, and what iOS does there: **widen** (the gate becomes
`any(android, ios)`), **iOS** (an iOS counterpart in the iOS layer), or **Android only** (nothing on iOS needs it).

| File | Site | Class | Note |
|---|---|---|---|
