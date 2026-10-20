---
title: A raw photo opens as "Preview only"
description: What Preview only means in the upstream LightCraft raw-photo workflow; current LightAdobado behavior is still being validated.
order: 10
---

> **Upstream note:** This guide describes the LightCraft 0.7.1 raw-photo workflow. It has not been
> checked against a tested LightAdobado package.

If a raw photo shows **Preview only** in a LightAdobado build, use **Photo** > **Reload from Disk**
only if that command is present, then report the camera model and raw variant in the
[LightAdobado issue tracker](https://github.com/smouj/LightAdobado/issues).

## Symptoms

- The **Edit** panel shows **Preview only**, with the text "LightAdobado can't decode this raw variant yet" and the name of the variant.
- You updated LightAdobado to a version that supports your camera, and the photo still looks the same.

## Fix

1. In the grid, select the photos that show **Preview only**.
2. Choose **Photo** > **Reload from Disk**. The right-click menu on a photo has the same entry.
3. Open a photo again. If **Preview only** is gone, the photo is now decoded from the raw data.

If **Preview only** stays, the current build may not decode that raw variant. Report the camera
model, raw variant, app commit or package, and a sample file only if you can share it safely. See
[Get help with a problem](get-help.md).

## Why this happens

When LightAdobado cannot decode a raw on import, it remembers that and shows the camera's embedded JPEG. It does not check again after an update on its own. **Reload from Disk** is the way to make it check. Compressed Olympus raws, some Canon CR3 variants and Nikon High Efficiency NEF files are examples of files that use the embedded preview.

Upstream LightCraft 0.7.1 added decoding for Canon EOS R5 Mark II C-RAW files. That release note is
historical and does not confirm support in LightAdobado.

Photos shown from the preview keep the camera's picture style and white balance, because they are the camera's own JPEG.

## Still stuck?

Open an issue in the [LightAdobado issue tracker](https://github.com/smouj/LightAdobado/issues).
