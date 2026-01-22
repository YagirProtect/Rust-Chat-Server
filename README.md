# Rust Chat Rooms (Tokio + TCP)

A small **client/server chat** written in Rust on top of **raw TCP**: **newline-delimited JSON packets**, a simple server **hub** with **rooms**, and a CLI client powered by **rustyline** (prompt + history).

This project is primarily a practical playground to learn:
- TCP stream framing (why “one packet == one line” matters)
- splitting a `TcpStream` into **reader/writer halves**
- coordinating async tasks via **Tokio mpsc channels**
- broadcasting to everyone in a room **without storing sockets** in the hub

<b>Created without vibecoding</b>

---

## Screenshots

<table>
  <tr>
    <td align="center" width="50%">
      <b>Client</b><br>
      <img src="/screenshots/Screenshot_1.png" width="95%" />
    </td>
    <td align="center" width="50%">
      <b>Server</b><br>
      <img src="/screenshots/Screenshot_2.png" width="95%" />
    </td>
  </tr>
</table>

---

## Features

### Client (CLI)
- `rustyline` prompt: `>> ` with **history** (arrow-up)
- Local command parsing + validation
- Background reader task → incoming packets channel → main `tokio::select!` loop
- Colored output + console clear (`crossterm`)
- “You:” echo with timestamp: `[HH:MM:SS] You: ...`

### Server
- Tokio TCP listener (one task per connection)
- Per-connection:
  - reader loop (`BufReader::lines()`) → incoming channel
  - writer loop → outgoing channel → socket write
- `ServerClientsHub`:
  - users + rooms
  - join/create rooms with size limit
  - broadcast room messages to everyone except sender
  - auto-cleanup: remove user from rooms and **delete empty rooms** on disconnect

### Protocol
- JSON `Packet { command, args }` (`serde_json`)
- **One JSON packet per line** (`\n`) → easy framing for both sides

---

## Quick start

### Run server
```bash
cargo run --bin server
```

### Run client
Open one (or multiple) terminals:
```bash
cargo run --bin client
```

Default address: `127.0.0.1:3000`

---

## Config (`config.json`)

Both binaries read/create `config.json` in the **current working directory**.

Example:
```json
{
  "user_name": "User",
  "address": "127.0.0.1:3000"
}
```

- Client uses `user_name` + `address`
- Server uses `address`

> Tip: if you want different configs for server and client, run them from different folders (or separate build/run dirs).

---

## Client commands

Anything that is **not** a command is treated as a message (only when connected + in room).

- `/help` — show commands
- `/connect [ip:port]` or `/cnt [ip:port]` — connect  
  (if omitted → uses `config.json` address)
- `/disconnect` — leave room or disconnect from server
- `/change_name <name>` — change local name (requires being disconnected)
- `/get_rooms` — request rooms list (table)
- `/create_room <name> <size>` — create room and auto-join
- `/join_room <name>` — join existing room
- `/clear` or `/cls` — clear console
- `/quit` or `/q` — quit

Examples:
```text
>> /connect
>> /get_rooms
>> /create_room lobby 8
>> hello
>> /disconnect
>> /quit
```

---

## Packet format (newline-delimited JSON)

One packet:
```json
{"command":"GetRooms","args":[]}
```

Writer always sends:
- `packet_json + "\n"`

Reader always does:
- `BufReader::lines()` → each line is one packet

---

## Project structure

- `src/bin/client.rs` — CLI entry point (select loop)
- `src/bin/server.rs` — server entry point (accept loop + per-client tasks)

### Shared
- `src/bin/shared_lib/c_command.rs` — `Packet` (serde JSON)
- `src/bin/shared_lib/c_commands_solver.rs` — `ECommand` + parsing
- `src/bin/shared_lib/c_config.rs` — config load/save (`config.json`)
- `src/bin/shared_lib/f_utils.rs` — timestamp helper (`chrono`)

### Client
- `src/bin/client_lib/classes/c_client.rs` — connect/disconnect, reader/writer loops, channels
- `src/bin/client_lib/cli_utils/f_rusty_line_input.rs` — `rustyline` + `ExternalPrinter`
- `src/bin/client_lib/cli_utils/f_print_utils.rs` — colored printing + clear console
- `src/bin/client_lib/f_from_local_client_parse.rs` — local commands handler
- `src/bin/client_lib/f_from_server_parse.rs` — server packets handler

### Server
- `src/bin/server_lib/c_hub.rs` — users + rooms hub
- `src/bin/server_lib/c_server_client.rs` — user state + outgoing sender
- `src/bin/server_lib/c_server_room.rs` — room model + members
- `src/bin/server_lib/f_server_logger.rs` — colored server logs

---

## Notes / limitations

- Learning project: no auth, no persistence, no encryption.
- Packet framing is line-based: JSON must stay on **one line**.

---

## License

MIT (or pick any license you want).
