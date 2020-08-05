# Setup

This should show you how to create and configure a Pylon project. It assumes that you have the CLI installed.

> ### Note
>
> The name of the command you'll be using is `pylon`, **not** `pylon-cli`.

## Creating the project

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
