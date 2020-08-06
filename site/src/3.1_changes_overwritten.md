# Why are my changes overwritten?

> This will be fixed sooner or later; it's not unfixable.

When you publish something with Pylon's online editor, using the CLI will overwrite its changes. It also goes the other way: editing something online will overwrite the CLi's changes.

This is pretty inconvenient, yeah, but it makes sense when you learn about how Pylon works. When you publish something, you're sending JSON to the API including a map of your entire project's _TypeScript_ files, but a single string for the actual code that's executed - the bundle. So yes, this means that the only reason you give Pylon your TS files is to save them for when you come back.

Since the CLI is local and you save your project to disk, there's no reason to send Pylon your actual project. All that's sent is the bundle and, in `main.ts`, the value of your configuration's `publish.main_content`.

Publishing something online obviously sends a bundle too; since there's only ever one bundle file, the others' changes are _fully_ overwritten. It's recommended that you don't change the warning in `publish.main_content` too much; you don't want to lose all your changes. Oh, and **make sure to check that there aren't any files in the online editor that you care about before using the CLI**.
