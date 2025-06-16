

Bun.serve({
  fetch(req, server) {
    if (server.upgrade(req)) {
      return;
    }

    return new Response("Upgrade failed", { status: 500 });
  },

  websocket: {
    message(ws, message) {
      console.log("Message:", message.toString());
      ws.send("Bun")
    },
    open(_ws) {
      console.log("Open");
    },
    close(_ws, _code, _message) {
      console.log("Close");
    },
    drain(_ws) {},
  },
});
