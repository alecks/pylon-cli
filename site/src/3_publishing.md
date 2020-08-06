# Publishing

Once everything's set up, you can start publishing with the CLI. Make sure you're in the directory of a properly-configured project before trying to publish.

## Editing the code

> All changes made with the CLI will overwrite _all_ changes made with the online editor, and vice-versa. See also: [Why are my changes overwritten?](./publishing__changes_overwritten.md).

Your bot's code is located in the `src` folder. Make sure, however, that you're in the root of the project; the CLI will look for `Pylon.toml` and `PylonSecrets.toml` in the _current_ directory. Open up `src/main.ts` and start editing!

If you're using an editor like [Visual Studio Code](https://code.visualstudio.com), you'll have instant linting and autocompletions. This is the result of having comments like these:

```ts
/// <reference types="@pylonbot/runtime" />
/// <reference types="@pylonbot/runtime-discord" />
```

They tell TypeScript that you're writing a Pylon project. You only need to keep them in `main.ts`; the rest of the project should inherit them.

## Saving and publishing your code

Once you're done editing, you can run `pylon publish`. This will build your project into a bundle - see `dist/bundle.js` - and send it off to Pylon. Error messages will be shown too, if you don't have a fancy code editor.
