# Mastodon Plays Snake

Now in rust! This project aims to bring [Polymars' Twitter Plays Snake](https://github.com/PolyMarsDev/Twitter-Plays-Snake) to Mastodon. Users can collaborate on a game of snake by voting for its next move on each turn. It updates every 30 minutes.

Bot: [@snake@mastodon.world](https://mastodon.world/@snake)

# Hosting

> [!NOTE]  
> Rehosting the bot is fine if it is for a private unfederated instance or will have its own unique spin on the concept but otherwise please avoid rehosting.

It is built and ran just like any other rust project but it relies on some environment variables:
- `INSTANCE` - Link to mastodon instance(eg [https://mastodon.world/](https://mastodon.world/]))
- `ACCESS_TOKEN` - Access token you get when creating and allowing an application to access your mastodon account
- `ID` - Account id(eg 113863635165154030)
