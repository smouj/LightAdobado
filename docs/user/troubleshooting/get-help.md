---
title: Get help with a problem
description: What to include when you ask for help with a LightAdobado bug, and where to post it.
order: 30
---

Open an issue in the [LightAdobado issue tracker](https://github.com/smouj/LightAdobado/issues). Include your app version, system, steps, and expected result. Current Adobado builds are still being validated; report the exact commit or package you used.

## What to include

1. **Your version.** Choose **Help** > **About LightAdobado** (on macOS, **LightAdobado** > **About LightAdobado**).
2. **Your system.** For example Windows 11, 64-bit. **Help** > **System Info…** shows details about your computer.
3. **The steps.** What you clicked, in order, and what happened.
4. **The camera and file type**, if the problem is with a photo.
5. **A sample file**, if a raw or other photo renders wrong. Maintainers ask for the original file, not an export. A link to a shared folder works. Uploading raws to [raw.pixls.us](https://raw.pixls.us) also helps, because every file there becomes a permanent test sample for that camera.
6. **A screenshot** of LightAdobado next to the camera JPEG or the other app, for colour problems.
7. **The log file.** Choose **Help** > **Open Log Folder** and attach `lightcraft.log`.

## Where the log is

LightAdobado writes its log to `logs/lightcraft.log` in its settings folder. On Windows that is `%APPDATA%\LightAdobado\logs\`. Each start moves the previous log to `lightcraft.1.log`, and that one to `lightcraft.2.log`, so the log of a run that crashed survives the next start.

## Still stuck?

If you need more help, add a comment to the same issue so the details stay together.
