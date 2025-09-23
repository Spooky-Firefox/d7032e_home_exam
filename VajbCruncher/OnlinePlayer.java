// OnlinePlayer.java

import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.io.ObjectInputStream;
import java.io.ObjectOutputStream;
import java.net.Socket;

/**
 * Minimal online-capable player. The server calls setConnection(...) after
 * accepting a socket. If no connection is set, it falls back to console I/O.
 */
public class OnlinePlayer extends Player {

    // Network I/O (null when offline)
    private Socket socket;
    private ObjectOutputStream out;
    private ObjectInputStream in;

    public OnlinePlayer() {
        super();
    }

    /**
     * Used by Server to wire up a freshly accepted socket and streams.
     */
    public void setConnection(Socket sock, ObjectInputStream in, ObjectOutputStream out) {
        this.socket = sock;
        this.in = in;
        this.out = out;
    }

    /**
     * Optional: close the connection cleanly (server may never need this).
     */
    public void closeConnection() {
        try {
            if (socket != null)
                socket.close();
        } catch (Exception ignored) {
        }
        socket = null;
        in = null;
        out = null;
    }

    /**
     * Send a message to this player. If connected, goes over the socket;
     * otherwise prints to local console.
     */
    @Override
    public void sendMessage(Object msg) {
        if (out != null) {
            try {
                out.writeObject(String.valueOf(msg));
                out.flush();
                out.reset(); // prevent memory leak
            } catch (Exception e) {
                System.err.println("[OnlinePlayer] send error: " + e.getMessage());
            }
        } else {
            // fallback to console
            System.out.println(msg);
        }
    }

    /**
     * Receive a line of input from this player. If connected, reads from the
     * socket;
     * otherwise reads from local console.
     */
    @Override
    public String receiveMessage() {
        if (in != null && out != null) {
            try {
                Object o = in.readObject();
                return (o == null) ? null : o.toString();
            } catch (Exception e) {
                System.err.println("[OnlinePlayer] receive error: " + e.getMessage());
                return null;
            }
        } else {
            // fallback to local console
            System.out.print("> ");
            try {
                BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
                return br.readLine();
            } catch (Exception e) {
                return null;
            }
        }
    }
}