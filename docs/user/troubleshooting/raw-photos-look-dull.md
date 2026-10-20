---
title: Raw photos look dull or washed out
description: Checks for flat RAW colour in the upstream LightCraft workflow; current LightAdobado behavior is still being validated.
order: 20
---

> **Upstream note:** The version-specific examples below describe LightCraft 0.4.0–0.7.1 and have
> not been checked against a tested LightAdobado package.

If a RAW photo looks flat in LightAdobado, compare it with the camera JPEG and note the app commit or
package. Report the camera model, file type, and steps in the [LightAdobado issue tracker](https://github.com/smouj/LightAdobado/issues).

## Symptoms

- A raw photo looks flat, grey or desaturated as soon as you open it.
- The preview or thumbnail looks right, but the opened photo does not.
- Blacks look lifted, and the shadows look grey.

## Fix

1. Note the LightAdobado version or source commit.
2. If **Photo** > **Reload from Disk** is available, try it on a copy of the photo.
3. Compare the result with the camera JPEG.
4. If the difference remains, report it as described in [Get help with a problem](get-help.md).

## Why this happens

LightAdobado develops a raw from the sensor data and tries to match the look of the camera's own JPEG. How well it matches depends on the camera model.

- **Sony ARW:** on 0.5.0 and older, some Lossless Compressed M-size files had lifted blacks, and an A7 II file lost most of its colour. Both are fixed since 0.6.0. Some files still fall short on in-camera Dynamic Range Optimizer shots.
- **Dark, muted scenes:** LightAdobado applies only part of the camera look to them, so they can start duller than the camera JPEG. This is not fixed in 0.7.1 ([issue 865](https://github.com/smouj/LightAdobado/issues/865)).
- **Fujifilm X-Pro1 and Sony A7 IV:** reported dull on 0.4.0 and fine on 0.6.0.
- **Leica DNG:** these files carry a colour matrix but no tone curve, so LightAdobado shows them darker and flatter than the camera JPEG. This is not fixed in 0.7.1 ([pull request 807](https://github.com/smouj/LightAdobado/pull/807)).
- **Adobe RGB:** if the camera was set to Adobe RGB, 0.6.0 and older read the embedded JPEG as sRGB and matched to a too-dull target. Fixed in 0.7.1: LightAdobado reads the JPEG in the colour space the raw file states.
- **Black-and-white picture style:** a raw shot in a monochrome picture style started about 1.3 EV darker than the same camera's colour shots. Since 0.7.1, LightAdobado takes the tone from the camera's black-and-white JPEG.

## A soft picture after zooming in

If a large raw looks soft right after you zoom in, wait a few seconds. LightAdobado first shows the fit-size picture enlarged, then renders the zoomed area from the full-resolution raw and swaps it in. On a 61 MP file this can take a few seconds. If it stays soft at **Zoom 100%** (**View** > **Zoom 100%**), report it.

## Still stuck?

Open an issue in the [LightAdobado issue tracker](https://github.com/smouj/LightAdobado/issues).
