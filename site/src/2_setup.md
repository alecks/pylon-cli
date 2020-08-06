# Setup

This should show you how to create and configure a Pylon project. It assumes that you have the CLI installed.

> The name of the command you'll be using is `pylon`, **not** `pylon-cli`.

## Getting your deployment ID and API token

First of all, you'll need an existing Pylon deployment. You probably already have one, but if you don't, it's simple:

1. login at https://pylon.bot;
2. add the bot to your server;
3. open "Default Script" in your server's Pylon dashboard.

Once you have the editor open, what you want is the ID in the URL. This is your _deployment ID_; you won't need a script ID. If the URL of your deployment is `https://pylon.bot/studio/deployments/119708358260293632/editor`, the ID you need is `119708358260293632`. The next thing you need is your API token. This is slightly tricky to get, but keep reading:

1. in the editor, open up the developer tools (inspect element);
2. here, open the "application" tab;
3. find "local storage" then open "https://pylon.bot";
4. copy the value next to "token".

If you're still lost, here's a video:

<iframe width="560" height="315" src="https://www.youtube.com/embed/ISXK-3umZ1w" frameborder="0" allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

## Creating a project

That's the last you'll see of the online editor! Open up your terminal again. The CLI has a handy `init` subcommand:

```console
pylon init my-project
```

This will create a project with a starter template, installing all required dependencies. Make sure to install [npm](https://npmjs.com) if you haven't already. You can really use whatever editor you want, but [Visual Studio Code](https://code.visualstudio.com) is recommended; it has great TypeScript integration.

As of now, the npm packages for SDK typings are outdated; don't worry if you're flooded with errors! You should download the newest typings from [here](https://pylon.bot/runtime-typings/0.0.1/pylon-runtime.d.ts) and [here](https://pylon.bot/runtime-typings/0.0.1/pylon-runtime-discord.d.ts). Once you've got them, replace the files in `node_modules/@pylonbot/runtime` and `node_modules/@pylonbot/runtime-discord` respectively.

## Configuring your project

You need to configure your project before publishing.

### Pylon.toml

Now you have a starter project, you'll have to put the values you got from the editor in the config. You'll find a file called `Pylon.toml`; open this up! The only thing you need to edit here is the `deployment_id` value. Just replace it with what you got in the URL.

### PylonSecrets.toml

The second config file is `PylonSecrets.toml`. This is where you'll keep your token, since leaking it is giving people access to your entire account. Opening it up, you'll see the `token` option; replace this with what you got from the developer tools.
