# Words With Coworkers

![](https://travis-ci.com/Tyler-Zhang/words-with-coworkers-slack-bot.svg?token=6De4uXx8K7DcJuRoXez4&branch=master)

Feeling too productive at work recently? Play a nice game of scrabble on the company slack!

## How it looks

![](https://i.imgur.com/fGuHP6A.png)

## How to Install

The app is currently not an official slack bot, so it will need to be manually installed on your workspace.

### 1. Install emojipack

The scrabble board is rendered using emojis. You will have to install the emoji pack. Thankfully, there's a
easy utility to do so!

We use the [emojipacks cli tool](https://github.com/lambtron/emojipacks).

```bash
# Get the emojipacks file defined in this repo
wget https://raw.githubusercontent.com/Tyler-Zhang/words-with-coworkers-slack-bot/master/emojis.yml

# Install emojipacks cli tool
$ npm install -g emojipacks

$ emojipacks

Slack subdomain: 20percentclub
Email address login: andyjiang@gmail.com
Password: *********
2FA Code: 123456  #  if 2FA is enabled
Path or URL of Emoji yaml file: ./emojis.yml
```

### 2. Install the bot on your workspace

- Goto [https://api.slack.com/apps](https://api.slack.com/apps) and log in to your workspace
- Press Create New App
- Set **App Name** to **Words With Coworkers** and **Development Slack Workspace** to your workspace
- Go to Create New Command in the left hand side navigator And input the following values:
  - **command**: /scrabbler
  - **Request URL**: https://words-with-coworkers.herokuapp.com/slack
  - **Short Description**: Anything you want!
  - **Usage Hint**: help

![](https://i.imgur.com/1TLMxWV.png)


### 3. Start playing!

Boom, you're done! Go to your slack workspace and use the command `/scrabbler help` for instructions on how to play.

You can play for as long as my Heroku Dyno is up :D.

## Why rust?

Why not?

## Todos

- [ ] Make into an actual slack app (implement OAuth)
- [ ] Verify webhook signatures
- [ ] Make user interface easier to understand
- [ ] Feature to wipe all slack-bot messages after the game is done
