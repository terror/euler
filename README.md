## euler

<div>
  <img align='right' width='100px' src='https://www.bbvaopenmind.com/wp-content/uploads/2018/04/Euler-1-dentro.jpg'/>
</div>

**euler** is a discord bot for the William's Math Team™ discord server.

### Features

- Query ChatGPT
- Grab McGill course information
- Train on random Leetcode problems

### Development

You'll need [cargo](https://doc.rust-lang.org/cargo/) installed on your machine
to spawn the various components the project needs to run locally.

1. Create a new discord application in the
[developer portal](https://discord.com/developers/docs/intro), and then
navigate to the *Bot* tab to grab your bot token.

Additionally, you'll want to check the *Message Content Intent* present within
the *Bot* interface.

2. Add the bot to your server by navigating to the following url in your
browser:

```
https://discord.com/api/oauth2/authorize?client_id=<CLIENT_ID>&permissions=0&scope=bot%20applications.commands
```

Where `CLIENT_ID` is found by navigating to the *OAuth2* tab in the developer
portal.

3. Set the following environment variables:

```
DISCORD_BOT_TOKEN=
OPENAI_API_KEY=
```

Where `DISCORD_BOT_TOKEN` is the token you grabbed in the first step.

*n.b. Your OpenAI key is required if you'd like for ChatGPT queries to work.*

4. Run the bot with an appropriate log level set:

```
RUST_LOG=info cargo run
```
