<h1 style="color:#814EEF;text-shadow: -2px 2px #360D8D;font-size:40px", align="center">Nearssage</h1>
<h3 align="center">Talk with nearby people 📍</h3>
An app I envisioned answering to a demanding necessity I've noticed in my environment, people  (at least where I live) don't strive to socialize even when meeting, they rather spend their time on social media instead of talking in real life. Fun experience: people in a terrace chatting on Instagram with someone sat a few tables away.

## <a name="what"></a>What is this? 🧐
Well, an app to chat and **meet** people nearby you. But, **why near you** and not worldwide? Because this **isn't a social messaging app made to hook you**, is an app to meet people in the real world.

## <a name="design"></a>Design 😏
All messaging app should be transparent on how they work, here is a brief description:
### <a name="designClient"></a>Client
- Chat (E2EE)
  - `X3DH + CSIDH` - (`Blake3`) *Exploring to remove `CSIDH` as it may have no cryptographic benefits*
  - `Double Ratchet` - *Yet to do*

### <a name="designServer"></a>Server
- Binary protocol
  - Key agreement - `ECDH`
  - Encryption - `ChaCha20`
  - Integrity - `CRC32`
  - Replay attack mitigation
- `UDP` Transport
- Embedded key-value ~~distributed~~ database

You can check out this by yourself!

## <a name="todo"></a>What remains? 🌅
- [ ] Figure out why is SEGFAULTING
- [ ] Server's endpoint
- [ ] App
