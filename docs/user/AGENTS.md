# User docs: writing and review rules

These guides belong to LightAdobado in Adobado Kit. Write for a photographer trying to complete a
task. Keep LightCraft names only when explaining upstream history, source identifiers, or a behavior
that was actually reported upstream.

## Page rules

- Lead with the answer or next useful step.
- Use plain language, short paragraphs, and numbered steps for actions.
- Match menu and control labels to the current app.
- Link to related guides instead of repeating them.
- Never present an upstream release, launcher, support channel, or feature as a LightAdobado release.
- The current LightAdobado build is not yet validated for everyday use. Label inherited instructions
  as upstream and say when they have not been checked against a tested Adobado package.
- Send bug reports to the [LightAdobado issue tracker](https://github.com/smouj/LightAdobado/issues).
  The [Adobado Kit website](https://smouj.github.io/Adobado_Kit/) is the family hub, not a support forum.
- Do not promise support through a community service unless the project publishes that channel.

## Frontmatter

    ---
    title: Raw photos look dull
    description: One sentence for search results and link previews.
    order: 10                 # optional
    verified: 0.1.0-alpha.1   # only when checked against this LightAdobado release
    ---

Omit verified when no LightAdobado release is available or the page has not been checked against it.
Do not use a LightCraft version as LightAdobado's verified version.

## Before publishing

- Check links and UI labels.
- Confirm every product claim against current source or a tested package.
- Keep upstream issue and pull-request links only when they explain upstream provenance.
- Check the current README for honest installation and test status.
