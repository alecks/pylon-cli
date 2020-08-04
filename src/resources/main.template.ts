/// <reference types="@pylonbot/runtime" />
/// <reference types="@pylonbot/runtime-discord" />

// Hey! Welcome to your new Pylon bot.
// Read the Pylon documenration at https://pylon.bot/docs/intro.
// The Pylon CLI documentation is available at https://pylon.alex.lgbt.

// Make sure not to remove the special comments above. These tell TypeScript that you're writing a Pylon script.
// On top of this, editing your script in the Pylon editor will overwite *all* changes behind the scenes. Be careful!

const commands = new discord.command.CommandGroup({
  defaultPrefix: "!",
  mentionPrefix: true,
});

commands.on(
  "hi",
  (ctx) => ({ member: ctx.guildMember() }),
  async (msg, { member }) => {
    msg.reply(`hi, ${member.user.getTag()}!`);
  }
);
