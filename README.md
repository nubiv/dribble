# Dribble

A Desktop version Peer-to-Peer WebRTC channel. No Signaling Server required 📡.

Please bear with the UI, currently it's just serving for the purpose of testing 😑.

## Instructions

1. On A:

- Click on the "createOffer" button. This will generate a code in the Local text box.
- Send the generated code to your remote pair B.

2. On B:

- Paste the received code into Remote text box.
- Click the "answer" button. This will generate a new code in the Local text box.
- Send this code to your remote pair A.

3. On A:

- Paste the received code into Remote text box.
- Click the "answer" button.

Now you should be able to see the P2P connection established. 🎉

## Credits

Inspired by [serverless-webrtc](https://github.com/cjb/serverless-webrtc/tree/master) 🫡.
